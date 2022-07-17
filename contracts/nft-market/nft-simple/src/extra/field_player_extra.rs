use crate::*;

use crate::extra::hand::Hand;
use crate::extra::player_position::PlayerPosition;
use crate::extra::player_role::PlayerRole;
use crate::extra::stats::{calculate_rarity, Stats};
use crate::Rarity::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayerExtra {
    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub player_position: PlayerPosition,
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
        let average_stats = self.get_stats_sum() as f32 / 26 as f32;

        calculate_rarity(average_stats)
    }
}

impl FieldPlayerStats {
    fn get_stats_sum(&self) -> u16 {
        self.get_skating() +
            self.get_shooting() +
            self.get_stick_handling() +
            self.get_strength() +
            self.get_iq() +
            self.get_defense()
    }

    fn get_skating(&self) -> u16 {
        self.acceleration as u16 +
            self.agility as u16 +
            self.balance as u16 +
            self.endurance as u16 +
            self.speed as u16
    }

    fn get_shooting(&self) -> u16 {
        self.slap_shot_accuracy as u16 +
            self.slap_shot_power as u16 +
            self.wrist_shot_accuracy as u16 +
            self.wrist_shot_power as u16
    }

    fn get_stick_handling(&self) -> u16 {
        self.deking as u16 +
            self.hand_eye as u16 +
            self.passing as u16 +
            self.puck_control as u16
    }

    fn get_strength(&self) -> u16 {
        self.aggressiveness as u16 +
            self.body_checking as u16 +
            self.durability as u16 +
            self.fighting_skill as u16 +
            self.strength as u16
    }

    fn get_iq(&self) -> u16 {
        self.discipline as u16 +
            self.offensive as u16 +
            self.poise as u16 +
            self.morale as u16
    }

    fn get_defense(&self) -> u16 {
        self.defensive_awareness as u16 +
            self.face_offs as u16 +
            self.shot_blocking as u16 +
            self.stick_checking as u16
    }
}