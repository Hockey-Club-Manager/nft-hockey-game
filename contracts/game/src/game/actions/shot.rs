use crate::game::actions::action::ActionTypes::{Goal, Pass, Rebound, Save, Shot, ShotBlocked, ShotMissed};
use crate::{FieldPlayer, Game};
use crate::game::actions::action::DoAction;
use crate::game::actions::utils::{get_opponent_user, get_relative_field_player_stat, has_won};
use crate::team::players::goalie::Goalie;
use crate::team::players::player::PlayerRole;
use crate::user_info::UserId;

const PROBABILITY_SAVE: usize = 30;
const PROBABILITY_SHOT_MISSED: usize = 20;


pub struct ShotAction;
impl DoAction for ShotAction {
    fn do_action(&self, game: &mut Game) {
        game.generate_an_event(Shot);

        let opponent_field_player_stat = self.get_opponent_field_player_stats(game);
        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(player_with_puck, player_with_puck.stats.get_shooting());

        if !has_won(player_stat, opponent_field_player_stat) {
            game.generate_an_event(ShotBlocked);
        } else {
            if PROBABILITY_SHOT_MISSED >= Game::get_random_in_range(1, 100, 1) {
                game.generate_an_event(ShotMissed);
            } else {
                self.fight_against_goalie(game, player_stat);
            }
        }
    }
}

impl ShotAction {
    fn get_opponent_field_player_stats(&self, game: &Game) -> f32 {
        let opponent_field_player = game.get_opponent_field_player();;
        get_relative_field_player_stat(
            &opponent_field_player,
            (opponent_field_player.stats.shot_blocking + opponent_field_player.stats.defensive_awareness) as f32 / 10.0
        )
    }

    fn fight_against_goalie(&self, game: &mut Game, field_player_stat: f32) {
        let user_id = game.player_with_puck.as_ref().unwrap().0;
        let user = game.get_user_info(user_id);
        let number_goalie = user.team.active_goalie.clone();
        let opponent_goalie = user.team.goalies.get(&number_goalie).unwrap();

        let user_id_player_with_puck = game.get_user_id_player_with_puck();
        if self.is_goalie_out(game, &user_id_player_with_puck) {
            self.score_goal(game, &user_id_player_with_puck);
        } else {
            let pass_before_shot = self.has_pass_before_shot(game);
            let reflexes = opponent_goalie.get_reflexes_rel_pass(pass_before_shot);
            let goalie_stat = self.get_relative_goalie_stat(opponent_goalie, reflexes);

            if has_won(field_player_stat, goalie_stat) {
                self.score_goal(game, &user_id_player_with_puck);
            } else {
                if PROBABILITY_SAVE >= Game::get_random_in_range(1, 100, 2) {
                    game.generate_an_event(Save);
                } else {
                    game.generate_an_event(Rebound);
                }
            }
        }
    }

    fn is_goalie_out(&self, game: &Game, user_id: &usize) -> bool {
        if *user_id == 1 {
            game.user1.is_goalie_out
        } else {
            game.user2.is_goalie_out
        }
    }

    fn score_goal(&self, game: &mut Game, user_id: &usize) {
        self.change_morale_after_goal(game);
        game.get_user_info_mut(user_id).team.score += 1;

        game.generate_an_event(Goal);

        game.zone_number = 2;
    }

    fn has_pass_before_shot(&self, game: &Game) -> bool {
        if game.last_action == Pass {
            return true;
        }

        return false;
    }

    fn get_relative_goalie_stat(&self, player: &Goalie, compared_stat: f32) -> f32 {
        (
            compared_stat as f32 +
                player.stats.morale as f32 +
                player.stats.get_strength()
        ) / 3.0
    }

    fn change_morale_after_goal(&self, game: &mut Game) {
        let user_id = game.player_with_puck.as_ref().unwrap().0;

        self.change_user_players_morale(game, &user_id);

        let mut opponent_id = 1;
        if user_id == 1 {
            opponent_id = 2;
        }

        self.change_opponent_players(game, &opponent_id);
    }

    fn change_user_players_morale(&self, game: &mut Game, user_id: &UserId) {
        let user= game.get_user_info_mut(user_id);
        let goalie_number = &mut user.team.active_goalie;
        let goalie = user.team.goalies.get_mut(goalie_number).unwrap();
        goalie.stats.morale += 2;

        let five = user.team.get_active_five();
        for (_player_pos, field_player) in &five.field_players.clone() {
            let field_player = user.team.get_field_player_mut(field_player);
            field_player.stats.morale += 2;
        }
    }

    fn change_opponent_players(&self, game: &mut Game, user_id: &UserId) {
        let opponent = game.get_user_info_mut(user_id);
        let opponent_goalie_number = &mut opponent.team.active_goalie;
        let opponent_goalie = opponent.team.goalies.get_mut(opponent_goalie_number).unwrap();
        opponent_goalie.stats.morale -= 1;

        let opponent_five = opponent.team.get_active_five();
        for (_player_pos, field_player) in opponent_five.field_players.clone() {
            let field_player = opponent.team.get_field_player_mut(&field_player);
            field_player.stats.morale -= 1;
        }
    }
}