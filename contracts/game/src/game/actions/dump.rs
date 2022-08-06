use crate::Game;
use crate::game::actions::action::{ActionTypes, DoAction};

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
    }

    fn do_dump_out(&self, game: &mut Game) {
        game.generate_an_event(ActionTypes::DumpOut);
    }
}