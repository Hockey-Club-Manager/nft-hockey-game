use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Timestamp};

use std::borrow::Borrow;
use std::collections::HashMap;
use crate::goalie::Goalie;
use crate::player_field::FieldPlayer;
use crate::user::User;
extern crate rand;

use rand::Rng;
use crate::action::{Action, ActionTypes};
use crate::action::ActionTypes::Pass;
use crate::player::PlayerPosition;
use crate::player::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};

pub struct UserInfo {
    pub(crate) user: User,
    pub(crate) field_players: HashMap<PlayerPosition, FieldPlayer>,
    pub(crate) goalie: Goalie,
    pub(crate) account_id: AccountId,
}

pub struct Event {
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: u8,
    pub(crate) time: Timestamp,
    pub(crate) field_players: Vec<FieldPlayer>,
    pub(crate) goalie: Goalie,
}

pub struct EventToSave {
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
}

pub struct Game {
    pub(crate) users: [UserInfo; 2],
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    pub(crate) total_time_spent: Vec<Timestamp>,
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) zone_number: i8,
    pub(crate) events: Vec<EventToSave>,
}

impl Game {

}