use crate::{Game, generate_an_event};
use crate::game::actions::action::{DoAction, get_opponents_field_player, get_relative_field_player_stat, has_won};
use crate::game::actions::action::ActionTypes::{Pass, PassCatched, PuckLose};
use crate::game::actions::utils::{generate_an_event, get_another_random_position, get_opponents_field_player, get_relative_field_player_stat, has_won};

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

                match user.team.active_five.field_players.get(&pass_to) {
                    Some(player) => game.player_with_puck = Option::from(player.clone()),
                    None => panic!("Player not found")
                }

                generate_an_event(Pass, game);
            } else {
                game.player_with_puck = Option::from(opponent);
                generate_an_event(PassCatched, game);
            }
        } else {
            let player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                             game.player_with_puck.as_ref().unwrap().stats.get_strength());
            let opponent_stat = get_relative_field_player_stat(&opponent, opponent.stats.get_strength());

            if !has_won(player_stat, opponent_stat) {
                game.player_with_puck = Option::from(opponent);
            }

            generate_an_event(PuckLose, game);
        }
    }
}
