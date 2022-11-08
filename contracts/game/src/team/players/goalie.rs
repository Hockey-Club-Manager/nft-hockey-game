use crate::*;
use crate::team::players::player::{Hand, PlayerRole, PlayerType};
use near_sdk::serde::{Deserialize, Serialize};
use crate::user_info::UserId;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Goalie {
    pub id: Option<TokenId>,
    pub img: Option<SRC>,
    pub name: Option<String>,
    pub user_id: Option<UserId>,

    pub reality: bool,
    pub nationality: String,
    pub birthday: u64,
    pub player_type: PlayerType,

    pub number: u8,
    pub hand: Hand,
    pub player_role: PlayerRole,
    pub stats: GoalieStats,
}

impl Goalie {
    pub fn get_reflexes_rel_pass(&self, pass_before_shot: bool) -> f32 {
        let reflexes = self.stats.get_reflexes();

        let pass_coeff: f32 = if pass_before_shot {
            match self.player_role {
                PlayerRole::Butterfly => 1.2,
                PlayerRole::Hybrid => 1.0,
                PlayerRole::Standup => 0.8,
                _ => panic!("Incorrect goalie role")
            }
        } else {
            match self.player_role {
                PlayerRole::Butterfly => 0.8,
                PlayerRole::Hybrid => 1.0,
                PlayerRole::Standup => 1.2,
                _ => panic!("Incorrect goalie role")
            }
        };

        reflexes * pass_coeff
    }
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
    pub fn get_reflexes(&self) -> f32 {
        (self.angles as f32 +
            self.breakaway as f32 +
            self.five_hole as f32 +
            self.glove_side_high as f32+
            self.glove_side_low as f32 +
            self.stick_side_high as f32 +
            self.stick_side_low as f32) / 7 as f32
    }

    pub fn get_puck_control(&self) -> f32 {
        (self.passing as f32 +
            self.poise as f32 +
            self.poke_check as f32 +
            self.puck_playing as f32 +
            self.rebound_control as f32 +
            self.recover as f32) / 6 as f32
    }

    pub fn get_strength(&self) -> f32 {
        (self.aggressiveness as f32 +
            self.agility as f32 +
            self.durability as f32 +
            self.endurance as f32 +
            self.speed as f32 +
            self.vision as f32 +
            self.morale as f32) / 7 as f32
    }

    pub fn increase_strength(&mut self, value: u8) {
        self.aggressiveness += value;
        self.agility += value;
        self.durability += value;
        self.endurance += value;
        self.speed += value;
        self.vision += value;
        self.morale += value;
    }
}
