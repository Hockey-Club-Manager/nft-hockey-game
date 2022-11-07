use std::collections::HashMap;
use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, Timestamp};
use near_sdk::serde::{Deserialize, Serialize};
use crate::team::players::field_player::{FieldPlayer};
use crate::game::actions::action::{Action, ActionData, ActionTypes};
use crate::game::actions::action::ActionData::*;
use crate::team::players::player::{PlayerPosition};
use crate::team::players::player::PlayerPosition::*;
use crate::{TokenBalance};
use crate::ActionTypes::{BigPenalty, SmallPenalty};
use crate::game::actions::random_actions::{BIG_PENALTY, SMALL_PENALTY};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};
use crate::PlayerPosition::LeftWing;
use crate::team::five::{ActiveFive, FiveIds, IceTimePriority};
use crate::team::numbers::FiveNumber;
use crate::team::numbers::FiveNumber::{First, PenaltyKill1, PenaltyKill2, PowerPlay1, PowerPlay2};
use crate::team::players::player::Hand::Left;
use crate::team::team_metadata::team_metadata_to_team;
use crate::user_info::{USER_ID1, USER_ID2, UserId};


#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    GameOver { winner_id: usize },
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    pub(crate) player_with_puck: Option<(UserId, TokenId)>,
    pub(crate) actions: Vec<ActionData>,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
    pub(crate) user1: UserInfo,
    pub(crate) user2: UserInfo,
    pub(crate) event_generation_delay: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub(crate) game_id: GameId,
    pub(crate) user1: UserInfo,
    pub(crate) user2: UserInfo,
    pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,

    pub(crate) player_with_puck: Option<(UserId, TokenId)>,
    pub(crate) zone_number: i8,
    pub(crate) turns: u128,
    pub(crate) last_action: ActionData,

    pub(crate) last_event_generation_time: Timestamp,

    // in nanoseconds
    pub(crate) event_generation_delay: u64,
    pub(crate) max_number_of_generated_events_in_block: u8,
    pub(crate) number_of_generated_events_in_current_block: u8,
}

impl Game {
    pub fn new (
        teams: (TeamMetadata, TeamMetadata),
        account_id_1: AccountId,
        account_id_2: AccountId,
        reward: TokenBalance,
        game_id: &GameId
    ) -> Game {
        let mut team1 = team_metadata_to_team(teams.0, 1);
        let team2 = team_metadata_to_team(teams.1, 2);

        let user_info1 = UserInfo {
            user_id: USER_ID1,
            team: team1,
            account_id: account_id_1,
            take_to_called: false,
            coach_speech_called: false,
            is_goalie_out: false,
        };

        let user_info2 = UserInfo {
            user_id: USER_ID2,
            team: team2,
            account_id: account_id_2,
            take_to_called: false,
            coach_speech_called: false,
            is_goalie_out: false,
        };

        let game = Game {
            game_id: *game_id,
            user1: user_info1,
            user2: user_info2,
            reward,
            winner_index: None,
            event_generation_delay: 0,
            player_with_puck: None,
            zone_number: 2,
            turns: 0,
            last_action: StartGame { action_type: ActionTypes::StartGame },
            last_event_generation_time: env::block_timestamp(),
            number_of_generated_events_in_current_block: 0,
            max_number_of_generated_events_in_block: 2
        };

        game
    }

    pub fn get_random_in_range(min: usize, max: usize, index: usize) -> usize {
        let random = *env::random_seed().get(index).unwrap();
        let random_in_range = (random as f64 / 256.0) * (max - min) as f64 + min as f64;
        random_in_range.floor() as usize
    }
}

impl Game {
    pub fn get_field_player_by_pos(&self, user_id: UserId, position: &PlayerPosition) -> &FieldPlayer {
        let player_id = self.get_field_player_id_by_pos(position, user_id.clone());

        let user_info = self.get_user_info(user_id);
        log!("user_id: {}", user_id.clone());
        user_info.team.get_field_player(&player_id)
    }

