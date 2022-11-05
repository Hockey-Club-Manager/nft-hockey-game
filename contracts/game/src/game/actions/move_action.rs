use near_sdk::log;
use crate::game::actions::action::ActionTypes::{Hit, Move, Offside};
use crate::game::actions::action::{ActionTypes, DoAction};
use crate::{Game};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};

pub struct MoveAction;
impl DoAction for MoveAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionTypes> {
        let mut actions = vec![Move];

        let rnd_offside = Game::get_random_in_range(1, 100, 21);
        if rnd_offside <= 15 {
            actions.push(Offside);
            game.zone_number = 2;
            return actions;
        }

        let opponent = game.get_opponent_field_player();
        let opponent_stat = get_relative_field_player_stat(
            &opponent.1,
            (opponent.1.stats.defensive_awareness as f32 + opponent.1.stats.get_strength()) / 2.0
        ) * opponent.0;

        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(player_with_puck, player_with_puck.stats.get_skating());

        let mut relative_side_zone: i8 = 1;
        if player_with_puck.get_user_id() == 2 {
            relative_side_zone = -1;
        }

        if has_won(player_stat, opponent_stat) {
            game.zone_number += relative_side_zone;
        } else {
            game.player_with_puck = Option::from((opponent.1.get_user_id(), opponent.1.get_player_id()));
            actions.push(Hit);
        }

        actions
    }
}