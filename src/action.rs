use std::borrow::{Borrow, BorrowMut};
use crate::player::{Player, PlayerPosition, PlayerRole};
use crate::player_field::FieldPlayer;
use crate::game::Game;
use crate::player::PlayerRole::{Dangler, Goon, Passer, Professor, Rock, Shooter, ToughGuy, TryHarder};

extern crate rand;

use rand::Rng;
use crate::goalie::Goalie;
use crate::player::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};

const PROBABILITY_PASS_NOT_HAPPENED: i32 = 20;


trait DoAction {
    fn do_action(&self, game: &mut Game);
}

pub struct Action;

impl Action {
    /*
    0 - pass_probability
    1 - shot_probability
    2 - move_probability
    3 - dangle_probability
     */
    fn get_probability_of_actions(&self, role: PlayerRole) -> Vec<i32> {

        match role {
            Passer => vec![4, 1, 3, 2],
            Professor => vec![4, 1, 3, 2],
            Shooter => vec![2, 4, 1, 3],
            ToughGuy => vec![2, 4, 1, 3],
            TryHarder => vec![3, 2, 4, 1],
            Goon => vec![3, 2, 4, 1],
            Dangler => vec![1, 3, 2, 4],
            Rock => vec![1, 3, 2, 4],
            _ => panic!("Player has no role")
        }
    }

    fn get_random_action(&self, is_attack_zone: bool, role: PlayerRole) -> Box<dyn DoAction> {
        let mut actions = self.get_probability_of_actions(role);

        let mut rng = rand::thread_rng();
        let rnd = rng.gen_range(0, 9);

        let probability_distribution = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4];

        return if !is_attack_zone && actions[3] == probability_distribution[rnd] {
            Box::new(Dangle {})
        } else if !is_attack_zone && actions[2] == probability_distribution[rnd] {
            Box::new(Move {})
        // } else if is_attack_zone && actions[1] == probability_distribution[rnd] {
        //     Box::new(Shot{})
        } else {
            Box::new(Pass {})
        }
    }

    pub fn do_random_action(self, mut game: Game) {
        let mut is_attack_zone = false;
        let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();
        if game.zone_number == 3 && user_id == 1 || game.zone_number == 1 && user_id == 2 {
            is_attack_zone = true;
        }

        let action = self.get_random_action(is_attack_zone, game.player_with_puck.unwrap().get_role());

        action.do_action(&mut game);
    }
}

pub struct Pass;
impl DoAction for Pass {
    fn do_action(&self, game: &mut Game) {
        let opponent = get_opponents_field_player(&game);

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1, 101);

        if random_number > PROBABILITY_PASS_NOT_HAPPENED {
            if has_won(game.player_with_puck.unwrap().stats.get_iq(), opponent.stats.get_iq()) {
                let pass_to = get_another_random_position(game.player_with_puck.as_ref().unwrap().get_player_position());

                let user = &game.users[game.player_with_puck.as_ref().unwrap().get_user_id() - 1];

                match user.field_players.get(&pass_to) {
                    Some(player) => game.player_with_puck = Option::from(*player),
                    None => panic!("Player not found")
                }
            } else {
                game.player_with_puck = Option::from(*opponent);
            }
        } else {
            if !has_won(game.player_with_puck.unwrap().stats.get_strength(), opponent.stats.get_strength()) {
                game.player_with_puck = Option::from(*opponent);
            }
        }
    }
}

pub struct Shot;

pub struct Move;
impl DoAction for Move {
    fn do_action(&self, game: &mut Game) {
        let opponent = get_opponents_field_player(&game);

        if has_won(game.player_with_puck.unwrap().stats.get_skating(), opponent.stats.get_strength()) {
            if game.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
                game.zone_number += 1;
            } else {
                game.zone_number -= 1;
            }
        } else {
            game.player_with_puck = Option::from(*opponent);
        }
    }
}

pub struct Dangle;
impl DoAction for Dangle {
    fn do_action(&self, game: &mut Game) {
        let opponent = get_opponents_field_player(&game);

        if has_won(game.player_with_puck.unwrap().stats.get_iq(), opponent.stats.get_strength()) {
            if game.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
                game.zone_number += 1;
            } else {
                game.zone_number -= 1;
            }
        } else {
            game.player_with_puck = Option::from(*opponent);
        }
    }
}

fn has_won(stat: u128, opponents_stat: u128) -> bool {
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

fn get_another_random_position(player_pos: PlayerPosition) -> PlayerPosition {
    let player_positions = get_other_positions(player_pos);

    let mut rng = rand::thread_rng();
    let random_pos = rng.gen_range(0, 5);

    player_positions[random_pos]
}

fn get_other_positions(player_pos: PlayerPosition) -> Vec<PlayerPosition> {
    let mut player_positions = vec![RightWing, LeftWing, Center, RightDefender, LeftDefender];

    for num in 0..5 {
        if player_pos == player_positions[num] {
            player_positions.remove(num);
            break;
        }
    }

    player_positions
}

fn get_opponents_goalie(game: &Game) -> &Goalie {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        &game.users[1].goalie
    } else {
        &game.users[0].goalie
    }
}

fn get_opponents_field_player(game: &Game) -> &FieldPlayer {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        &game.users[1].field_players[&game.player_with_puck.unwrap().get_player_position()]
    } else {
        &game.users[0].field_players[&game.player_with_puck.unwrap().get_player_position()]
    }
}