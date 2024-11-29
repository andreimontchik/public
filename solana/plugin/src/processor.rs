pub mod noop_processor;

use {
    crate::{AccountInfoMessage, AccountUpdateMessage, Messages},
    anyhow::Result,
    log::{error, info},
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::Receiver,
            Arc, Mutex,
        },
        thread, time,
    },
};

pub trait Processor {
    fn new(config_file_name: &str) -> Self
    where
        Self: Sized;

    fn add_account(&mut self, msg: &AccountInfoMessage) -> Result<()>;
    fn update_account(&mut self, msg: &AccountUpdateMessage) -> Result<()>;

    fn process(&mut self, msgs: &Messages) -> Result<()> {
        match msgs {
            Messages::AccountInfo(msg) => self.add_account(msg),
            Messages::AccountUpdate(msg) => self.update_account(msg),
        }
    }
}

#[derive(Debug)]
pub struct ProcessorManager {
    receiver: Arc<Mutex<Receiver<Messages>>>,
    receiver_thread_handle: Option<thread::JoinHandle<()>>,
    should_run: Arc<AtomicBool>,
}

impl ProcessorManager {
    pub fn new(rcv: Receiver<Messages>) -> ProcessorManager {
        ProcessorManager {
            receiver: Arc::new(Mutex::new(rcv)),
            receiver_thread_handle: None,
            should_run: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start<T: Processor>(&mut self, processor_config: String) {
        info!("Starting ProcessorManager...");
        let receiver_handle = Arc::clone(&self.receiver);
        let should_run = Arc::clone(&self.should_run);
        let receiver_thread_handle = std::thread::spawn(move || {
            let mut processor: Box<dyn Processor> = Box::new(T::new(&processor_config));
            info!("Entering the Processor loop.");
            while should_run.load(Ordering::Relaxed) {
                match receiver_handle.lock() {
                    Ok(receiver) => match receiver.recv() {
                        Ok(msg) => {
                            if let Err(error) = processor.process(&msg) {
                                error!("Failed to process message ({})! {}", msg, error)
                            }
                        }
                        Err(error) => error!("Failed to receive message! {}", error),
                    },
                    Err(error) => error!("Failed to obtain receiver! {}", error),
                }
            }
            info!("Exited the Processor loop.");
        });

        // Check is the Processor thread took off.
        thread::sleep(time::Duration::from_secs(5));
        if receiver_thread_handle.is_finished() {
            error!("The Processor thread finished unexpectedy!");
            if let Err(panic) = receiver_thread_handle.join() {
                panic!("The Processor thread panicked! {:?}", panic);
            }
        } else {
            self.receiver_thread_handle = Some(receiver_thread_handle);
        }

        info!("The ProcessorManager started.");
    }

    pub fn stop(&mut self) {
        info!("Stopping the ProcessorManager...");
        self.should_run.store(false, Ordering::Relaxed);
        if let Some(handle) = self.receiver_thread_handle.take() {
            if let Err(err) = handle.join() {
                error!("Error joining the Channel Receiver thread handle {:?}", err);
            }
        }
        info!("ProcessorManager stopped.");
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        anyhow::bail,
        solana_sdk::{pubkey::Pubkey, signature::Signature},
    };
    struct MockedProcessor {
        add_account_called: bool,
        add_owner_called: bool,
        process_account_udpate_called: bool,
    }

    impl Processor for MockedProcessor {
        fn new(_config: &str) -> Self
        where
            Self: Sized,
        {
            MockedProcessor {
                add_account_called: false,
                add_owner_called: false,
                process_account_udpate_called: false,
            }
        }

        fn add_account(&mut self, _msg: &AccountInfoMessage) -> Result<()> {
            self.add_account_called = true;
            Ok(())
        }
        fn update_account(&mut self, _msg: &AccountUpdateMessage) -> Result<()> {
            self.process_account_udpate_called = true;
            Ok(())
        }
    }

    struct MockedFaultyProcessor {}

    impl Processor for MockedFaultyProcessor {
        fn new(_config: &str) -> Self
        where
            Self: Sized,
        {
            Self {}
        }

        fn add_account(&mut self, _msg: &AccountInfoMessage) -> Result<()> {
            bail!("Unrecognised account: ({})!", Pubkey::new_unique())
        }
        fn update_account(&mut self, _msg: &AccountUpdateMessage) -> Result<()> {
            bail!("Unrecognised account: ({})!", Pubkey::new_unique())
        }
    }

    #[test]
    fn test_process() {
        let mut processor = MockedProcessor::new("");
        assert!(!processor.add_owner_called);
        assert!(!processor.add_account_called);
        assert!(!processor.process_account_udpate_called);

        // Add account
        processor.add_owner_called = false;
        assert!(processor
            .add_account(&AccountInfoMessage {
                name: "Dummy account".to_string(),
                address: Pubkey::new_unique()
            })
            .is_ok());
        assert!(!processor.add_owner_called);
        assert!(processor.add_account_called);
        assert!(!processor.process_account_udpate_called);

        // Process account update
        processor.add_account_called = false;
        assert!(processor
            .update_account(&AccountUpdateMessage {
                slot: 1,
                address: Pubkey::new_unique(),
                data: Vec::<u8>::new(),
                txn_signature: Some(Signature::new_unique()),
            })
            .is_ok());
        assert!(!processor.add_owner_called);
        assert!(!processor.add_account_called);
        assert!(processor.process_account_udpate_called);
    }

    #[test]
    fn test_failed_add_owner() {
        let msg = &&Messages::AccountInfo(AccountInfoMessage {
            name: "Dummy owner".to_string(),
            address: Pubkey::new_unique(),
        });

        let mut processor = MockedFaultyProcessor {};
        match processor.process(msg) {
            Ok(_) => panic!("Unexpected processing result!"),
            Err(_) => (),
        };
    }

    #[test]
    fn test_failed_add_account() {
        let msg = &&Messages::AccountInfo(AccountInfoMessage {
            name: "Dummy account".to_string(),
            address: Pubkey::new_unique(),
        });

        let mut processor = MockedFaultyProcessor {};
        match processor.process(msg) {
            Ok(_) => panic!("Unexpected processing result!"),
            Err(_) => (),
        };
    }

    #[test]
    fn test_failed_account_update() {
        let address = Pubkey::new_unique();
        let msg = Messages::AccountUpdate(AccountUpdateMessage {
            slot: 1,
            address,
            data: Vec::<u8>::new(),
            txn_signature: Some(Signature::new_unique()),
        });

        let mut processor = MockedFaultyProcessor {};
        match processor.process(&msg) {
            Ok(_) => panic!("Unexpected processing result!"),
            Err(_) => (),
        }
    }
}
