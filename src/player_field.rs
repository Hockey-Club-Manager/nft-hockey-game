use crate::player::{Action, Player, PlayerPosition, PlayerRole};
use crate::player::ActionType::{Dangle, Move, Pass, Shot};
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder, Rock, Dangler};

// #[derive(BorshDeserialize, BorshSerialize)]
pub struct FieldPlayerStats {
    pub(crate) skating: u128,
    shooting: u128,
    strength: u128,
    iq: u128,
    morale: u128,
}

impl FieldPlayerStats {
    pub fn new(skating: u128,
               shooting: u128,
               strength: u128,
               iq: u128,
               morale: u128,)
               -> FieldPlayerStats {
        FieldPlayerStats {
            skating,
            shooting,
            strength,
            iq,
            morale,
        }
    }
}

// #[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct FieldPlayer {
    holds_puck: bool,
    position: PlayerPosition,
    role: PlayerRole,
    user_id: u32,
    pub(crate) stats: FieldPlayerStats,
}

impl FieldPlayer {
    pub fn new(holds_puck: bool,
               position: PlayerPosition,
               role: PlayerRole,
               user_id: u32,
               stats: FieldPlayerStats) -> FieldPlayer {
        FieldPlayer {
            holds_puck,
            position,
            role,
            user_id,
            stats,
        }
    }

    fn probability_of_actions(&self) -> Vec<Action> {
        return match self.role {
            Passer => to_action(4, 1, 3, 2),
            Professor => to_action(4, 1, 3, 2),
            Shooter => to_action(2, 4, 1, 3),
            ToughGuy => to_action(2, 4, 1, 3),
            TryHarder => to_action(3, 2, 4, 1),
            Goon => to_action(3, 2, 4, 1),
            Dangler => to_action(1, 3, 2, 4),
            Rock => to_action(1, 3, 2, 4),
            _ => panic!()
        };
    }
}

impl Player for FieldPlayer {
    fn get_user_id(&self) -> u32 { self.user_id }
    fn get_position(&self) -> PlayerPosition { self.position.into() }
    fn get_role(&self) -> PlayerRole { self.role.into() }
    fn get_holds_puck(&self) -> bool { self.holds_puck }
}

fn to_action(pass_probability: u8, shot_probability: u8, move_probability: u8, dangle_probability: u8) -> Vec<Action> {
    let result:Vec<Action> = vec![
        Action {
            type_action: Pass,
            probability: pass_probability,
        },
        Action {
            type_action: Shot,
            probability: shot_probability,
        },
        Action {
            type_action: Move,
            probability: move_probability,
        },
        Action {
            type_action: Dangle,
            probability: dangle_probability,
        }
    ];

    result
}