    pub fn get_field_player_id_by_pos(&self, position: &PlayerPosition, user_id: UserId) -> TokenId {
        let user_info = self.get_user_info(user_id);
        let five = user_info.team.get_active_five();

        match five.field_players.get(position) {
            Some(id) => {
                if *id == "" {
                    self.get_player_id_by_penalty_pos(position, five)
                } else {
                    id.clone()
                }
            },
            None => self.get_player_id_by_penalty_pos(position, five)
        }
    }

    fn get_player_id_by_penalty_pos(
        &self,
        position: &PlayerPosition,
        five: &ActiveFive
    ) -> TokenId {
        match position {
            RightWing => {
                let player_id = five.field_players.get(&RightDefender).unwrap();
                player_id.clone()
            },
            LeftWing => {
                let number_of_players = five.get_number_of_players();
                let player_id = if number_of_players == 4 {
                    five.field_players.get(&RightWing).unwrap()
                } else {
                    five.field_players.get(&LeftDefender).unwrap()
                };

                player_id.clone()
            },
            _ => panic!("Player id not found. Position: {}", position)
        }
    }

    pub fn get_field_player_by_pos_mut(&mut self, user_id: UserId, position: &PlayerPosition) -> &mut FieldPlayer {
        let player_id = self.get_field_player_id_by_pos(position, user_id);

        let user_info = self.get_user_info_mut(&user_id);
        user_info.team.get_field_player_mut(&player_id)
    }

    pub fn get_player_pos(&self, player_id: &TokenId, user_id: UserId) -> &PlayerPosition {
        let user_info = self.get_user_info(user_id);
        user_info.team.get_field_player_pos(player_id)
    }

    pub fn get_user_id_player_with_puck(&self) -> usize {
        let player_with_puck = self.get_player_with_puck();
        player_with_puck.get_user_id()
    }

    pub fn get_player_with_puck_mut(&mut self) -> &mut FieldPlayer {
        let unwrapped_player = self.get_player_id_with_puck();
        let user = self.get_user_info_mut(&unwrapped_player.0);

        user.team.field_players.get_mut(&unwrapped_player.1).unwrap()
    }

    pub fn get_player_with_puck(&self) -> &FieldPlayer {
        let unwrapped_player = self.get_player_id_with_puck();
        let user = self.get_user_info(unwrapped_player.0);

        user.team.field_players.get(&unwrapped_player.1).unwrap()
    }

    pub fn get_player_id_with_puck(&self) -> (UserId, TokenId) {
        self.player_with_puck.clone().unwrap()
    }

    pub fn get_user_info_mut(&mut self, user_id: &usize) -> &mut UserInfo {
        if *user_id == USER_ID1 {
            return &mut self.user1;
        }

        &mut self.user2
    }

    pub fn get_user_info(&self, user_id: usize) -> &UserInfo {
        if user_id == USER_ID1 {
            return &self.user1;
        }

        &self.user2
    }

    pub fn get_user_info_by_acc_id(&mut self, account_id: &AccountId) -> &mut UserInfo {
        if *account_id == self.user1.account_id {
            return &mut self.user1;
        }
        if *account_id == self.user2.account_id {
            return &mut self.user2;
        }

        panic!("Account id not found!")
    }

    pub fn get_opponent_info(&self, user_id: usize) -> &UserInfo {
        if user_id == USER_ID1 {
            return self.get_user_info(USER_ID2);
        }

        self.get_user_info(USER_ID1)
    }

    pub fn get_opponent_info_mut(&mut self, user_id: &usize) -> &mut UserInfo {
        if *user_id == USER_ID1 {
            return self.get_user_info_mut(&USER_ID2);
        }

        self.get_user_info_mut(&USER_ID1)
    }

    pub fn get_opponent_field_player(&self) -> (f32, &FieldPlayer) {
        let user_player_ids = self.player_with_puck.clone().unwrap();
        let position = self.get_coeff_player_pos(&user_player_ids);

        return if user_player_ids.0 == USER_ID1 {
            (position.0, self.get_field_player_by_pos(USER_ID2, &position.1))
        } else {
            (position.0, self.get_field_player_by_pos(USER_ID1, &position.1))
        }
    }

