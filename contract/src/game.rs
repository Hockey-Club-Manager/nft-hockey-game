use near_sdk::{AccountId, Timestamp};

use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::env::panic;
use crate::goalie::{Goalie, GoalieStats};
use crate::player_field::{FieldPlayer, FieldPlayerStats};
use crate::user::User;
use crate::action::{Action, ActionTypes, generate_an_event, get_relative_field_player_stat, has_won};
use crate::action::ActionTypes::{Battle, EndOfPeriod, Goal, Save};
use crate::player::{PlayerPosition, PlayerRole};
use crate::player::PlayerPosition::{Center, LeftDefender, RightDefender, RightWing};
use crate::{StorageKey, TokenBalance};

extern crate rand;

use rand::Rng;
use crate::player::PlayerRole::{Dangler, Goon, Post2Post, Professor, Shooter, TryHarder, Wall};
use crate::StorageKey::FieldPlayers;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserInfo {
    pub(crate) user: User,
    pub(crate) field_players: UnorderedMap<PlayerPosition, FieldPlayer>,
    pub(crate) goalie: Goalie,
    pub(crate) account_id: AccountId,
}

pub struct Team {
    pub(crate) field_players: UnorderedMap<PlayerPosition, FieldPlayer>,
    pub(crate) goalie: Goalie,
}

pub struct Event {
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
    pub(crate) my_team: Team,
    pub(crate) opponent_team: Team,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct EventToSave {
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
}

impl From<Event> for EventToSave {
    fn from(event: Event) -> Self {
        Self {
            action: event.action,
            zone_number: event.zone_number,
            time: event.time,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    pub(crate) user1: UserInfo,
    pub(crate) user2: UserInfo,
    pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    pub(crate) total_time_spent: Vec<Timestamp>,
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) zone_number: i8,
    pub(crate) turns: u128,
    pub(crate) events: Vec<EventToSave>,
}

impl Game {
    pub fn new (account_id_1: AccountId, account_id_2: AccountId, reward: TokenBalance) -> Game {
        let (user1, user2) = Game::create_two_players();

        let field_players1 = Game::create_field_players_with_random_stats(user1.id);
        let goalie1 = Game::create_goalie_with_random_stats(Post2Post ,user1.id);

        let field_players2 = Game::create_field_players_with_random_stats(user1.id);
        let goalie2 = Game::create_goalie_with_random_stats(Wall ,user1.id);

        let user_info1 = UserInfo {
            user: user1,
            field_players: field_players1,
            goalie: goalie1,
            account_id: account_id_1,
        };

        let user_info2 = UserInfo {
            user: user2,
            field_players: field_players2,
            goalie: goalie2,
            account_id: account_id_2,
        };

        Game {
            user1: user_info1,
            user2: user_info2,
            reward,
            winner_index: None,
            total_time_spent: [0, 0].to_vec(),
            player_with_puck: None,
            zone_number: 2,
            turns: 0,
            events: vec![],
        }
    }

    // creates and returns two players with distinct IDs
    fn create_two_players() -> (User, User) {
        (
            User { id: 1, score: 0 },
            User { id: 2, score: 0 }
        )
    }

    fn create_field_players_with_random_stats(user_id: usize) -> UnorderedMap<PlayerPosition, FieldPlayer> {
        let mut field_players = UnorderedMap::new(FieldPlayers);

        let center = Game::create_field_player_with_random_stats(Shooter, Center, user_id);
        let right_wind = Game::create_field_player_with_random_stats(TryHarder, RightWing, user_id);
        let left_wind = Game::create_field_player_with_random_stats(Dangler, RightWing, user_id);
        let right_defender = Game::create_field_player_with_random_stats(Goon, RightDefender, user_id);
        let left_defender = Game::create_field_player_with_random_stats(Professor, LeftDefender, user_id);

        field_players.insert(&center.get_player_position(), &center);
        field_players.insert(&right_wind.get_player_position(), &right_wind);
        field_players.insert(&left_wind.get_player_position(), &left_wind);
        field_players.insert(&right_defender.get_player_position(), &right_defender);
        field_players.insert(&left_defender.get_player_position(), &left_defender);
        field_players
    }

    fn create_field_player_with_random_stats(role: PlayerRole, position: PlayerPosition, user_id: usize) -> FieldPlayer {
        let mut rng = rand::thread_rng();

        FieldPlayer::new(
            position,
            role,
            user_id,
            FieldPlayerStats::new(
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60.0, 95.0),
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60, 95) as u128
            ))
    }

    fn create_goalie_with_random_stats(role: PlayerRole, user_id: usize) -> Goalie {
        let mut rng = rand::thread_rng();

        Goalie::new(
            role,
            user_id,
            GoalieStats::new(
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60, 95) as u128,
                rng.gen_range(60, 95) as u128,
            )
        )
    }
}

impl Game {
    fn get_center_forward_in_the_zone(&self, user: &UserInfo) -> FieldPlayer {
        match user.field_players.get(&Center) {
            Some(player) => player,
            _ => panic!("Player not found")
        }
    }

    fn battle(&mut self) {
        let player1 = self.get_center_forward_in_the_zone(&self.user1);
        let player2 = self.get_center_forward_in_the_zone(&self.user2);

        let player1_stat = get_relative_field_player_stat(&player1, player1.stats.strength);
        let player2_stat = get_relative_field_player_stat(&player2, player2.stats.strength);

        if has_won(player1_stat, player2_stat) {
            self.player_with_puck = Option::from(player1);
        } else {
            self.player_with_puck = Option::from(player2);
        }
    }

    fn face_off(&mut self) {
        self.battle();
        generate_an_event(Battle, self);
    }

    fn step(&mut self) {
        let action_type = self.get_last_action();
        let action = Action;

        match action_type {
            Goal => self.face_off(),
            Save => self.face_off(),
            EndOfPeriod => self.face_off(),
             _ => action.do_random_action(self)
         };

        self.turns += 1;

        if [30, 60, 90].contains(&self.turns) {
            generate_an_event(EndOfPeriod, self);
            self.zone_number = 2;
        }
    }

    fn get_last_action(&self) -> &ActionTypes {
        &self.events[self.events.len() - 1].action
    }

    pub fn get_user_info(&mut self, user_id: usize) -> &mut UserInfo {
        if user_id == 1 {
            &mut self.user1
        } else {
            &mut self.user2
        }
    }
}