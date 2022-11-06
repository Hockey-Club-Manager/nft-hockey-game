use crate::{Game, PlayerPosition, TokenId};
use crate::game::actions::action::{ActionData, ActionTypes, DoAction};
use crate::game::actions::action::ActionData::{Dump, Icing, PassCaught};
use crate::game::actions::action::ActionTypes::{DumpIn, DumpOut};
use crate::team::five::{ActiveFive, FiveIds};
use crate::team::numbers::FiveNumber::{PenaltyKill1, PenaltyKill2};


const ICING_PROBABILITY: usize = 10;
const POSITION_PROBABILITY: usize = 50;

const PROBABILITY_PASS_CATCH: usize = 25;
const PROBABILITY_DUMP_OUT_TO_DEFENDER: usize = 50;


pub struct DumpAction;

impl DoAction for DumpAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        return if game.zone_number == 2 {
            self.do_dump_in(game)
        } else {
            self.do_dump_out(game)
        }
    }
}

impl DumpAction {
    fn do_dump_in(&self, game: &mut Game) -> Vec<ActionData> {
        let icing_actions = self.is_icing_in_neutral_zone(game);
        if icing_actions.is_none() {
            return self.dump_to_attack_zone(game, DumpIn);
        }

        icing_actions.unwrap()
    }

    fn dump_to_attack_zone(&self, game: &mut Game, action_type: ActionTypes) -> Vec<ActionData> {
        let user_player_id = game.get_player_id_with_puck();

        let user = game.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();

        let player_position = game.get_player_pos(&user_player_id.1, user_player_id.0);
        let position_to_dump = self.get_random_pos_to_dump(player_position, &active_five);

        let player_id_to_dump = game.get_field_player_id_by_pos(
            position_to_dump, user.user_id.clone());

        let player_with_puck = user.team.get_field_player(&user_player_id.1);
        let player_with_puck_position = user.team.get_field_player_pos(
            &player_with_puck.get_player_id());

        let player = user.team.get_field_player(&player_id_to_dump);
        let player_position = user.team.get_field_player_pos(&player.get_player_id());

        let action = vec![Dump {
            action_type,
            account_id: user.account_id.clone(),
            from_player_number: player_with_puck.number,
            from: player_with_puck_position.clone(),
            to_player_number: player.number,
            to: player_position.clone(),
        }];

        if user_player_id.0 == 1 {
            game.zone_number = 3;
        } else {
            game.zone_number = 1;
        }

        game.player_with_puck = Option::from((user_player_id.0, player_id_to_dump.clone()));

        action
    }

    fn is_icing_in_neutral_zone(
        &self,
        game: &mut Game
    ) -> Option<Vec<ActionData>> {
        let player_with_puck_id = game.get_player_id_with_puck();

        let user = game.get_user_info(player_with_puck_id.0);
        let active_five = user.team.get_active_five();

        match active_five.current_number {
            PenaltyKill1 | PenaltyKill2 => {},
            _ => {
                match user.team.get_field_player_pos(&player_with_puck_id.1) {
                    PlayerPosition::LeftDefender | PlayerPosition::RightDefender => {
                        let rnd = Game::get_random_in_range(1, 100, 3);

                        if ICING_PROBABILITY >= rnd {
                            let player_with_puck = user.team.get_field_player(&player_with_puck_id.1);
                            let actions = vec![
                                Icing {
                                    action_type: ActionTypes::Icing,
                                    account_id: user.account_id.clone(),
                                    player_number: player_with_puck.number,
                                }
                            ];

                            return Some(actions);
                        }
                    },
                    _ => {}
                }
            }
        }

        return None;
    }

    fn get_random_pos_to_dump(
        &self,
        player_position: &PlayerPosition,
        five: &ActiveFive
    ) -> &PlayerPosition {
        if five.current_number == PenaltyKill1 || five.current_number == PenaltyKill2 {
            let number_of_field_players = five.get_number_of_players();
            return if number_of_field_players == 4 {
                &PlayerPosition::RightWing
            } else {
                &PlayerPosition::Center
            }
        }

        let rnd = Game::get_random_in_range(1, 100, 4);

        return if POSITION_PROBABILITY >= rnd && PlayerPosition::LeftWing != *player_position {
            &PlayerPosition::LeftWing
        } else if PlayerPosition::RightWing != * player_position {
            &PlayerPosition::RightWing
        } else {
            &PlayerPosition::LeftWing
        }
    }

    fn do_dump_out(&self, game: &mut Game) -> Vec<ActionData> {
        let pass_actions = self.is_pass_catch(game);
        if pass_actions.is_some() {
            return pass_actions.unwrap();
        }

        let icing_actions = self.is_icing_in_defender_zone(game);
        if icing_actions.is_some() {
            return icing_actions.unwrap();
        }

        let rnd = Game::get_random_in_range(1, 100, 9);

        return if PROBABILITY_DUMP_OUT_TO_DEFENDER >= rnd {
            self.dump_to_attack_zone(game, DumpOut)
        } else {
            self.dump_neutral_zone(game)
        };
    }

