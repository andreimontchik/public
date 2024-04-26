pub mod noop_processor;

use {
    crate::Message,
    log::{error, info},
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::Receiver,
            Arc, Mutex,
        },
        thread, time,
    },
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("({msg})")]
    InvalidMessageType { msg: String },
}

pub type Result<T> = std::result::Result<T, ProcessorError>;

pub trait Processor {
    fn new(config_file_name: &str) -> Self
    where
        Self: Sized;

    fn add_owner(&mut self, msg: &Message) -> Result<()>;
    fn add_account(&mut self, msg: &Message) -> Result<()>;
    fn update_account(&mut self, msg: &Message) -> Result<()>;

    fn process(&mut self, msg: &Message) -> Result<()> {
        match msg {
            Message::OwnerInfo { .. } => self.add_owner(msg),
            Message::AccountInfo { .. } => self.add_account(msg),
            Message::AccountUpdate { .. } => self.update_account(msg),
        }
    }
}

#[derive(Debug)]
pub struct ProcessorManager {
    receiver: Arc<Mutex<Receiver<Message>>>,
    receiver_thread_handle: Option<thread::JoinHandle<()>>,
    should_run: Arc<AtomicBool>,
}

impl ProcessorManager {
    pub fn new(rcv: Receiver<Message>) -> ProcessorManager {
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
