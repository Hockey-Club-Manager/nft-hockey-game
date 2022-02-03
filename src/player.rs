use crate::player::ActionType::{Dangle, Move, Pass, Shot};
use crate::player::PlayerPosition::GoaliePos;
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder};
extern crate rand;

use rand::Rng;

// #[derive(BorshDeserialize, BorshSerialize, PartialEq)]
#[derive(PartialEq, Clone, Copy)]
pub enum PlayerPosition {
    Center,
    LeftWing,
    RightWing,
    LeftDefender,
    RightDefender,
    GoaliePos,
}

// #[derive(BorshDeserialize, BorshSerialize, PartialEq)]
#[derive(PartialEq, Clone, Copy)]
pub enum PlayerRole {
    // Winger
    Passer,
    Shooter,
    TryHarder,
    Dangler,

    // Defender
    Rock,
    Goon,
    Professor,
    ToughGuy,

    // goalie
    Wall,
    Post2Post,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ActionType {
    Pass,
    Shot,
    Move,
    Dangle,
    Battle,
}

pub struct Action {
    pub(crate) type_action: ActionType,
    pub(crate) probability: u8,
}

pub trait Player {
    fn get_user_id(&self) -> u32;
    fn get_position(&self) -> PlayerPosition;
    fn get_role(&self) -> PlayerRole;
    fn get_holds_puck(&self) -> bool;
}

pub fn is_won(stat: u128, opponents_stat: u128) -> bool {
    let sum = stat + opponents_stat;

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1, sum as i32 + 1);

    return if stat > opponents_stat {
        if random_number as u128 > opponents_stat {
            true
        } else {
            false
        }
    } else {
        if random_number as u128 > stat {
            false
        } else {
            true
        }
    }
}