    fn is_pass_catch(&self, game: &mut Game) -> Option<Vec<ActionData>> {
        let rnd = Game::get_random_in_range(1, 100, 7);

        if PROBABILITY_PASS_CATCH >= rnd {
            let user_player_id = game.get_player_id_with_puck();
            let player_position_with_puck = game.get_player_pos(
                &user_player_id.1, user_player_id.0.clone());
            let user_with_puck = game.get_user_info(user_player_id.0);
            let active_five = user_with_puck.team.get_active_five();
            let position_to_dump = self.get_random_pos_to_dump(
                player_position_with_puck, active_five);
            let player_to_dump = game.get_field_player_by_pos(
                user_player_id.0, position_to_dump);
            let player_with_puck = user_with_puck.team.get_field_player(&user_player_id.1);

            let interception_position = self.get_interception_position(
                player_position_with_puck);

            let opponent = game.get_opponent_info(user_player_id.0);

            let field_player_id = game.get_field_player_id_by_pos(
                &interception_position, opponent.user_id.clone());
            let opponent_player = opponent.team.get_field_player(&field_player_id);

            let events = vec![
                PassCaught {
                    action_type: ActionTypes::PassCaught,
                    account_id: opponent.account_id.clone(),
                    from_player_number: player_with_puck.number,
                    from: player_position_with_puck.clone(),
                    to_player_number: player_to_dump.number,
                    to: position_to_dump.clone(),
                    caught_player_number: opponent_player.number,
                    caught_player_position: interception_position
                }];

            game.player_with_puck = Option::from((opponent.user_id, field_player_id.clone()));

            return Some(events);
        }

        None
    }

    fn get_interception_position(&self, player_position: &PlayerPosition) -> PlayerPosition {
        return match player_position {
            PlayerPosition::LeftDefender | PlayerPosition::LeftWing => {
                PlayerPosition::RightDefender
            },
            PlayerPosition::RightDefender | PlayerPosition::RightWing => {
                PlayerPosition::LeftDefender
            },
            _ => panic!("Unknown position")
        };
    }

    fn is_icing_in_defender_zone(&self, game: &mut Game) -> Option<Vec<ActionData>> {
        let rnd = Game::get_random_in_range(1, 100, 8);
        let player_with_puck_id = game.get_player_id_with_puck();

        let user = game.get_user_info(player_with_puck_id.0);
        let active_five = user.team.get_active_five();

        match active_five.current_number {
            PenaltyKill1 | PenaltyKill2 => {},
            _ => {
                if ICING_PROBABILITY >= rnd {
                    let player_with_puck = user.team.get_field_player(&player_with_puck_id.1);

                    let actions = vec![Icing {
                        action_type: ActionTypes::Icing,
                        account_id: user.account_id.clone(),
                        player_number: player_with_puck.number,
                    }];

                    return Some(actions);
                }
            }
        }

        None
    }

    fn dump_neutral_zone(&self, game: &mut Game) -> Vec<ActionData> {
        game.zone_number = 2;

        let user_player_id = game.get_player_id_with_puck();
        let position_to_dump = self.get_random_winger_position(game, &user_player_id);

        let user = game.get_user_info(user_player_id.0);

        let player_id_to_dump = game.get_field_player_id_by_pos(&position_to_dump,
                                                                user.user_id.clone());

        let player_with_puck = user.team.get_field_player(&user_player_id.1);
        let player_with_puck_position = user.team.get_field_player_pos(&user_player_id.1);

        let player = user.team.get_field_player(&player_id_to_dump);
        let player_position = user.team.get_field_player_pos(&player_id_to_dump);

        let action = vec![Dump {
            action_type: DumpOut,
            account_id: user.account_id.clone(),
            from_player_number: player_with_puck.number,
            from: player_with_puck_position.clone(),
            to_player_number: player.number,
            to: player_position.clone(),
        }];

        game.player_with_puck = Option::from((user.user_id, player_id_to_dump.clone()));

        action
    }

    fn get_random_winger_position(
        &self,
        game: &mut Game,
        user_player_id: &(usize, TokenId)
    ) -> PlayerPosition {
        let player_position = game.get_player_pos(&user_player_id.1, user_player_id.0);

        let mut positions = vec![PlayerPosition::RightWing, PlayerPosition::Center, PlayerPosition::LeftWing];

        for i in 0..positions.len() {
            if positions[i] == *player_position {
                positions.remove(i);
                break;
            }
        }

        let rnd = Game::get_random_in_range(1, positions.len(), 10);

        return positions[rnd];
    }
}