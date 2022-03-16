use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use crate::player::{Player, PlayerRole};
use crate::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayerStats {
    pub(crate) strength: f64,
    pub(crate) iq: u128,
    pub(crate) morale: u128,

    skating: u128,
    shooting: u128,
}

impl FieldPlayerStats {
    pub fn new(skating: u128,
               shooting: u128,
               strength: f64,
               iq: u128,
               morale: u128,)
               -> FieldPlayerStats {
        FieldPlayerStats {
            skating,
            shooting,
            strength,
            iq,
            morale,
        }
    }

    pub fn get_skating(&self) -> u128 { self.skating }
    pub fn get_shooting(&self) -> u128 { self.shooting }
    pub fn get_strength(&self) -> f64 { self.strength }
    pub fn get_iq(&self) -> u128 { self.iq }
    pub fn get_morale(&self) -> u128 { self.morale }
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayer {
    pub(crate) img: SRC,
    pub(crate) stats: FieldPlayerStats,
    pub(crate) native_position: PlayerPosition,
    pub(crate) position: PlayerPosition,
    pub(crate) position_coefficient: f32,

    name: String,
    number: u8,
    role: PlayerRole,
    user_id: usize,
}

impl FieldPlayer {
    pub fn new(native_position: PlayerPosition,
               position: PlayerPosition,
               name: String,
               number: u8,
               role: PlayerRole,
               user_id: usize,
               stats: FieldPlayerStats,
               img: SRC) -> FieldPlayer {
        let mut player = FieldPlayer {
            img,
            stats,
            native_position,
            position,
            position_coefficient: 1.0,
            name,
            number,
            role,
            user_id,
        };

        player.set_position_coefficient();
        player
    }

    pub fn get_player_position(&self) -> PlayerPosition { self.position }

    pub fn get_role(&self) -> PlayerRole { self.role }

    pub fn get_user_id(&self) -> usize { self.user_id }

    pub fn set_position_coefficient(&mut self) {
        let native_pos = 1.0 as f32;
        let other_edge = 0.95 as f32;
        let another_pos = 0.8 as f32;
        let center = 0.75 as f32;

        match self.position {
            Center => match self.native_position {
                Center => self.position_coefficient = native_pos,
                RightWing => self.position_coefficient = another_pos,
                LeftWing => self.position_coefficient = another_pos,
                LeftDefender => self.position_coefficient = another_pos,
                RightDefender => self.position_coefficient = another_pos,
                _ => panic!("Native position not set")
            },
            RightWing => match self.native_position {
                Center => self.position_coefficient = center,
                RightWing => self.position_coefficient = native_pos,
                LeftWing => self.position_coefficient = other_edge,
                LeftDefender => self.position_coefficient = another_pos,
                RightDefender => self.position_coefficient = another_pos,
                _ => panic!("Native position not set")
            },
            LeftWing => match self.native_position {
                Center => self.position_coefficient = center,
                RightWing => self.position_coefficient = other_edge,
                LeftWing => self.position_coefficient = native_pos,
                LeftDefender => self.position_coefficient = another_pos,
                RightDefender => self.position_coefficient = another_pos,
                _ => panic!("Native position not set")
            },
            RightDefender => match self.native_position {
                Center => self.position_coefficient = center,
                RightWing => self.position_coefficient = another_pos,
                LeftWing => self.position_coefficient = another_pos,
                LeftDefender => self.position_coefficient = other_edge,
                RightDefender => self.position_coefficient = native_pos,
                _ => panic!("Native position not set")
            },
            LeftDefender => match self.native_position {
                Center => self.position_coefficient = center,
                RightWing => self.position_coefficient = another_pos,
                LeftWing => self.position_coefficient = another_pos,
                LeftDefender => self.position_coefficient = native_pos,
                RightDefender => self.position_coefficient = other_edge,
                _ => panic!("Native position not set")
            },
            _ => panic!("Position not set")
        }
    }
}

impl Player for FieldPlayer {
    fn get_user_id(&self) -> usize { self.user_id }
    fn get_role(&self) -> PlayerRole { self.role.into() }
}

