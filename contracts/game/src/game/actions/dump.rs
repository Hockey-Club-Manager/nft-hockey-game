use crate::{FieldPlayer, Game, PlayerPosition, UserInfo};
use crate::game::actions::action::{ActionTypes, DoAction};
use crate::game::actions::action::ActionTypes::{Icing, PassCatched};
use crate::team::five::FiveIds;
use crate::team::numbers::FiveNumber;


const ICING_PROBABILITY: usize = 10;
const POSITION_PROBABILITY: usize = 50;

const PROBABILITY_PASS_CATCH: usize = 25;


pub struct DumpAction;

impl DoAction for DumpAction {
    fn do_action(&self, game: &mut Game) {
        if game.zone_number == 2 {
            self.do_dump_in(game);
        } else {
            self.do_dump_out(game);
        }
    }
}

impl DumpAction {
    fn do_dump_in(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::DumpIn);

        if self.is_icing_in_neutral_zone(game) {
            return;
        }

        let user_player_id = game.get_player_id_with_puck();
        let position_to_dump = self.get_random_wing_pos();

        let user = game.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();

        let player_id_to_dump = active_five.field_players.get(position_to_dump).unwrap();

        game.player_with_puck = Option::from((user_player_id.0, player_id_to_dump.clone()));

        if user_player_id.0 == 1 {
            game.zone_number = 3;
        } else {
            game.zone_number = 1;
        }
    }

    fn is_icing_in_neutral_zone(
        &self,
        game: &mut Game
    ) -> bool {
        let player_with_puck = game.get_player_id_with_puck();

        let user = game.get_user_info(player_with_puck.0);
        let active_five = user.team.get_active_five();

        match active_five.number {
            FiveNumber::PenaltyKill1 | FiveNumber::PenaltyKill2 => {},
            _ => {
                match user.team.get_field_player_pos(&player_with_puck.1) {
                    PlayerPosition::LeftDefender | PlayerPosition::RightDefender => {
                        let rnd = Game::get_random_in_range(1, 100, 3);

                        if ICING_PROBABILITY >= rnd {
                            game.generate_an_event(Icing);
                            return true;
                        }
                    },
                    _ => {}
                }
            }
        }

        false
    }

    fn get_random_wing_pos(&self) -> &PlayerPosition {
        let rnd = Game::get_random_in_range(1, 100, 4);

        return if POSITION_PROBABILITY >= rnd {
            &PlayerPosition::LeftWing
        } else {
            &PlayerPosition::RightWing
        }
    }

    fn do_dump_out(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::DumpOut);

        if self.is_pass_catch(game) {
            return;
        }

        if self.is_icing_in_defender_zone(game) {
            return;
        }
    }

    fn is_pass_catch(&self, game: &mut Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 7);

        if PROBABILITY_PASS_CATCH >= rnd {
            let user_player_id = game.get_player_id_with_puck();
            let player_position = game.get_player_pos(&user_player_id.1, user_player_id.0.clone());

            let interception_position = self.get_interception_position(player_position);

            let opponent = game.get_opponent_info(user_player_id.0);
            let opponent_active_five = opponent.team.get_active_five();

            let field_player_id = opponent_active_five.field_players.get(&interception_position).unwrap();
            game.player_with_puck = Option::from((opponent.user_id, field_player_id.clone()));

            game.generate_an_event(PassCatched);

            return true;
        }

        false
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

    fn is_icing_in_defender_zone(&self, game: &mut Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 8);
        let player_with_puck = game.get_player_id_with_puck();

        let user = game.get_user_info(player_with_puck.0);
        let active_five = user.team.get_active_five();

        match active_five.number {
            FiveNumber::PenaltyKill1 | FiveNumber::PenaltyKill2 => {},
            _ => {
                if ICING_PROBABILITY >= rnd {
                    game.generate_an_event(Icing);

                    return true;
                }
            }
        }

        false
    }
}