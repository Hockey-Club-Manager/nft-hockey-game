use near_sdk::{env, log, serde_json};
use crate::{Event, FieldPlayer, Game, PlayerPosition, UserInfo};
use crate::game::actions::action::ActionTypes;
use crate::game::actions::action::ActionTypes::Pass;
use crate::PlayerPosition::*;
use crate::team::five::IceTimePriority;
use crate::team::numbers::{FiveNumber, GoalieNumber};
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
    let user_player_ids = game.player_with_puck.unwrap();

    let user = game.get_user_info(user_player_ids.0);
    let position = user.team.get_field_player_pos(&user_player_ids.1);

    return if user_player_ids.0 == 1 {
        game.get_field_player_by_pos(2, position)
    } else {
        game.get_field_player_by_pos(1, position)
    }
}

pub fn get_relative_field_player_stat(player: &FieldPlayer, compared_stat: f32) -> f32 {
    let stat_avg = (compared_stat as f32 +
        player.stats.morale as f32 +
        player.stats.get_strength()) / 3.0;

    stat_avg * player.teamwork.unwrap() as f32
}

pub fn reduce_strength(game: &mut Game) {
    let five1 = game.user1.team.get_active_five();
    for (_player_pos, field_player_id) in &five1.field_players {
        let field_player = game.user1.team.get_field_player_mut(field_player_id);
        let amount_of_spent_strength = get_amount_of_spent_strength(&five2.ice_time_priority);

        field_player.stats.decrease_strength(amount_of_spent_strength);
    }

    let five2 = game.user2.team.get_active_five();
    for (_player_pos, field_player_id) in &five2.field_players {
        let field_player = game.user2.team.get_field_player_mut(field_player_id);

        let amount_of_spent_strength = get_amount_of_spent_strength(&five2.ice_time_priority);
        field_player.stats.decrease_strength(amount_of_spent_strength);
    }
}

fn get_amount_of_spent_strength(ice_time_priority: &IceTimePriority) -> u8 {
    match ice_time_priority {
        IceTimePriority::SuperLowPriority => { 1 }
        IceTimePriority::LowPriority => { 2 }
        IceTimePriority::Normal => { 3 }
        IceTimePriority::HighPriority => { 4 }
        IceTimePriority::SuperHighPriority => { 5 }
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
