use std::arch::global_asm;
use crate::Game;
use crate::game::actions::action::DoAction;


const PROBABILITY_GIVEAWAY: usize = 6;
const PROBABILITY_TAKEAWAY: usize = 15;
const PROBABILITY_PUCK_OUT: f32 = 0.005;
const PROBABILITY_BIG_PENALTY: usize = 1;
const PROBABILITY_SMALL_PENALTY: usize = 10;
const PROBABILITY_FIGHT: f32 = 0.25;
const PROBABILITY_NET_OFF: f32 = 0.01;


pub trait RandomAction {
    fn check_probability(&self, game: &Game) -> bool;
    fn do_action(&self, game: &mut Game);
}

pub struct Giveaway;
impl RandomAction for Giveaway {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_GIVEAWAY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct Takeaway;
impl RandomAction for Takeaway {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_TAKEAWAY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct PuckOut;
impl RandomAction for PuckOut {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_PUCK_OUT >= rnd as f32 {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct BigPenalty;
impl RandomAction for BigPenalty {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_BIG_PENALTY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct SmallPenalty;
impl RandomAction for SmallPenalty {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_SMALL_PENALTY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct Fight;
impl RandomAction for Fight {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_FIGHT >= rnd as f32 {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}

pub struct NetOff;
impl RandomAction for NetOff {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_NET_OFF >= rnd as f32 {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) {
        todo!()
    }
}