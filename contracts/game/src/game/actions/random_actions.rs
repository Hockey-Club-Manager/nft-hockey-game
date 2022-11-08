use near_sdk::log;
use crate::{FieldPlayer, Game, TokenId};
use crate::ActionData::Penalty;
use crate::game::actions::action::ActionData::{Battle, PenaltyShot, Dangle, Move};
use crate::game::actions::action::{ActionData, ActionTypes};
use crate::game::actions::dangle::DangleAction;
use crate::game::actions::move_action::MoveAction;
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};
use crate::user_info::UserId;


const PROBABILITY_GIVEAWAY: usize = 6;
const PROBABILITY_TAKEAWAY: usize = 15;
const PROBABILITY_PUCK_OUT: f32 = 0.005;
const PROBABILITY_BIG_PENALTY: usize = 1;
const PROBABILITY_SMALL_PENALTY: usize = 10;
const PROBABILITY_FIGHT: f32 = 0.25;
const PROBABILITY_NET_OFF: f32 = 0.01;
const PROBABILITY_BATTLE: usize = 20;

pub const SMALL_PENALTY: u8 = 5; // number of events
pub const BIG_PENALTY: u8 = 12; // number of events


pub trait RandomAction {
    fn check_probability(&self, game: &Game) -> bool;
    fn do_action(&self, game: &mut Game) -> Vec<ActionData>;
}

pub struct Giveaway;
impl RandomAction for Giveaway {
    fn check_probability(&self, _: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 11);
        if PROBABILITY_GIVEAWAY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        log!("Giveaway");
        let rnd = Game::get_random_in_range(1, 100, 12);

        if PROBABILITY_BATTLE >= rnd {
            battle(game)
        } else {
            let player_with_puck = game.get_player_with_puck();
            let user = game.get_user_info(player_with_puck.get_user_id());
            let player_with_puck_position = user.team.get_field_player_pos(&player_with_puck.get_player_id());

            let opponent_player = game.get_opponent_field_player();
            let opponent_user = game.get_user_info(opponent_player.1.get_user_id());
            let opponent_player_position = opponent_user.team.get_field_player_pos(&opponent_player.1.get_player_id());

            let action = vec![ActionData::Giveaway {
                action_type: ActionTypes::Giveaway,
                account_id1: user.account_id.clone(),
                player_number1: player_with_puck.number,
                player_position1: player_with_puck_position.clone(),
                account_id2: opponent_user.account_id.clone(),
                player_number2: opponent_player.1.number,
                player_position2: opponent_player_position.clone()
            }];

            game.player_with_puck = Option::from((opponent_player.1.get_user_id(), opponent_player.1.get_player_id()));

            action
        }
    }
}

fn battle(game: &mut Game) -> Vec<ActionData> {
    let player_with_puck = game.get_player_with_puck();
    let opponent_player = game.get_opponent_field_player();

    let player1_stat = (
        player_with_puck.stats.puck_control +
        player_with_puck.stats.aggressiveness +
        player_with_puck.stats.strength
    ) as f32 / 3.0;

    let player2_stat = (
        opponent_player.1.stats.puck_control +
        opponent_player.1.stats.aggressiveness +
        opponent_player.1.stats.strength
    ) as f32 / 3.0;

    let compared_stat1 = get_relative_field_player_stat(player_with_puck, player1_stat);
    let compared_stat2= get_relative_field_player_stat(opponent_player.1, player2_stat) * opponent_player.0;

    let user_with_puck = game.get_user_info(player_with_puck.get_user_id());
    let player_with_puck_position = user_with_puck.team.get_field_player_pos(
        &player_with_puck.get_player_id());

    let opponent_user = game.get_user_info(opponent_player.1.get_user_id());
    let opponent_player_position = opponent_user.team.get_field_player_pos(
        &opponent_player.1.get_player_id());

    let mut actions = vec![Battle {
        action_type: ActionTypes::Battle,
        account_id1: user_with_puck.account_id.clone(),
        player_number1: player_with_puck.number,
        player_position1: player_with_puck_position.clone(),
        account_id2: opponent_user.account_id.clone(),
        player_number2: opponent_player.1.number,
        player_position2: opponent_player_position.clone(),
    }];

    if has_won(compared_stat2, compared_stat1) {
        actions.push(ActionData::BattleWon {
            action_type: ActionTypes::BattleWon,
            account_id: opponent_user.account_id.clone(),
            player_number: opponent_player.1.number,
            player_position: opponent_player_position.clone(),
        });

        game.player_with_puck = Option::from(
            (opponent_player.1.get_user_id(), opponent_player.1.get_player_id()));
    } else {
        actions.push(ActionData::BattleWon {
            action_type: ActionTypes::BattleWon,
            account_id: user_with_puck.account_id.clone(),
            player_number: player_with_puck.number,
            player_position: player_with_puck_position.clone(),
        })
    }

    actions
}

