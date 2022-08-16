use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum NumberGoalie {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}

#[derive(PartialEq, Clone, Copy, BorshDeserialize, BorshSerialize)]
#[derive(Eq, PartialOrd, Hash, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GoalieSubstitution {
    #[serde(alias = "GS1")]
    GoalieSubstitution1,

    #[serde(alias = "GS2")]
    GoalieSubstitution2,
}