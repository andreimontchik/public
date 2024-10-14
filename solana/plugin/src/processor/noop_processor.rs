use {
    super::{Processor, Result},
    crate::{AccountInfoMessage, AccountUpdateMessage},
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

    fn add_account(&mut self, msg: &AccountInfoMessage) -> Result<()> {
        info!("Adding ({:?}).", msg);
        Ok(())
    }

    fn update_account(&mut self, msg: &AccountUpdateMessage) -> Result<()> {
        info!("Updating ({:?}).", msg);
        Ok(())
    }
}
