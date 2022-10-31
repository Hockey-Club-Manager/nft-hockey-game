use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_exe;
use crate::{PlayerPosition};
use crate::PlayerPosition::{AdditionalPosition, LeftWing, RightWing};
use crate::team::five::{ActiveFive, FiveIds, IceTimePriority};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::team::numbers::FiveNumber::*;
use crate::team::players::goalie::Goalie;
use crate::team::players::player::{GoalieSubstitution};
use crate::team::players::player::GoalieSubstitution::{GoalieSubstitution1, GoalieSubstitution2};
use crate::user_info::UserId;


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
    pub(crate) active_five: ActiveFive,

    pub(crate) field_players: HashMap<TokenId, FieldPlayer>,

    pub(crate) penalty_players: Vec<TokenId>,
    pub(crate) players_to_penalty: Vec<TokenId>,

    pub(crate) goalie_substitutions: HashMap<GoalieSubstitution, TokenId>,
    pub(crate) active_goalie_substitutions: GoalieSubstitution,

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
            let player_position = self.remove_player_id_from_five(penalty_player_id);
            log!("Removed position: {}", player_position);

            if count_players_in_five == 3 {
                let available_players = self.get_available_players(&brigades, &fives);

                let player_id = self.get_player_id_with_max_defence(&available_players);
                let active_five = self.get_active_five_mut();
                active_five.field_players.insert(player_position, player_id);
            } else if player_position != RightWing {
                let rw_id = self.get_player_id_by_pos(&RightWing);
                self.remove_player_id_from_five(&rw_id);

                let available_players = self.get_available_players(&brigades, &fives);
                let player_id = self.get_player_id_with_max_defence(&available_players);

                let active_five = self.get_active_five_mut();
                active_five.field_players.insert(player_position.clone(), player_id.clone());
                log!("Insert: {} on position: {}", player_position, player_id);

                if five_number == PenaltyKill1 {
                    let five = self.fives.get_mut(&PenaltyKill2).unwrap();
                    five.field_players.remove(&RightWing);
                } else {
                    let five = self.fives.get_mut(&PenaltyKill1).unwrap();
                    five.field_players.remove(&RightWing);
                }
            }
        } else {
            self.active_five.current_number = PenaltyKill1;

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

                log!("Inserted player_id {}", player_id.clone());
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
        let mut count: usize = 0;

        for (_pos, id) in &active_five.field_players {
            if *id != "" {
                count += 1;
            }
        }

        (active_five.current_number, count)
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
            let five = self.fives.get(five_number).unwrap();

            for (_pos, token_id) in &five.field_players {
                result.push(token_id.clone());
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
            log!("{} = {}", player_id, id);
            if *player_id == *id {
                return pos;
            }
        }

        panic!("Player not found: {}", player_id)
    }

    pub fn get_five_number_of_player(&self) -> usize {
        let active_five = self.get_active_five();
        let mut count = 0;

        for (_pos, id) in &active_five.field_players {
            if *id != "" {
                count += 1;
            }
        }
        count
    }

    pub fn get_active_five(&self) -> &ActiveFive {
        &self.active_five
    }

    pub fn get_active_five_mut(&mut self) -> &mut ActiveFive {
        &mut self.active_five
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
        let time_field = self.active_five.time_field.unwrap();

        let result = match self.active_five.ice_time_priority {
            IceTimePriority::SuperLowPriority => time_field >= SUPER_LOW_PRIORITY,
            IceTimePriority::LowPriority => time_field >= LOW_PRIORITY,
            IceTimePriority::Normal => time_field >= NORMAL,
            IceTimePriority::HighPriority => time_field >= HIGH_PRIORITY,
            IceTimePriority::SuperHighPriority => time_field >= SUPER_HIGH_PRIORITY,
        };

        result
    }
    
    pub fn change_active_five(&mut self) {
        let current_number = match self.active_five.current_number {
            First => {
                Second
            },
            Second => {
                Third
            },
            Third => {
                Fourth
            },
            Fourth => {
                First
            },
            PowerPlay1 => {
                PowerPlay2
            }
            PowerPlay2 => {
                PowerPlay1
            },
            PenaltyKill1 => {
                PenaltyKill2
            },
            PenaltyKill2 => {
                PenaltyKill1
            }
        };

        self.active_five.current_number = current_number;
        self.active_five.replaced_position.clear();
        self.active_five.time_field = Option::from(0 as u8);
    }

    pub fn get_five(&self, number: &FiveNumber) -> &FiveIds {
        self.fives.get(&number).expect("Five not found")
    }

    fn get_number_of_field_players(&self, five_number: &FiveNumber) -> usize {
        self.fives.get(&five_number).expect("Five not found").get_number_of_players()
    }

    pub fn swap_players_in_active_five(&mut self, player_with_puck: Option<TokenId>) {
        let current_five_number = self.active_five.current_number.clone();
        let players = self.get_players_in_five(&current_five_number);
        let number_of_players_in_current_five = self.get_number_of_field_players(&current_five_number);

        let active_five = self.get_active_five_mut();
        let number_of_players_in_active_five = active_five.get_number_of_players();

        if active_five.current_number == current_five_number &&
            number_of_players_in_active_five == number_of_players_in_current_five {
            return;
        }

        for (position, player_id) in &players {
            let is_replaced_position = active_five.replaced_position.contains(position);
            let is_player_with_puck = match player_with_puck.is_some() {
                true => {
                    if *player_id == *player_with_puck.as_ref().unwrap() {
                        true
                    } else {
                        false
                    }
                }
                false => false
            };

            if !is_player_with_puck && !is_replaced_position {
                let replaced_player_id = active_five.field_players
                    .insert(position.clone(), player_id.clone());
                active_five.replaced_position.push(position.clone());

                let goalie_substitution1_id = self.goalie_substitutions
                    .get(&GoalieSubstitution1)
                    .expect("Goalie substitution not found");

                let goalie_substitution2_id = self.goalie_substitutions
                    .get(&GoalieSubstitution2)
                    .expect("Goalie substitution not found");

                if replaced_player_id.is_some() {
                    let unwrapped_id = replaced_player_id.unwrap();
                    if unwrapped_id == *goalie_substitution1_id ||
                        unwrapped_id == *goalie_substitution2_id {
                        self.goalie_out();
                    }
                }

                return;
            }
        }
    }

    pub fn swap_all_players_in_active_five(&mut self) {
        let current_five_number = self.active_five.current_number.clone();
        let players = self.get_players_in_five(&current_five_number);

        let active_five = self.get_active_five_mut();
        active_five.field_players.clear();

        for (pos, id) in &players {
           active_five.field_players.insert(pos.clone(), id.clone());
        }

        if active_five.is_goalie_out {
            self.goalie_out();
        }
    }

    fn get_players_in_five(&self, number: &FiveNumber) -> HashMap<PlayerPosition, TokenId> {
        self.get_five(number).field_players.clone()
    }

    pub fn goalie_out(&mut self) {
        let goalie_substitute_id = self.goalie_substitutions.get(&self.active_goalie_substitutions).unwrap().clone();
        let active_five = self.get_active_five_mut();
        active_five.is_goalie_out = true;
        let number_of_players = active_five.get_number_of_players();

        if number_of_players == 5 {
            active_five.field_players.insert(AdditionalPosition, goalie_substitute_id.clone());
        } else if number_of_players == 4 {
            active_five.field_players.insert(LeftWing, goalie_substitute_id.clone());
        } else {
            active_five.field_players.insert(RightWing, goalie_substitute_id.clone());
        }
    }

    pub fn goalie_back(&mut self) {
        let active_five = self.get_active_five_mut();
        active_five.is_goalie_out = false;

        let number_of_players = active_five.get_number_of_players();

        if number_of_players == 4 {
            active_five.field_players.remove(&RightWing);
        } else if number_of_players == 5 {
            active_five.field_players.remove(&LeftWing);
        } else if number_of_players == 6 {
            active_five.field_players.remove(&AdditionalPosition);
        }
    }
}