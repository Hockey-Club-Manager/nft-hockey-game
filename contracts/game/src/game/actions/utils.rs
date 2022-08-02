use crate::{FieldPlayer, Game, PlayerPosition, UserInfo};
use crate::game::actions::action::ActionTypes;
use crate::game::actions::action::ActionTypes::Pass;
use crate::game::game::EventToSave;
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

pub fn get_another_random_position(player_pos: &PlayerPosition) -> PlayerPosition {
    let player_positions = get_other_positions(player_pos);

    let random_pos = Game::get_random_in_range(0, 4, 18);

    player_positions[random_pos]
}

pub fn get_other_positions(player_pos: &PlayerPosition) -> Vec<PlayerPosition> {
    let mut player_positions = vec![RightWing, LeftWing, Center, RightDefender, LeftDefender];

    for num in 0..5 {
        if *player_pos == player_positions[num] {
            player_positions.remove(num);
            break;
        }
    }

    player_positions
}

pub fn get_opponents_goalie(game: &Game) -> &Goalie {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        game.user2.team.goalies.get(&game.user2.team.active_goalie).unwrap()
    } else {
        game.user1.team.goalies.get(&game.user1.team.active_goalie).unwrap()
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

pub fn get_relative_goalie_stat(player: &Goalie, compared_stat: f32) -> f32 {
    (
        compared_stat+
        player.stats.morale +
        player.stats.get_strength()
    ) / 3.0
}

pub fn reduce_strength(game: &mut Game) {
    for (_player_pos, field_player) in &mut game.user1.team.active_five.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * 0.996;
    }
    for (_player_pos, field_player) in &mut game.user2.team.active_five.field_players.iter_mut() {
        field_player.stats.strength = field_player.stats.strength * 0.996;
    }
}

pub fn change_morale_after_goal(game: &mut Game) {
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

pub fn get_opponent_user(game: &Game) -> &UserInfo {
    let user_id = game.player_with_puck.as_ref().unwrap().get_user_id();

    return if user_id == 1 {
        &game.user2
    } else {
        &game.user1
    }
}
