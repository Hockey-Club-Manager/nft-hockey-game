use std::fmt;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct PlayerMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub starts_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub extra: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
pub enum PlayerType {
    FieldPlayer,
    Goalie,
}

#[derive(Copy, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Hand {
    #[serde(alias = "L")]
    Left,
    #[serde(alias = "R")]
    Right,
}

#[derive(PartialEq, Clone, Copy, BorshDeserialize, BorshSerialize)]
#[derive(Eq, PartialOrd, Hash, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerPosition {
    #[serde(alias = "C")]
    Center,

    #[serde(alias = "LW")]
    LeftWing,

    #[serde(alias = "RW")]
    RightWing,

    #[serde(alias = "LD")]
    LeftDefender,

    #[serde(alias = "RD")]
    RightDefender,

    #[serde(alias = "ADD")]
    AdditionalPosition,

    #[serde(alias = "G")]
    GoaliePos,
}

#[derive(PartialEq, Clone, Copy, BorshDeserialize, BorshSerialize)]
#[derive(Eq, PartialOrd, Hash, Ord)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GoalieSubstitution {
    #[serde(alias = "GS1")]
    GoalieSubstitution1,

    #[serde(alias = "GS2")]
    GoalieSubstitution2,
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
#[derive(Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerRole {
    // Forward
    Playmaker,
    Enforcer,
    Shooter,
    TryHarder,
    DefensiveForward,
    Grinder,

    // Defenseman
    DefensiveDefenseman,
    OffensiveDefenseman,
    TwoWay,
    ToughGuy,

    // goalie
    Standup,
    Butterfly,
    Hybrid,
}