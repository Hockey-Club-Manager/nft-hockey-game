use std::collections::HashMap;
use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, Timestamp};
use near_sdk::serde::{Deserialize, Serialize};
use crate::team::players::field_player::{FieldPlayer};
use crate::game::actions::action::{Action, ActionTypes};
use crate::game::actions::action::ActionTypes::*;
use crate::team::players::player::{PlayerPosition};
use crate::team::players::player::PlayerPosition::*;
use crate::{TokenBalance};
use crate::game::actions::utils::{get_relative_field_player_stat, has_won};
use crate::PlayerPosition::LeftWing;
use crate::team::five::{ActiveFive, FiveIds, IceTimePriority};
use crate::team::numbers::FiveNumber::{First, PenaltyKill1, PenaltyKill2, PowerPlay1, PowerPlay2};
use crate::team::team_metadata::team_metadata_to_team;
use crate::user_info::UserId;


#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    GameOver { winner_id: usize },
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    pub(crate) player_with_puck: Option<(UserId, TokenId)>,
    pub(crate) actions: Vec<ActionTypes>,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
    pub(crate) user1: UserInfo,
    pub(crate) user2: UserInfo,
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
    pub(crate) last_action: ActionTypes,

    pub(crate) last_event_generation_time: Timestamp,

    // in nanoseconds
    pub(crate) event_generation_delay: u64,
    pub(crate) number_of_generated_events_in_block: u8,
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
            user_id: 1,
            team: team1,
            account_id: account_id_1,
            take_to_called: false,
            coach_speech_called: false,
            is_goalie_out: false,
        };

        let user_info2 = UserInfo {
            user_id: 2,
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
            last_event_generation_time: env::block_timestamp(),
            event_generation_delay: SECOND,
            player_with_puck: None,
            zone_number: 2,
            turns: 0,
            last_action: StartGame,
            number_of_generated_events_in_block: 0
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
        let user_info = self.get_user_info(user_id);
        let five = user_info.team.get_active_five();
        let player_id = match five.field_players.get(position) {
            Some(id) => {
                if *id == "" {
                    self.get_player_id_by_penalty_pos(position, five)
                } else {
                    id.clone()
                }
            },
            None => self.get_player_id_by_penalty_pos(position, five)
        };

        user_info.team.get_field_player(&player_id)
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
            let user_info = self.get_user_info_mut(&user_id);
        let player_id = user_info.team.get_active_five().field_players.get(position).unwrap().clone();

        user_info.team.get_field_player_mut(&player_id)
    }

    pub fn get_field_player_id_by_pos(&self, user_id: UserId, position:& PlayerPosition) -> TokenId {
        let user_info = self.get_user_info(user_id);
        user_info.team.get_active_five().field_players.get(position).unwrap().clone()
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
        if *user_id == 1 {
            return &mut self.user1;
        }

        &mut self.user2
    }

    pub fn get_user_info(&self, user_id: usize) -> &UserInfo {
        if user_id == 1 {
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
        if user_id == 0 {
            return self.get_user_info(1);
        }

        self.get_user_info(0)
    }

    pub fn get_opponent_info_mut(&mut self, user_id: &usize) -> &mut UserInfo {
        if *user_id == 0 {
            return self.get_user_info_mut(&1);
        }

        self.get_user_info_mut(&0)
    }

    pub fn get_opponent_field_player(&self) -> (f32, &FieldPlayer) {
        let user_player_ids = self.player_with_puck.clone().unwrap();
        let position = self.get_coeff_player_pos(&user_player_ids);

        return if user_player_ids.0 == 1 {
            (position.0, self.get_field_player_by_pos(2, &position.1))
        } else {
            (position.0, self.get_field_player_by_pos(1, &position.1))
        }
    }

    pub fn get_opponent_field_player_mut(&mut self) -> (f32, &mut FieldPlayer) {
        let user_player_ids = self.player_with_puck.clone().unwrap();

        let position = self.get_coeff_player_pos(&user_player_ids);

        return if user_player_ids.0 == 1 {
            (position.0, self.get_field_player_by_pos_mut(2, &position.1))
        } else {
            (position.0, self.get_field_player_by_pos_mut(1, &position.1))
        }
    }

    fn get_coeff_player_pos(&self, user_player_ids: &(UserId, TokenId)) -> (f32, PlayerPosition) {
        let user = self.get_user_info(user_player_ids.0);
        let active_five = user.team.get_active_five();

        let opponent_user = self.get_opponent_info(user.user_id);
        let opponent_active_five = opponent_user.team.get_active_five();

        let field_player_pos = user.team.get_field_player_pos(&user_player_ids.1);

        let result = self.get_opponent_position(&active_five.field_players,&opponent_active_five.field_players,field_player_pos);
        (result.0, result.1.clone())
    }

    pub fn get_opponent_position(
        &self,
        players: &HashMap<PlayerPosition, TokenId>,
        opponent_players: &HashMap<PlayerPosition, TokenId>,
        position: &PlayerPosition
    ) -> (f32, &PlayerPosition) {
        match position {
            Center => {
                if opponent_players.get(&AdditionalPosition).is_some() && players.get(&AdditionalPosition).is_some() {
                    (1.0, &Center)
                } else if opponent_players.get(&AdditionalPosition).is_some() {
                    (1.5, &Center)
                } else {
                    (0.5, &Center)
                }
            },
            AdditionalPosition => {
                if opponent_players.get(&AdditionalPosition).is_some() {
                    (1.0, &AdditionalPosition)
                } else if players.len() == 4 {
                    (1.5, &RightDefender)
                }
                else {
                    (1.5, &Center)
                }
            },
            LeftWing => {
                let number_of_opponent_players = opponent_players.len();

                if number_of_opponent_players == 3 || number_of_opponent_players == 4 {
                    (1.5, &RightDefender)
                } else {
                    (1.0, &RightDefender)
                }
            },
            RightWing => {
                let number_of_players = players.len();
                let number_of_opponent_players = opponent_players.len();
                if (number_of_opponent_players == 5 || number_of_opponent_players == 6) && (number_of_players == 5 || number_of_players == 6) {
                    (1.0, &LeftDefender)
                } else if number_of_players > number_of_opponent_players {
                    (0.5, &LeftDefender)
                } else {
                    (1.5, &LeftDefender)
                }
            },
            LeftDefender => {
                let number_of_players = players.len();
                let number_of_opponent_players = opponent_players.len();
                if (number_of_opponent_players == 5 || number_of_opponent_players == 6) && (number_of_players == 5 || number_of_players == 6) {
                    (1.0, &RightWing)
                } else if opponent_players.get(&RightWing).is_some() {
                    if number_of_opponent_players >= number_of_players  {
                        (1.5, &RightWing)
                    } else {
                        (0.5, &RightWing)
                    }
                } else {
                    (1.0, &RightDefender)
                }
            },
            RightDefender => {
                let number_of_players = players.len();
                let number_of_opponent_players = opponent_players.len();
                if (number_of_opponent_players == 5 || number_of_opponent_players == 6) && (number_of_players == 5 || number_of_players == 6) {
                    (1.0, &LeftWing)
                } else if number_of_players > number_of_opponent_players {
                    (0.5, &LeftDefender)
                } else {
                    (1.5, &LeftDefender)
                }
            },
            _ => panic!("Cannot find opponent position")
        }
    }

    pub fn f(&self, position: &PlayerPosition) -> &PlayerPosition {
        match position {
            Center => &Center,
            LeftWing => &RightDefender,
            RightWing => &LeftDefender,
            LeftDefender => &RightWing,
            RightDefender => &LeftWing,
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

    pub fn generate_event(&mut self, actions: &Vec<ActionTypes>) -> Event {
        let non_game_events: Vec<ActionTypes> = vec![
            TakeTO,
            CoachSpeech,
            GoalieOut,
            GoalieBack,

            FirstTeamChangeActiveFive,
            SecondTeamChangeActiveFive,

            EndedPenaltyForTheFirstTeam,
            EndedPenaltyForTheSecondTeam,
        ];

        for action in actions {
            if !non_game_events.contains(action) {
                self.last_action = *action;
            }
        }

        Event {
            user1: self.user1.clone(),
            user2: self.user2.clone(),
            time: self.last_event_generation_time.clone(),
            zone_number: self.zone_number.clone(),
            actions: actions.clone(),
            player_with_puck: self.player_with_puck.clone(),
        }
    }

    pub fn do_penalty(
        &mut self,
        penalty_time: u8,
        penalty_player_id: &TokenId,
        user_id: &UserId,
        penalty_user_id: &UserId
    ) {
        self.penalty_player(penalty_time, penalty_player_id, penalty_user_id);

        let penalty_user = self.get_user_info_mut(penalty_user_id);
        penalty_user.team.do_penalty(&penalty_player_id);

        let user = self.get_user_info_mut(user_id);
        let active_five = user.team.active_five.get_current_five_number();

        let brigades = vec![PenaltyKill1, PenaltyKill2, PowerPlay1, PowerPlay2];
        if !brigades.contains(&active_five) {
            user.team.active_five.last_number = user.team.active_five.current_number;
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
    pub fn get_game_state(&mut self) -> (GameState, Option<ActionTypes>) {
        return if self.is_game_over() {
            (
                GameState::GameOver { winner_id: self.get_winner_id() },
                Some(GameFinished)
            )
        } else {
            let state = GameState::InProgress;

            if self.turns == NUMBER_OF_STEPS {
                (state, Some(Overtime))
            } else {
                (state, None)
            }
        };
    }

    pub fn step(&mut self) -> Vec<ActionTypes> {
        let mut actions = self.do_action();

        actions.append(&mut self.check_teams_to_change_active_five());
        actions.append(&mut self.reduce_penalty());

        let end_of_period_event = self.check_end_of_period();
        if end_of_period_event.is_some() {
            actions.push(end_of_period_event.unwrap());
        }

        actions
    }

    fn do_action(&mut self) -> Vec<ActionTypes> {
        let action = Action;

        let actions = match self.last_action {
            StartGame | Goal | EndOfPeriod => {
                self.zone_number = 2;
                self.face_off(&Center)
            },
            Fight | PuckOut | NetOff => {
                let random_position = self.get_random_position();
                self.face_off(&random_position)
            }
            Save => {
                self.face_off_after_save()
            },
            SmallPenalty | BigPenalty | Icing => {
                self.zone_number = match self.get_user_id_player_with_puck() {
                    1 => 1,
                    2 => 3,
                    _ => panic!("User id not found :(")
                };

                let random_position = self.get_random_position();
                self.face_off(&random_position)
            },

            _ => action.do_action(self)
        };


        self.turns += 1;

        actions
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

    fn face_off_after_save(&mut self) -> Vec<ActionTypes> {
        let user_player_id = self.get_player_id_with_puck();
        let position_player_with_puck = self.get_player_pos(&user_player_id.1, user_player_id.0);

        let user = self.get_user_info(user_player_id.0);
        let active_five = user.team.get_active_five();

        let opponent_user = self.get_opponent_info(user_player_id.0);
        let opponent_active_five = opponent_user.team.get_active_five();

        let position = match position_player_with_puck {
            LeftWing | LeftDefender => {
                *self.get_opponent_position(&active_five.field_players, &opponent_active_five.field_players, &LeftWing).1
            },
            RightWing | RightDefender => {
                *self.get_opponent_position(&active_five.field_players, &opponent_active_five.field_players, &RightWing).1
            },
            Center => {
                self.get_random_position()
            },
            _ => panic!("Player position not found after save")
        };

        self.face_off(&position)
    }

    fn face_off(&mut self, player_position: &PlayerPosition) -> Vec<ActionTypes> {
        let mut actions = vec![FaceOff];

        let player1 = self.get_field_player_by_pos(1, player_position);
        let user = self.get_user_info(player1.get_user_id());
        let active_five = user.team.get_active_five();

        let opponent_user = self.get_opponent_info(user.user_id);
        let opponent_five = opponent_user.team.get_active_five();

        let opponent_pos = self.get_opponent_position(&active_five.field_players, &opponent_five.field_players, player_position);
        let player2 = self.get_field_player_by_pos(2, opponent_pos.1);

        let compared_stat1 = get_relative_field_player_stat(player1, player1.stats.face_offs as f32);
        let compared_stat2= get_relative_field_player_stat(player2, player2.stats.face_offs as f32) * opponent_pos.0;

        if has_won(compared_stat1, compared_stat2) {
            self.player_with_puck = Option::from((player1.get_user_id(), player1.get_player_id()));
        } else {
            self.player_with_puck = Option::from((player2.get_user_id(), player2.get_player_id()));
        }

        actions.push(FaceOffWin);

        actions
    }

    fn increase_five_time_field(&mut self) {
        let five1 = self.user1.team.get_active_five_mut();
        five1.time_field = Some(five1.time_field.unwrap() + 1);

        let five2 = self.user2.team.get_active_five_mut();
        five2.time_field = Some(five2.time_field.unwrap() + 1);
    }

    fn check_teams_to_change_active_five(&mut self) -> Vec<ActionTypes> {
        let mut actions = Vec::new();

        if self.user1.team.need_change() {
            self.reduce_strength(self.user1.user_id);
            self.user1.team.change_active_five();
            self.change_player_with_puck(self.user1.user_id);

            actions.push(FirstTeamChangeActiveFive);
        }
        if self.user2.team.need_change() {
            self.reduce_strength(self.user2.user_id);
            self.user2.team.change_active_five();
            self.change_player_with_puck(self.user2.user_id);

            actions.push(SecondTeamChangeActiveFive);
        }

        actions
    }

    fn change_player_with_puck(&mut self, user_id: UserId) {
        let wrapped_player_with_puck = self.player_with_puck.clone();

        let player_with_puck = if wrapped_player_with_puck.is_some() {
            wrapped_player_with_puck.unwrap()
        } else {
            return;
        };

        if player_with_puck.0 != user_id {
            return;
        }

        let pos_player_with_puck = self.get_player_pos(&player_with_puck.1, player_with_puck.0);
        let new_player_with_puck = self.get_field_player_by_pos(user_id, pos_player_with_puck);
        let player_id = new_player_with_puck.id.clone().expect("Player id not found");

        self.player_with_puck = Some((user_id, player_id));
    }

    fn check_end_of_period(&mut self) -> Option<ActionTypes> {
        if [25, 50, 75].contains(&self.turns) {
            return Some(EndOfPeriod);
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

    fn reduce_penalty(&mut self) -> Vec<ActionTypes> {
        let mut actions = Vec::new();

        let first_team_action = self.reduce_user_player_penalty(&1);
        if first_team_action.is_some() {
            actions.push(first_team_action.unwrap());
        }

        let second_team_action = self.reduce_user_player_penalty(&2);
        if second_team_action.is_some() {
            actions.push(second_team_action.unwrap());
        }

        actions
    }

    fn reduce_user_player_penalty(&mut self, user_id: &UserId) -> Option<ActionTypes> {
        let user = self.get_user_info_mut(user_id);

        let number_of_players_in_five = user.team.get_five_number_of_player();

        let number_of_penalty_players = user.team.penalty_players.len();
        let mut is_ended_penalty = false;

        let mut liberated_players: Vec<usize> = Vec::new();

        for i in 0.. number_of_penalty_players {
            if i > 1 {
                break;
            }

            let player_id = user.team.penalty_players.get(i).unwrap().clone();
            let player = user.team.get_field_player_mut(&player_id);
            player.number_of_penalty_events = Some(player.number_of_penalty_events.unwrap() - 1);

            if player.number_of_penalty_events.unwrap() == 0 {
                liberated_players.push(i);

                if number_of_players_in_five == 3 {
                    let active_five = user.team.get_active_five_mut();
                    active_five.field_players.insert(RightWing, player_id);
                } else if number_of_players_in_five == 4 {
                    user.team.active_five.last_number = user.team.active_five.current_number;
                    user.team.active_five.current_number = First;

                    is_ended_penalty = true;
                }
            }
        }

        let mut index = liberated_players.len();
        while index > 0 {
            index -= 1;
            let penalty_index = liberated_players[index];
            user.team.penalty_players.remove(penalty_index);
        }

        let mut result = if is_ended_penalty {
            if *user_id == 1 as usize {
                self.change_player_with_puck(1);
                Some(EndedPenaltyForTheFirstTeam)
            } else {
                self.change_player_with_puck(2);
                Some(EndedPenaltyForTheSecondTeam)
            }
        } else {
            None
        };

        result
    }
}