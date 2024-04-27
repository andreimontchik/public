use {
    crate::{
        message_filter::MessageFilter,
        processor::{noop_processor::NoopProcessor, ProcessorManager},
        read_from_file, to_pubkey, AsyncPluginError, Message,
    },
    log::{debug, info},
    serde_derive::Deserialize,
    serde_json,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoV3, ReplicaAccountInfoVersions, Result,
    },
    solana_sdk::{clock::Slot, pubkey::Pubkey},
    std::{
        fmt,
        sync::mpsc::{self, Sender},
    },
    thiserror::Error,
};

impl From<AsyncPluginError> for GeyserPluginError {
    fn from(err: AsyncPluginError) -> Self {
        GeyserPluginError::Custom(Box::new(err))
    }
}

#[derive(Error, Debug, Deserialize)]
pub struct PluginConfig {
    pub processor: String,
    pub owners: Vec<AccountConfig>,
    pub accounts: Vec<AccountConfig>,
}

impl PluginConfig {
    pub fn load(config_file_name: &str) -> Self {
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}
impl fmt::Display for PluginConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountConfig {
    name: String,
    address: String,
}

#[derive(Debug)]
pub struct AsyncPlugin {
    sender: Sender<Message>,
    processor_manager: ProcessorManager,
    message_filter: MessageFilter,
}

impl AsyncPlugin {
    fn new() -> AsyncPlugin {
        let (sender, receiver) = mpsc::channel::<Message>();
        AsyncPlugin {
            sender,
            processor_manager: ProcessorManager::new(receiver),
            message_filter: MessageFilter::new(),
        }
    }

    fn send_message(&self, msg: Message) -> crate::Result<()> {
        self.sender
            .send(msg)
            .map_err(|err| AsyncPluginError::FailedToSendMessage { err: err.to_string() })
    }

    fn handle_account_update(&self, slot: Slot, msg: &ReplicaAccountInfoV3) -> crate::Result<()> {
        debug!(
            "Handling account update. Slot: {}, Owner: {:?}, Address: {:?}",
            slot,
            Pubkey::try_from(msg.owner),
            Pubkey::try_from(msg.pubkey)
        );

        if !self.message_filter.is_registered(msg.owner, msg.pubkey) {
            return Ok(());
        }

        let address = Pubkey::try_from(msg.pubkey).map_err(|_| AsyncPluginError::InvalidPubKey {
            msg: format!("{:?}", msg),
        })?;

        self.send_message(Message::AccountUpdate {
            slot,
            address: address.clone(),
            data: msg.data.to_vec(),
            txn_signature: msg.txn.map(|txn| *txn.signature()),
        })
    }
}

impl GeyserPlugin for AsyncPlugin {
    fn name(&self) -> &'static str {
        "AsyncPlugin"
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn on_load(&mut self, config_file: &str) -> Result<()> {
        solana_logger::setup_with_default("info");

        info!("Loading the Plugin configuration from '{}'.", config_file);
        let config = PluginConfig::load(config_file);

        self.processor_manager.start::<NoopProcessor>(config.processor);

        // Register owners
        for owner in config.owners {
            let address = to_pubkey(&owner.address)?;
            self.message_filter.add_owner(&address);
            self.send_message(Message::OwnerInfo {
                name: owner.name,
                address,
            })?;
        }

        // Register accounts
        for account in config.accounts {
            let address = to_pubkey(&account.address)?;
            self.message_filter.add_account(&address);
            self.send_message(Message::AccountInfo {
                name: account.name,
                address,
            })?;
        }

        Ok(())
    }

    fn on_unload(&mut self) {
        self.processor_manager.stop();
    }

    fn update_account(
        &self,
        msg_wrapper: ReplicaAccountInfoVersions,
        slot: Slot,
        _is_startup: bool,
    ) -> Result<()> {
        match msg_wrapper {
            ReplicaAccountInfoVersions::V0_0_1(_) => Ok(()), // Ignore
            ReplicaAccountInfoVersions::V0_0_2(_) => Ok(()), // Ignore
            ReplicaAccountInfoVersions::V0_0_3(msg) => Ok(self.handle_account_update(slot, msg)?),
        }
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPluginPostgres pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    Box::into_raw(Box::new(AsyncPlugin::new()))
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::NamedTempFile};

    #[test]
    fn test_plugin_config() {
        const PROCESSOR_CFG: &str = "Processor configuration";
        const OWNER_NAME: &str = "Owner Name";
        let owner_address = Pubkey::new_unique().to_string();
        const ACCOUNT_NAME: &str = "Account Name";
        let account_address = Pubkey::new_unique().to_string();
        let config_str = format!(
            r#"
        {{
            "libpath": "Dummy path",
            "processor":"{}",
            "owners": [{{
                "name": "{}",
                "address": "{}"
            }}],
            "accounts": [{{
                "name": "{}",
                "address": "{}"
            }}]
    }}"#,
            PROCESSOR_CFG, OWNER_NAME, owner_address, ACCOUNT_NAME, account_address
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = PluginConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.processor, PROCESSOR_CFG);
        assert_eq!(config.owners.len(), 1);
        assert_eq!(config.owners[0].name, OWNER_NAME);
        assert_eq!(config.owners[0].address, owner_address);
        assert_eq!(config.accounts.len(), 1);
        assert_eq!(config.accounts[0].name, ACCOUNT_NAME);
        assert_eq!(config.accounts[0].address, account_address);
    }
}
