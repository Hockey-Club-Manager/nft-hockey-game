use near_sdk::log;
use crate::game::actions::action::ActionData::{Hit, Move, Offside};
use crate::game::actions::action::{ActionData, ActionTypes, DoAction};
use crate::{Game};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};
use crate::user_info::USER_ID2;

pub struct MoveAction;
impl DoAction for MoveAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(player_with_puck, player_with_puck.stats.get_skating());

        let user = game.get_user_info(player_with_puck.get_user_id());
        let position_player_with_puck = user.team.get_field_player_pos(
            &player_with_puck.get_player_id());

        let opponent = game.get_opponent_field_player();
        let opponent_stat = get_relative_field_player_stat(
            &opponent.1,
            (opponent.1.stats.defensive_awareness as f32 + opponent.1.stats.get_strength()) / 2.0
        ) * opponent.0;

        let mut new_zone_number: u8 = 3;
        if player_with_puck.get_user_id() == USER_ID2 {
            new_zone_number = 1;
        }

        let mut actions = vec![Move {
            action_type: ActionTypes::Move,
            zone_number: new_zone_number,
            account_id: user.account_id.clone(),
            player_number: player_with_puck.number,
            player_position: position_player_with_puck.clone()
        }];

        let rnd_offside = Game::get_random_in_range(1, 100, 21);
        if rnd_offside <= 15 {
            actions.push(Offside {
                action_type: ActionTypes::Move,
                zone_number: 2,
                account_id: user.account_id.clone(),
                player_number: player_with_puck.number,
                player_position: position_player_with_puck.clone()
            });

            game.zone_number = 2;
            return actions;
        }

        if has_won(player_stat, opponent_stat) {
            game.zone_number = new_zone_number;
        } else {
            let opponent_user = game.get_opponent_info(user.user_id);
            let opponent_position = opponent_user.team
                .get_field_player_pos(&opponent.1.get_player_id());

            actions.push(Hit {
                action_type: ActionTypes::Hit,
                account_id: opponent_user.account_id.clone(),
                player_number: opponent.1.number,
                player_position: opponent_position.clone(),
                opponent_number: player_with_puck.number
            });

            game.player_with_puck = Option::from((opponent.1.get_user_id(), opponent.1.get_player_id()));
        }

        actions
    }
}