    fn get_coeff_player_pos(&self, user_player_ids: &(UserId, TokenId)) -> (f32, PlayerPosition) {
        let user = self.get_user_info(user_player_ids.0);
        let active_five = user.team.get_active_five();

        let opponent_user = self.get_opponent_info(user.user_id);
        let opponent_active_five = opponent_user.team.get_active_five();

        let field_player_pos = user.team.get_field_player_pos(&user_player_ids.1);

        let result = self.get_opponent_position(
            active_five.get_number_of_players(),
            opponent_active_five.get_number_of_players(),
            field_player_pos);

        (result.0, result.1.clone())
    }

    pub fn get_opponent_position(
        &self,
        number_of_players: usize,
        number_of_opponent_players: usize,
        field_player_pos: &PlayerPosition
    ) -> (f32, &PlayerPosition) {
        match field_player_pos {
            AdditionalPosition => {
                if number_of_players == number_of_opponent_players {
                    (1.0, &AdditionalPosition)
                } else if number_of_opponent_players == 5 || number_of_opponent_players == 3 {
                    (0.5, &Center)
                } else {
                    (1.0, &RightDefender)
                }
            },
            Center => {
                if number_of_players == 6 && number_of_opponent_players == 3
                    || number_of_players == 6 && number_of_opponent_players == 5 {
                    (0.5, &Center)
                } else if number_of_opponent_players == 6 && number_of_players == 3
                    || number_of_opponent_players == 6 && number_of_players == 5 {
                    (1.5, &Center)
                } else {
                    (1.0, &Center)
                }
            },
            LeftWing => {
                if number_of_opponent_players == 3 {
                    (0.5, &RightDefender)
                } else if number_of_opponent_players == 4 {
                    (0.5, &RightWing)
                }
                else {
                    (1.0, &RightDefender)
                }
            },
            RightWing => {
                if (number_of_opponent_players == 5 || number_of_opponent_players == 6)
                    && (number_of_players == 5 || number_of_players == 6)
                    || (number_of_players == 5 && number_of_opponent_players == 4) {
                    (1.0, &LeftDefender)
                } else if number_of_players == 4 && number_of_opponent_players == 4 {
                    (1.5, &RightDefender)
                } else if number_of_players < number_of_opponent_players {
                    (1.5, &RightDefender)
                } else {
                    (0.5, &LeftDefender)
                }
            },
            LeftDefender => {
                if (number_of_opponent_players == 5 || number_of_opponent_players == 6)
                    && (number_of_players == 5 || number_of_players == 6) {
                    (1.0, &RightWing)
                } else if number_of_opponent_players >= 4 {
                    if number_of_opponent_players >= number_of_players {
                        (1.5, &RightWing)
                    } else {
                        (0.5, &RightWing)
                    }
                } else if number_of_players == 3 && number_of_opponent_players == 3 {
                    (1.0, &RightDefender)
                } else {
                    (0.5, &RightDefender)
                }
            },
            RightDefender => {
                if (number_of_opponent_players == 5 || number_of_opponent_players == 6)
                    && (number_of_players == 5 || number_of_players == 6) {
                    (1.0, &LeftWing)
                } else if number_of_players == 5 && number_of_opponent_players == 4
                    || number_of_players == 4 && number_of_opponent_players == 4 {
                    (0.5, &RightWing)
                } else if number_of_opponent_players >= 4 {
                    if number_of_opponent_players >= number_of_players {
                        (1.5, &LeftDefender)
                    } else {
                        (0.5, &LeftDefender)
                    }
                } else if number_of_players == 3 && number_of_opponent_players == 3 {
                    (1.0, &LeftDefender)
                } else {
                    (0.5, &LeftDefender)
                }
            },
            _ => panic!("Cannot find opponent position")
        }
    }

    pub fn reduce_strength(&mut self, user_id: usize) {
        let user = self.get_user_info_mut(&user_id);

        let five1 = user.team.get_active_five().clone();
        for (_player_pos, field_player_id) in &five1.field_players {
            let amount_of_spent_strength = get_amount_of_spent_strength(five1.ice_time_priority);

            let field_player = user.team.get_field_player_mut(field_player_id);
            field_player.stats.decrease_strength(amount_of_spent_strength);
        }
    }

