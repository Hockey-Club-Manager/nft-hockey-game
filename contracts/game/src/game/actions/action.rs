use crate::team::players::player::{PlayerRole};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, log};
use crate::team::players::player::PlayerRole::*;

use near_sdk::serde::{Deserialize, Serialize};
use crate::game::actions::shot::ShotAction;
use crate::game::actions::dangle::DangleAction;
use crate::game::actions::dump::DumpAction;
use crate::game::actions::move_action::MoveAction;
use crate::game::actions::pass::PassAction;
use crate::game::actions::random_actions::{BigPenalty, Fight, Giveaway, NetOff, PuckOut, RandomAction, SmallPenalty, Takeaway};

use crate::game::game::{Game};
use crate::team::five::{ActiveFive, FiveIds, Tactics};
use crate::team::numbers::{FiveNumber};
use crate::TokenId;


#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionTypes {
    StartGame,
    EndOfPeriod,
    Overtime,
    GameFinished,

    Pass,
    PassCaught,

    Shot,
    ShotBlocked,
    ShotMissed,
    Goal,
    Save,
    Rebound,

    FaceOff,
    FaceOffWin,

    Move,
    Hit,

    Offside,

    Dangle,
    PokeCheck,

    DumpIn,
    DumpOut,
    Icing,

    Giveaway,
    Takeaway,
    PuckOut,
    BigPenalty,
    SmallPenalty,
    NetOff,
    Fight,
    Battle,

    PuckLose,

    TakeTO,
    CoachSpeech,
    GoalieOut,
    GoalieBack,

    PenaltyShot,

    FirstTeamChangeActiveFive,
    SecondTeamChangeActiveFive,

    EndedPenaltyForTheFirstTeam,
    EndedPenaltyForTheSecondTeam,
}

pub enum ActionData {

}

pub trait DoAction {
    fn do_action(&self, game: &mut Game) -> Vec<ActionTypes>;
}

pub struct Action;
impl Action {
    pub fn do_action(self, game: &mut Game) -> Vec<ActionTypes> {
        let events =  self.random_action_happened(game);
        if events.is_none() {
            return self.choose_and_do_action(game);
        }

        return events.unwrap();
    }

    fn random_action_happened(&self, game: &mut Game) -> Option<Vec<ActionTypes>> {
        let mut random_actions: Vec<Box<dyn RandomAction>> = vec![
            Box::new(Giveaway),
            Box::new(Takeaway),
            Box::new(PuckOut),
            Box::new(Fight),
            Box::new(NetOff),
        ];

        let number_of_penalty_players1 = game.user1.team.get_number_of_penalty_players();
        let number_of_penalty_players2 = game.user2.team.get_number_of_penalty_players();

        if number_of_penalty_players1 < 2 && number_of_penalty_players2 < 2 {
            random_actions.push(Box::new(BigPenalty));
            random_actions.push(Box::new(SmallPenalty));
        }

        for action in &random_actions {
            if action.check_probability(game) {
                return Some(action.do_action(game));
            }
        }

        None
    }

    fn choose_and_do_action(&self, game: &mut Game) -> Vec<ActionTypes> {
        let mut is_attack_zone = false;
        let user_player_id = game.get_player_id_with_puck();
        if game.zone_number == 3 && user_player_id.0 == 1 || game.zone_number == 1 && user_player_id.0 == 2 {
            is_attack_zone = true;
        }

        let user = game.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();
        let player_with_puck_role = user.team.get_field_player(&user_player_id.1).player_role;

        log!("{} {}", user_player_id.0, user_player_id.1);
        let action = self.get_action(is_attack_zone, player_with_puck_role, active_five);

        action.do_action(game)
    }

    fn get_action(&self, is_attack_zone: bool, role: PlayerRole, active_five: &ActiveFive) -> Box<dyn DoAction> {
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
            log!("Dump");
            Box::new(DumpAction {})
        } else if is_attack_zone && percent * action_probability[2] >= rnd {
            log!("Shot");
            Box::new(ShotAction {})
        } else if !is_attack_zone && percent * action_probability[1] >= rnd {
            log!("Move");
            Box::new(MoveAction {})
        } else if !is_attack_zone && percent * action_probability[0] >= rnd {
            log!("Dangle");
            Box::new(DangleAction {})
        } else {
            log!("Pass");
            Box::new(PassAction {})
        }
    }

    /*
        0 - dump_probability
        1 - shot_probability
        2 - move_probability
        3 - dangle_probability
        4 - pass_probability
     */
    fn get_probability_of_actions(&self, role: PlayerRole, active_five: &ActiveFive) -> Vec<i32> {
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

        match active_five.current_number {
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
}