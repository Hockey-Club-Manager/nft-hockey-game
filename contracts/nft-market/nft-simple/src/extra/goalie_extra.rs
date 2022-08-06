use crate::*;

use crate::extra::hand::Hand;
use crate::extra::player_position::PlayerPosition;
use crate::extra::player_role::PlayerRole;
use crate::extra::player_type::PlayerType;
use crate::extra::stats::{calculate_rarity, Stats};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GoalieExtra {
    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,
    pub player_type: PlayerType,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub native_position: PlayerPosition,
    pub stats: GoalieStats,
}

#[derive(Serialize, Deserialize)]
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

impl Stats for GoalieStats {
    fn get_rarity(&self) -> Rarity {
        let average_stats = self.get_stats_avg();

        calculate_rarity(average_stats)
    }
}

impl GoalieStats {
    fn get_stats_avg(&self) -> f32 {
        self.get_reflexes() +
            self.get_puck_control() +
            self.get_strength() / 3 as f32
    }

    fn get_reflexes(&self) -> f32 {
        (self.angles as f32 +
            self.breakaway as f32 +
            self.five_hole as f32 +
            self.glove_side_high as f32+
            self.glove_side_low as f32 +
            self.stick_side_high as f32 +
            self.stick_side_low as f32) / 7 as f32
    }

    fn get_puck_control(&self) -> f32 {
        (self.passing as f32 +
            self.poise as f32 +
            self.poke_check as f32 +
            self.puck_playing as f32 +
            self.rebound_control as f32 +
            self.recover as f32) / 6 as f32
    }

    fn get_strength(&self) -> f32 {
        (self.aggressiveness as f32 +
            self.agility as f32 +
            self.durability as f32 +
            self.endurance as f32 +
            self.speed as f32 +
            self.vision as f32 +
            self.morale as f32) / 7 as f32
    }
}