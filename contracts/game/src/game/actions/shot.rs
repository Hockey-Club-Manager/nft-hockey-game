use crate::game::actions::action::ActionTypes::{Goal, Rebound, Save, Shot, ShotBlocked, ShotMissed};
use crate::{FieldPlayer, Game};
use crate::game::actions::action::DoAction;
use crate::game::actions::utils::{change_morale_after_goal, generate_an_event, get_opponent_user, get_opponents_field_player, get_opponents_goalie, get_opponents_goalie_number, get_relative_field_player_stat, get_relative_goalie_stat, has_pass_before_shot, has_won};
use crate::team::players::player::PlayerRole;

const PROBABILITY_SAVE: usize = 30;
const PROBABILITY_SHOT_MISSED: usize = 20;


pub struct ShotAction;
impl DoAction for ShotAction {
    fn do_action(&self, game: &mut Game) {
        generate_an_event(Shot, game);

        let mut opponent_field_player_stat = self.get_opponent_field_player_stats(game);
        let player_with_puck = game.get_player_with_puck();
        let mut player_stat = get_relative_field_player_stat(player_with_puck, player_with_puck.stats.get_shooting());

        if !has_won(player_stat, opponent_field_player_stat) {
            generate_an_event(ShotBlocked, game);
        } else {
            if PROBABILITY_SHOT_MISSED >= Game::get_random_in_range(1, 100, 1) {
                generate_an_event(ShotMissed, game);
            } else {
                self.fight_against_goalie(game, player_with_puck, player_stat);
            }
        }
    }
}

impl ShotAction {
    fn get_opponent_field_player_stats(&self, game: &Game) -> f32 {
        let opponent_field_player = get_opponents_field_player(game);
        get_relative_field_player_stat(
            &opponent_field_player,
            (opponent_field_player.stats.shot_blocking + opponent_field_player.stats.defensive_awareness) / 10.0
        )
    }

    fn fight_against_goalie(&self, game: &mut Game, player_with_puck: &FieldPlayer, field_player_stat: f32) {
        let opponent_goalie = get_opponents_goalie(game);

        let is_goalie_out = if player_with_puck.get_user_id() == 1 {
            &game.user1.is_goalie_out
        } else {
            &game.user2.is_goalie_out
        };

        if is_goalie_out {
            self.score_goal(game, player_with_puck);
        } else {
            let pass_before_shot = has_pass_before_shot(game);
            let mut reflexes = opponent_goalie.get_reflexes_rel_pass(pass_before_shot);
            let goalie_stat = get_relative_goalie_stat(opponent_goalie, reflexes);

            if has_won(field_player_stat, goalie_stat) {
                self.score_goal(game, player_with_puck);
            } else {
                if PROBABILITY_SAVE >= Game::get_random_in_range(1, 100, 2) {
                    generate_an_event(Save, game);
                } else {
                    generate_an_event(Rebound, game);
                }
            }
        }
   }

    fn score_goal(&self, game: &mut Game, player_with_puck: &FieldPlayer) {
        change_morale_after_goal(game);
        game.get_user_info(player_with_puck.get_user_id()).team.score += 1;

        generate_an_event(Goal, game);

        game.zone_number = 2;
    }
}