    pub fn generate_event(&mut self, actions: &Vec<ActionData>) -> Event {
        for action in actions {
            match action {
                TakeTO {..} | CoachSpeech {..} | GoalieBack {..}
                | GoalieOut {..} | EndedPenalty {..} | DelayedPenaltySignal {..}
                | Penalty {..} => {},
                _ => {
                    self.last_action = action.clone();
                }
            }
        }

        Event {
            user1: self.user1.clone(),
            user2: self.user2.clone(),
            time: self.last_event_generation_time.clone(),
            zone_number: self.zone_number.clone(),
            actions: actions.clone(),
            player_with_puck: self.player_with_puck.clone(),
            event_generation_delay: self.event_generation_delay / SECOND
        }
    }
}

pub fn get_amount_of_spent_strength(ice_time_priority: IceTimePriority) -> u8 {
    match ice_time_priority {
        IceTimePriority::SuperLowPriority => { 1 }
        IceTimePriority::LowPriority => { 2 }
        IceTimePriority::Normal => { 3 }
        IceTimePriority::HighPriority => { 4 }
        IceTimePriority::SuperHighPriority => { 5 }
    }
}

impl Game {
    pub fn get_game_state(&mut self) -> (GameState, Option<ActionData>) {
        return if self.is_game_over() {
            (
                GameState::GameOver { winner_id: self.get_winner_id() },
                None
            )
        } else {
            let state = GameState::InProgress;

            if self.turns == NUMBER_OF_STEPS {
                (state, Some(Overtime { action_type: ActionTypes::Overtime }))
            } else {
                (state, None)
            }
        };
    }

    pub fn step(&mut self) -> Vec<ActionData> {
        let mut actions = self.do_action();

        self.increase_five_time_field();
        match self.last_action {
            Icing {..} => { },
            _ => {
                self.check_teams_to_change_active_five();
                actions.append(&mut self.reduce_penalty());

                self.swap_players_in_five(&USER_ID1);
                self.swap_players_in_five(&USER_ID2);
            }
        };

        actions.append(&mut self.check_and_do_penalties());

        let end_of_period_event = self.check_end_of_period();
        if end_of_period_event.is_some() {
            actions.push(end_of_period_event.unwrap());
        }

        actions
    }

    fn do_action(&mut self) -> Vec<ActionData> {
        let action = Action;

        let actions = match self.last_action {
            StartGame {..} => {
                self.zone_number = 2;
                self.event_generation_delay += 3 * SECOND;

                let mut actions = vec![StartGame {
                    action_type: ActionTypes::StartGame
                }];
                actions.append(&mut self.face_off(&Center));

                actions
            },
            Goal { .. } | EndOfPeriod {..} => {
                self.zone_number = 2;
                self.event_generation_delay += 3 * SECOND;

                self.player_with_puck = None;
                self.swap_all_players_in_fives();
                self.face_off(&Center)
            },
            PuckOut {..} | NetOff {..}=> {
                let random_position = self.get_random_position();
                self.event_generation_delay += 3 * SECOND;

                self.player_with_puck = None;
                self.swap_all_players_in_fives();
                self.face_off(&random_position)
            },
            Offside {..} => {
                let random_position = self.get_random_position_after_offside();
                self.event_generation_delay += 3 * SECOND;

                self.player_with_puck = None;
                self.swap_all_players_in_fives();
                self.face_off(&random_position)
            }
            Save {..} => {
                self.event_generation_delay += 3 * SECOND;

                self.swap_all_players_in_fives();
                self.face_off_after_save()
            },
            Icing {..} => {
                self.zone_number = match self.get_user_id_player_with_puck() {
                    1 => 1,
                    2 => 3,
                    _ => panic!("User id not found :(")
                };

                let random_position = self.get_random_position();
                self.event_generation_delay += 3 * SECOND;

                self.player_with_puck = None;
                self.swap_all_players_on_opponent_team();
                self.face_off(&random_position)
            },
            Penalty {..} | Fight {..} | FightWon {..}  => {
                self.zone_number = match self.get_user_id_player_with_puck() {
                    1 => 1,
                    2 => 3,
                    _ => panic!("User id not found :(")
                };

                let random_position = self.get_random_position();
                self.event_generation_delay += 3 * SECOND;

                self.player_with_puck = None;
                self.swap_all_players_in_fives();
                self.face_off(&random_position)
            },
            PenaltyShot {..} => {
                self.event_generation_delay += 5 * SECOND;
                self.do_penalty_shot()
            }

            _ => action.do_action(self)
        };

        self.turns += 1;

        actions
    }

