use crate::{Game, generate_an_event, PlayerPosition};
use crate::game::actions::action::{DoAction, get_opponents_field_player, get_relative_field_player_stat, has_won};
use crate::game::actions::action::ActionTypes::{Pass, PassCatched, PuckLose};
use crate::game::actions::utils::{generate_an_event, get_another_random_position, get_opponents_field_player, get_relative_field_player_stat, has_won};
use crate::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};

pub struct PassAction;
impl DoAction for PassAction {
    fn do_action(&self, game: &mut Game) {
        let opponent= get_opponents_field_player(game);
        let mut opponent_stat = get_relative_field_player_stat(&opponent, opponent.stats.get_iq());

        let player_with_puck = game.get_player_with_puck();
        let player_with_puck_stat = get_relative_field_player_stat(player_with_puck,
                                                                   player_with_puck.stats.get_iq());

        let player_with_puck_id = game.player_with_puck.unwrap();
        let player_with_puck_pos = *game.get_player_pos(&player_with_puck_id.1, player_with_puck_id.0);

        let pass_to = get_another_random_position(&player_with_puck_pos);
        let is_diagonal_pass = is_diagonal_pass(vec![player_with_puck_pos.clone(), pass_to]);

        if is_diagonal_pass {
            let center = game.get_field_player_by_pos(opponent.user_id.unwrap(), &Center);
            opponent_stat += center.stats.get_iq();
        }

        if has_won(player_with_puck_stat, opponent_stat) {
            let new_player_with_id = game.get_field_player_id_by_pos(player_with_puck.get_user_id(), &pass_to);
            game.player_with_puck = Option::from((player_with_puck.get_user_id(), new_player_with_id.clone()));

            generate_an_event(Pass, game);
        } else {
            let new_player_with_id = game.get_field_player_id_by_pos(opponent.get_user_id(), &player_with_puck_pos);
            game.player_with_puck = Option::from((opponent.get_user_id(), new_player_with_id.clone()));
            generate_an_event(PassCatched, game);
        }
    }
}

fn is_diagonal_pass(positions: Vec<PlayerPosition>) -> bool {
    if positions.contains(&LeftDefender) && positions.contains(&RightWing) ||
        positions.contains(&RightDefender) && positions.contains(&LeftWing) {
        true
    }

    false
}