pub struct Takeaway;
impl RandomAction for Takeaway {
    fn check_probability(&self, _: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 13);
        if PROBABILITY_TAKEAWAY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        log!("Takeaway");
        let rnd = Game::get_random_in_range(1, 100, 14);

        return if PROBABILITY_BATTLE >= rnd {
            battle(game)
        } else {
            let player_with_puck = game.get_player_with_puck();
            let user = game.get_user_info(player_with_puck.get_user_id());
            let player_with_puck_position = user.team.get_field_player_pos(&player_with_puck.get_player_id());

            let opponent_player = game.get_opponent_field_player();
            let opponent_user = game.get_user_info(opponent_player.1.get_user_id());
            let opponent_player_position = opponent_user.team.get_field_player_pos(&opponent_player.1.get_player_id());

            let action = vec![ActionData::Takeaway {
                action_type: ActionTypes::Takeaway,
                account_id1: user.account_id.clone(),
                player_number1: player_with_puck.number,
                player_position1: player_with_puck_position.clone(),
                account_id2: opponent_user.account_id.clone(),
                player_number2: opponent_player.1.number,
                player_position2: opponent_player_position.clone()
            }];

            game.player_with_puck = Option::from((opponent_player.1.get_user_id(), opponent_player.1.get_player_id()));

            action
        }
    }
}

pub struct PuckOut;
impl RandomAction for PuckOut {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 15);
        if PROBABILITY_PUCK_OUT >= rnd as f32 {
            return true;
        }

        false
    }

    fn do_action(&self, _: &mut Game) -> Vec<ActionData> {
        log!("PuckOut");
        vec![ActionData::PuckOut { action_type: ActionTypes::PuckOut }]
    }
}

pub struct BigPenalty;
impl RandomAction for BigPenalty {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 16);
        log!("rnd: {}", rnd);
        if PROBABILITY_BIG_PENALTY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        log!("BigPenalty");
        let player_with_puck = game.get_player_with_puck();
        let opponent_player = game.get_opponent_field_player();

        let player_stat1 = player_with_puck.stats.discipline as f32;
        let player_stat2 = opponent_player.1.stats.discipline as f32;

        if has_won(player_stat1, player_stat2) {
            // The rules were violated by the opponent of the player with the puck
            match game.last_action {
                Move {..} | Dangle {..} => {
                    return vec![generate_penalty_shot_action(game, player_with_puck)];
                },
                _ => {}
            }

            let penalty_player_id = opponent_player.1.get_player_id();
            let penalty_user_id = opponent_player.1.get_user_id();

            move_player_to_big_penalties(game, penalty_player_id,
                                           penalty_user_id);
        } else {
            // The player with the puck broke the rules

            let opponent_player_id = opponent_player.1.get_player_id();
            let user_id = opponent_player.1.get_user_id();

            let penalty_player_id = player_with_puck.get_player_id();
            let penalty_user_id = player_with_puck.get_user_id();
            move_player_to_big_penalties(game, penalty_player_id,
                                           penalty_user_id);

            game.player_with_puck = Some((user_id, opponent_player_id));
        }

        vec![ActionData::DelayedPenaltySignal {
            action_type: ActionTypes::DelayedPenaltySignal,
            type_of_penalty: ActionTypes::BigPenalty,
        }]
    }
}

