use crate::player::{Player, PlayerPosition, PlayerRole};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FieldPlayerStats {
    skating: u128,
    shooting: u128,
    pub(crate) strength: f64,
    iq: u128,
    pub(crate) morale: u128,
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
    // TODO nft_token
    pub(crate) position: PlayerPosition,
    role: PlayerRole,
    user_id: usize,
    pub(crate) stats: FieldPlayerStats,
}

impl FieldPlayer {
    pub fn new(position: PlayerPosition,
               role: PlayerRole,
               user_id: usize,
               stats: FieldPlayerStats) -> FieldPlayer {
        FieldPlayer {
            position,
            role,
            user_id,
            stats,
        }
    }

    pub fn get_player_position(&self) -> PlayerPosition { self.position }

    pub fn get_role(&self) -> PlayerRole { self.role }

    pub fn get_user_id(&self) -> usize { self.user_id }
}

impl Player for FieldPlayer {
    fn get_user_id(&self) -> usize { self.user_id }
    fn get_role(&self) -> PlayerRole { self.role.into() }
}

