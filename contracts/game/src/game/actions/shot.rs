use crate::game::actions::action::ActionTypes::{Goal, Rebound, Save, Shot};
use crate::{Game, generate_an_event};
use crate::game::actions::action::{DoAction, get_relative_field_player_stat, has_pass_before_shot, has_won};

pub struct ShotAction;
impl DoAction for ShotAction {
    fn do_action(&self, game: &mut Game) {
        generate_an_event(Shot, game);

        let pass_before_shot = has_pass_before_shot(game);
        let opponent = get_opponents_goalie(game);

        let p_w: (f64, f64) = if opponent.get_role() == Post2Post {
            (1.0, 0.7)
        } else {
            (0.7, 1.0)
        };

        let  mut player_stat = get_relative_field_player_stat(&game.player_with_puck.as_ref().unwrap(),
                                                              game.player_with_puck.as_ref().unwrap().stats.get_shooting() as f64);

        let is_goalie_out = if game.player_with_puck.as_ref().unwrap().get_user_id() == 1 {
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
            game.get_user_info(game.player_with_puck.as_ref().unwrap().get_user_id()).team.score += 1;

            generate_an_event(Goal, game);

            game.zone_number = 2;
        } else {
            if PROBABILITY_SAVE_NOT_HAPPENED >= Game::get_random_in_range(1, 101) {
                generate_an_event(Rebound, game);
            } else {
                generate_an_event(Save, game);
            }
        }
    }
}