    fn swap_all_players_in_fives(&mut self) {
        let player_with_puck = self.player_with_puck.clone();
        let user1 = self.get_user_info_mut(&1);
        user1.team.swap_all_players_in_active_five(&player_with_puck);

        let user2 = self.get_user_info_mut(&2);
        user2.team.swap_all_players_in_active_five(&player_with_puck);
    }

    fn swap_all_players_on_opponent_team(&mut self) {
        let player_with_puck = self.player_with_puck.clone();

        let user_id = self.get_user_id_player_with_puck();

        if user_id == USER_ID1 {
            let user2 = self.get_user_info_mut(&USER_ID2);
            user2.team.swap_all_players_in_active_five(&player_with_puck);
        } else {
            let user1 = self.get_user_info_mut(&USER_ID1);
            user1.team.swap_all_players_in_active_five(&player_with_puck);
        }
    }

    fn check_and_do_penalties(&mut self) -> Vec<ActionData> {
        match self.player_with_puck.clone() {
            None =>  vec![],
            Some((user_id, _player_id)) => {
                let opponent_id = if user_id == USER_ID1 {
                    USER_ID2
                } else {
                    USER_ID1
                };
                self.do_penalties(opponent_id)
            }
        }
    }

    fn do_penalties(&mut self, penalty_user_id: UserId) -> Vec<ActionData> {
        let user_id = if penalty_user_id == USER_ID1 {
            USER_ID2
        } else {
            USER_ID1
        };

        let mut actions = Vec::new();

        let players_to_big_penalty = self.get_players_to_big_penalty(&user_id);
        actions.append(&mut self.dp(players_to_big_penalty, &user_id, &penalty_user_id, BIG_PENALTY));

        let players_to_small_penalty = self.get_players_to_small_penalty(&user_id);
        actions.append(&mut self.dp(players_to_small_penalty, &user_id, &penalty_user_id, SMALL_PENALTY));

        self.clear_players_to_penalties(&user_id);

        actions
    }

    fn get_players_to_big_penalty(&self, penalty_user_id: &UserId) -> Vec<TokenId> {
        let penalty_user = self.get_user_info(penalty_user_id.clone());
        penalty_user.team.players_to_big_penalty.clone()
    }

    fn get_players_to_small_penalty(&self, penalty_user_id: &UserId) -> Vec<TokenId> {
        let penalty_user = self.get_user_info(penalty_user_id.clone());
        penalty_user.team.players_to_small_penalty.clone()
    }

    fn clear_players_to_penalties(&mut self, penalty_user_id: &UserId) {
        let penalty_user = self.get_user_info_mut(penalty_user_id);
        penalty_user.team.players_to_small_penalty.clear();
        penalty_user.team.players_to_big_penalty.clear();
    }

    fn dp(
        &mut self,
        players_to_penalty: Vec<TokenId>,
        user_id: &UserId,
        penalty_user_id: &UserId,
        penalty_time: u8
    ) -> Vec<ActionData> {
        let mut actions = Vec::new();

        for player_id in &players_to_penalty {
            actions.push(self.do_penalty(
                penalty_time,
                &player_id,
                &user_id,
                &penalty_user_id
            ));
        }

        actions
    }

