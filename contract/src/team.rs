use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::FieldPlayer;
use crate::goalie::Goalie;
use crate::team::IceTimePriority::{HighPriority, LowPriority, Normal, SuperHighPriority, SuperLowPriority};

const SUPER_LOW_PRIORITY: u8 = 5;
const LOW_PRIORITY: u8 = 10;
const NORMAL: u8 = 15;
const HIGH_PRIORITY: u8 = 20;
const SUPER_HIGH_PRIORITY: u8 = 25;

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Team {
    pub(crate) fives: HashMap<Fives, Five>,
    pub(crate) goalies: HashMap<Goalies, Goalie>,
    pub(crate) active_five: Five,

    pub(crate) active_goalie: Goalie,
    pub(crate) score: u8,
}

impl Team {
    fn need_change(&self) -> bool {
        match self.active_five.ice_time_priority {
            SuperLowPriority => self.active_five.time_field >= SUPER_LOW_PRIORITY,
            LowPriority => self.active_five.time_field >= LOW_PRIORITY,
            Normal => self.active_five.time_field >= NORMAL,
            HighPriority => self.active_five.time_field >= HIGH_PRIORITY,
            SuperHighPriority => self.active_five.time_field >= SUPER_HIGH_PRIORITY,
        }
    }

    pub fn change_active_five(&mut self) {

    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamJson {
    pub(crate) five: Five,
    pub(crate) goalie: Goalie,
    pub(crate) score: u8,
}


#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash, Clone, BorshDeserialize, BorshSerialize)]
pub enum Goalies {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Clone, BorshDeserialize, BorshSerialize)]
pub enum Fives {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}

#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct Five {
    pub(crate) field_players: HashMap<String, FieldPlayer>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) time_field: u8,
}