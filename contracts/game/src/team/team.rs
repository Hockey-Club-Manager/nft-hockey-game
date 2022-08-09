use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::collections::Vector;
use crate::{PlayerPosition, UserInfo};
use crate::PlayerPosition::{LeftDefender, RightDefender};
use crate::team::five::{FiveIds, IceTimePriority};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::team::numbers::FiveNumber::*;
use crate::team::players::goalie::Goalie;
use crate::team::players::player::PlayerRole;
use crate::team::players::player::PlayerRole::*;


const SUPER_LOW_PRIORITY: u8 = 5;
const LOW_PRIORITY: u8 = 10;
const NORMAL: u8 = 15;
const HIGH_PRIORITY: u8 = 20;
const SUPER_HIGH_PRIORITY: u8 = 25;


#[derive(Clone, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Team {
    pub(crate) fives: HashMap<FiveNumber, FiveIds>,
    pub(crate) active_five: FiveNumber,

    pub(crate) field_players: HashMap<TokenId, FieldPlayer>,
    pub(crate) penalty_players: Vec<TokenId>,

    pub(crate) goalies: HashMap<GoalieNumber, Goalie>,
    pub(crate) active_goalie: GoalieNumber,

    pub(crate) score: u8,
}

impl Team {
    pub fn calculate_teamwork(&mut self) {
        for (_five_number, five_ids) in &self.fives {
            let field_players = &mut self.field_players;
            five_ids.calculate_team_work(field_players);
        }
    }

    pub fn get_field_player_mut(&mut self, id: &TokenId) -> &mut FieldPlayer {
        self.field_players.get_mut(id).unwrap()
    }

    pub fn get_field_player(&self, id: &TokenId) -> &FieldPlayer {
        self.field_players.get(id).unwrap()
    }

    pub fn get_field_player_pos(&self, player_id: &TokenId) -> &PlayerPosition {
        let five = self.get_active_five();
        for (pos, id) in &five.field_players {
            if *player_id == *id {
                return pos;
            }
        }

        panic!("Player not found")
    }

    pub fn get_active_five(&self) -> &FiveIds {
        self.fives.get(&self.active_five).unwrap()
    }

    pub fn get_active_five_mut(&mut self) -> &mut FiveIds {
        self.fives.get_mut(&self.active_five).unwrap()
    }

    pub fn reduce_morale(&mut self) {
        for (_five_number, five) in &self.fives {
            let field_players = &mut self.field_players;
            five.reduce_morale(field_players)
        }

        for (_goalie_number, goalie) in &mut self.goalies {
            goalie.stats.morale -= 3;
        }
    }

    pub fn increase_morale(&mut self) {
        for (_five_number, five) in &self.fives {
            let field_players = &mut self.field_players;
            five.increase_morale(field_players)
        }

        for (_goalie_number, goalie) in &mut self.goalies {
            goalie.stats.morale += 2;
        }
    }

    pub fn need_change(&self) -> bool {
        let active_five = self.fives.get(&self.active_five).unwrap();

        let time_field = active_five.time_field.unwrap();

        let result = match active_five.ice_time_priority {
            IceTimePriority::SuperLowPriority => time_field >= SUPER_LOW_PRIORITY,
            IceTimePriority::LowPriority => time_field >= LOW_PRIORITY,
            IceTimePriority::Normal => time_field >= NORMAL,
            IceTimePriority::HighPriority => time_field >= HIGH_PRIORITY,
            IceTimePriority::SuperHighPriority => time_field >= SUPER_HIGH_PRIORITY,
        };

        result
    }
    
    pub fn change_active_five(&mut self) {
        // TODO: reduce strength
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
            PowerPlay1 => {
                self.active_five = PowerPlay2;
            }
            PowerPlay2 => {
                self.active_five = PowerPlay1
            },
            PenaltyKill1 => {
                self.active_five = PenaltyKill2
            },
            PenaltyKill2 => {
                self.active_five = PenaltyKill1
            }
        }

        let active_five = self.get_active_five_mut();
        active_five.time_field = Option::from(0 as u8);
    }
}