    pub fn do_penalty(
        &mut self,
        penalty_time: u8,
        penalty_player_id: &TokenId,
        user_id: &UserId,
        penalty_user_id: &UserId
    ) -> ActionData {
        let action_type = if penalty_time == BIG_PENALTY {
            BigPenalty
        } else {
            SmallPenalty
        };

        let player_with_puck = self.player_with_puck.clone();
        let penalty_user = self.get_user_info(penalty_user_id.clone());
        let penalty_player = penalty_user.team.get_field_player(penalty_player_id);

        let action = Penalty {
            action_type,
            account_id: penalty_user.account_id.clone(),
            player_img: penalty_player.img.clone().expect("Player img not found"),
            player_name: penalty_player.name.clone().expect("Player name not found"),
            player_number: penalty_player.number,
        };

        self.penalty_player(penalty_time, penalty_player_id, penalty_user_id);

        let penalty_user_mut = self.get_user_info_mut(penalty_user_id);
        penalty_user_mut.team.do_penalty(&penalty_player_id);

        penalty_user_mut.team.swap_all_players_in_active_five(&player_with_puck);

        let active_five_number = penalty_user_mut.team.active_five.get_current_five_number();
        let number_of_players = penalty_user_mut.team.get_five_number_of_players(&active_five_number);

        self.check_and_change_active_five_to_pp(user_id, number_of_players);

        action
    }

    fn check_and_change_active_five_to_pp(&mut self, user_id: &UserId, opponent_number_of_players: usize) {
        let user = self.get_user_info_mut(user_id);
        let active_five_number = user.team.active_five.get_current_five_number();
        let number_of_players = user.team.get_five_number_of_players(&active_five_number);

        let brigades = vec![PowerPlay1, PowerPlay2];
        if !brigades.contains(&active_five_number) && number_of_players > opponent_number_of_players {
            user.team.active_five.current_number = PowerPlay1;
        }
    }

    pub fn penalty_player(
        &mut self,
        penalty_time: u8,
        penalty_player_id: &TokenId,
        penalty_user_id: &UserId
    ) {
        let penalty_user = self.get_user_info_mut(penalty_user_id);
        penalty_user.team.penalty_players.push(penalty_player_id.clone());

        let player = penalty_user.team.get_field_player_mut(penalty_player_id);
        player.number_of_penalty_events = Some(penalty_time);
    }

    fn do_penalty_shot(&mut self) -> Vec<ActionData> {
        let player_with_puck = self.get_player_with_puck();
        let player_stat = (player_with_puck.stats.get_skating()
            + player_with_puck.stats.get_shooting()
            + player_with_puck.stats.morale as f32
            + player_with_puck.stats.get_iq()) / 4.0;

        let (user_id, _player_id) = self.player_with_puck.clone()
            .expect("Cannot find player with puck");

        let user_opponent = self.get_opponent_info(user_id);
        let number_goalie = user_opponent.team.active_goalie.clone();
        let opponent_goalie = user_opponent.team.goalies.get(&number_goalie).unwrap();

        let goalie_stat = (opponent_goalie.stats.get_reflexes()
            + opponent_goalie.stats.morale as f32) / 2.0;

        return if has_won(player_stat, goalie_stat) {
            let user = self.get_user_info(user_id);
            let action = vec![Goal {
                action_type: ActionTypes::Goal,
                account_id: user.account_id.clone(),
                player_name1: player_with_puck.name.clone().expect("Player name not found"),
                player_img: player_with_puck.img.clone().expect("Player img not found"),
                player_number1: player_with_puck.number,
                player_name2: None,
                player_number2: None
            }];

            self.get_user_info_mut(&user_id).team.score += 1;

            action
        } else {
            vec![Save {
                action_type: ActionTypes::Save,
                account_id: user_opponent.account_id.clone(),
                goalie_number: opponent_goalie.number,
            }]
        }
    }

    fn get_random_position(&self) -> PlayerPosition {
        let positions = match self.zone_number {
            1 => vec![LeftDefender, RightDefender],
            2 => vec![Center],
            3 => vec![LeftWing, RightWing],
            _ => panic!("Undefined zone number")
        };

        let rnd = Game::get_random_in_range(1, positions.len(), 22);

        positions[rnd]
    }

    fn get_random_position_after_offside(&self) -> PlayerPosition {
        let user_id = self.get_user_id_player_with_puck();
        let positions = if user_id == 1 {
            vec![LeftWing, RightWing]
        } else {
            vec![LeftDefender, RightDefender]
        };

        let rnd = Game::get_random_in_range(1, positions.len(), 22);

        positions[rnd]
    }