fn generate_penalty_shot_action(game: &Game, player_with_puck: &FieldPlayer) -> ActionData {
    let user = game.get_user_info(player_with_puck.get_user_id());
    let opponent = game.get_opponent_info(player_with_puck.get_user_id());
    let goalie = opponent.team.get_active_goalie();

    return PenaltyShot {
        action_type: ActionTypes::PenaltyShot,
        account_id1: user.account_id.clone(),
        player_name: player_with_puck.name.clone().expect("Player name not found"),
        player_img: player_with_puck.img.clone().expect("Player img not found"),
        player_number: player_with_puck.number,
        account_id2: opponent.account_id.clone(),
        goalie_name: goalie.name.clone().expect("Goalie name not found"),
        goalie_img: goalie.img.clone().expect("Goalie img not found"),
        goalie_number: goalie.number,
    }
}

fn move_player_to_big_penalties(game: &mut Game, player_id: TokenId, user_id: UserId) {
    let penalty_user = game.get_user_info_mut(&user_id);
    penalty_user.team.players_to_big_penalty.push(player_id);
}


pub struct SmallPenalty;
impl RandomAction for SmallPenalty {
    fn check_probability(&self, _: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 17);
        if PROBABILITY_SMALL_PENALTY >= rnd {
            return true;
        }

        false
    }

    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        log!("Small penalty");
        let player_with_puck = game.get_player_with_puck();
        let opponent_player = game.get_opponent_field_player();

        let player_stat1 = player_with_puck.stats.discipline as f32;
        let player_stat2 = opponent_player.1.stats.discipline as f32;

        if has_won(player_stat1, player_stat2) {
            // The rules were violated by the opponent of the player with the puck
            match game.last_action {
                Move {..} | Dangle {..} => {
                    return vec![generate_penalty_shot_action(game, player_with_puck)];
                },
                _ => {}
            }

            let penalty_player_id = opponent_player.1.get_player_id();
            let penalty_user_id = opponent_player.1.get_user_id();

            move_player_to_small_penalties(game, penalty_player_id,
                                           penalty_user_id);
        } else {
            // The player with the puck broke the rules

            let opponent_player_id = opponent_player.1.get_player_id();
            let user_id = opponent_player.1.get_user_id();

            let penalty_player_id = player_with_puck.get_player_id();
            let penalty_user_id = player_with_puck.get_user_id();
            move_player_to_small_penalties(game, penalty_player_id,
                                           penalty_user_id);

            game.player_with_puck = Some((user_id, opponent_player_id));
        }

        vec![ActionData::DelayedPenaltySignal {
            action_type: ActionTypes::DelayedPenaltySignal,
            type_of_penalty: ActionTypes::BigPenalty,
        }]
    }
}

