use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};


#[derive(PartialEq, Clone, Copy, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub enum PlayerPosition {
    Center,
    LeftWing,
    RightWing,
    LeftDefender,
    RightDefender,
    GoaliePos,
}

#[derive(PartialEq, Clone, Copy, BorshDeserialize, BorshSerialize)]
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