    fn face_off_after_save(&mut self) -> Vec<ActionData> {
        let user_player_id = self.get_player_id_with_puck();
        let position_player_with_puck = self.get_player_pos(
            &user_player_id.1,
            user_player_id.0);

        let user = self.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();

        let opponent_user = self.get_opponent_info(user_player_id.0);
        let opponent_active_five = opponent_user.team.get_active_five();

        let position = match position_player_with_puck {
            LeftWing | LeftDefender => {
                *self.get_opponent_position(
                    active_five.get_number_of_players(),
                    opponent_active_five.get_number_of_players(),
                    &LeftWing).1
            },
            RightWing | RightDefender => {
                *self.get_opponent_position(
                    active_five.get_number_of_players(),
                    opponent_active_five.get_number_of_players(),
                    &RightWing).1
            },
            Center => {
                self.get_random_position()
            },
            _ => panic!("Player position not found after save")
        };

        self.face_off(&position)
    }

    fn face_off(&mut self, player_position: &PlayerPosition) -> Vec<ActionData> {
        let player1 = self.get_field_player_by_pos(1, player_position);
        let user = self.get_user_info(player1.get_user_id());
        let position1 = user.team.get_field_player_pos(&player1.get_player_id());
        let active_five = user.team.get_active_five();

        let opponent_user = self.get_opponent_info(user.user_id);
        let opponent_five = opponent_user.team.get_active_five();

        let opponent_pos = self.get_opponent_position(
            active_five.get_number_of_players(),
            opponent_five.get_number_of_players(),
            position1);
        let player2 = self.get_field_player_by_pos(2, opponent_pos.1);

        let compared_stat1 = get_relative_field_player_stat(
            player1, player1.stats.face_offs as f32);

        let compared_stat2= get_relative_field_player_stat(
            player2, player2.stats.face_offs as f32) * opponent_pos.0;

        let mut actions = vec![FaceOff {
            action_type: ActionTypes::FaceOff,
            account_id1: user.account_id.clone(),
            player_number1: player1.number,
            player_position1: position1.clone(),
            account_id2: opponent_user.account_id.clone(),
            player_number2: player2.number,
            player_position2: opponent_pos.1.clone(),
        }];

        if has_won(compared_stat1, compared_stat2) {
            actions.push(FaceOffWin {
                action_type: ActionTypes::FaceOffWin,
                account_id: user.account_id.clone(),
                player_number: player1.number,
                player_position: position1.clone(),
            });

            self.player_with_puck = Option::from((player1.get_user_id(), player1.get_player_id()));
        } else {
            actions.push(FaceOffWin {
                action_type: ActionTypes::FaceOffWin,
                account_id: opponent_user.account_id.clone(),
                player_number: player2.number,
                player_position: opponent_pos.1.clone(),
            });

            self.player_with_puck = Option::from((player2.get_user_id(), player2.get_player_id()));
        }

        actions
    }

    fn increase_five_time_field(&mut self) {
        let five1 = self.user1.team.get_active_five_mut();
        five1.time_field = Some(five1.time_field.unwrap() + 1);

        let five2 = self.user2.team.get_active_five_mut();
        five2.time_field = Some(five2.time_field.unwrap() + 1);
    }

    fn check_teams_to_change_active_five(&mut self) {
        if self.user1.team.need_change() {
            self.reduce_strength(self.user1.user_id);
            self.user1.team.change_active_five();
        }
        if self.user2.team.need_change() {
            self.reduce_strength(self.user2.user_id);
            self.user2.team.change_active_five();
        }
    }

    fn check_end_of_period(&mut self) -> Option<ActionData> {
        if [25, 50, 75].contains(&self.turns) {
            let period = if self.turns >= 75 {
                3
            } else if self.turns >= 50 {
                2
            } else {
                1
            };

            self.swap_users();

            return Some(EndOfPeriod { action_type: ActionTypes::EndOfPeriod, number: period });
        }

        None
    }

    fn is_game_over(&self) -> bool {
        if self.turns >= NUMBER_OF_STEPS && self.user1.team.score != self.user2.team.score {
            true
        } else {
            false
        }
    }

    fn get_winner_id(&self) -> usize {
         if self.user2.team.score > self.user1.team.score {
             2
         } else {
             1
         }
    }

