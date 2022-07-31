use crate::Game;
use crate::game::actions::action::DoAction;

pub struct DumpAction;

impl DoAction for DumpAction {
    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}