fn move_player_to_small_penalties(game: &mut Game, player_id: TokenId, user_id: UserId) {
    let penalty_user = game.get_user_info_mut(&user_id);
    penalty_user.team.players_to_small_penalty.push(player_id);
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

    fn do_action(&self, game: &mut Game) -> Vec<ActionData> {
        log!("Fight");
        let player_with_puck = game.get_player_with_puck();
        let opponent_player = game.get_opponent_field_player();

        let compared_stat1 = get_relative_field_player_stat(player_with_puck,
                                                            player_with_puck.stats.fighting_skill as f32);
        let compared_stat2= get_relative_field_player_stat(opponent_player.1,
                                                           opponent_player.1.stats.fighting_skill as f32) * opponent_player.0;

        let player1_id = player_with_puck.get_player_id();
        let user1_id = player_with_puck.get_user_id();
        let player2_id = opponent_player.1.get_player_id();
        let user2_id = opponent_player.1.get_user_id();

        let user_id_with_puck = player_with_puck.get_user_id();
        let user_with_puck = game.get_user_info(user_id_with_puck);
        let opponent_info = game.get_opponent_info(user_id_with_puck);

        let mut actions = vec![
            ActionData::Fight {
                action_type: ActionTypes::Fight,
                account_id1: user_with_puck.account_id.clone(),
                player_number1: player_with_puck.number,
                account_id2: opponent_info.account_id.clone(),
                player_number2: opponent_player.1.number,
            }
        ];

        if has_won(compared_stat2, compared_stat1) {
            actions.push(ActionData::FightWon {
                action_type: ActionTypes::StartGame,
                account_id: opponent_info.account_id.clone(),
                player_name: opponent_player.1.name.clone().expect("Player name not found"),
                player_img: opponent_player.1.img.clone().expect("Player img not found"),
                player_number: opponent_player.1.number,
            });

            self.increase_morale_opponent_team(game, &user_id_with_puck);
            self.reduce_morale_team_with_puck(game, &user_id_with_puck);
        } else {
            actions.push(ActionData::FightWon {
                action_type: ActionTypes::StartGame,
                account_id: user_with_puck.account_id.clone(),
                player_name: player_with_puck.name.clone().expect("Player name not found"),
                player_img: player_with_puck.img.clone().expect("Player img not found"),
                player_number: player_with_puck.number,
            });

            self.increase_morale_team_with_puck(game, &user_id_with_puck);
            self.reduce_morale_opponent_team(game, &user_id_with_puck);
        }

        actions.push(get_penalty_by_fight_action(game.do_penalty(BIG_PENALTY,
                        &player1_id,
                        &user2_id,
                        &user1_id)));

        actions.push(game.do_penalty(BIG_PENALTY,
                        &player2_id,
                        &user1_id,
                        &user2_id));


        actions
    }
}

fn get_penalty_by_fight_action(action_data: ActionData) -> ActionData {
    match action_data {
        Penalty { action_type, account_id, is_fight,
            player_img, player_name, player_number, } => {
            Penalty {
                action_type,
                account_id,
                is_fight: true,
                player_img,
                player_name,
                player_number
            }
        },
        _ => panic!("")
    }
}

impl Fight {
    fn increase_morale_opponent_team(&self, game: &mut Game, user_with_puck_id: &usize) {
        let opponent_user = game.get_opponent_info_mut(user_with_puck_id);
        opponent_user.team.increase_morale();
    }

    fn reduce_morale_opponent_team(&self, game: &mut Game, user_with_puck_id: &usize) {
        let opponent_user = game.get_opponent_info_mut(user_with_puck_id);
        opponent_user.team.reduce_morale();
    }

    fn increase_morale_team_with_puck(&self, game: &mut Game, user_with_puck_id: &usize) {
        let user = game.get_user_info_mut(user_with_puck_id);
        user.team.increase_morale();
    }

    fn reduce_morale_team_with_puck(&self, game: &mut Game, user_with_puck_id: &usize) {
        let user = game.get_user_info_mut(user_with_puck_id);
        user.team.increase_morale();
    }
}

pub struct NetOff;
impl RandomAction for NetOff {
    fn check_probability(&self, game: &Game) -> bool {
        let rnd = Game::get_random_in_range(1, 100, 18);
        if PROBABILITY_NET_OFF >= rnd as f32 && (game.zone_number != 2) {
            return true;
        }

        false
    }

    fn do_action(&self, _: &mut Game) -> Vec<ActionData> {
        log!("NetOff");
        vec![ActionData::NetOff { action_type: ActionTypes::NetOff }]
    }
}