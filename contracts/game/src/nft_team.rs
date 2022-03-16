use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json;
use crate::{Fives, IceTimePriority, PlayerPosition};
use crate::goalie::{Goalie, GoalieStats};
use crate::player::PlayerRole;
use crate::player_field::FieldPlayerStats;
use crate::team::{Five, Goalies, Team};

#[derive(Serialize, Deserialize)]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
}

#[derive(Serialize, Deserialize)]
pub enum PlayerType {
    PlayerField,
    Goalie,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<Fives, FiveMetadata>,
    pub(crate) goalies: HashMap<Goalies, TokenMetadata>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveMetadata {
    pub(crate) field_players: HashMap<PlayerPosition, TokenMetadata>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct  Extra {
    pub number: u8,
    pub player_role: PlayerRole,
    pub player_position: PlayerPosition,
    pub player_type: PlayerType,
    pub stats: Vec<u128>,
}


pub fn team_metadata_to_team(team_metadata: TeamMetadata, user_id: usize) -> Team {
    let mut fives = HashMap::new();

    for (number, five_metadata) in team_metadata.fives {
        let mut field_players = HashMap::new();

        for (player_pos, field_player) in five_metadata.field_players {
            field_players.insert(player_pos, to_field_player(field_player, player_pos.clone(), user_id));
        }

        fives.insert(number, Five {
            field_players,
            number: number.clone(),
            ice_time_priority: five_metadata.ice_time_priority,
            time_field: 0
        });
    }

    let mut goalies = HashMap::new();
    for (number, goalie) in team_metadata.goalies {
        goalies.insert(number, to_goalie(goalie, user_id));
    }

    Team {
        fives: fives.clone(),
        goalies: goalies.clone(),
        active_five: fives.get(&Fives::First).unwrap().clone(),
        active_goalie: goalies.get(&Goalies::MainGoalkeeper).unwrap().clone(),
        score: 0
    }
}

fn to_field_player(field_player_metadata: TokenMetadata, position: PlayerPosition, user_id: usize) -> FieldPlayer {
    let extra: Extra = match field_player_metadata.extra {
        Some(extra) => serde_json::from_str(&extra).unwrap(),
        None => panic!("Extra not found"),
    };

    let img = match field_player_metadata.media {
        Some(img) => img,
        None => String::from("")
    };

    let stats = FieldPlayerStats::new(extra.stats[0],
                                      extra.stats[1],
                                      extra.stats[2] as f64,
                                      extra.stats[3],
                                      extra.stats[4]);

    FieldPlayer::new(
        extra.player_position,
        position,
        field_player_metadata.title.unwrap(),
        extra.number,
        extra.player_role,
        user_id,
        stats,
        img,
    )
}

fn to_goalie(goalie_metadata: TokenMetadata, user_id: usize) -> Goalie {
    let extra: Extra = match goalie_metadata.extra {
        Some(extra) => serde_json::from_str(&extra).unwrap(),
        None => panic!("Extra not found"),
    };

    let img = match goalie_metadata.media {
        Some(media) => media,
        None => String::from("")
    };

    let stats = GoalieStats::new(extra.stats[0],
                                      extra.stats[1],
                                      extra.stats[2],
                                      extra.stats[3],
                                      extra.stats[4]);

    Goalie::new(stats, goalie_metadata.title.unwrap(), extra.number, extra.player_role, user_id, img)
}