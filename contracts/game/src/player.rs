use std::fmt;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Hash, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerPosition {
    Center,
    LeftWing,
    RightWing,
    LeftDefender,
    RightDefender,
    GoaliePos,
}

impl fmt::Display for PlayerPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerPosition::Center => write!(f, "5"),
            PlayerPosition::LeftWing => write!(f, "4"),
            PlayerPosition::RightWing => write!(f, "3"),
            PlayerPosition::LeftDefender => write!(f, "2"),
            PlayerPosition::RightDefender => write!(f, "1"),
            PlayerPosition::GoaliePos => write!(f, "0"),
            _ => write!(f, "-1")
        }
    }
}


#[derive(PartialEq, Clone, Copy, BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerRole {
    // Forward
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

