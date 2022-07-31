use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use crate::team::numbers::FiveNumber;


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveIds {
    pub(crate) field_players: HashMap<PlayerPosition, TokenId>,
    pub(crate) number: FiveNumber,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
    pub(crate) time_field: Option<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Tactics {
    Safe,
    Defensive,
    Neutral,
    Offensive,
    Aggressive,
}
