use crate::player::{Action, Player, PlayerPosition, PlayerRole};

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

pub struct Goalie {
    holds_puck: bool,
    position: PlayerPosition,
    role: PlayerRole,
    user_id: u32,
    stats: GoalieStats,
}

impl Goalie {
    pub fn new(holds_puck: bool,
               position: PlayerPosition,
               role: PlayerRole,
               user_id: u32,
               stats: GoalieStats) -> Goalie {
        Goalie {
            holds_puck,
            position,
            role,
            user_id,
            stats,
        }
    }
}

impl Player for Goalie {
    fn get_user_id(&self) -> u32 { self.user_id }
    fn get_position(&self) -> PlayerPosition { self.position.into() }
    fn get_role(&self) -> PlayerRole { self.role.into() }
    fn get_holds_puck(&self) -> bool { self.holds_puck }
}
