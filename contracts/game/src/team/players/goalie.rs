use crate::*;
use crate::team::players::player::{Hand, PlayerRole};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Goalie {
    pub img: Option<SRC>,
    pub name: Option<String>,
    pub user_id: Option<usize>,

    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,
    pub player_type: String,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub stats: GoalieStats,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GoalieStats {
    // Reflexes
    pub angles: u8,
    pub breakaway: u8,
    pub five_hole: u8,
    pub glove_side_high: u8,
    pub glove_side_low: u8,
    pub stick_side_high: u8,
    pub stick_side_low: u8,

    // Puck control
    pub passing: u8,
    pub poise: u8,
    pub poke_check: u8,
    pub puck_playing: u8,
    pub rebound_control: u8,
    pub recover: u8,

    // strength
    pub aggressiveness: u8,
    pub agility: u8,
    pub durability: u8,
    pub endurance: u8,
    pub speed: u8,
    pub vision: u8,
    pub morale: u8,
}

impl GoalieStats {
}
