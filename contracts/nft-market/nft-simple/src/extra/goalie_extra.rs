use crate::*;

use crate::extra::hand::Hand;
use crate::extra::player_position::PlayerPosition;
use crate::extra::player_role::PlayerRole;
use crate::extra::stats::{calculate_rarity, Stats};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GoalieExtra {
    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub player_position: PlayerPosition,
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
        let average_stats = self.get_stats_sum() as f32 / 20 as f32;

        calculate_rarity(average_stats)
    }
}

impl GoalieStats {
    fn get_stats_sum(&self) -> u16 {
        self.get_reflexes() +
            self.get_puck_control() +
            self.get_strength()
    }

    fn get_reflexes(&self) -> u16 {
        self.angles as u16 +
            self.breakaway as u16 +
            self.five_hole as u16 +
            self.glove_side_high as u16 +
            self.glove_side_low as u16 +
            self.stick_side_high as u16 +
            self.stick_side_low as u16
    }

    fn get_puck_control(&self) -> u16 {
        self.passing as u16 +
            self.poise as u16 +
            self.poke_check as u16 +
            self.puck_playing as u16 +
            self.rebound_control as u16 +
            self.recover as u16
    }

    fn get_strength(&self) -> u16 {
        self.aggressiveness as u16 +
            self.agility as u16 +
            self.durability as u16 +
            self.endurance as u16 +
            self.speed as u16 +
            self.vision as u16 +
            self.morale as u16
    }
}