use near_sdk::{env, log, serde_json};
use crate::{Event, FieldPlayer, Game, PlayerPosition, UserInfo};
use crate::game::actions::action::ActionTypes;
use crate::game::actions::action::ActionTypes::Pass;
use crate::PlayerPosition::*;
use crate::team::numbers::GoalieNumber;
use crate::team::players::goalie::Goalie;

pub fn has_won(stat: f32, opponents_stat: f32) -> bool {
    let sum = stat + opponents_stat;

    let random_number = Game::get_random_in_range(1, sum.round() as usize + 1, 19);

    return if stat > opponents_stat {
        if random_number as f32 > opponents_stat {
            true
        } else {
            false
        }
    } else {
        if random_number as f32 > stat {
            false
        } else {
            true
        }
    }
}

pub fn get_opponents_field_player(game: &Game) -> &FieldPlayer {
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

pub fn get_relative_field_player_stat(player: &FieldPlayer, compared_stat: f32) -> f32 {
    let stat_avg = (compared_stat as f32 +
        player.stats.morale as f32 +
        player.stats.get_strength()) / 3.0;

    stat_avg * player.teamwork.unwrap() as f32
}

pub fn reduce_strength(game: &mut Game) {
    for (_player_pos, field_player) in &mut game.user1.team.active_five.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * 0.996;
    }
    for (_player_pos, field_player) in &mut game.user2.team.active_five.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * 0.996;
    }
}

pub fn generate_an_event(action: ActionTypes, game: &mut Game) {
    let generated_event = Event {
        user1: game.user1.clone(),
        user2: game.user2.clone(),
        time: game.last_event_generation_time.clone(),
        zone_number: game.zone_number.clone(),
        action,
        player_with_puck: game.player_with_puck.clone(),
    };

    let json_event = match serde_json::to_string(&generated_event) {
        Ok(res) => res,
        Err(e) => panic!("{}", e)
    };
    log!("{}", json_event);
}

pub fn get_opponent_user(game: &Game) -> &UserInfo {
    let user_id = game.player_with_puck.unwrap().0;

    return if user_id == 1 {
        &game.user2
    } else {
        &game.user1
    }
}
