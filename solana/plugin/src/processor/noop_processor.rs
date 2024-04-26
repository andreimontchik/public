use {
    super::{Processor, ProcessorError, Result},
    crate::Message,
    log::info,
};

pub struct NoopProcessor {}

impl Processor for NoopProcessor {
    fn new(config_file_name: &str) -> Self
    where
        Self: Sized,
    {
        info!(
            "Creating the NoopProcessor with the following configuration: ({}).",
            config_file_name
        );
        NoopProcessor {}
    }

    fn add_owner(&mut self, msg: &Message) -> Result<()> {
        if let Message::OwnerInfo { .. } = msg {
            info!("Processing the OwnerInfo message ({}).", &msg);
            Ok(())
        } else {
            Err(ProcessorError::InvalidMessageType { msg: msg.to_string() })
        }
    }

    fn add_account(&mut self, msg: &Message) -> Result<()> {
        if let Message::AccountInfo { .. } = msg {
            info!("Processing the AccountInfo message ({}).", &msg);
            Ok(())
        } else {
            Err(ProcessorError::InvalidMessageType { msg: msg.to_string() })
        }
    }

    fn update_account(&mut self, msg: &Message) -> Result<()> {
        if let Message::AccountUpdate { .. } = msg {
            info!("Processing the AccountUpdate message ({}).", &msg);
            Ok(())
        } else {
            Err(ProcessorError::InvalidMessageType { msg: msg.to_string() })
        }
    }
}
