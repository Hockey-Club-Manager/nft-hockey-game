use crate::player::{Player, PlayerRole};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GoalieStats {
    pub(crate) glove_and_blocker: u128,
    pub(crate) pads: u128,
    pub(crate) stand: u128,
    pub(crate) stretch: u128,
    pub(crate) morale: u128,
}

impl GoalieStats {
    pub fn new(glove_and_blocker: u128,
               pads: u128,
               stand: u128,
               stretch: u128,
               morale: u128) -> GoalieStats {
        GoalieStats {
            glove_and_blocker,
            pads,
            stand,
            stretch,
            morale,
        }
    }

    pub fn get_glove_and_blocker(&self) -> u128 { self.glove_and_blocker }
    pub fn get_pads(&self) -> u128 { self.pads }
    pub fn get_stand(&self) -> u128 { self.stand }
    pub fn get_stretch(&self) -> u128 { self.stretch }
    pub fn get_morale(&self) -> u128 { self.morale }
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Goalie {
    pub(crate) stats: GoalieStats,

    name: String,
    number: u8,
    role: PlayerRole,
    user_id: usize,
}

impl Goalie {
    pub fn new(stats: GoalieStats,
               name: String,
               number: u8,
               role: PlayerRole,
               user_id: usize,
              ) -> Goalie {
        Goalie {
            stats,
            name,
            number,
            role,
            user_id,
        }
    }
}

impl Player for Goalie {
    fn get_user_id(&self) -> usize { self.user_id }
    fn get_role(&self) -> PlayerRole { self.role.into() }
}
