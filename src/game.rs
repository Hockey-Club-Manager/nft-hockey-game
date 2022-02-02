use std::collections::HashMap;
use crate::goalie::Goalie;
use crate::player_field::FieldPlayer;
use crate::user::User;
extern crate rand;

use rand::Rng;
use crate::player::ActionType;
use crate::player::ActionType::{Dangle, Move, Pass};

// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct UserInfo {
    user: User,
    field_players: Vec<FieldPlayer>,
    goalie: Goalie,
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
    pub(crate) players: [UserInfo; 2],
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    // pub(crate) total_time_spent: Vec<Timestamp>,
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) number_of_zone: u8,
}

impl Game {
    fn pass(&self, competitor_iq: u128, competitor_strength: u128) {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1, 101);

        if random_number > 20 {
            if self.player_with_puck.as_ref().unwrap().won_pass(competitor_strength) {
                // TODO
            } else {
                // TODO
            }
        } else {
            if self.player_with_puck.as_ref().unwrap().won_battle(competitor_iq) {
                // TODO
            } else {
                // TODO
            }
        }
    }

    fn move_(&self, competitor_strength: u128) {
        if self.player_with_puck.as_ref().unwrap().won_move(competitor_strength) {
            // TODO
        } else {
            // TODO
        }
    }

    fn dangle(&self, competitor_strength: u128) {
        if self.player_with_puck.as_ref().unwrap().won_dangle(competitor_strength){
            // TODO
        } else {
            // TODO
        }
    }

    pub fn make_an_action(&self, competitor: FieldPlayer, action: ActionType) {
        assert_ne!(self.player_with_puck.is_some(), false, "No player with the puck");

        return match action {
            Pass => self.pass(competitor.stats.get_iq(), competitor.stats.get_strength()),
            Move => self.move_(competitor.stats.get_strength()),
            Dangle => self.dangle(competitor.stats.get_strength()),
            _ => panic!("Action is undefined")
        }
    }
}