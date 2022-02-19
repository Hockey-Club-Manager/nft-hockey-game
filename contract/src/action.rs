use crate::player::{Player, PlayerPosition, PlayerRole};
use crate::player_field::FieldPlayer;
use crate::game::{EventToSave, Game};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic;
use crate::player::PlayerRole::{Dangler, Goon, Passer, Post2Post, Professor, Rock, Shooter, ToughGuy, TryHarder};

use crate::action::ActionTypes::{Battle, Dangle, Goal, HitThePuck, Move, Pass};

use crate::goalie::Goalie;
use crate::player::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};
use near_sdk::serde::{Deserialize, Serialize};
use crate::UserInfo;

const PROBABILITY_PASS_NOT_HAPPENED: i32 = 20;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionTypes {
    Pass,
    Shot,
    Move,
    Dangle,
    Battle,
    Goal,
    Save,
    HitThePuck,
    EndOfPeriod,
    Take_TO,
    CoachSpeech,
    GoalieOut,
    GoalieBack,
}

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
        let actions = self.get_probability_of_actions(role);

        let rnd = Game::get_random_in_range(0, 10);

        let probability_distribution = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4];

        return if !is_attack_zone && actions[3] == probability_distribution[rnd] {
            Box::new(DangleAction {})
        } else if !is_attack_zone && actions[2] == probability_distribution[rnd] {
            Box::new(MoveAction {})
        } else if is_attack_zone && actions[1] == probability_distribution[rnd] {
            Box::new(ShotAction{})
        } else {
            Box::new(PassAction {})
        }
    }

    pub fn do_random_action(self, game: &mut Game) {
        let mut is_attack_zone = false;
        let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();
        if game.zone_number == 3 && user_id == 1 || game.zone_number == 1 && user_id == 2 {
            is_attack_zone = true;
        }

        let action = self.get_random_action(is_attack_zone, game.player_with_puck.as_ref().unwrap().get_role());

        reduce_strength(game);

        action.do_action(game);
    }
}

pub struct PassAction;
impl DoAction for PassAction {
    fn do_action(&self, game: &mut Game) {
        let opponent= get_opponents_field_player(game);

        let random_number = Game::get_random_in_range(1, 101);

        if random_number as i32 > PROBABILITY_PASS_NOT_HAPPENED {
            let player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                             game.player_with_puck.as_ref().unwrap().stats.get_iq() as f64);
            let opponent_stat = get_relative_field_player_stat(&opponent, opponent.stats.get_iq() as f64);

            if has_won(player_stat, opponent_stat) {
                let pass_to = get_another_random_position(game.player_with_puck.as_ref().unwrap().get_player_position());

                let user = &game.get_user_info(game.player_with_puck.as_ref().unwrap().get_user_id());

                match user.field_players.get(&pass_to.to_string()) {
                    Some(player) => game.player_with_puck = Option::from(*player),
                    None => panic!("Player not found")
                }

                generate_an_event(Pass, game);
            } else {
                game.player_with_puck = Option::from(opponent);
                generate_an_event(Battle, game);
            }
        } else {
            let player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                             game.player_with_puck.as_ref().unwrap().stats.get_strength());
            let opponent_stat = get_relative_field_player_stat(&opponent, opponent.stats.get_strength());

            if !has_won(player_stat, opponent_stat) {
                game.player_with_puck = Option::from(opponent);
            }

            generate_an_event(Battle, game);
        }
    }
}

pub struct ShotAction;
impl DoAction for ShotAction {
    fn do_action(&self, game: &mut Game) {
        let pass_before_shot = has_pass_before_shot(game);
        let opponent = get_opponents_goalie(game);

        let p_w: (f64, f64) = if opponent.get_role() == Post2Post {
            (1.0, 0.7)
        } else {
            (0.7, 1.0)
        };

        let  mut player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                                 game.player_with_puck.as_ref().unwrap().stats.get_shooting() as f64);

        let is_goalie_out = if game.player_with_puck.unwrap().get_user_id() == 1 {
            &game.user1.is_goalie_out
        } else {
            &game.user2.is_goalie_out
        };

        if *is_goalie_out {
            player_stat += 20.0;
        }

        let opponent_user = get_opponent_user(game);
        let opponent_stat = if opponent_user.is_goalie_out {
          10.0
        } else if pass_before_shot {
            (((opponent.stats.stand + opponent.stats.stretch) as f64 * p_w.0) / 2 as f64 +
                opponent.stats.morale as f64) / 2 as f64
        } else {
            (((opponent.stats.glove_and_blocker + opponent.stats.pads) as f64 * p_w.1) / 2 as f64 +
                opponent.stats.morale as f64) / 2 as f64
        };

        if has_won(player_stat, opponent_stat as f64) {
            change_morale_after_a_goal(game);
            game.get_user_info(game.player_with_puck.as_ref().unwrap().get_user_id()).user.score += 1;

            generate_an_event(Goal, game);

            game.zone_number = 2;
        } else {
            generate_an_event(HitThePuck, game);

            let player_pos = get_random_position_after_rebound();
            battle_by_position(player_pos, game);

            generate_an_event(Battle, game);
        }
    }
}

pub struct MoveAction;
impl DoAction for MoveAction {
    fn do_action(&self, game: &mut Game) {
        let opponent = get_opponents_field_player(game);

        let player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                         game.player_with_puck.as_ref().unwrap().stats.get_skating() as f64);
        let opponent_stat = get_relative_field_player_stat(&opponent, opponent.stats.get_strength());

        let mut relative_side_zone: i8 = 1;
        if game.player_with_puck.as_ref().unwrap().get_user_id() == 2 {
            relative_side_zone = -1;
        }

