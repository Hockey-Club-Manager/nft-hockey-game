use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::json_types::Base64VecU8;
use crate::{Fives, IceTimePriority, PlayerPosition};
use crate::player::PlayerRole;
use crate::team::Goalies;

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
    pub(crate) field_players: HashMap<String, TokenMetadata>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
}

#[derive(Serialize, Deserialize)]
pub struct JsonPlayer {
    pub(crate) name: String,
    pub(crate) number: u8,
    pub(crate) player_type: PlayerType,
    pub(crate) role: PlayerRole,
    pub(crate) native_position: PlayerPosition,
    pub(crate) stats: Vec<u8>,
}