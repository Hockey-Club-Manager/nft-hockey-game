use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerPosition {
    #[serde(alias = "C")]
    Center,

    #[serde(alias = "LW")]
    LeftWing,

    #[serde(alias = "RW")]
    RightWing,

    #[serde(alias = "LD")]
    LeftDefender,

    #[serde(alias = "RD")]
    RightDefender,

    #[serde(alias = "G")]
    GoaliePos,
}