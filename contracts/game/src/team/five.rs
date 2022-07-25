use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{FieldPlayer, PlayerPosition};
use crate::PlayerPosition::{LeftDefender, RightDefender};
use crate::team::numbers::FiveNumber;
use crate::team::players::player::PlayerRole;
use crate::team::players::player::PlayerRole::{DefensiveDefenseman, DefensiveForward, Enforcer, OffensiveDefenseman, Playmaker, Shooter, ToughGuy, TryHarder, TwoWayDefenseman};
use crate::team::tactics::{IceTimePriority, Tactics};

const NATIONALITY_TEAMWORK: f32 = 1.05;
const DEFENSEMEN_TEAMWORK: f32 = 1.1;
const TOUGH_ENFORCER_TEAMWORK: f32 = 1.2;
const DEFENDERS_TEAMWORK: f32 = 1.2;


#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Five {
    pub(crate) field_players: HashMap<PlayerPosition, FieldPlayer>,
    pub(crate) number: FiveNumber,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
    pub(crate) time_field: u8,
}

impl Five {
    pub fn calculate_teamwork(&mut self) {
        let mut player_per_nationality: HashMap<String, Vec<PlayerPosition>> = HashMap::new();
        let mut player_per_role: HashMap<PlayerRole, Vec<PlayerPosition>> = HashMap::new();

        let mut team_work_line: f32 = 1.0;

        for (position, field_player) in &mut self.field_players {
            field_player.teamwork = Option::from(field_player.get_position_coefficient());

            self.insert_player_nationality(&mut player_per_nationality, field_player, &position);
            self.insert_player_role(&mut player_per_role, field_player, &position);

            if field_player.player_role == ToughGuy || field_player == Enforcer {
                team_work_line *= 0.9;
            } else if field_player.player_role == TryHarder || field_player == TwoWayDefenseman {
                team_work_line *= 1.1;
            }
        }

        self.change_teamwork_by_roles(&player_per_role);
        self.change_teamwork_by_nationality(&player_per_nationality);
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

    fn change_teamwork_by_roles(&mut self, player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>) {
        self.check_offensive_defensive_defensemen(player_per_role);
        self.check_enf_tough(&player_per_role);
        self.check_defensive_forward(player_per_role);
    }

    fn check_offensive_defensive_defensemen(&mut self, player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>) {
        match player_per_role.get(&DefensiveDefenseman) {
            Some(def_positions) => {
                match player_per_role.get(&OffensiveDefenseman) {
                    Some(off_positions) => {
                        for def_position in def_positions {
                            self.change_teamwork_by_position(def_position, DEFENSEMEN_TEAMWORK);
                        }

                        for off_position in off_positions {
                            self.change_teamwork_by_position(off_position, DEFENSEMEN_TEAMWORK);
                        }
                    }
                    None => {}
                }
            },
            None => {}
        }
    }

    fn change_teamwork_by_position(&mut self, position: &PlayerPosition, teamwork: f32) {
        let player = self.field_players.get_mut(position).unwrap();
        player.teamwork = Option::from(player.teamwork.unwrap() * teamwork);
    }

    fn check_enf_tough(&mut self, player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>) {
        if player_per_role.get(&ToughGuy).is_some() {
            self.change_team_work_by_enf_tough(player_per_role, &ToughGuy);
        }
        if player_per_role.get(&Enforcer).is_some() {
            self.change_team_work_by_enf_tough(player_per_role, &Enforcer);
        }
    }

    fn change_team_work_by_enf_tough(
        &mut self,
        player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>,
        player_role: &PlayerRole
    ) {
        let number_of_tough = player_per_role.get(player_role).unwrap().len();
        let teamwork = TOUGH_ENFORCER_TEAMWORK * number_of_tough;
        self.change_teamwork_by_role(teamwork, &Playmaker, player_per_role);
        self.change_teamwork_by_role(teamwork, &Shooter, player_per_role);
    }

    fn change_teamwork_by_role(&mut self, teamwork: f32, player_role: &PlayerRole,
                               player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>) {
        match player_per_role.get(&player_role) {
            Some(playmaker_positions) => {
                for position in playmaker_positions {
                    self.change_teamwork_by_position(position, teamwork);
                }
            },
            None => {}
        }
    }

    fn check_defensive_forward(&mut self, player_per_role: &HashMap<PlayerRole, Vec<PlayerPosition>>) {
        if player_per_role.get(&DefensiveForward) {
            self.change_teamwork_by_position(&LeftDefender, DEFENDERS_TEAMWORK);
            self.change_teamwork_by_position(&RightDefender, DEFENDERS_TEAMWORK);
        }
    }

    fn change_teamwork_by_nationality(&mut self, player_per_nationality: &HashMap<String, Vec<PlayerPosition>>) {
        for (_nationality, positions) in player_per_nationality {
            if positions.len() > 1 {
                for position in positions {
                    self.change_teamwork_by_position(position, NATIONALITY_TEAMWORK);
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