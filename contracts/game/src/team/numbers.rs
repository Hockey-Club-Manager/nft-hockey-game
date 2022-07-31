use crate::*;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Hash, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum GoalieNumber {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum FiveNumber {
    First,
    Second,
    Third,
    Fourth,
}
