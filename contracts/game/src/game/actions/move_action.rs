use crate::game::actions::action::ActionTypes::{Hit, Move};
use crate::game::actions::action::{DoAction, get_opponents_field_player, get_relative_field_player_stat, has_won};
use crate::{Game, generate_an_event};
use crate::game::actions::utils::{generate_an_event, get_opponents_field_player, get_relative_field_player_stat, has_won};

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

        generate_an_event(Move, game);
        if has_won(player_stat, opponent_stat) {
            game.zone_number += relative_side_zone;
        } else {
            game.player_with_puck = Option::from(opponent);
            generate_an_event(Hit, game);
        }
    }
}
