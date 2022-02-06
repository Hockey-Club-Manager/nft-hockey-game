use crate::player::{Player, PlayerPosition, PlayerRole};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub struct GoalieStats {
    glove_and_blocker: u128,
    pads: u128,
    stand: u128,
    stretch: u128,
    morale: u128,
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

pub struct Goalie {
    // TODO nft_token
    position: PlayerPosition,
    role: PlayerRole,
    user_id: usize,
    pub(crate) stats: GoalieStats,
}

impl Goalie {
    pub fn new(position: PlayerPosition,
               role: PlayerRole,
               user_id: usize,
               stats: GoalieStats) -> Goalie {
        Goalie {
            position,
            role,
            user_id,
            stats,
        }
    }
}

impl Player for Goalie {
    fn get_user_id(&self) -> usize { self.user_id }
    fn get_role(&self) -> PlayerRole { self.role.into() }
}
