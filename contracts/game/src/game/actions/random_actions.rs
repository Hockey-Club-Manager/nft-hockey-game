use std::arch::global_asm;
use crate::{Game, PlayerPosition};
use crate::game::actions::action::ActionTypes::Battle;
use crate::game::actions::action::{ActionTypes, DoAction};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};


const PROBABILITY_GIVEAWAY: usize = 6;
const PROBABILITY_TAKEAWAY: usize = 15;
const PROBABILITY_PUCK_OUT: f32 = 0.005;
const PROBABILITY_BIG_PENALTY: usize = 1;
const PROBABILITY_SMALL_PENALTY: usize = 10;
const PROBABILITY_FIGHT: f32 = 0.25;
const PROBABILITY_NET_OFF: f32 = 0.01;
const PROBABILITY_BATTLE: usize = 20;


pub trait RandomAction {
    fn check_probability(&self, game: &Game) -> bool;
    fn do_action(&self, game: &mut Game);
}

pub struct Giveaway;
impl RandomAction for Giveaway {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_GIVEAWAY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::Giveaway);

        let rnd = Game::get_random_in_range(1, 100, 12);

        if PROBABILITY_BATTLE >= rnd {
            battle(game);
        } else {
            let opponent_player = game.get_opponent_field_player();
            game.player_with_puck = Option::from((opponent_player.get_user_id(), opponent_player.get_player_id()));
        }
    }
}

fn battle(game: &mut Game) {
    let player_with_puck = game.get_player_with_puck();
    let opponent_player = game.get_opponent_field_player();

    let player1_stat = (
        player_with_puck.stats.puck_control +
        player_with_puck.stats.aggressiveness +
        player_with_puck.stats.strength
    ) as f32 / 3.0;

    let player2_stat = (
        opponent_player.stats.puck_control +
        opponent_player.stats.aggressiveness +
        opponent_player.stats.strength
    ) as f32 / 3.0;

    let compared_stat1 = get_relative_field_player_stat(player_with_puck, player1_stat);
    let compared_stat2= get_relative_field_player_stat(opponent_player, player2_stat);

    if has_won(compared_stat2, compared_stat1) {
        game.player_with_puck = Option::from((opponent_player.get_user_id(), opponent_player.get_player_id()));
    }

    game.generate_an_event(Battle);
}

pub struct Takeaway;
impl RandomAction for Takeaway {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 13);
        if PROBABILITY_TAKEAWAY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::Giveaway);

        let rnd = Game::get_random_in_range(1, 100, 14);

        if PROBABILITY_BATTLE >= rnd {
            battle(game);
        } else {
            let opponent_player = game.get_opponent_field_player();
            game.player_with_puck = Option::from((opponent_player.get_user_id(), opponent_player.get_player_id()));
        }
    }
}

pub struct PuckOut;
impl RandomAction for PuckOut {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 15);
        if PROBABILITY_PUCK_OUT >= rnd as f32 {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::PuckOut);
    }
}

pub struct BigPenalty;
impl RandomAction for BigPenalty {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 16);
        if PROBABILITY_BIG_PENALTY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct SmallPenalty;
impl RandomAction for SmallPenalty {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 17);
        if PROBABILITY_SMALL_PENALTY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct Fight;
impl RandomAction for Fight {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_FIGHT >= rnd as f32 {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        let player_with_puck = game.get_player_with_puck();
        let user_id_with_puck = player_with_puck.get_user_id();

        let opponent_player = game.get_opponent_field_player();

        let compared_stat1 = get_relative_field_player_stat(player_with_puck, player_with_puck.stats.fighting_skill as f32);
        let compared_stat2= get_relative_field_player_stat(opponent_player, opponent_player.stats.fighting_skill as f32);

        if has_won(compared_stat2, compared_stat1) {
            game.player_with_puck = Option::from((opponent_player.get_user_id(), opponent_player.get_player_id()));

            self.increase_morale_opponent_team(game, &user_id_with_puck);
            self.reduce_morale_team_with_puck(game, &user_id_with_puck);
        } else {
            self.increase_morale_team_with_puck(game, &user_id_with_puck);
            self.reduce_morale_opponent_team(game, &user_id_with_puck);
        }

        game.generate_an_event(ActionTypes::Fight);
    }
}

impl Fight {
    fn increase_morale_opponent_team(&self, game: &mut Game, user_with_puck_id: &usize) {
        let opponent_user = game.get_opponent_info_mut(user_with_puck_id);
        opponent_user.team.increase_morale();
    }

    fn reduce_morale_opponent_team(&self, game: &mut Game, user_with_puck_id: &usize) {
        let opponent_user = game.get_opponent_info_mut(user_with_puck_id);
        opponent_user.team.reduce_morale();
    }

    fn increase_morale_team_with_puck(&self, game: &mut Game, user_with_puck_id: &usize) {
        let user = game.get_user_info_mut(user_with_puck_id);
        user.team.increase_morale();
    }

    fn reduce_morale_team_with_puck(&self, game: &mut Game, user_with_puck_id: &usize) {
        let user = game.get_user_info_mut(user_with_puck_id);
        user.team.increase_morale();
    }
}

pub struct NetOff;
impl RandomAction for NetOff {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 18);
        if PROBABILITY_NET_OFF >= rnd as f32 && (game.zone_number != 2) {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::NetOff);
    }
}