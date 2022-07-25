use crate::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash, Clone, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GoalieNumber {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Clone, Copy, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum FiveNumber {
    First,
    Second,
    Third,
    Fourth,
}