    fn reduce_penalty(&mut self) -> Vec<ActionData> {
        let mut actions = Vec::new();

        let mut first_team_action = self.reduce_user_player_penalty(&1);
        actions.append(&mut first_team_action);

        let mut second_team_action = self.reduce_user_player_penalty(&2);
        actions.append(&mut second_team_action);

        actions
    }

    fn reduce_user_player_penalty(&mut self, user_id: &UserId) -> Vec<ActionData> {
        let user = self.get_user_info_mut(user_id);
        let number_of_penalty_players = user.team.penalty_players.len();

        let mut liberated_players: Vec<usize> = Vec::new();

        for i in 0.. number_of_penalty_players {
            // Simultaneous report for only two deletions
            if i > 1 {
                break;
            }

            let player_id = user.team.penalty_players.get(i).unwrap().clone();
            let player = user.team.get_field_player_mut(&player_id);
            player.number_of_penalty_events = Some(player.number_of_penalty_events.unwrap() - 1);

            if player.number_of_penalty_events.unwrap() == 0 {
                liberated_players.push(i);
            }
        }

        let mut index = liberated_players.len();
        let mut actions = Vec::new();
        while index > 0 {
            index -= 1;
            let penalty_index = liberated_players[index];

            let player_id = user.team.penalty_players.remove(penalty_index);
            let player = user.team.get_field_player(&player_id);

            actions.push(EndedPenalty {
                action_type: ActionTypes::EndedPenalty,
                account_id: user.account_id.clone(),
                player_number: player.number,
            })
        }

        for _ in 0..liberated_players.len() {
            self.put_players_on_field(user_id);
        }

        actions
    }

    fn swap_players_in_five(&mut self, user_id: &UserId) {
        let player_with_puck: Option<TokenId>= if self.player_with_puck.is_some() {
            Some(self.player_with_puck.clone().unwrap().1)
        } else {
            None
        };

        let user = self.get_user_info_mut(user_id);

        user.team.swap_players_in_active_five(player_with_puck);
    }

    // If a goal is scored
    pub fn remove_penalty_players(&mut self, user_id: &UserId) -> Option<ActionData> {
        let user_info = self.get_user_info_mut(user_id);

        let small_penalties_len = user_info.team.players_to_small_penalty.len();
        if small_penalties_len != 0 {
            user_info.team.players_to_small_penalty.remove(small_penalties_len - 1);
            return None;
        }

        let big_penalties_len = user_info.team.players_to_big_penalty.len();
        if big_penalties_len != 0 {
            user_info.team.players_to_small_penalty.remove(big_penalties_len - 1);
            return None;
        }

        if user_info.team.penalty_players.len() != 0 {

            let player_id = user_info.team.penalty_players.remove(0);
            let player = user_info.team.get_field_player(&player_id);

            let action = Some(EndedPenalty {
                action_type: ActionTypes::EndedPenalty,
                account_id: user_info.account_id.clone(),
                player_number: player.number,
            });

            self.put_players_on_field(user_id);

            return action;
        }

        None
    }

    // If the number of players in pk = 3 we need to release the 4th in pks and the 5th in pps
    fn put_players_on_field(&mut self, user_id: &UserId) {
        let user_info = self.get_user_info_mut(user_id);

        let number_of_players_in_pks = user_info.team
            .get_five_number_of_players(&PenaltyKill1);

        if number_of_players_in_pks == 3 {
            user_info.team.release_removed_players_in_brigades();
        } else {
            user_info.team.active_five.current_number = First;
            user_info.team.insert_player_to_active_five(&LeftWing);
        }
    }

    fn swap_users(&mut self) {
        let temp_user = self.user1.clone();
        self.user1 = self.user2.clone();
        self.user2 = temp_user;

        self.user1.user_id = USER_ID1;
        self.user2.user_id = USER_ID2;

        for (_, player) in &mut self.user1.team.field_players {
            player.user_id = Some(USER_ID1);
        }
        for (_, goalie) in &mut self.user1.team.goalies {
            goalie.user_id = Some(USER_ID1);
        }

        for (_, player) in &mut self.user2.team.field_players {
            player.user_id = Some(USER_ID2);
        }
        for (_, goalie) in &mut self.user2.team.goalies {
            goalie.user_id = Some(USER_ID2);
        }
    }
}