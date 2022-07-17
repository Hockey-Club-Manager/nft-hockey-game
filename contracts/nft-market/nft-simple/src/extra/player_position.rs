use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerPosition {
    Center,
    LeftWing,
    RightWing,
    LeftDefender,
    RightDefender,
    GoaliePos,
}