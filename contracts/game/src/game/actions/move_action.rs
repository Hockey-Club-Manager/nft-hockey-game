use crate::game::actions::action::ActionTypes::{Hit, Move};
use crate::game::actions::action::{DoAction};
use crate::{Game};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};

pub struct MoveAction;
impl DoAction for MoveAction {
    fn do_action(&self, game: &mut Game) {
        game.generate_an_event(Move);

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
            game.generate_an_event(Hit);
        }
    }
}