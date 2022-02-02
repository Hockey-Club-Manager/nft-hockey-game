use std::collections::HashMap;
use crate::goalie::Goalie;
use crate::player_field::FieldPlayer;
use crate::user::User;
extern crate rand;

use rand::Rng;
use crate::player::ActionType;
use crate::player::ActionType::{Dangle, Move, Pass, Shot};

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
    fn pass(&self, competitor: FieldPlayer) {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1, 101);

        if random_number > 20 {
            if self.player_with_puck.as_ref()
                                    .unwrap()
                                    .won_pass(competitor.stats.get_strength()) {
                // TODO
            } else {
                // TODO
            }
        } else {
            if self.player_with_puck.as_ref()
                                    .unwrap()
                                    .won_battle(competitor.stats.get_iq()) {
                // TODO
            } else {
                // TODO
            }
        }
    }

    fn move_(&mut self, competitor: FieldPlayer) {
        if self.player_with_puck.as_ref()
                                .unwrap()
                                .won_move(competitor.stats.get_strength()) {
            if self.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
                self.number_of_zone += 1;
            } else {
                self.number_of_zone -= 1;
            }
        } else {
            self.player_with_puck = Option::from(competitor);
        }
    }

    fn dangle(&mut self, competitor: FieldPlayer) {
        if self.player_with_puck.as_ref()
                                .unwrap()
                                .won_dangle(competitor.stats.get_strength()){
            if self.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
                self.number_of_zone += 1;
            } else {
                self.number_of_zone -= 1;
            }
        } else {
            self.player_with_puck = Option::from(competitor);
        }
    }

    fn make_an_action_against_field_player (&mut self, competitor: FieldPlayer, action: ActionType) {
        assert_ne!(self.player_with_puck.is_some(), false, "No player with the puck");

        return match action {
            Pass => self.pass(competitor),
            Move => self.move_(competitor),
            Dangle => self.dangle(competitor),
            _ => panic!("Action is undefined")
        }
    }
}