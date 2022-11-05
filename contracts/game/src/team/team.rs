use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{PlayerPosition};
use crate::PlayerPosition::{AdditionalPosition, LeftWing, RightWing};
use crate::team::five::{ActiveFive, FiveIds, IceTimePriority};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::team::numbers::FiveNumber::*;
use crate::team::players::goalie::Goalie;
use crate::team::players::player::{GoalieSubstitution};
use crate::team::players::player::GoalieSubstitution::{GoalieSubstitution1, GoalieSubstitution2};
use crate::team::players::player::Hand::Left;
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
    pub(crate) players_to_big_penalty: Vec<TokenId>,
    pub(crate) players_to_small_penalty: Vec<TokenId>,

    pub(crate) goalie_substitutions: HashMap<GoalieSubstitution, TokenId>,
    pub(crate) active_goalie_substitution: GoalieSubstitution,

    pub(crate) goalies: HashMap<GoalieNumber, Goalie>,
    pub(crate) active_goalie: GoalieNumber,

    pub(crate) score: u8,
}

impl Team {
    pub fn get_number_of_penalty_players(&self) -> usize {
        let number_of_players_to_penalty = self.players_to_big_penalty.len()
            + self.players_to_small_penalty.len();

        number_of_players_to_penalty + self.penalty_players.len()
    }

    pub fn calculate_teamwork(&mut self) {
        for (_five_number, five_ids) in &self.fives {
            let field_players = &mut self.field_players;
            five_ids.calculate_team_work(field_players);
        }
    }

    pub fn do_penalty(&mut self, penalty_player_id: &TokenId) {
        let (five_number, count_players_in_five) = self.get_number_players_count_active_five();

        if five_number == PenaltyKill1 || five_number == PenaltyKill2 {
            self.do_penalty_for_pk(penalty_player_id, count_players_in_five);
        } else {
            self.replace_penalty_player(penalty_player_id);
            self.active_five.current_number = First;
        }
    }

    fn get_number_players_count_active_five(&self) -> (FiveNumber, usize) {
        let active_five_number = self.active_five.current_number.clone();
        let active_five = self.get_five(&active_five_number);

        let mut count: usize = 0;

        for (_pos, id) in &active_five.field_players {
            if *id != "" {
                count += 1;
            }
        }

        (active_five.number, count)
    }

    fn get_five(&self, number: &FiveNumber) -> &FiveIds {
        self.fives.get(&number).expect("Five not found")
    }

    fn do_penalty_for_pk(&mut self, penalty_player_id: &TokenId, count_players_in_five: usize) {
        if count_players_in_five == 3 {
            self.replace_penalty_player(penalty_player_id);
        } else {
            self.replace_penalty_player(penalty_player_id);
            self.remove_extra_players();
        }
    }

    fn replace_penalty_player(&mut self, penalty_player_id: &TokenId,) {
        let fives = vec![First, Second, Third, Fourth];

        self.replace_penalty_player_in_brigades(
            &fives, &vec![PenaltyKill1, PenaltyKill2], penalty_player_id);
        self.replace_penalty_player_in_brigades(
            &fives, &vec![PowerPlay1, PowerPlay2], penalty_player_id);
    }

    /// fives: 1, 2, 3, 4. brigades: pk1, pk2 or pp1, pp2
    fn replace_penalty_player_in_brigades(
        &mut self,
        fives: &Vec<FiveNumber>,
        brigades: &Vec<FiveNumber>,
        penalty_player_id: &TokenId
    ) {
        for brigade_number in brigades {
            self.replace_penalty_player_in_brigade(
                fives,
                brigades,
                brigade_number,
                penalty_player_id
            );
        }
    }

    fn replace_penalty_player_in_brigade(
        &mut self,
        fives: &Vec<FiveNumber>,
        brigades: &Vec<FiveNumber>,
        brigade_number: &FiveNumber,
        penalty_player_id: &TokenId
    ) {
        let brigade = self.get_five(brigade_number);
        let penalty_player_position =
            self.get_player_position(brigade, penalty_player_id);

        let pos_id = match penalty_player_position {
            Some(position) => {
                let available_players = self.get_available_players(brigades, fives);
                let player_id = if vec![PenaltyKill1, PenaltyKill2].contains(brigade_number) {
                    self.get_player_id_with_max_defence(&available_players)
                } else {
                    self.get_player_id_with_max_iq(&available_players)
                };

                Some((position, player_id))
            },
            None => { None }
        };

        match pos_id {
            Some((position, player_id)) => {
                let brigade_mut = self.get_five_mut(brigade_number);
                brigade_mut.field_players.insert(position, player_id);
            },
            None => {}
        }
    }

