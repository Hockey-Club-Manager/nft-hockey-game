use crate::game::actions::action::ActionTypes::{Hit, Move};
use crate::game::actions::action::{DoAction};
use crate::{Game};
use crate::game::actions::utils::{generate_an_event, get_opponents_field_player, get_relative_field_player_stat, has_won};

pub struct MoveAction;
impl DoAction for MoveAction {
    fn do_action(&self, game: &mut Game) {
        generate_an_event(Move, game);

        let opponent = get_opponents_field_player(game);
        let opponent_stat = get_relative_field_player_stat(
            &opponent,
            (opponent.stats.defensive_awareness + opponent.stats.get_strength()) / 2
        );

        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(&player_with_puck, player_with_puck.stats.get_skating());

        let mut relative_side_zone: i8 = 1;
        if player_with_puck.get_user_id() == 2 {
            relative_side_zone = -1;
        }

        if has_won(player_stat, opponent_stat) {
            game.zone_number += relative_side_zone;
        } else {
            game.player_with_puck = Option::from((opponent.get_user_id(), opponent.get_player_id()));
            generate_an_event(Hit, game);
        }
    }
}
