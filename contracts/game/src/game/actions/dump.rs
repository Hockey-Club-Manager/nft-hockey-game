use crate::{FieldPlayer, Game, PlayerPosition, UserInfo};
use crate::game::actions::action::{ActionTypes, DoAction};
use crate::game::actions::action::ActionTypes::Icing;
use crate::team::five::FiveIds;
use crate::team::numbers::FiveNumber;


const ICING_PROBABILITY: usize = 10;
const POSITION_PROBABILITY: usize = 50;


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

        if self.is_icing(game) {
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

    fn is_icing(
        &self,
        game: &mut Game
    ) -> bool {
        let player_with_puck = game.get_player_with_puck();

        let user = game.get_user_info(player_with_puck.get_user_id());
        let active_five = user.team.get_active_five();

        match active_five.number {
            FiveNumber::PenaltyKill1 | FiveNumber::PenaltyKill2 => {},
            _ => {
                match user.team.get_field_player_pos(&player_with_puck.get_player_id()) {
                    PlayerPosition::LeftDefender | PlayerPosition::RightDefender => {
                        let rnd = Game::get_random_in_range(1, 100, 11);

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
        let rnd = Game::get_random_in_range(1, 100, 12);

        return if POSITION_PROBABILITY >= rnd {
            &PlayerPosition::LeftWing
        } else {
            &PlayerPosition::RightWing
        }
    }

    fn do_dump_out(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::DumpOut);
    }
}