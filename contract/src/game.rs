use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Timestamp};

use std::borrow::Borrow;
use std::collections::HashMap;
use crate::goalie::Goalie;
use crate::player_field::FieldPlayer;
use crate::user::User;
extern crate rand;

use rand::Rng;
use crate::action::{Action, ActionTypes, generate_an_event, get_opponents_field_player, get_relative_field_player_stat, has_won};
use crate::action::ActionTypes::{Battle, EndOfPeriod, Goal, HitThePuck, Pass};
use crate::player::{Player, PlayerPosition};
use crate::player::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};

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
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    pub(crate) total_time_spent: Vec<Timestamp>,
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) zone_number: i8,
    pub(crate) turns: u128,
    pub(crate) events: Vec<EventToSave>,
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

        self.turns += 1;

        match action_type {
            Goal => self.battle(),
            HitThePuck => self.battle(),
            EndOfPeriod => self.battle(),
             _ => action.do_random_action(self)
         };

        if [30, 60, 90].contains(&self.turns) {
            generate_an_event(EndOfPeriod, game);
        }
    }

    fn get_last_action(&self) -> &ActionTypes {
        &self.events[self.events.len() - 1].action
    }
}