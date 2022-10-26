use crate::{Game, PlayerPosition, TokenId};
use crate::game::actions::action::{ActionTypes, DoAction};
use crate::game::actions::action::ActionTypes::{Icing, PassCatched};
use crate::team::five::FiveIds;
use crate::team::numbers::FiveNumber::{PenaltyKill1, PenaltyKill2};


const ICING_PROBABILITY: usize = 10;
const POSITION_PROBABILITY: usize = 50;

const PROBABILITY_PASS_CATCH: usize = 25;
const PROBABILITY_DUMP_OUT_TO_DEFENDER: usize = 50;


pub struct DumpAction;

impl DoAction for DumpAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionTypes> {
        return if game.zone_number == 2 {
            self.do_dump_in(game)
        } else {
            self.do_dump_out(game)
        }
    }
}

impl DumpAction {
    fn do_dump_in(&self, game: &mut Game) -> Vec<ActionTypes> {
        let icing_actions = self.is_icing_in_neutral_zone(game);
        if icing_actions.is_none() {
            self.dump_to_attack_zone(game);

            return vec![ActionTypes::DumpIn];
        }

        icing_actions.unwrap()
    }

    fn dump_to_attack_zone(&self, game: &mut Game) {
        let user_player_id = game.get_player_id_with_puck();

        let user = game.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();

        let player_position = game.get_player_pos(&user_player_id.1, user_player_id.0);
        let position_to_dump = self.get_random_pos_to_dump(player_position, &active_five);

        let player_id_to_dump = active_five.field_players.get(position_to_dump).unwrap();

        game.player_with_puck = Option::from((user.user_id, player_id_to_dump.clone()));

        if user_player_id.0 == 1 {
            game.zone_number = 3;
        } else {
            game.zone_number = 1;
        }
    }

    fn is_icing_in_neutral_zone(
        &self,
        game: &mut Game
    ) -> Option<Vec<ActionTypes>> {
        let player_with_puck = game.get_player_id_with_puck();

        let user = game.get_user_info(player_with_puck.0);
        let active_five = user.team.get_active_five();

        match active_five.number {
            PenaltyKill1 | PenaltyKill2 => {},
            _ => {
                match user.team.get_field_player_pos(&player_with_puck.1) {
                    PlayerPosition::LeftDefender | PlayerPosition::RightDefender => {
                        let rnd = Game::get_random_in_range(1, 100, 3);

                        if ICING_PROBABILITY >= rnd {
                            let mut actions = vec![ActionTypes::DumpIn, Icing];

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
        five: &FiveIds
    ) -> &PlayerPosition {
        if five.number == PenaltyKill1 || five.number == PenaltyKill2 {
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

    fn do_dump_out(&self, game: &mut Game) -> Vec<ActionTypes> {
        let pass_actions = self.is_pass_catch(game);
        if pass_actions.is_some() {
            return pass_actions.unwrap();
        }

        let icing_actons = self.is_icing_in_defender_zone(game);
        if icing_actons.is_some() {
            return icing_actons.unwrap();
        }

        let rnd = Game::get_random_in_range(1, 100, 9);

        if PROBABILITY_DUMP_OUT_TO_DEFENDER >= rnd {
            self.dump_to_attack_zone(game);
        } else {
            self.dump_neutral_zone(game);
        }

        vec![ActionTypes::DumpOut]
    }

    fn is_pass_catch(&self, game: &mut Game) -> Option<Vec<ActionTypes>> {
        let rnd = Game::get_random_in_range(1, 100, 7);

        if PROBABILITY_PASS_CATCH >= rnd {
            let user_player_id = game.get_player_id_with_puck();
            let player_position = game.get_player_pos(&user_player_id.1, user_player_id.0.clone());

            let interception_position = self.get_interception_position(player_position);

            let opponent = game.get_opponent_info(user_player_id.0);
            let opponent_active_five = opponent.team.get_active_five();

            let field_player_id = opponent_active_five.field_players.get(&interception_position).unwrap();
            game.player_with_puck = Option::from((opponent.user_id, field_player_id.clone()));

            let mut events = vec![ActionTypes::DumpOut, PassCatched];

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

    fn is_icing_in_defender_zone(&self, game: &mut Game) -> Option<Vec<ActionTypes>> {
        let rnd = Game::get_random_in_range(1, 100, 8);
        let player_with_puck = game.get_player_id_with_puck();

        let user = game.get_user_info(player_with_puck.0);
        let active_five = user.team.get_active_five();

        match active_five.number {
            PenaltyKill1 | PenaltyKill2 => {},
            _ => {
                if ICING_PROBABILITY >= rnd {
                    let mut actions = vec![ActionTypes::DumpOut, Icing];

                    return Some(actions);
                }
            }
        }

        None
    }

    fn dump_neutral_zone(&self, game: &mut Game) {
        let user_player_id = game.get_player_id_with_puck();
        let position_to_dump = self.get_random_winger_position(game, &user_player_id);

        let user = game.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();

        let player_id_to_dump = active_five.field_players.get(&position_to_dump).unwrap();

        game.player_with_puck = Option::from((user.user_id, player_id_to_dump.clone()));

        game.zone_number = 2;
    }

    fn get_random_winger_position(
        &self,
        game: &mut Game,
        user_player_id: &(usize, TokenId)
    ) -> PlayerPosition {
        let player_position = game.get_player_pos(&user_player_id.1, user_player_id.0);

        let mut positions = vec![PlayerPosition::RightWing, PlayerPosition::Center, PlayerPosition::LeftWing];
        match positions.binary_search(player_position) {
            Ok(index) => {
                positions.remove(index);
            }
            _ => {}
        };

        let rnd = Game::get_random_in_range(1, positions.len(), 10);

        return positions[rnd];
    }
}