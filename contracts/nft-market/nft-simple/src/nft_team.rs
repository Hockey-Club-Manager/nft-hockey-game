use crate::*;
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::{TokenId, TokenMetadata};

type SRC = String;

#[derive(Serialize, Deserialize)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub enum Fives {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<Fives, FiveMetadata>,
    pub(crate) goalies: HashMap<Goalies, TokenMetadata>,
    pub(crate) active_five: FiveMetadata,

    pub(crate) active_goalie: TokenMetadata,
    pub(crate) score: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveMetadata {
    pub(crate) field_players: HashMap<String, TokenMetadata>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) time_field: u8,
}