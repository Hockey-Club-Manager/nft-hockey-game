use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use crate::team::players::player::{Hand, PlayerRole};
use crate::PlayerPosition::*;


#[derive(Clone, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayer {
    pub img: Option<SRC>,
    pub name: Option<String>,
    pub teamwork: Option<f32>,

    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,
    pub player_type: String,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub native_position: PlayerPosition,
    pub stats: FieldPlayerStats,

    pub user_id: Option<usize>,
}

impl FieldPlayer {
    pub fn get_role(&self) -> PlayerRole { self.player_role }

    pub fn get_user_id(&self) -> usize { self.user_id.unwrap() }

    pub fn get_position_coefficient(&self) -> f32 {
        let native_pos = 1.0 as f32;
        let other_edge = 0.95 as f32;
        let another_pos = 0.8 as f32;
        let center = 0.75 as f32;

        match self.player_position {
            Center => match self.native_position {
                Center => native_pos,
                RightWing => another_pos,
                LeftWing => another_pos,
                LeftDefender => another_pos,
                RightDefender => another_pos,
                _ => panic!("Native position not set")
            },
            RightWing => match self.native_position {
                Center =>  center,
                RightWing => native_pos,
                LeftWing => other_edge,
                LeftDefender => another_pos,
                RightDefender => another_pos,
                _ => panic!("Native position not set")
            },
            LeftWing => match self.native_position {
                Center => center,
                RightWing => other_edge,
                LeftWing => native_pos,
                LeftDefender => another_pos,
                RightDefender => another_pos,
                _ => panic!("Native position not set")
            },
            RightDefender => match self.native_position {
                Center => center,
                RightWing =>  another_pos,
                LeftWing => another_pos,
                LeftDefender => other_edge,
                RightDefender => native_pos,
                _ => panic!("Native position not set")
            },
            LeftDefender => match self.native_position {
                Center => center,
                RightWing => another_pos,
                LeftWing => another_pos,
                LeftDefender => native_pos,
                RightDefender => other_edge,
                _ => panic!("Native position not set")
            },
            _ => panic!("Position not set")
        }
    }
}

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize)]
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

impl FieldPlayerStats {
}
