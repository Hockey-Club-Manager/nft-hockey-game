use std::borrow::Borrow;
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder, Rock, Dangler};
extern crate rand;

use rand::Rng;
use crate::player::{Player, PlayerPosition, PlayerRole};


// #[derive(BorshDeserialize, BorshSerialize)]
#[derive(Copy, Clone)]
pub struct FieldPlayerStats {
    skating: u128,
    shooting: u128,
    strength: u128,
    iq: u128,
    morale: u128,
}

impl FieldPlayerStats {
    pub fn new(skating: u128,
               shooting: u128,
               strength: u128,
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
    pub fn get_strength(&self) -> u128 { self.strength }
    pub fn get_iq(&self) -> u128 { self.iq }
    pub fn get_morale(&self) -> u128 { self.morale }
}

// #[derive(BorshDeserialize, BorshSerialize, Clone)]
#[derive(Copy, Clone)]
pub struct FieldPlayer {
    holds_puck: bool,
    position: PlayerPosition,
    role: PlayerRole,
    user_id: usize,
    pub(crate) stats: FieldPlayerStats,
}

impl FieldPlayer {
    pub fn new(holds_puck: bool,
               position: PlayerPosition,
               role: PlayerRole,
               user_id: usize,
               stats: FieldPlayerStats) -> FieldPlayer {
        FieldPlayer {
            holds_puck,
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

