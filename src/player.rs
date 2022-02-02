use crate::player::ActionType::{Dangle, Move, Pass, Shot};
use crate::player::PlayerPosition::GoaliePos;
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder};

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


