use {
    crate::{
        message_filter::MessageFilter,
        processor::{noop_processor::NoopProcessor, ProcessorManager},
        read_from_file, to_pubkey, AccountUpdateMessage, Messages,
    },
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoV3, ReplicaAccountInfoVersions,
        Result as GeyserPluginResult,
    },
    anyhow::Result,
    log::{debug, info},
    serde_derive::Deserialize,
    serde_json,
    solana_sdk::{clock::Slot, pubkey::Pubkey},
    std::{
        fmt,
        sync::mpsc::{self, Sender},
    },
};

// TODO: separate accounts configuration from the plugin configuration
#[derive(Debug, Deserialize)]
pub struct PluginConfig {
    pub processor: String,
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
    sender: Sender<Messages>,
    processor_manager: ProcessorManager,
    message_filter: MessageFilter,
}

impl AsyncPlugin {
    fn new() -> AsyncPlugin {
        let (sender, receiver) = mpsc::channel::<Messages>();
        AsyncPlugin {
            sender,
            processor_manager: ProcessorManager::new(receiver),
            message_filter: MessageFilter::new(),
        }
    }

    fn send_message(&self, msg: Messages) -> Result<()> {
        Ok(self.sender.send(msg)?)
    }

    fn handle_account_update(&self, slot: Slot, msg: &ReplicaAccountInfoV3) -> Result<()> {
        debug!(
            "Handling account update. Slot: {}, Owner: {:?}, Address: {:?}",
            slot,
            Pubkey::try_from(msg.owner),
            Pubkey::try_from(msg.pubkey)
        );

        if !self.message_filter.is_registered(msg.pubkey) {
            return Ok(());
        }

        let address = Pubkey::try_from(msg.pubkey)?;

        self.send_message(Messages::AccountUpdate(AccountUpdateMessage {
            slot,
            address: address.clone(),
            data: msg.data.to_vec(),
            txn_signature: msg.txn.map(|txn| *txn.signature()),
        }))
    }
}

impl GeyserPlugin for AsyncPlugin {
    fn name(&self) -> &'static str {
        "AsyncPlugin"
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> GeyserPluginResult<()> {
        solana_logger::setup_with_default("info");

        info!("Loading the Plugin configuration from '{}'.", config_file);
        let config = PluginConfig::load(config_file);

        self.processor_manager.start::<NoopProcessor>(config.processor);

        // Register accounts
        for account in config.accounts {
            let address =
                to_pubkey(&account.address).map_err(|error| GeyserPluginError::Custom(error.into()))?;

            self.message_filter.add_account(&address);
            self.send_message(Messages::AccountInfo(crate::AccountInfoMessage {
                name: account.name,
                address,
            }))
            .map_err(|error| GeyserPluginError::Custom(error.into()))?;
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
    ) -> GeyserPluginResult<()> {
        match msg_wrapper {
            ReplicaAccountInfoVersions::V0_0_1(_) => Ok(()), // Ignore
            ReplicaAccountInfoVersions::V0_0_2(_) => Ok(()), // Ignore
            ReplicaAccountInfoVersions::V0_0_3(msg) => self
                .handle_account_update(slot, msg)
                .map_err(|error| GeyserPluginError::Custom(error.into())),
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
        const ACCOUNT1_NAME: &str = "Account1 Name";
        let account1_address = Pubkey::new_unique().to_string();
        const ACCOUNT2_NAME: &str = "Account2 Name";
        let account2_address = Pubkey::new_unique().to_string();
        let config_str = format!(
            r#"
        {{
            "libpath": "Dummy path",
            "processor":"{}",
            "accounts": [
            {{
                "name": "{}",
                "address": "{}"
            }},
            {{
                "name": "{}",
                "address": "{}"
            }}]
        }}"#,
            PROCESSOR_CFG, ACCOUNT1_NAME, account1_address, ACCOUNT2_NAME, account2_address
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = PluginConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.processor, PROCESSOR_CFG);
        assert_eq!(config.accounts.len(), 2);
        assert_eq!(config.accounts[0].name, ACCOUNT1_NAME);
        assert_eq!(config.accounts[0].address, account1_address);
        assert_eq!(config.accounts[1].name, ACCOUNT2_NAME);
        assert_eq!(config.accounts[1].address, account2_address);
    }
}
