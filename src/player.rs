use crate::player::PlayerPosition::GoaliePos;
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder};
extern crate rand;

use rand::Rng;

// #[derive(BorshDeserialize, BorshSerialize, PartialEq)]
#[derive(PartialEq, Clone, Copy, Eq, Hash)]
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

pub trait Player {
    fn get_user_id(&self) -> usize;
    fn get_role(&self) -> PlayerRole;
}

