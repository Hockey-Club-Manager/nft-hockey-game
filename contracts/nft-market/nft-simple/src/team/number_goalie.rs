use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub enum NumberGoalie {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}
