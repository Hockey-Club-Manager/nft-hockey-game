use crate::game::actions::action::ActionData::{Dangle, Offside, PokeCheck};
use crate::game::actions::action::{ActionData, ActionTypes, DoAction};
use crate::{Game};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};

pub struct DangleAction;
impl DoAction for DangleAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        let player_with_puck = game.get_player_with_puck();
        let player_stat = get_relative_field_player_stat(
            &player_with_puck, player_with_puck.stats.get_stick_handling()
        );

        let user = game.get_user_info(player_with_puck.get_user_id());
        let position_player_with_puck = user.team.get_field_player_pos(
            &player_with_puck.get_player_id());

        let mut actions = vec![Dangle {
            action_type: ActionTypes::Dangle,
            account_id: user.account_id.clone(),
            player_number: player_with_puck.number,
            player_position: position_player_with_puck.clone(),
        }];

        let rnd_offside = Game::get_random_in_range(1, 100, 21);
        if rnd_offside <= 15 {
            actions.push(Offside {
                action_type: ActionTypes::Offside,
                account_id: user.account_id.clone(),
                player_number: player_with_puck.number,
                player_position: position_player_with_puck.clone(),
            });
            game.zone_number = 2;
            return actions;
        }

        let mut relative_side_zone: i8 = 1;
        if player_with_puck.get_user_id() == 2 {
            relative_side_zone = -1;
        }

        let opponent = game.get_opponent_field_player();
        let opponent_stat = get_relative_field_player_stat(
            &opponent.1,
            ((opponent.1.stats.defensive_awareness + opponent.1.stats.stick_checking) as f32 / 2.0) as f32
        ) * opponent.0;
        
        if has_won(player_stat, opponent_stat) {
            game.zone_number += relative_side_zone;
        } else {
            let opponent_user = game.get_opponent_info(user.user_id);
            let opponent_position = opponent_user.team
                .get_field_player_pos(&opponent.1.get_player_id());

            actions.push(PokeCheck {
                action_type: ActionTypes::Hit,
                account_id: opponent_user.account_id.clone(),
                player_number: opponent.1.number,
                player_position: opponent_position.clone(),
            });

            game.player_with_puck = Option::from((opponent.1.get_user_id(), opponent.1.get_player_id()));
        }

        actions
    }
}