    fn get_player_position(
        &self,
        five: &FiveIds,
        player_id: &TokenId
    ) -> Option<PlayerPosition> {
        for (position, id) in &five.field_players {
            if *id == *player_id {
                return Some(position.clone());
            }
        }

        None
    }

    fn get_available_players(
        &self,
        brigades: &Vec<FiveNumber>,
        fives: &Vec<FiveNumber>
    ) -> Vec<TokenId> {
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

    fn remove_extra_players(&mut self) {
        self.remove_extra_players_from_pks();
        self.remove_extra_players_from_pps();
    }

    /// Right forwards from penalty kills
    fn remove_extra_players_from_pks(&mut self) {
        let pks = vec![PenaltyKill1, PenaltyKill2];
        for number in &pks {
            let brigade = self.get_five_mut(number);
            brigade.field_players.remove(&RightWing);
        }
    }

    /// Left forwards from power plays
    fn remove_extra_players_from_pps(&mut self) {
        let pks = vec![PowerPlay1, PowerPlay2];
        for number in &pks {
            let brigade = self.get_five_mut(number);
            brigade.field_players.remove(&LeftWing);
        }
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

    fn get_player_id_with_max_iq(&self, available_players: &Vec<TokenId>) -> TokenId {
        let mut player_id_with_max_iq: TokenId = "".into();
        let mut max_iq: f32 = 0.0;

        for player_id in available_players {
            let player = self.field_players.get(player_id).unwrap();
            let player_iq = player.stats.get_iq();

            if player_iq > max_iq {
                player_id_with_max_iq = player_id.clone();
                max_iq = player_iq;
            }
        }

        player_id_with_max_iq
    }

    pub fn release_removed_players_in_brigades(&mut self) {
        let fives = vec![First, Second, Third, Fourth];

        let pps = vec![PowerPlay1, PowerPlay2];
        self.release_removed_players(&fives, &pps, &LeftWing);

        let pks = vec![PenaltyKill1, PenaltyKill2];
        self.release_removed_players(&fives, &pks, &RightWing);

        if pps.contains(&self.active_five.current_number) {
            self.insert_player_to_active_five(&LeftWing);
        } else if pks.contains(&self.active_five.current_number) {
            self.insert_player_to_active_five(&RightWing);
        }
    }

    pub fn insert_player_to_active_five(&mut self, player_position: &PlayerPosition) {
        let right_wing_id = self.get_player_id_by_pos(player_position);

        self.active_five.field_players.insert(player_position.clone(), right_wing_id);
        self.active_five.replaced_position.push(player_position.clone());
    }

    fn get_player_id_by_pos(&self, player_position: &PlayerPosition) -> TokenId {
        let five = self.get_five(&self.active_five.current_number);
        five.field_players.get(player_position)
            .expect("Cannot find right winger").clone()
    }

    fn release_removed_players(
        &mut self,
        fives: &Vec<FiveNumber>,
        brigades: &Vec<FiveNumber>,
        vacated_position: &PlayerPosition
    ) {
        for brigade_number in brigades{
            let available_players = self.get_available_players(&brigades, &fives);
            let player_id = self.get_player_id_with_max_iq(&available_players);

            let brigade = self.get_five_mut(brigade_number);
            brigade.field_players.insert(vacated_position.clone(), player_id);
        }
    }

    fn get_players_in_fives(&self, five_numbers: &Vec<FiveNumber>) -> Vec<TokenId> {
        let mut result: Vec<TokenId> = Vec::new();

        for five_number in five_numbers {
            let five = self.fives.get(five_number).expect("Five not found");

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

    pub fn get_five_number_of_players(&self, number_five: &FiveNumber) -> usize {
        let five = self.get_five(&number_five);

        let mut count = 0;

        for (_pos, id) in &five.field_players {
            if *id != "" {
                count += 1;
            }
        }

        count
    }

    pub fn get_active_five_number_of_player(&self) -> usize {
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

    pub fn get_five_mut(&mut self, number: &FiveNumber) -> &mut FiveIds {
        self.fives.get_mut(&number).expect("Five not found")
    }

    fn get_number_of_field_players(&self, five_number: &FiveNumber) -> usize {
        self.fives.get(&five_number).expect("Five not found").get_number_of_players()
    }

    pub fn swap_players_in_active_five(&mut self, player_with_puck: Option<TokenId>) {
        let players_to_big_penalty = self.players_to_big_penalty.clone();
        let players_to_small_penalty = self.players_to_small_penalty.clone();

        let current_five_number = self.active_five.current_number.clone();
        let players = self.get_players_in_five(&current_five_number);
        let number_of_players_in_current_five = self.get_number_of_field_players(&current_five_number);

        let active_five = self.get_active_five_mut();
        let number_of_players_in_active_five = active_five.get_number_of_players();

        if active_five.current_number == current_five_number &&
            number_of_players_in_active_five == number_of_players_in_current_five {
            return;
        }

        let players_in_active_five = active_five.field_players.clone();
        active_five.field_players.clear();

        let number_of_players_to_replace = 2;
        let mut number_of_replaced_players = 0;

        for (position, player_id) in &players {
            let is_replaced_position = active_five.replaced_position.contains(position);
            let is_player_available = is_player_available(player_id,
                                                          &player_with_puck,
                                                          &players_to_big_penalty,
                                                          &players_to_small_penalty);

            if is_player_available && !is_replaced_position
                && number_of_players_to_replace > number_of_replaced_players {
                active_five.field_players.insert(position.clone(), player_id.clone());
                active_five.replaced_position.push(position.clone());
                number_of_replaced_players += 1;
            } else {
                let id = players_in_active_five.get(position).unwrap();
                active_five.field_players.insert(position.clone(), id.clone());
            }
        }

        if self.active_five.is_goalie_out && self.is_goalie_substitutions_in_active_five() {
            self.goalie_out();
        }
    }

    fn is_goalie_substitutions_in_active_five(&self) -> bool {
        let goalie_sub1_id = self.goalie_substitutions
            .get(&GoalieSubstitution1)
            .expect("GoalieSubstitution1 not found");

        let goalie_sub2_id = self.goalie_substitutions
            .get(&GoalieSubstitution2)
            .expect("GoalieSubstitution2 not found");

        for (_position, id) in &self.active_five.field_players {
            if *id == *goalie_sub1_id || *id == *goalie_sub2_id {
                return true;
            }
        }

        false
    }

    pub fn swap_all_players_in_active_five(&mut self) {
        let current_five_number = self.active_five.current_number.clone();
        let players = self.get_players_in_five(&current_five_number);

        let active_five = self.get_active_five_mut();
        active_five.field_players.clear();
        active_five.replaced_position.clear();

        for (pos, id) in &players {
            active_five.field_players.insert(pos.clone(), id.clone());
            active_five.replaced_position.push(pos.clone());
        }

        if active_five.is_goalie_out {
            self.goalie_out();
        }
    }

    fn get_players_in_five(&self, number: &FiveNumber) -> HashMap<PlayerPosition, TokenId> {
        self.get_five(number).field_players.clone()
    }

    pub fn goalie_out(&mut self) {
        let goalie_substitute_id = self.goalie_substitutions.get(&self.active_goalie_substitution).unwrap().clone();
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


fn is_player_available(
    player_id: &TokenId,
    player_with_puck: &Option<TokenId>,
    players_to_big_penalty: &Vec<TokenId>,
    players_to_small_penalty: &Vec<TokenId>
) -> bool {
    match player_with_puck.is_some() {
        true => {
            if *player_id == *player_with_puck.as_ref().unwrap() {
                return false;
            }
        }
        false => {}
    };

    if players_to_small_penalty.contains(player_id) && players_to_big_penalty.contains(player_id) {
        return false;
    }

    true
}
