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
use crate::team::five::IceTimePriority;
use crate::team::five::Tactics::Neutral;
use crate::team::team::Team;
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
    pub(crate) action: ActionTypes,
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
    pub(crate) last_event_generation_time: Timestamp,
    pub(crate) player_with_puck: Option<(UserId, TokenId)>,
    pub(crate) zone_number: i8,
    pub(crate) turns: u128,
    pub(crate) last_action: ActionTypes,
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
            user_id: 1,
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
            player_with_puck: None,
            zone_number: 2,
            turns: 0,
            last_action: StartGame
        };
        game.generate_an_event(StartGame);

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
        let player_id = five.field_players.get(position).unwrap();
        user_info.team.get_field_player(player_id)
    }

    pub fn get_field_player_id_by_pos(&self, user_id: UserId, position:& PlayerPosition) -> TokenId {
        let user_info = self.get_user_info(user_id);
        user_info.team.get_active_five().field_players.get(position).unwrap().clone()
    }

    pub fn get_player_pos(&self, player_id: &TokenId, user_id: UserId) -> &PlayerPosition {
        let user_info = self.get_user_info(user_id);
        user_info.team.get_field_player_pos(player_id)
    }

    pub fn get_player_with_puck_mut(&mut self) -> &mut FieldPlayer {
        let unwrapped_player = self.get_player_id_with_puck();
        let user = self.get_user_info_mut(unwrapped_player.0);

        user.team.field_players.get_mut(&unwrapped_player.1).unwrap()
    }

    pub fn get_player_id_with_puck(&self) -> (UserId, TokenId) {
        self.player_with_puck.clone().unwrap()
    }

    pub fn get_player_with_puck(&self) -> &FieldPlayer {
        let unwrapped_player = self.get_player_id_with_puck();
        let user = self.get_user_info(unwrapped_player.0);

        user.team.field_players.get(&unwrapped_player.1).unwrap()
    }

    pub fn get_user_info_mut(&mut self, user_id: usize) -> &mut UserInfo {
        if user_id == 1 {
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

    pub fn get_user_id_player_with_puck(&self) -> usize {
        let player_with_puck = self.get_player_with_puck();
        player_with_puck.user_id.unwrap()
    }

    pub fn get_opponents_field_player(&self) -> &FieldPlayer {
        let user_player_ids = self.player_with_puck.clone().unwrap();

        let user = self.get_user_info(user_player_ids.0);
        let position = user.team.get_field_player_pos(&user_player_ids.1);

        return if user_player_ids.0 == 1 {
            self.get_field_player_by_pos(2, position)
        } else {
            self.get_field_player_by_pos(1, position)
        }
    }

    pub fn reduce_strength(&mut self) {
        let five1 = self.user1.team.get_active_five().clone();
        for (_player_pos, field_player_id) in five1.field_players {
            let amount_of_spent_strength = self.get_amount_of_spent_strength(five1.ice_time_priority);

            let field_player = self.user1.team.get_field_player_mut(&field_player_id);
            field_player.stats.decrease_strength(amount_of_spent_strength);
        }

        let five2 = self.user2.team.get_active_five().clone();
        for (_player_pos, field_player_id) in five2.field_players {
            let amount_of_spent_strength = self.get_amount_of_spent_strength(five2.ice_time_priority);

            let field_player = self.user2.team.get_field_player_mut(&field_player_id);
            field_player.stats.decrease_strength(amount_of_spent_strength);
        }
    }

    fn get_amount_of_spent_strength(&self, ice_time_priority: IceTimePriority) -> u8 {
        match ice_time_priority {
            IceTimePriority::SuperLowPriority => { 1 }
            IceTimePriority::LowPriority => { 2 }
            IceTimePriority::Normal => { 3 }
            IceTimePriority::HighPriority => { 4 }
            IceTimePriority::SuperHighPriority => { 5 }
        }
    }


    pub fn generate_an_event(&self ,action: ActionTypes) {
        let generated_event = Event {
            user1: self.user1.clone(),
            user2: self.user2.clone(),
            time: self.last_event_generation_time.clone(),
            zone_number: self.zone_number.clone(),
            action,
            player_with_puck: self.player_with_puck.clone(),
        };

        let json_event = match serde_json::to_string(&generated_event) {
            Ok(res) => res,
            Err(e) => panic!("{}", e)
        };
        log!("{}", json_event);
    }
}

impl Game {
    fn battle(&mut self) {
        let player_id1 = self.get_center_id_forward_in_the_zone(&self.user1);
        let player_id2 = self.get_center_id_forward_in_the_zone(&self.user2);

        let player1 = self.user1.team.get_field_player(&player_id1);
        let player2 = self.user2.team.get_field_player(&player_id2);

        let player1_stat = (player1.stats.puck_control + player1.stats.aggressiveness + player1.stats.strength) as f32 / 3.0;
        let player2_stat = (player2.stats.puck_control + player2.stats.aggressiveness + player2.stats.strength) as f32 / 3.0;

        let compared_stat1 = get_relative_field_player_stat(player1, player1_stat);
        let compared_stat2= get_relative_field_player_stat(player2, player2_stat);

        if has_won(compared_stat1, compared_stat2) {
            self.player_with_puck = Option::from((player1.get_user_id(), player1.get_player_id()));
        } else {
            self.player_with_puck = Option::from((player2.get_user_id(), player2.get_player_id()));
        }
    }

    fn battle_by_position(&mut self, pos: &PlayerPosition) {
        let player1 = self.get_field_player_by_pos(1, pos);
        let player2 = self.get_field_player_by_pos(2, pos);

        let player1_stat = (player1.stats.puck_control + player1.stats.aggressiveness + player1.stats.strength) as f32 / 3.0;
        let player2_stat = (player2.stats.puck_control + player2.stats.aggressiveness + player2.stats.strength) as f32 / 3.0;

        let compared_stat1 = get_relative_field_player_stat(player1, player1_stat);
        let compared_stat2= get_relative_field_player_stat(player2, player2_stat);

        if has_won(compared_stat1, compared_stat2) {
            self.player_with_puck = Option::from((player1.get_user_id(), player1.get_player_id()));
        } else {
            self.player_with_puck = Option::from((player2.get_user_id(), player2.get_player_id()));
        }
    }

    fn face_off(&mut self) {
        self.generate_an_event(FaceOff);

        let player_id1 = self.get_center_id_forward_in_the_zone(&self.user1);
        let player_id2 = self.get_center_id_forward_in_the_zone(&self.user2);

        let player1 = self.user1.team.get_field_player(&player_id1);
        let player2 = self.user2.team.get_field_player(&player_id2);

        let compared_stat1 = get_relative_field_player_stat(player1, player1.stats.face_offs as f32);
        let compared_stat2= get_relative_field_player_stat(player2, player2.stats.face_offs as f32);

        if has_won(compared_stat1, compared_stat2) {
            self.player_with_puck = Option::from((player1.get_user_id(), player1.get_player_id()));
        } else {
            self.player_with_puck = Option::from((player2.get_user_id(), player2.get_player_id()));
        }

        self.generate_an_event(FaceOffWin);
    }

    fn fight(&mut self) {

    }

    fn get_center_id_forward_in_the_zone(&self, user: &UserInfo) -> TokenId {
        match user.team.get_active_five().field_players.get(&Center) {
            Some(player) => player.clone(),
            _ => panic!("Player not found")
        }
    }

    pub fn step(&mut self) -> GameState {
        let action = Action;

        match self.last_action {
            StartGame | Goal | Save | Penalty | Icing | NetOff | PuckOff | EndOfPeriod => self.face_off(),
            Rebound => {
                let player_pos = get_random_position_after_rebound();
                self.battle_by_position(&player_pos);

                self.generate_an_event(Battle);
            },
             _ => action.do_random_action(self)
        };

        self.turns += 1;

        if self.user1.team.need_change() {
            self.user1.team.change_active_five();

            self.generate_an_event(FirstTeamChangeActiveFive);
        }
        if self.user2.team.need_change() {
            self.user2.team.change_active_five();

            self.generate_an_event(SecondTeamChangeActiveFive);
        }

        if [30, 60, 90].contains(&self.turns) {
            self.generate_an_event(EndOfPeriod);
            self.zone_number = 2;
        }

        let state = if self.is_game_over() {
            self.generate_an_event(GameFinished);
            GameState::GameOver { winner_id: self.get_winner_id() }
        } else {
            GameState::InProgress
        };

        if state == GameState::InProgress && self.turns == 90 {
            self.generate_an_event(Overtime);
        }

        state
    }


    fn is_game_over(&self) -> bool {
        if self.turns >= 90 && self.user1.team.score != self.user2.team.score {
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
}

fn get_random_position_after_rebound() -> PlayerPosition {
    let rnd = Game::get_random_in_range(0, 10, 20);

    let probability_distribution = vec![1, 1, 2, 2, 3, 3, 3, 3, 4, 5];

    let num_player_pos = probability_distribution[rnd];

    match num_player_pos {
        1 => LeftDefender,
        2 => RightDefender,
        3 => Center,
        4 => LeftWing,
        5 => RightWing,
        _ => panic!("Player position not found")
    }
}