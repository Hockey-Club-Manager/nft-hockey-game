use crate::player::ActionType::{Dangle, Move, Pass, Shot};
use crate::player::PlayerPosition::Goalie;
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder};

// #[derive(BorshDeserialize, BorshSerialize, PartialEq)]
#[derive(PartialEq)]
pub enum PlayerPosition {
    Center,
    LeftWing,
    RightWing,
    LeftDefender,
    RightDefender,
    Goalie,
}

// #[derive(BorshDeserialize, BorshSerialize, PartialEq)]
#[derive(PartialEq)]
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

pub enum ActionType {
    Pass,
    Shot,
    Move,
    Dangle,
}

pub struct Action {
    type_action: ActionType,
    probability: u8,
}

pub enum Stats {
	PlayerStats,
	GoalieStats,
}

// #[derive(BorshDeserialize, BorshSerialize)]
pub struct PlayerStats {
    skating: u128,
    shooting: u128,
    strength: u128,
    iq: u128,
    morale: u128,
}

impl PlayerStats {
    pub fn new(skating: u128,
               shooting: u128,
               strength: u128,
               iq: u128,
               morale: u128,)
               -> PlayerStats {
        PlayerStats {
            skating,
            shooting,
            strength,
            iq,
            morale,
        }
    }
}

// #[derive(BorshDeserialize, BorshSerialize)]
pub struct GoalieStats {
    glove_and_blocker: u128,
    pads: u128,
    stand: u128,
    stretch: u128,
    morale: u128,
}

impl GoalieStats {
    pub fn new(glove_and_blocker: u128,
           pads: u128,
           stand: u128,
           stretch: u128,
           morale: u128) -> GoalieStats {
        GoalieStats {
            glove_and_blocker,
            pads,
            stand,
            stretch,
            morale,
        }
    }
}

// #[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Player {
    holds_puck: bool,
    position: PlayerPosition,
    role: PlayerRole,
    user_id: u32,
    pub(crate) stats: Stats,
}

impl Player {
    pub fn new(holds_puck: bool,
           position: PlayerPosition,
           role: PlayerRole,
           user_id: u32,
           stats: Stats) -> Player {
        Player {
            holds_puck,
            position,
            role,
            user_id,
            stats,
        }
    }

    fn get_user_id(&self) -> u32 { self.user_id }
    fn get_position(&self) -> PlayerPosition { self.position.into() }
    fn get_role(&self) -> PlayerRole { self.role.into() }
    fn get_holds_puck(&self) -> bool { self.holds_puck }

    fn probability_of_actions(&self) -> Vec<Action> {
        if self.role == Passer || self.role == Professor {
            let result:Vec<u8> = vec![4, 1, 3, 2];
            return to_action(result)
        }
        if self.role == Shooter || self.role == ToughGuy {
            let result:Vec<u8> = vec![2, 4, 1, 3];
            return to_action(result)
        }
        if self.role == TryHarder || self.role == Goon {
            let result:Vec<u8> = vec![3, 2, 4, 1];
            return to_action(result)
        }
        else {
            let result:Vec<u8> = vec![1, 3, 2, 4];
            return to_action(result)
        }
    }
}

fn to_action(actions: Vec<u8>) -> Vec<Action> {
    let result:Vec<Action> = vec![
        Action {
            type_action: Pass,
            probability: actions[0],
        },
        Action {
            type_action: Shot,
            probability: actions[1],
        },
        Action {
            type_action: Move,
            probability: actions[2],
        },
        Action {
            type_action: Dangle,
            probability: actions[3],
        }
    ];

    result
}