        if has_won(player_stat, opponent_stat) {
            if game.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
                game.zone_number += relative_side_zone;
            } else {
                game.zone_number -= relative_side_zone;
            }

            generate_an_event(Move, game);
        } else {
            game.player_with_puck = Option::from(opponent);
            generate_an_event(Battle, game);
        }
    }
}

pub struct DangleAction;
impl DoAction for DangleAction {
    fn do_action(&self, game: &mut Game) {
        let opponent = get_opponents_field_player(game);

        let player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                                 game.player_with_puck.as_ref().unwrap().stats.get_iq() as f64);
        let opponent_stat = get_relative_field_player_stat(&opponent, opponent.stats.get_strength());

        let mut relative_side_zone: i8 = 1;
        if game.player_with_puck.as_ref().unwrap().get_user_id() == 2 {
            relative_side_zone = -1;
        }

        if has_won(player_stat, opponent_stat) {
            if game.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
                game.zone_number += relative_side_zone;
            } else {
                game.zone_number -= relative_side_zone;
            }

            generate_an_event(Dangle, game);
        } else {
            game.player_with_puck = Option::from(opponent);

            generate_an_event(Battle, game);
        }
    }
}

pub fn has_won(stat: f64, opponents_stat: f64) -> bool {
    let sum = stat + opponents_stat;

    let random_number = Game::get_random_in_range(1, sum.round() as usize + 1);

    return if stat > opponents_stat {
        if random_number as f64 > opponents_stat {
            true
        } else {
            false
        }
    } else {
        if random_number as f64 > stat {
            false
        } else {
            true
        }
    }
}

fn get_another_random_position(player_pos: PlayerPosition) -> PlayerPosition {
    let player_positions = get_other_positions(player_pos);

    let random_pos = Game::get_random_in_range(0, 4);

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
        &game.user2.goalie
    } else {
        &game.user1.goalie
    }
}

pub fn get_opponents_field_player(game: &mut Game) -> FieldPlayer {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        *match game.user2.field_players.get(&game.player_with_puck.as_ref().unwrap().position.to_string()) {
            Some(player) => player,
            _ => panic!("Player not found")
        }
    } else {
        let user = &game.user1;
        *match user.field_players.get(&game.player_with_puck.as_ref().unwrap().position.to_string()){
            Some(player) => player,
            _ => panic!("Player not found")
        }
    }
}

pub fn get_relative_field_player_stat(player: &FieldPlayer, stat: f64) -> f64 {
    (stat as f64 + player.stats.get_morale() as f64 + player.stats.get_strength() as f64) / 3 as f64
}

pub fn reduce_strength(game: &mut Game) {
    let q = 0.99;
    let n = 20;


    for (_player_pos, field_player) in &mut game.user1.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * f64::powf(q, (n - 1) as f64);
    }
    for (_player_pos, field_player) in &mut game.user2.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * f64::powf(q, (n - 1) as f64);
    }
}

fn change_morale_after_a_goal(game: &mut Game) {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    let player_goalie = &mut game.get_user_info(user_id).goalie;
    player_goalie.stats.morale += 2;

    for (_player_pos, field_player) in &mut game.get_user_info(user_id).field_players.iter_mut() {
        field_player.stats.morale += 2;
    }

    let mut opponent_id = 1;
    if user_id == 1 {
        opponent_id = 2;
    }

    game.get_user_info(opponent_id).goalie.stats.morale -= 1;

    for (_player_pos, field_player) in &mut game.get_user_info(opponent_id).field_players.iter_mut() {
        field_player.stats.morale -= 1;
    }
}

pub fn has_pass_before_shot(game: &Game) -> bool {
    if game.events.len() == 0 {
        return false;
    }

    let action = &game.events[game.events.len() - 1].action;
    if *action == Pass {
        true
    } else {
        false
    }
}

pub fn generate_an_event(action: ActionTypes, game: &mut Game) {
    let new_event = EventToSave {
        action,
        time: game.last_event_generation_time,
        zone_number: game.zone_number,
        player_with_puck: game.player_with_puck,
    };

    game.events.push(new_event);
}

fn get_random_position_after_rebound() -> PlayerPosition {
    let rnd = Game::get_random_in_range(0, 10);

    let probability_distribution = vec![1, 1, 2, 2, 3, 3, 3, 3, 4, 5];

    let num_player_pos = probability_distribution[rnd];

    match num_player_pos {
        1 => LeftDefender,
        2 => RightDefender,
        3 => Center,
        4 => LeftWing,
        5 => RightWing,
        _ => panic!("Player position not found")
    }
}

fn battle_by_position(pos: PlayerPosition, game: &mut Game) {
    let player1 = &game.user1.field_players.get(&pos.to_string());
    let player2 = &game.user2.field_players.get(&pos.to_string());

    let player1_stat = match player1 {
        Some(player) => get_relative_field_player_stat(player, player.stats.strength),
        _ => panic!("Player not found")
    };

    let player2_stat = match player2 {
        Some(player) => get_relative_field_player_stat(player, player.stats.strength),
        _ => panic!("Player not found")
    };

    if has_won(player1_stat, player2_stat) {
        match *player1 {
            Some(player) => game.player_with_puck = Option::from(*player),
            _ => panic!("Player not found")
        }
    } else {
        match *player2 {
            Some(player) => game.player_with_puck = Option::from(*player),
            _ => panic!("Player not found")
        }
    }
}

fn get_opponent_user(game: &Game) -> &UserInfo {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        &game.user2
    } else {
        &game.user1
    }
}