mod message_filter;
mod plugin;
mod processor;

use {
    solana_sdk::{clock::Slot, pubkey::Pubkey, signature::Signature},
    std::{fmt, fs::File, io::Read, str::FromStr},
    thiserror::Error,
};

type AddressType = [u8; 32];

#[derive(Error, Debug)]
pub enum AsyncPluginError {
    #[error("({msg})")]
    InvalidConfiguration { msg: String },
    #[error("({msg})")]
    InvalidPubKey { msg: String },
    #[error("({msg})")]
    ProcessorError { msg: String },
    #[error("({msg})")]
    CalculationError { msg: String },
    #[error("({code})")]
    InvalidAccountType { code: String },
    #[error("({code})")]
    InvalidToken { code: String },
    #[error("({code})")]
    InvalidLiquidityGroup { code: String },
    #[error("{err}")]
    FailedToSendMessage { err: String },
}

pub type Result<T> = std::result::Result<T, AsyncPluginError>;

#[derive(Debug, PartialEq)]
pub enum Message {
    OwnerInfo {
        name: String,
        address: Pubkey,
    },
    AccountInfo {
        name: String,
        address: Pubkey,
    },
    AccountUpdate {
        slot: Slot,
        address: Pubkey,
        data: Vec<u8>,
        txn_signature: Option<Signature>,
    },
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn read_from_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents
}

pub fn to_pubkey(pubkey_str: &str) -> Result<Pubkey> {
    Pubkey::from_str(pubkey_str).map_err(|_| AsyncPluginError::InvalidPubKey {
        msg: pubkey_str.to_string(),
    })
}
