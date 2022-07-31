use crate::team::players::player::{PlayerPosition, PlayerRole};
use crate::team::players::field_player::FieldPlayer;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::team::players::player::PlayerRole::*;

use crate::game::actions::action::ActionTypes::*;

use crate::team::players::player::PlayerPosition::*;
use near_sdk::serde::{Deserialize, Serialize};
use crate::game::actions::shot::ShotAction;
use crate::game::actions::dangle::DangleAction;
use crate::game::actions::move_action::MoveAction;
use crate::game::actions::pass::PassAction;

use crate::game::game::{EventToSave, Game};
use crate::team::five::{FiveIds, Tactics};
use crate::team::numbers::{FiveNumber, GoalieNumber};
use crate::user_info::UserInfo;

const PROBABILITY_PASS_NOT_HAPPENED: i32 = 20;
const PROBABILITY_SAVE_NOT_HAPPENED: usize = 30;

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
0 - pass_probability
1 - shot_probability
2 - move_probability
3 - dangle_probability
4 - dump_probability
 */
    fn get_probability_of_actions(&self, role: PlayerRole, active_five: &FiveIds) -> Vec<i32> {
        let mut actions = match role {
            Playmaker => vec![2, 2, 3, 3, 1],
            Enforcer => vec![3, 2, 1, 1, 4],
            Shooter => vec![1, 4, 3, 2, 1],
            TryHarder => vec![3, 1, 4, 1, 2],
            DefensiveForward => vec![3, 2, 1, 1, 4],
            Grinder => vec![1, 1, 4, 1, 4],
            DefensiveDefenseman => vec![3, 2, 1, 1, 4],
            OffensiveDefenseman => vec![1, 4, 2, 3, 1],
            TwoWay => vec![2, 3, 3, 2, 1],
            ToughGuy => vec![1, 1, 4, 1, 3],
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

        let mut percent = 0;
        let mut action_probability: Vec<i32> = Vec::new();
        for i in 0..actions.len() {
            percent += actions[i];
            action_probability.push(percent);
        }
        percent = 100 / percent;

        let rnd = Game::get_random_in_range(1, 101) as i32;

        return if !is_attack_zone && percent * action_probability[0] >= rnd {
            Box::new(DangleAction {})
        } else if !is_attack_zone && percent * action_probability[1] >= rnd {
            Box::new(MoveAction {})
        } else if is_attack_zone && percent * action_probability[2] >= rnd {
            Box::new(ShotAction{})
        } else {
            Box::new(PassAction {})
        }
    }

    pub fn do_random_action(self, game: &mut Game) {
        let mut is_attack_zone = false;
        let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();
        if game.zone_number == 3 && user_id == 1 || game.zone_number == 1 && user_id == 2 {
            is_attack_zone = true;
        }

        let active_five = if user_id == 1 {
            &game.user1.team.fives.get(&game.user1.team.active_five).unwrap()
        } else {
            &game.user2.team.fives.get(&game.user2.team.active_five).unwrap()
        };


        let action = self.get_random_action(is_attack_zone, game.player_with_puck.as_ref().unwrap().get_role(), active_five);

        reduce_strength(game);

        action.do_action(game);
    }
}

pub fn has_won(stat: f64, opponents_stat: f64) -> bool {
    let sum = stat + opponents_stat;

    let random_number = Game::get_random_in_range(1, sum.round() as usize + 1);

    return if stat > opponents_stat {
        if random_number as f64 > opponents_stat {
            true
        } else {
            false
        }
    } else {
        if random_number as f64 > stat {
            false
        } else {
            true
        }
    }
}

fn get_another_random_position(player_pos: PlayerPosition) -> PlayerPosition {
    let player_positions = get_other_positions(player_pos);

    let random_pos = Game::get_random_in_range(0, 4);

    player_positions[random_pos]
}

fn get_other_positions(player_pos: PlayerPosition) -> Vec<PlayerPosition> {
    let mut player_positions = vec![RightWing, LeftWing, Center, RightDefender, LeftDefender];

    for num in 0..5 {
        if player_pos == player_positions[num] {
            player_positions.remove(num);
            break;
        }
    }

    player_positions
}

fn get_opponents_goalie(game: &Game) -> &GoalieNumber {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        &game.user2.team.active_goalie
    } else {
        &game.user1.team.active_goalie
    }
}

pub fn get_opponents_field_player(game: &mut Game) -> FieldPlayer {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        match game.user2.team.active_five.field_players.get(&game.player_with_puck.as_ref().unwrap().position) {
            Some(player) => player.clone(),
            _ => panic!("Player not found")
        }
    } else {
        let user = &game.user1;
        match user.team.active_five.field_players.get(&game.player_with_puck.as_ref().unwrap().position){
            Some(player) => player.clone(),
            _ => panic!("Player not found")
        }
    }
}

pub fn get_relative_field_player_stat(player: &FieldPlayer, stat: f64) -> f64 {
    (stat as f64 + player.stats.get_morale() as f64 + player.stats.get_strength() as f64) * player.position_coefficient as f64 / 3 as f64
}

pub fn reduce_strength(game: &mut Game) {
    for (_player_pos, field_player) in &mut game.user1.team.active_five.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * 0.996;
    }
    for (_player_pos, field_player) in &mut game.user2.team.active_five.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * 0.996;
    }
}

fn change_morale_after_a_goal(game: &mut Game) {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    let player_goalie = &mut game.get_user_info(user_id).team.active_goalie;
    player_goalie.stats.morale += 2;

    for (_player_pos, field_player) in &mut game.get_user_info(user_id).team.active_five.field_players.iter_mut() {
        field_player.stats.morale += 2;
    }

    let mut opponent_id = 1;
    if user_id == 1 {
        opponent_id = 2;
    }

    game.get_user_info(opponent_id).team.active_goalie.stats.morale -= 1;

    for (_player_pos, field_player) in &mut game.get_user_info(opponent_id).team.active_five.field_players.iter_mut() {
        field_player.stats.morale -= 1;
    }
}

pub fn has_pass_before_shot(game: &Game) -> bool {
    if game.events.len() == 0 {
        return false;
    }

    let action = &game.events[game.events.len() - 1].action;
    if *action == Pass {
        true
    } else {
        false
    }
}

pub fn generate_an_event(action: ActionTypes, game: &mut Game) {
    let new_event = EventToSave {
        action,
        time: game.last_event_generation_time.clone(),
        zone_number: game.zone_number.clone(),
        player_with_puck: game.player_with_puck.clone(),
    };

    game.events.push(new_event);
}

fn get_opponent_user(game: &Game) -> &UserInfo {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        &game.user2
    } else {
        &game.user1
    }
}