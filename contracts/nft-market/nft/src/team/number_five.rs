use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub const NUMBER_OF_FIVES: usize = 8;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum NumberFive {
    First,
    Second,
    Third,
    Fourth,

    PowerPlay1,
    PowerPlay2,
    PenaltyKill1,
    PenaltyKill2,
}
