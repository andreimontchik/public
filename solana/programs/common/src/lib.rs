use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct FailOnDivisionPayload {
    pub dividend: u8,
    pub divisor: u8,
    pub remainder: u8,
}
