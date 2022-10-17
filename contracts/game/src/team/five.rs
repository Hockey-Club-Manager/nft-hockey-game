use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use crate::PlayerPosition::{LeftDefender, RightDefender};
use crate::team::numbers::FiveNumber;
use crate::team::players::player::PlayerRole;
use crate::team::players::player::PlayerRole::{DefensiveDefenseman, DefensiveForward, Enforcer, OffensiveDefenseman, Playmaker, Shooter, ToughGuy, TryHarder, TwoWay};


const NATIONALITY_TEAMWORK: f32 = 1.05;
const DEFENSEMEN_TEAMWORK: f32 = 1.1;
const TOUGH_ENFORCER_TEAMWORK: f32 = 1.2;
const DEFENDERS_TEAMWORK: f32 = 1.2;


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveIds {
    pub(crate) field_players: HashMap<PlayerPosition, TokenId>,
    pub(crate) number: FiveNumber,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
    pub(crate) time_field: Option<u8>,
}

// teamwork
impl FiveIds {
    pub fn calculate_team_work(&self, field_players: &mut HashMap<TokenId, FieldPlayer>) {
        let mut player_per_nationality: HashMap<String, Vec<PlayerPosition>> = HashMap::new();
        let mut player_per_role: HashMap<PlayerRole, Vec<PlayerPosition>> = HashMap::new();

        let mut team_work_line: f32 = 1.0;

        for (position, field_player_id) in &self.field_players {
            let field_player = field_players.get_mut(field_player_id).unwrap();
            field_player.teamwork = Option::from(field_player.get_position_coefficient(position));

            self.insert_player_nationality(&mut player_per_nationality, field_player, position);
            self.insert_player_role(&mut player_per_role, field_player, position);

            if field_player.player_role == ToughGuy || field_player.player_role == Enforcer {
                team_work_line *= 0.9;
            } else if field_player.player_role == TryHarder || field_player.player_role == TwoWay {
                team_work_line *= 1.1;
            }
        }

        self.change_teamwork_by_roles(&player_per_role, field_players);
        self.change_teamwork_by_nationality(&player_per_nationality, field_players);
        self.change_line_teamwork(team_work_line, field_players);
    }

    fn insert_player_nationality(
        &self,
        player_per_nationality: &mut HashMap<String, Vec<PlayerPosition>>,
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
        position: &PlayerPosition
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
        &self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        self.check_offensive_defensive_defensemen(player_per_role, field_players);
        self.check_enf_tough(&player_per_role, field_players);
        self.check_defensive_forward(player_per_role, field_players);
    }

    fn check_offensive_defensive_defensemen(
        &self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        match player_per_role.get(&DefensiveDefenseman) {
            Some(def_positions) => {
                match player_per_role.get(&OffensiveDefenseman) {
                    Some(off_positions) => {
                        for def_position in def_positions {
                            self.change_teamwork_by_position(def_position, DEFENSEMEN_TEAMWORK, field_players);
                        }

                        for off_position in off_positions {
                            self.change_teamwork_by_position(off_position, DEFENSEMEN_TEAMWORK, field_players);
                        }
                    }
                    None => {}
                }
            },
            None => {}
        }
    }

    fn change_teamwork_by_position(
        &self,
        position: &PlayerPosition,
        teamwork: f32,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        let token_id = self.field_players.get(position).unwrap();
        let player = field_players.get_mut(token_id).unwrap();
        player.teamwork = Option::from(player.teamwork.unwrap() * teamwork);
    }

    fn check_enf_tough(
        &self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        if player_per_role.get(&ToughGuy).is_some() {
            self.change_team_work_by_enf_tough(player_per_role, &ToughGuy, field_players);
        }
        if player_per_role.get(&Enforcer).is_some() {
            self.change_team_work_by_enf_tough(player_per_role, &Enforcer, field_players);
        }
    }

    fn change_team_work_by_enf_tough(
        &self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        player_role: &PlayerRole,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        let number_of_tough = player_per_role.get(player_role).unwrap().len();
        let teamwork = TOUGH_ENFORCER_TEAMWORK * (number_of_tough as f32);
        self.change_teamwork_by_role(teamwork, &Playmaker, player_per_role, field_players);
        self.change_teamwork_by_role(teamwork, &Shooter, player_per_role, field_players);
    }

    fn change_teamwork_by_role(
        &self,
        teamwork: f32,
        player_role: &PlayerRole,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        match player_per_role.get(&player_role) {
            Some(playmaker_positions) => {
                for position in playmaker_positions {
                    self.change_teamwork_by_position(position, teamwork, field_players);
                }
            },
            None => {}
        }
    }

    fn check_defensive_forward(
        &self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        if player_per_role.get(&DefensiveForward).is_some() {
            self.change_teamwork_by_position(&LeftDefender, DEFENDERS_TEAMWORK, field_players);
            self.change_teamwork_by_position(&RightDefender, DEFENDERS_TEAMWORK, field_players);
        }
    }

    fn change_teamwork_by_nationality(
        &self,
        player_per_nationality: &HashMap<String, Vec<PlayerPosition>>,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        for (_nationality, positions) in player_per_nationality {
            if positions.len() > 1 {
                for position in positions {
                    self.change_teamwork_by_position(position, NATIONALITY_TEAMWORK, field_players);
                }
            }
        }
    }

    fn change_line_teamwork(
        &self, team_work_line: f32,
        field_players: &mut HashMap<TokenId, FieldPlayer>
    ) {
        for (_position, field_player) in field_players {
            if field_player.teamwork.is_none() {
                field_player.teamwork = Some(1.0);
            }
            field_player.teamwork = Option::from(field_player.teamwork.unwrap() * team_work_line);
        }
    }
}

impl FiveIds {
    pub fn reduce_morale(&self, field_players: &mut HashMap<TokenId, FieldPlayer>) {
        for (_player_position, player_id) in &self.field_players {
            let player = field_players.get_mut(player_id).unwrap();
            player.stats.morale -= 3;
        }
    }

    pub fn increase_morale(&self, field_players: &mut HashMap<TokenId, FieldPlayer>) {
        for (_player_position, player_id) in &self.field_players {
            let player = field_players.get_mut(player_id).unwrap();
            player.stats.morale += 2;
        }
    }

    pub fn get_number_of_players(&self) -> usize {
        let mut count = 0;
        for (_pos, id) in &self.field_players {
            if *id == "" {
                count += 1;
            }
        }

        count
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Tactics {
    Safe,
    Defensive,
    Neutral,
    Offensive,
    Aggressive,
}
