use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::serde_json;
use crate::team::players::goalie::{Goalie};
use crate::team::players::player::{PlayerMetadata};
use crate::team::five::{FiveIds};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::team::team::Team;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<NumberFive, FiveIds>,
    pub(crate) goalies: HashMap<NumberGoalie, TokenMetadata>,
    pub(crate) field_players_metadata: HashMap<TokenId, PlayerMetadata>,
}


pub fn team_metadata_to_team(team_metadata: TeamMetadata, user_id: usize) -> Team {
    let mut fives: HashMap<FiveNumber, FiveIds> = HashMap::new();

    for (number, mut five_ids) in team_metadata.fives {
        five_ids.time_field = Option::from(0 as u8);

        five_ids.calculate_teamwork();

        fives.insert(number, five_ids);
    }

    let mut goalies = HashMap::new();
    for (number, goalie) in team_metadata.goalies {
        goalies.insert(number, to_goalie(goalie, user_id));
    }

    Team {
        fives,
        active_five: FiveNumber::First,
        field_players: to_field_players(&team_metadata.field_players_metadata, &user_id),

        goalies,
        active_goalie: GoalieNumber::MainGoalkeeper,

        score: 0,
    }
}

fn to_field_players(field_players_metadata: &HashMap<TokenId, PlayerMetadata>, user_id: &usize) -> HashMap<TokenId, FieldPlayer> {
    let mut result: HashMap<TokenId, FieldPlayer> = HashMap::new();
    for (token_id, field_player_metadata) in field_players_metadata {
        let field_player = to_field_player(field_player_metadata, user_id);
        result.insert(token_id.clone(), field_player);
    }

    result
}

fn to_field_player(field_player_metadata: &PlayerMetadata, user_id: &usize) -> FieldPlayer {
    let mut result: FieldPlayer = match field_player_metadata.as_ref().extra {
        Some(extra) => serde_json::from_str(&extra).unwrap(),
        None => panic!("Extra not found"),
    };

    result.img = field_player_metadata.as_ref().media;
    result.name = field_player_metadata.as_ref().title;
    result.user_id = Some(*user_id);

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