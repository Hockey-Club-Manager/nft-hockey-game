use std::cmp::max;
use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Index;
use near_sdk::collections::Vector;
use crate::{PlayerPosition, UserInfo};
use crate::PlayerPosition::{Center, LeftDefender, RightDefender, RightWing};
use crate::team::five::{FiveIds, IceTimePriority};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::team::numbers::FiveNumber::*;
use crate::team::players::goalie::Goalie;
use crate::team::players::player::PlayerRole;
use crate::team::players::player::PlayerRole::*;


const SUPER_LOW_PRIORITY: u8 = 3;
const LOW_PRIORITY: u8 = 5;
const NORMAL: u8 = 7;
const HIGH_PRIORITY: u8 = 10;
const SUPER_HIGH_PRIORITY: u8 = 12;


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

    pub fn do_penalty(&mut self, penalty_player_id: &TokenId) {
        let brigades =vec![PenaltyKill1, PenaltyKill2, PowerPlay1, PowerPlay2];
        let fives = vec![First, Second, Third, Fourth];

        let (five_number, count_players_in_five) = self.get_number_players_count_active_five();

        if five_number == PenaltyKill1 || five_number == PenaltyKill2 {
            if count_players_in_five == 3 {
                let player_position = self.remove_player_id_from_five(penalty_player_id);

                let available_players = self.get_available_players(&brigades, &fives);

                let player_id = self.get_player_id_with_max_defence(&available_players);
                let active_five = self.get_active_five_mut();
                active_five.field_players.insert(player_position, player_id);
            } else {
                let player_position = self.remove_player_id_from_five(penalty_player_id);
                if player_position != RightWing {
                    let rw_id = self.get_player_id_by_pos(&RightWing);

                    self.remove_player_id_from_five(&rw_id);

                    let available_players = self.get_available_players(&brigades, &fives);
                    let player_id = self.get_player_id_with_max_defence(&available_players);

                    let active_five = self.get_active_five_mut();
                    active_five.field_players.insert(player_position, player_id);

                    if five_number == PenaltyKill1 {
                        let five = self.fives.get_mut(&PenaltyKill2).unwrap();
                        five.field_players.remove(&RightWing);
                    } else {
                        let five = self.fives.get_mut(&PenaltyKill1).unwrap();
                        five.field_players.remove(&RightWing);
                    }
                }
            }
        } else {
            self.active_five = PenaltyKill1;

            let (penalty_player_in_brigade, players_in_brigade) = self.remove_player_from_pk1(penalty_player_id);

            if penalty_player_in_brigade {
                let players_in_fives = self.get_players_in_fives(&fives);

                let mut available_players: Vec<TokenId> = Vec::new();
                for player_id in &players_in_fives {
                    if !players_in_brigade.contains(player_id) && !self.penalty_players.contains(player_id) {
                        available_players.push(player_id.clone());
                    }
                }

                let player_position = self.remove_player_id_from_five(penalty_player_id);
                let player_id = self.get_player_id_with_max_defence(&available_players);

                let new_active_five = self.get_active_five_mut();
                new_active_five.time_field = Option::from(0 as u8);

                new_active_five.field_players.insert(player_position, player_id);
            } else {
                let new_active_five = self.get_active_five_mut();
                new_active_five.time_field = Option::from(0 as u8);
            }
        }
    }

    fn remove_player_from_pk1(&self, penalty_player_id: &TokenId) -> (bool, Vec<TokenId>) {
        let mut players: Vec<TokenId> = Vec::new();
        let pk1 = self.get_active_five();

        let mut contains = false;
        for (_player_pos, player_id) in &pk1.field_players {
            if *player_id == *penalty_player_id {
                contains = true;
            } else {
                players.push(player_id.clone());
            }
        }

        (contains, players)
    }

    fn get_player_id_by_pos(&self, position: &PlayerPosition) -> TokenId {
        let active_five = self.get_active_five();
        active_five.field_players.get(&position).unwrap().clone()
    }

    fn get_available_players(&self, brigades: &Vec<FiveNumber>, fives: &Vec<FiveNumber>) -> Vec<TokenId> {
        let players_in_brigades = self.get_players_in_fives(brigades);
        let players_in_fives = self.get_players_in_fives(fives);

        let mut available_players: Vec<TokenId> = Vec::new();
        for player_id in &players_in_fives {
            if !players_in_brigades.contains(player_id) && !self.penalty_players.contains(player_id) {
                available_players.push(player_id.clone());
            }
        }

        available_players
    }

    fn get_number_players_count_active_five(&self) -> (FiveNumber, usize) {
        let active_five = self.get_active_five();
        (active_five.number, active_five.field_players.len())
    }

    fn remove_player_id_from_five(&mut self, penalty_player_id: &TokenId) -> PlayerPosition {
        let player_position = self.get_field_player_pos(penalty_player_id).clone();
        let active_five = self.get_active_five_mut();

        active_five.field_players.remove(&player_position);

        player_position
    }

    fn get_player_id_with_max_defence(&self, available_players: &Vec<TokenId>) -> TokenId {
        let mut player_id_with_max_defense: TokenId = "".into();
        let mut max_defence: f32 = 0.0;

        for player_id in available_players {
            let player = self.field_players.get(player_id).unwrap();
            let player_defence = player.stats.get_defense();

            if player_defence > max_defence {
                player_id_with_max_defense = player_id.clone();
                max_defence = player_defence;
            }
        }

        player_id_with_max_defense
    }

    fn get_players_in_fives(&self, five_numbers: &Vec<FiveNumber>) -> Vec<TokenId> {
        let mut result: Vec<TokenId> = Vec::new();

        for five_number in five_numbers.into_iter() {
            if !five_numbers.contains(&five_number) {
                let brigade = self.fives.get(five_number).unwrap();

                for (_pos, token_id) in &brigade.field_players {
                    result.push(token_id.clone());
                }
            }
        }

        result
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