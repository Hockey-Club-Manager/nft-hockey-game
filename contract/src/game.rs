use near_sdk::{AccountId, Timestamp};

use std::collections::HashMap;
use crate::goalie::Goalie;
use crate::player_field::FieldPlayer;
use crate::user::User;
use crate::action::{Action, ActionTypes, generate_an_event, get_relative_field_player_stat, has_won};
use crate::action::ActionTypes::{Battle, EndOfPeriod, Goal, Save};
use crate::player::{PlayerPosition};
use crate::player::PlayerPosition::{Center};
use crate::TokenBalance;

pub struct UserInfo {
    pub(crate) user: User,
    pub(crate) field_players: HashMap<PlayerPosition, FieldPlayer>,
    pub(crate) goalie: Goalie,
    pub(crate) account_id: AccountId,
}

pub struct Team {
    pub(crate) field_players: HashMap<PlayerPosition, FieldPlayer>,
    pub(crate) goalie: Goalie,
}

pub struct Event {
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
    pub(crate) my_team: Team,
    pub(crate) opponent_team: Team,
}

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

pub struct Game {
    pub(crate) users: [UserInfo; 2],
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

    }

    // creates and returns two players with distinct IDs
    fn create_two_players() -> (User, User) {
        (
            User { id: 1, score: 0 },
            User { id: 2, score: 0 }
        )
    }
}

impl Game {
    fn get_center_forward_in_the_zone(&self, user: &UserInfo) -> FieldPlayer {
        user.field_players[&Center]
    }

    fn battle(&mut self) {
        let player1 = self.get_center_forward_in_the_zone(&self.users[0]);
        let player2 = self.get_center_forward_in_the_zone(&self.users[1]);

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
}