use crate::team::players::player::{PlayerPosition, PlayerRole};
use crate::team::players::field_player::FieldPlayer;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::team::players::player::PlayerRole::*;

use crate::game::actions::action::ActionTypes::*;

use crate::team::players::player::PlayerPosition::*;
use near_sdk::serde::{Deserialize, Serialize};
use crate::game::actions::shot::ShotAction;
use crate::game::actions::dangle::DangleAction;
use crate::game::actions::dump::DumpAction;
use crate::game::actions::move_action::MoveAction;
use crate::game::actions::pass::PassAction;

use crate::game::game::{Game};
use crate::team::five::{FiveIds, Tactics};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::user_info::UserInfo;


#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionTypes {
    Pass,
    Shot,
    Move,
    Hit,
    Dangle,
    PokeCheck,
    Battle,
    Goal,
    Save,
    Rebound,
    StartGame,
    EndOfPeriod,
    GameFinished,
    FaceOff,
    PassCatched,
    PuckLose,
    Overtime,

    ShotBlocked,
    ShotMissed,

    TakeTO,
    CoachSpeech,
    GoalieOut,
    GoalieBack,

    FirstTeamChangeActiveFive,
    SecondTeamChangeActiveFive,
}

pub trait DoAction {
    fn do_action(&self, game: &mut Game);
}

pub struct Action;
impl Action {
    /*
        0 - dump_probability
        1 - shot_probability
        2 - move_probability
        3 - dangle_probability
        4 - pass_probability
     */
    fn get_probability_of_actions(&self, role: PlayerRole, active_five: &FiveIds) -> Vec<i32> {
        let mut actions = match role {
            Playmaker => vec![1, 2, 3, 3, 2],
            Enforcer => vec![4, 2, 1, 1, 3],
            Shooter => vec![1, 4, 3, 2, 1],
            TryHarder => vec![2, 1, 4, 1, 3],
            DefensiveForward => vec![4, 2, 1, 1, 3],
            Grinder => vec![4, 1, 4, 1, 1],
            DefensiveDefenseman => vec![4, 2, 1, 1, 3],
            OffensiveDefenseman => vec![1, 4, 2, 3, 1],
            TwoWay => vec![1, 3, 3, 2, 2],
            ToughGuy => vec![3, 1, 4, 1, 1],
            _ => panic!("Player has no role")
        };

        match active_five.number {
            FiveNumber::PowerPlay1 | FiveNumber::PowerPlay2 => {
                actions[0] += 3;
                actions[1] += 2;
            },
            FiveNumber::PenaltyKill1 | FiveNumber::PenaltyKill2 => {
                actions[4] += 3;
            }
            _ => {}
        }

        match active_five.tactic {
            Tactics::Safe => actions[4] += 2,
            Tactics::Defensive => {
                actions[4] += 1;
                actions[0] += 1;
            },
            Tactics::Neutral => {},
            Tactics::Offensive => {
                actions[1] += 1;
                actions[2] += 1;
                actions[3] += 1;
            },
            Tactics::Aggressive => {
                actions[1] += 2;
                actions[2] += 2;
                actions[3] += 2;
            },
        }

        actions
    }

    fn get_random_action(&self, is_attack_zone: bool, role: PlayerRole, active_five: &FiveIds) -> Box<dyn DoAction> {
        let actions = self.get_probability_of_actions(role, active_five);

        let mut percent = 0.0;
        let mut action_probability: Vec<f32> = Vec::new();
        for i in 0..actions.len() {
            percent += actions[i] as f32;
            action_probability.push(percent);
        }
        percent = 100.0 / percent;

        let rnd = Game::get_random_in_range(1, 101, 0) as f32;

        return if !is_attack_zone && percent * action_probability[0] >= rnd {
            Box::new(DumpAction {})
        } else if is_attack_zone && percent * action_probability[2] >= rnd {
            Box::new(ShotAction {})
        } else if !is_attack_zone && percent * action_probability[1] >= rnd {
            Box::new(MoveAction {})
        } else if !is_attack_zone && percent * action_probability[0] >= rnd {
            Box::new(DangleAction {})
        } else {
            Box::new(PassAction {})
        }
    }

    pub fn do_random_action(self, game: &mut Game) {
        let mut is_attack_zone = false;
        let user_player_id = game.get_player_id_with_puck();
        if game.zone_number == 3 && user_player_id.0 == 1 || game.zone_number == 1 && user_player_id.0 == 2 {
            is_attack_zone = true;
        }

        let user = game.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();
        let player_with_puck_role = user.team.get_field_player(&user_player_id.1).player_role;

        let action = self.get_random_action(is_attack_zone, player_with_puck_role, active_five);

        action.do_action(game);
    }
}
