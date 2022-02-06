use std::borrow::Borrow;
use std::collections::HashMap;
use crate::goalie::Goalie;
use crate::player_field::FieldPlayer;
use crate::user::User;
extern crate rand;

use rand::Rng;
use crate::player::PlayerPosition;
use crate::player::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};

// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct UserInfo {
    user: User,
    pub(crate) field_players: HashMap<PlayerPosition, FieldPlayer>,
    pub(crate) goalie: Goalie,
    // pub(crate) account_id: AccountId,
}

/*
// #[derive(BorshDeserialize, BorshSerialize)]
pub struct GameToSave {
    pub(crate) user_1: UserInfo,
    pub(crate) user_2: UserInfo,
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,

    // pub(crate) field: LookupMap<u8, CellData>,
}
*/

pub struct Game {
    pub(crate) users: [UserInfo; 2],
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    // pub(crate) total_time_spent: Vec<Timestamp>,
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) zone_number: u8,
}

impl Game {
}