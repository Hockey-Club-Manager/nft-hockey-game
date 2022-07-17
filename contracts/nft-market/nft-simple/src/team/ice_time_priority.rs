use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}
