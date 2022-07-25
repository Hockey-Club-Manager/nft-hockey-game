use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::serde_json;
use crate::{PlayerPosition};
use crate::team::players::goalie::{Goalie};
use crate::team::players::player::{PlayerMetadata};
use crate::team::five::{Five, FiveNumber, GoalieNumber, IceTimePriority, Tactics};
use crate::team::team::Team;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<FiveNumber, FiveMetadata>,
    pub(crate) goalies: HashMap<GoalieNumber, PlayerMetadata>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveMetadata {
    pub(crate) field_players: HashMap<PlayerPosition, PlayerMetadata>,
    pub(crate) number: FiveNumber,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
}

pub fn team_metadata_to_team(team_metadata: TeamMetadata, user_id: usize) -> Team {
    let mut fives = HashMap::new();

    for (number, five_metadata) in team_metadata.fives {
        let mut field_players = HashMap::new();

        for (player_pos, field_player) in five_metadata.field_players {
            field_players.insert(player_pos, to_field_player(field_player, player_pos.clone(), user_id));
        }

        let mut five = Five {
            field_players,
            number: number.clone(),
            ice_time_priority: five_metadata.ice_time_priority,
            tactic: five_metadata.tactic,
            time_field: 0
        };

        five.calculate_teamwork();

        fives.insert(number, five);
    }

    let mut goalies = HashMap::new();
    for (number, goalie) in team_metadata.goalies {
        goalies.insert(number, to_goalie(goalie, user_id));
    }

    Team {
        fives: fives.clone(),
        goalies: goalies.clone(),
        active_five: FiveNumber::First,
        active_goalie: GoalieNumber::MainGoalkeeper,
        score: 0
    }
}

fn to_field_player(field_player_metadata: PlayerMetadata, position: PlayerPosition, user_id: usize) -> FieldPlayer {
    let mut result: FieldPlayer = match field_player_metadata.extra {
        Some(extra) => serde_json::from_str(&extra).unwrap(),
        None => panic!("Extra not found"),
    };

    result.img = field_player_metadata.media;
    result.name = field_player_metadata.title;
    result.player_position = Some(position);
    result.user_id = Some(user_id);

    result.set_position_coefficient();

    result
}

fn to_goalie(goalie_metadata: PlayerMetadata, user_id: usize) -> Goalie {
    let mut result: Goalie = match goalie_metadata.extra {
        Some(extra) => serde_json::from_str(&extra).unwrap(),
        None => panic!("Extra not found"),
    };

    result.img = goalie_metadata.media;
    result.name = goalie_metadata.title;
    result.user_id = Some(user_id);

    result
}