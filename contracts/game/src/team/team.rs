use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{PlayerPosition, UserInfo};
use crate::team::five::{Five, FiveNumber, GoalieNumber};
use crate::team::five::FiveNumber::{First, Fourth, Second, Third};
use crate::team::numbers::GoalieNumber;
use crate::team::players::goalie::Goalie;

const SUPER_LOW_PRIORITY: u8 = 5;
const LOW_PRIORITY: u8 = 10;
const NORMAL: u8 = 15;
const HIGH_PRIORITY: u8 = 20;
const SUPER_HIGH_PRIORITY: u8 = 25;

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Team {
    pub(crate) fives: HashMap<FiveNumber, Five>,
    pub(crate) goalies: HashMap<GoalieNumber, Goalie>,
    pub(crate) active_five: FiveNumber,

    pub(crate) active_goalie: GoalieNumber,
    pub(crate) score: u8,
}

impl Team {
    pub fn need_change(&self) -> bool {
        // let active_five = self.fives.get(&self.active_five).unwrap();
        //
        // let d = active_five.ice_time_priority;
        // let field = active_five.time_field;
        //
        // match active_five.ic{
        //     SuperLowPriority => active_five.time_field >= SUPER_LOW_PRIORITY,
        //     LowPriority => active_five.time_field >= LOW_PRIORITY,
        //     Normal => active_five.time_field >= NORMAL,
        //     HighPriority => active_five.time_field >= HIGH_PRIORITY,
        //     SuperHighPriority => active_five.ime_field >= SUPER_HIGH_PRIORITY,
        // }
        false
    }
    pub fn change_active_five(&mut self) {
        match self.active_five {
            First => {
                self.active_five = Second;
            },
            Second => {
                self.active_five = Third;
            },
            Third => {
                self.active_five = Fourth
            },
            Fourth => {
                self.active_five = First;
            },
        }

        let active_five = self.fives.get_mut(&self.active_five).unwrap();
        active_five.time_field = 0;
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamJson {
    pub(crate) five: Five,
    pub(crate) goalie: Goalie,
    pub(crate) score: u8,
}