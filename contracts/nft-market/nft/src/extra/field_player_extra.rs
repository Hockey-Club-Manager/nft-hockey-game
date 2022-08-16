use crate::*;

use crate::extra::hand::Hand;
use crate::extra::player_position::PlayerPosition;
use crate::extra::player_role::PlayerRole;
use crate::extra::player_type::PlayerType;
use crate::extra::stats::{calculate_rarity, Stats};
use crate::Rarity::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayerExtra {
    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,
    pub player_type: PlayerType,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub native_position: PlayerPosition,
    pub stats: FieldPlayerStats,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayerStats {
    // Skating
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub endurance: u8,
    pub speed: u8,

    // Shooting
    pub slap_shot_accuracy: u8,
    pub slap_shot_power: u8,
    pub wrist_shot_accuracy: u8,
    pub wrist_shot_power: u8,

    // StickHandling
    pub deking: u8,
    pub hand_eye: u8,
    pub passing: u8,
    pub puck_control: u8,

    // Strength
    pub aggressiveness: u8,
    pub body_checking: u8,
    pub durability: u8,
    pub fighting_skill: u8,
    pub strength: u8,

    // IQ
    pub discipline: u8,
    pub offensive: u8,
    pub poise: u8,
    pub morale: u8,

    // Defense
    pub defensive_awareness: u8,
    pub face_offs: u8,
    pub shot_blocking: u8,
    pub stick_checking: u8,
}

impl Stats for FieldPlayerStats {
    fn get_rarity(&self) -> Rarity {
        let average_stats = self.get_stats_avg();

        calculate_rarity(average_stats)
    }
}

impl FieldPlayerStats {
    fn get_stats_avg(&self) -> f32 {
        (self.get_skating() +
            self.get_shooting() +
            self.get_stick_handling() +
            self.get_strength() +
            self.get_iq() +
            self.get_defense()) / 6 as f32
    }

    fn get_skating(&self) -> f32 {
        (self.acceleration as f32 +
            self.agility as f32 +
            self.balance as f32 +
            self.endurance as f32 +
            self.speed as f32) / 5 as f32
    }

    fn get_shooting(&self) -> f32 {
        (self.slap_shot_accuracy as f32 +
            self.slap_shot_power as f32 +
            self.wrist_shot_accuracy as f32 +
            self.wrist_shot_power as f32) / 4 as f32
    }

    fn get_stick_handling(&self) -> f32 {
        (self.deking as f32 +
            self.hand_eye as f32 +
            self.passing as f32 +
            self.puck_control as f32) / 4 as f32
    }

    fn get_strength(&self) -> f32 {
        (self.aggressiveness as f32 +
            self.body_checking as f32 +
            self.durability as f32 +
            self.fighting_skill as f32 +
            self.strength as f32) / 5 as f32
    }

    fn get_iq(&self) -> f32 {
        (self.discipline as f32 +
            self.offensive as f32 +
            self.poise as f32 +
            self.morale as f32) / 4 as f32
    }

    fn get_defense(&self) -> f32 {
        (self.defensive_awareness as f32 +
            self.face_offs as f32 +
            self.shot_blocking as f32 +
            self.stick_checking as f32) / 4 as f32
    }
}