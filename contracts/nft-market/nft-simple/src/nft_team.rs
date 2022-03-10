use std::collections::HashMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::TokenId;

type SRC = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, BorshDeserialize, BorshSerialize)]
pub enum Fives {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, BorshDeserialize, BorshSerialize)]
pub enum Goalies {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftTeam {
    pub(crate) fives: HashMap<Fives, NftFive>,
    pub(crate) goalies: HashMap<Goalies, TokenId>,
    pub(crate) active_five: NftFive,

    pub(crate) active_goalie: TokenId,
    pub(crate) score: u8,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftFive {
    pub(crate) field_players: HashMap<String, TokenId>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) time_field: u8,
}