use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{PlayerPosition, UserInfo};
use crate::PlayerPosition::{LeftDefender, RightDefender};
use crate::team::five::{FiveIds};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::team::numbers::FiveNumber::*;
use crate::team::players::goalie::Goalie;
use crate::team::players::player::PlayerRole;
use crate::team::players::player::PlayerRole::*;

const NATIONALITY_TEAMWORK: f32 = 1.05;
const DEFENSEMEN_TEAMWORK: f32 = 1.1;
const TOUGH_ENFORCER_TEAMWORK: f32 = 1.2;
const DEFENDERS_TEAMWORK: f32 = 1.2;

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

    pub(crate) goalies: HashMap<GoalieNumber, Goalie>,
    pub(crate) active_goalie: GoalieNumber,

    pub(crate) score: u8,
}

// teamwork
impl Team {
    pub fn calculate_teamwork(&mut self) {
        for (_five_number, five_ids) in &self.fives {
            self.calculate_five_teamwork(five_ids);
        }
    }

    fn calculate_five_teamwork(&mut self, five_ids: &FiveIds) {
        let mut player_per_nationality: HashMap<String, Vec<PlayerPosition>> = HashMap::new();
        let mut player_per_role: HashMap<PlayerRole, Vec<PlayerPosition>> = HashMap::new();

        let mut team_work_line: f32 = 1.0;

        for (position, field_player_id) in &five_ids.field_players {
            let field_player = self.get_field_player_mut(field_player_id);
            field_player.teamwork = Option::from(field_player.get_position_coefficient(position));

            self.insert_player_nationality(&mut player_per_nationality, field_player, &position);
            self.insert_player_role(&mut player_per_role, field_player, &position);

            if field_player.player_role == ToughGuy || field_player.player_role == Enforcer {
                team_work_line *= 0.9;
            } else if field_player.player_role == TryHarder || field_player.player_role == TwoWay {
                team_work_line *= 1.1;
            }
        }

        self.change_teamwork_by_roles(&player_per_role, five_ids);
        self.change_teamwork_by_nationality(five_ids, &player_per_nationality);
        self.change_line_teamwork(team_work_line);
    }

    fn insert_player_nationality(
        &self, player_per_nationality: &mut HashMap<String, Vec<PlayerPosition>>,
        field_player: &FieldPlayer,
        position: &PlayerPosition
    ) {
        match player_per_nationality.get_mut(&field_player.nationality) {
            Some(field_players) => {
                field_players.push(*position);
            }
            None => {
                player_per_nationality.insert(field_player.clone().nationality, vec![*position]);
            }
        }
    }

    fn insert_player_role(
        &self,
        player_per_role: &mut HashMap<PlayerRole, Vec<PlayerPosition>>,
        field_player: &FieldPlayer,
        position: &PlayerPosition,
    ) {
        match player_per_role.get_mut(&field_player.player_role) {
            Some(field_players) => {
                field_players.push(*position);
            },
            None => {
                player_per_role.insert(field_player.player_role, vec![*position]);
            }
        }
    }

    fn change_teamwork_by_roles(
        &mut self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        five_ids: &FiveIds
    ) {
        self.check_offensive_defensive_defensemen(player_per_role, five_ids);
        self.check_enf_tough(five_ids, &player_per_role);
        self.check_defensive_forward(five_ids, player_per_role);
    }

    fn check_offensive_defensive_defensemen(&mut self, player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>, five_ids: &FiveIds) {
        match player_per_role.get(&DefensiveDefenseman) {
            Some(def_positions) => {
                match player_per_role.get(&OffensiveDefenseman) {
                    Some(off_positions) => {
                        for def_position in def_positions {
                            self.change_teamwork_by_position(five_ids, def_position, DEFENSEMEN_TEAMWORK);
                        }

                        for off_position in off_positions {
                            self.change_teamwork_by_position(five_ids, off_position, DEFENSEMEN_TEAMWORK);
                        }
                    }
                    None => {}
                }
            },
            None => {}
        }
    }

    fn change_teamwork_by_position(&mut self, five_ids: &FiveIds, position: &PlayerPosition, teamwork: f32) {
        let token_id = five_ids.field_players.get(position).unwrap();
        let player = self.get_field_player_mut(token_id);
        player.teamwork = Option::from(player.teamwork.unwrap() * teamwork);
    }

    fn check_enf_tough(
        &mut self,
        five_ids: &FiveIds,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>
    ) {
        if player_per_role.get(&ToughGuy).is_some() {
            self.change_team_work_by_enf_tough(five_ids, player_per_role, &ToughGuy);
        }
        if player_per_role.get(&Enforcer).is_some() {
            self.change_team_work_by_enf_tough(five_ids, player_per_role, &Enforcer);
        }
    }

    fn change_team_work_by_enf_tough(
        &mut self,
        five_ids: &FiveIds,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        player_role: &PlayerRole
    ) {
        let number_of_tough = player_per_role.get(player_role).unwrap().len();
        let teamwork = TOUGH_ENFORCER_TEAMWORK * (number_of_tough as f32);
        self.change_teamwork_by_role(five_ids, teamwork, &Playmaker, player_per_role);
        self.change_teamwork_by_role(five_ids, teamwork, &Shooter, player_per_role);
    }

    fn change_teamwork_by_role(
        &mut self,
        five_ids: &FiveIds,
        teamwork: f32,
        player_role: &PlayerRole,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>
    ) {
        match player_per_role.get(&player_role) {
            Some(playmaker_positions) => {
                for position in playmaker_positions {
                    self.change_teamwork_by_position(five_ids, position, teamwork);
                }
            },
            None => {}
        }
    }

    fn check_defensive_forward(
        &mut self,
        five_ids: &FiveIds,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>
    ) {
        if player_per_role.get(&DefensiveForward).is_some() {
            self.change_teamwork_by_position(five_ids, &LeftDefender, DEFENDERS_TEAMWORK);
            self.change_teamwork_by_position(five_ids, &RightDefender, DEFENDERS_TEAMWORK);
        }
    }

    fn change_teamwork_by_nationality(
        &mut self,
        five_ids: &FiveIds,
        player_per_nationality: &HashMap<String, Vec<PlayerPosition>>
    ) {
        for (_nationality, positions) in player_per_nationality {
            if positions.len() > 1 {
                for position in positions {
                    self.change_teamwork_by_position(five_ids, position, NATIONALITY_TEAMWORK);
                }
            }
        }
    }

    fn change_line_teamwork(&mut self, team_work_line: f32) {
        for (_position, field_player) in &mut self.field_players {
            field_player.teamwork = Option::from(field_player.teamwork.unwrap() * team_work_line);
        }
    }
}

impl Team {
    pub fn get_field_player_mut(&mut self, id: &TokenId) -> &mut FieldPlayer {
        self.field_players.get_mut(id).unwrap()
    }

    pub fn get_field_player(&self, id: &TokenId) -> &FieldPlayer {
        self.field_players.get(id).unwrap()
    }
    pub fn get_field_player_pos(&self, player_id: &TokenId) -> &PlayerPosition {
        let five = self.get_active_five();
        for (pos, id) in five.field_players {
            if *player_id == id {
                return &pos;
            }
        }

        panic!("Player not found")
    }

    pub fn get_active_five(&self) -> &FiveIds {
        self.fives.get(&self.active_five).unwrap()
    }

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

        let active_five = self.fives.get_mut(&self.active_five).unwrap();
        active_five.time_field = Option::from(0 as u8);
    }
}