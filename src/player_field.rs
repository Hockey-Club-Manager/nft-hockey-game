use std::borrow::Borrow;
use crate::player::{Action, ActionType, is_won, Player, PlayerPosition, PlayerRole};
use crate::player::ActionType::{Dangle, Move, Pass, Shot};
use crate::player::PlayerRole::{Goon, Passer, Professor, Shooter, ToughGuy, TryHarder, Rock, Dangler};
extern crate rand;

use rand::Rng;


// #[derive(BorshDeserialize, BorshSerialize)]
#[derive(Copy, Clone)]
pub struct FieldPlayerStats {
    skating: u128,
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

    pub fn get_skating(&self) -> u128 { self.skating }
    pub fn get_shooting(&self) -> u128 { self.shooting }
    pub fn get_strength(&self) -> u128 { self.strength }
    pub fn get_iq(&self) -> u128 { self.iq }
    pub fn get_morale(&self) -> u128 { self.morale }
}

// #[derive(BorshDeserialize, BorshSerialize, Clone)]
#[derive(Copy, Clone)]
pub struct FieldPlayer {
    holds_puck: bool,
    position: PlayerPosition,
    role: PlayerRole,
    user_id: usize,
    pub(crate) stats: FieldPlayerStats,
}

impl FieldPlayer {
    pub fn new(holds_puck: bool,
               position: PlayerPosition,
               role: PlayerRole,
               user_id: usize,
               stats: FieldPlayerStats) -> FieldPlayer {
        FieldPlayer {
            holds_puck,
            position,
            role,
            user_id,
            stats,
        }
    }

    pub fn get_user_id(&self) -> usize { self.user_id }

    fn probability_of_actions(&self) -> Vec<Action> {
        match self.role {
            Passer => to_action(4, 1, 3, 2),
            Professor => to_action(4, 1, 3, 2),
            Shooter => to_action(2, 4, 1, 3),
            ToughGuy => to_action(2, 4, 1, 3),
            TryHarder => to_action(3, 2, 4, 1),
            Goon => to_action(3, 2, 4, 1),
            Dangler => to_action(1, 3, 2, 4),
            Rock => to_action(1, 3, 2, 4),
            _ => panic!("Player has no role")
        }
    }

    /*
    1 - 1
    2 - 2 3
    3 - 4 5 6
    4 - 7 8 9 10
     */
    pub fn get_random_action(&self, is_attack_zone: bool) -> ActionType {
        let mut actions = self.probability_of_actions();
        actions.sort_by(|a, b| b.probability.cmp(&a.probability)); // descending

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1, 11);

        return if random_number >= 7 && (is_attack_zone || actions[0].type_action != Shot) {
            actions[0].type_action
        } else if random_number >= 4 && (is_attack_zone || actions[1].type_action != Shot) {
            actions[1].type_action
        } else if random_number >= 2 && (is_attack_zone || actions[2].type_action != Shot) {
            actions[2].type_action
        } else if random_number == 1 && (is_attack_zone || actions[3].type_action != Shot) {
            actions[3].type_action
        } else {
            actions[2].type_action
        }
    }

    pub fn won_battle(&self, strength: u128) -> bool {
        is_won(self.stats.strength, strength)
    }

    pub fn won_dangle(&self, strength: u128) -> bool {
        is_won(self.stats.iq, strength)
    }

    pub fn won_move(&self, strength: u128) -> bool {
        is_won(self.stats.skating, strength)
    }

    pub fn won_pass(&self, iq: u128) -> bool {
        is_won(self.stats.iq, iq)
    }
}

impl Player for FieldPlayer {
    fn get_user_id(&self) -> usize { self.user_id }
    fn get_position(&self) -> PlayerPosition { self.position.into() }
    fn get_role(&self) -> PlayerRole { self.role.into() }
    fn get_holds_puck(&self) -> bool { self.holds_puck }
}

fn to_action(pass_probability: u8,
             shot_probability: u8,
             move_probability: u8,
             dangle_probability: u8)
             -> Vec<Action> {
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
