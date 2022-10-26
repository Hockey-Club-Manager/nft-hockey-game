use crate::game::actions::action::ActionTypes::{Dangle, PokeCheck};
use crate::game::actions::action::{ActionTypes, DoAction};
use crate::{Game};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};

pub struct DangleAction;
impl DoAction for DangleAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionTypes> {
        let mut actions = Vec::new();
        actions.push(Dangle);

        let opponent = game.get_opponent_field_player();
        let opponent_stat = get_relative_field_player_stat(
            &opponent.1,
            ((opponent.1.stats.defensive_awareness + opponent.1.stats.stick_checking) as f32 / 2.0) as f32
        ) * opponent.0;

        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(
            &player_with_puck, player_with_puck.stats.get_stick_handling()
        );

        let mut relative_side_zone: i8 = 1;
        if player_with_puck.get_user_id() == 2 {
            relative_side_zone = -1;
        }

        if has_won(player_stat, opponent_stat) {
            game.zone_number += relative_side_zone;
        } else {
            game.player_with_puck = Option::from((opponent.1.get_user_id(), opponent.1.get_player_id()));

            actions.push(PokeCheck);
        }

        actions
    }
}
