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

pub fn get_relative_field_player_stat(player: &FieldPlayer, compared_stat: f32) -> f32 {
    let stat_avg = (compared_stat as f32 +
        player.stats.morale as f32 +
        player.stats.get_strength()) / 3.0;

    stat_avg * player.teamwork.unwrap() as f32
}

pub fn get_opponent_user(game: &Game) -> &UserInfo {
    let user_id = game.player_with_puck.clone().unwrap().0;

    return if user_id == 1 {
        &game.user2
    } else {
        &game.user1
    }
}
