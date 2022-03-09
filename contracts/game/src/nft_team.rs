use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use crate::{Fives, IceTimePriority};
use crate::goalie::Goalie;
use crate::team::{Five, Goalies};

pub type TokenId = String;

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