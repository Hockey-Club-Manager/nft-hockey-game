use crate::game::actions::action::ActionData::{Goal, Pass, Rebound, Save, Shot, ShotBlocked, ShotMissed};
use crate::{Game, PlayerPosition};
use crate::game::actions::action::{ActionData, ActionTypes, DoAction};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};
use crate::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};
use crate::team::players::goalie::Goalie;
use crate::user_info::UserId;

const PROBABILITY_SAVE: usize = 30;
const PROBABILITY_SHOT_MISSED: usize = 20;


pub struct ShotAction;
impl DoAction for ShotAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        let opponent_field_player_stat = self.get_opponent_field_player_stats(game);
        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(
            player_with_puck, player_with_puck.stats.get_shooting());

        let user = game.get_user_info(player_with_puck.get_user_id());
        let player_position = user.team.get_field_player_pos(
            &player_with_puck.get_player_id());
        let mut actions = vec![Shot {
            action_type: ActionTypes::Shot,
            account_id: (user.account_id.clone()),
            player_number: player_with_puck.number,
            player_position: player_position.clone()
        }];

        if !has_won(player_stat, opponent_field_player_stat) {
            let opponent_player = game.get_opponent_field_player();
            let opponent_user = game.get_opponent_info(user.user_id);
            let opponent_user_id = opponent_player.1.get_user_id();
            let opponent_player_id = opponent_player.1.get_player_id();
            let opponent_position = opponent_user.team.get_field_player_pos(
                &opponent_player_id);

            actions.push(ShotBlocked {
                action_type: ActionTypes::ShotBlocked,
                account_id: opponent_user.account_id.clone(),
                player_number: opponent_player.1.number,
                player_position: opponent_position.clone()
            });

            game.player_with_puck = Option::from((opponent_user_id, opponent_player_id));
        } else {
            if PROBABILITY_SHOT_MISSED >= Game::get_random_in_range(1, 100, 1) {
                actions.push(self.do_shot_missed(game));
            } else {
                actions.append(&mut self.fight_against_goalie(game, player_stat));
            }
        }

        actions
    }
}

impl ShotAction {
    fn get_opponent_field_player_stats(&self, game: &Game) -> f32 {
        let opponent_field_player = game.get_opponent_field_player();
        get_relative_field_player_stat(
            &opponent_field_player.1,
            (opponent_field_player.1.stats.shot_blocking
                + opponent_field_player.1.stats.defensive_awareness) as f32 / 10.0
        ) * opponent_field_player.0
    }

    fn do_shot_missed(&self, game: &mut Game) -> ActionData {
        let random_user_id = Game::get_random_in_range(1, 2, 19);
        let user_with_puck_id = game.get_user_id_player_with_puck();

        let positions = if random_user_id == user_with_puck_id {
            vec![LeftWing, RightWing]
        } else {
            vec![LeftDefender, RightDefender]
        };

        let rnd = Game::get_random_in_range(1, 2, 21);

        let random_position = positions[rnd];
        let player_id = game.get_field_player_id_by_pos(&random_position, random_user_id);

        let user = game.get_user_info(random_user_id);
        let player = user.team.get_field_player(&player_id);

        // random_position may not be available. get_field_player_id_by_pos will return the player to a different position
        let player_position = user.team.get_field_player_pos(&player_id);

        let action = ShotMissed {
            action_type: ActionTypes::ShotMissed,
            account_id: user.account_id.clone(),
            player_number: player.number,
            player_position: player_position.clone(),
        };

        game.player_with_puck = Option::from((random_user_id, player_id));

        action
    }

    fn fight_against_goalie(&self, game: &mut Game, field_player_stat: f32) -> Vec<ActionData> {
        let user_id_player_with_puck = game.get_user_id_player_with_puck();
        return if self.is_goalie_out(game, &user_id_player_with_puck) {
            self.score_goal(game, &user_id_player_with_puck)
        } else {
            let user_id = game.player_with_puck.as_ref().unwrap().0;
            let user_opponent = game.get_opponent_info(user_id);
            let number_goalie = user_opponent.team.active_goalie.clone();
            let opponent_goalie = user_opponent.team.goalies.get(&number_goalie).unwrap();

            let pass_before_shot = self.has_pass_before_shot(game);
            let reflexes = opponent_goalie.get_reflexes_rel_pass(pass_before_shot);
            let goalie_stat = self.get_relative_goalie_stat(
                opponent_goalie, reflexes
            );

            if has_won(field_player_stat, goalie_stat) {
                self.score_goal(game, &user_id_player_with_puck)
            } else {
                if PROBABILITY_SAVE >= Game::get_random_in_range(1, 100, 2) {
                    vec![Save {
                        action_type: ActionTypes::Save,
                        account_id: user_opponent.account_id.clone(),
                        goalie_number: opponent_goalie.number,
                    }]
                } else {
                    let goalie_number = opponent_goalie.number;
                    vec![self.do_rebound(game, goalie_number)]
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

    fn score_goal(&self, game: &mut Game, user_id: &usize) -> Vec<ActionData> {
        self.change_morale_after_goal(game);
        game.get_user_info_mut(user_id).team.score += 1;

        let penalty_action = if *user_id == 1 as usize {
            game.remove_penalty_players(&2)
        } else {
            game.remove_penalty_players(&1)
        };

        let mut actions = Vec::new();
        if penalty_action.is_some() {
            actions.push(penalty_action.unwrap());
        }

        let user = game.get_user_info(user_id.clone());
        let player_with_puck = game.get_player_with_puck();

        let (pass_player_name, pass_player_num) = match game.last_action.clone() {
            Pass { from_player_name, from_player_number, .. } => {
                (Some(from_player_name), Some(from_player_number))
            },
            _ => (None, None)
        };

        actions.push(Goal {
            action_type: ActionTypes::Goal,
            account_id: user.account_id.clone(),
            player_name1: player_with_puck.name.clone().expect("Player name not found"),
            player_img: player_with_puck.img.clone().expect("Player img not found"),
            player_number1: player_with_puck.number,
            player_name2: pass_player_name,
            player_number2: pass_player_num
        });

        actions
    }

    fn has_pass_before_shot(&self, game: &Game) -> bool {
        match game.last_action {
            Pass {..} => true,
            _ => false
        }
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

    fn do_rebound(&self, game: &mut Game, goalie_number: u8) -> ActionData {
        let random_user_id = Game::get_random_in_range(1, 2, 19);
        let user_with_puck_id = game.get_user_id_player_with_puck();

        let positions = if random_user_id == user_with_puck_id {
            vec![LeftWing, RightWing, Center]
        } else {
            vec![LeftDefender, RightDefender, Center]
        };

        let rnd = Game::get_random_in_range(1, 3, 20);

        let random_position = positions[rnd];
        let player_id = game.get_field_player_id_by_pos(&random_position, random_user_id.clone());

        let user = game.get_user_info(random_user_id);
        let player = user.team.get_field_player(&player_id);
        let player_position = user.team.get_field_player_pos(&player_id);

        let action = Rebound {
            action_type: ActionTypes::Rebound,
            account_id: user.account_id.clone(),
            goalie_number,
            player_number: player.number,
            player_position: player_position.clone(),
        };

        game.player_with_puck = Option::from((random_user_id, player_id));

        action
    }
}