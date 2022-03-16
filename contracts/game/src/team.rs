use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{FieldPlayer, PlayerPosition, TeamMetadata, UserInfo};
use crate::goalie::Goalie;
use crate::player::PlayerRole;
use crate::team::Fives::{First, Fourth, Second, Third};
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

impl Team {
    pub fn need_change(&self) -> bool {
        match self.active_five.ice_time_priority {
            SuperLowPriority => self.active_five.time_field >= SUPER_LOW_PRIORITY,
            LowPriority => self.active_five.time_field >= LOW_PRIORITY,
            Normal => self.active_five.time_field >= NORMAL,
            HighPriority => self.active_five.time_field >= HIGH_PRIORITY,
            SuperHighPriority => self.active_five.time_field >= SUPER_HIGH_PRIORITY,
        }
    }

    pub fn change_active_five(&mut self) {
        match self.active_five.number {
            First => {
                self.fives.insert(First, self.active_five.clone());

                if self.fives.len() > 1 {
                    self.active_five = self.fives.get(&Second).unwrap().clone()
                }
            },
            Second => {
                self.fives.insert(Second, self.active_five.clone());

                if self.fives.len() > 2 {
                    self.active_five = self.fives.get(&Third).unwrap().clone()
                } else {
                    self.active_five = self.fives.get(&First).unwrap().clone()
                }
            },
            Third => {
                self.fives.insert(Third, self.active_five.clone());

                if self.fives.len() > 3 {
                    self.active_five = self.fives.get(&Fourth).unwrap().clone()
                } else {
                    self.active_five = self.fives.get(&First).unwrap().clone()
                }
            },
            Fourth => {
                self.fives.insert(Fourth, self.active_five.clone());

                self.active_five = self.fives.get(&First).unwrap().clone()
            },
        }

        self.active_five.time_field = 0;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamJson {
    pub(crate) five: Five,
    pub(crate) goalie: Goalie,
    pub(crate) score: u8,
}

pub fn swap_positions(user_info: &mut UserInfo, number_five: Fives, position1: String, position2: String) {
    let mut five = user_info.team.fives.get_mut(&number_five).unwrap().clone();
    let mut first_player = five.field_players.get(&position1).unwrap().clone();
    let mut second_player = five.field_players.get(&position2).unwrap().clone();

    first_player.position = second_player.position;
    second_player.position = first_player.position;

    first_player.set_position_coefficient();
    second_player.set_position_coefficient();

    five.field_players.insert(position1, second_player);
    five.field_players.insert(position2, first_player);

    user_info.team.fives.insert(number_five, five);
}