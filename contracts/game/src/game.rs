use crate::*;
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, Metadata, Timestamp};
use near_sdk::serde::{Deserialize, Serialize};
use crate::goalie::{Goalie, GoalieStats};
use crate::player_field::{FieldPlayer, FieldPlayerStats};
use crate::user::UserInfo;
use crate::action::{Action, ActionTypes, generate_an_event, get_relative_field_player_stat, has_won, reduce_strength};
use crate::action::ActionTypes::{Battle, EndOfPeriod, FaceOff, FirstTeamChangeActiveFive, GameFinished, Goal, Overtime, Rebound, Save, SecondTeamChangeActiveFive, StartGame};
use crate::player::{PlayerPosition, PlayerRole};
use crate::player::PlayerPosition::{Center, LeftDefender, RightDefender, RightWing};
use crate::{TokenBalance};
use crate::game::Tactics::Neutral;
use crate::nft_team::team_metadata_to_team;
use crate::player::PlayerRole::{Dangler, Goon, Post2Post, Professor, Shooter, TryHarder, Wall};
use crate::PlayerPosition::LeftWing;
use crate::team::{Five, Fives, Goalies, IceTimePriority, Team, TeamJson};
use crate::team::IceTimePriority::{HighPriority, LowPriority, Normal, SuperHighPriority, SuperLowPriority};

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    GameOver { winner_id: usize },
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Tactics {
    SuperDefensive,
    Defensive,
    Neutral,
    Offensive,
    SupperOffensive,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
    pub(crate) my_team: TeamJson,
    pub(crate) opponent_team: TeamJson,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct EventToSave {
    pub(crate) action: ActionTypes,
    pub(crate) zone_number: i8,
    pub(crate) time: Timestamp,
    pub(crate) player_with_puck: Option<FieldPlayer>,
}

impl From<Event> for EventToSave {
    fn from(event: Event) -> Self {
        Self {
            action: event.action,
            zone_number: event.zone_number,
            time: event.time,
            player_with_puck: event.player_with_puck,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    pub(crate) user1: UserInfo,
    pub(crate) user2: UserInfo,
    pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    pub(crate) last_event_generation_time: Timestamp,
    pub(crate) player_with_puck: Option<FieldPlayer>,
    pub(crate) zone_number: i8,
    pub(crate) turns: u128,
    pub(crate) events: Vec<EventToSave>,
}

impl Game {
    pub fn new (teams: (TeamMetadata, TeamMetadata), account_id_1: AccountId, account_id_2: AccountId, reward: TokenBalance) -> Game {
        let team1 = team_metadata_to_team(teams.0, 1);
        let team2 = team_metadata_to_team(teams.1, 2);

        let user_info1 = UserInfo {
            user_id: 1,
            team: team1,
            account_id: account_id_1,
            take_to_called: false,
            coach_speech_called: false,
            is_goalie_out: false,
            tactic: Neutral,
        };

        let user_info2 = UserInfo {
            user_id: 1,
            team: team2,
            account_id: account_id_2,
            take_to_called: false,
            coach_speech_called: false,
            is_goalie_out: false,
            tactic: Neutral,
        };

        let mut game = Game {
            user1: user_info1,
            user2: user_info2,
            reward,
            winner_index: None,
            last_event_generation_time: env::block_timestamp(),
            player_with_puck: None,
            zone_number: 2,
            turns: 0,
            events: vec![],
        };
        generate_an_event(StartGame, &mut game);

        game
    }

    // fn create_team(user_id: usize) -> Team {
    //     let mut fives = HashMap::new();
    //
    //     let first_five = Game::create_five(user_id, Fives::First, SuperHighPriority);
    //     fives.insert(Fives::First, first_five.clone());
    //     fives.insert(Fives::Second, Game::create_five(user_id, Fives::Second, HighPriority));
    //     fives.insert(Fives::Third, Game::create_five(user_id, Fives::Third, Normal));
    //     fives.insert(Fives::Fourth, Game::create_five(user_id, Fives::Fourth, LowPriority));
    //
    //     let mut goalies = HashMap::new();
    //
    //     let main_goalkeeper = Game::create_goalie_with_random_stats(String::from("Bakin"), 1, Wall ,user_id, String::from(""));
    //     goalies.insert(Goalies::MainGoalkeeper, main_goalkeeper.clone());
    //     goalies.insert(Goalies::SubstituteGoalkeeper, Game::create_goalie_with_random_stats(String::from("Noname"), 2, Post2Post ,user_id, String::from("")));
    //
    //
    //     Team {
    //         fives,
    //         goalies,
    //         active_five: first_five.clone(),
    //         active_goalie: main_goalkeeper.clone(),
    //         score: 0,
    //     }
    // }
    //
    // fn create_five(user_id: usize, number: Fives, ice_time_priority: IceTimePriority) -> Five {
    //     Five {
    //         field_players: Game::create_field_players_with_random_stats(user_id),
    //         number,
    //         ice_time_priority,
    //         time_field: 0,
    //     }
    // }
    //
    // fn create_field_players_with_random_stats(user_id: usize) -> HashMap<String, FieldPlayer> {
    //     let mut field_players = HashMap::new();
    //
    //     let center = Game::create_field_player_with_random_stats(String::from("Schukin"), 10,Shooter, Center,Center, 1.0, user_id, String::from(""));
    //     let right_wind = Game::create_field_player_with_random_stats(String::from("Antipov"), 77,TryHarder, RightWing, RightWing, 1.0, user_id, String::from(""));
    //     let left_wind = Game::create_field_player_with_random_stats(String::from("Kislyak"), 99, Dangler, LeftWing, LeftWing, 1.0, user_id, String::from(""));
    //     let right_defender = Game::create_field_player_with_random_stats(String::from("Ponomarev"), 27,Goon, RightDefender, RightDefender, 1.0, user_id, String::from(""));
    //     let left_defender = Game::create_field_player_with_random_stats(String::from("Tsarev"), 31, Professor, LeftDefender, LeftDefender, 1.0, user_id, String::from(""));
    //
    //     field_players.insert(center.get_player_position().to_string(), center);
    //     field_players.insert(right_wind.get_player_position().to_string(), right_wind);
    //     field_players.insert(left_wind.get_player_position().to_string(), left_wind);
    //     field_players.insert(right_defender.get_player_position().to_string(), right_defender);
    //     field_players.insert(left_defender.get_player_position().to_string(), left_defender);
    //     field_players
    // }
    //
    // fn create_field_player_with_random_stats(name: String, number: u8, role: PlayerRole,
    //                                          native_position: PlayerPosition, position: PlayerPosition,
    //                                          position_coefficient: f32, user_id: usize, img: SRC) -> FieldPlayer {
    //     FieldPlayer::new(
    //         native_position,
    //         position,
    //         position_coefficient,
    //         name,
    //         number,
    //         role,
    //         user_id,
    //         FieldPlayerStats::new(
    //             Game::get_random_in_range(60, 90) as u128,
    //             Game::get_random_in_range(60, 90) as u128,
    //             Game::get_random_in_range(60, 90) as f64,
    //             Game::get_random_in_range(60, 90) as u128,
    //             Game::get_random_in_range(60, 90) as u128
    //         ),
    //         img,
    //     )
    // }
    //
    // fn create_goalie_with_random_stats(name: String, number: u8, role: PlayerRole, user_id: usize, img: SRC) -> Goalie {
    //     Goalie::new(
    //         GoalieStats::new(
    //             Game::get_random_in_range(60, 90)  as u128,
    //             Game::get_random_in_range(60, 90)  as u128,
    //             Game::get_random_in_range(60, 90) as u128,
    //             Game::get_random_in_range(60, 90) as u128,
    //             Game::get_random_in_range(60, 90) as u128,
    //         ),
    //         name,
    //         number,
    //         role,
    //         user_id,
    //         img,
    //     )
    // }

    pub fn get_random_in_range(min: usize, max: usize) -> usize {
        let random = *env::random_seed().get(0).unwrap();
        let random_in_range = (random as f64 / 256.0) * (max - min) as f64 + min as f64;
        random_in_range.floor() as usize
    }
}

impl Game {
    fn get_center_forward_in_the_zone(&self, user: &UserInfo) -> FieldPlayer {
        match user.team.active_five.field_players.get(&Center.to_string()) {
            Some(player) => player.clone(),
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
        generate_an_event(FaceOff, self);

        self.battle();
        reduce_strength(self);

        generate_an_event(Battle, self);
    }

    pub fn step(&mut self) -> GameState {
        let action_type = self.get_last_action();
        let action = Action;

        match action_type {
            StartGame => self.face_off(),
            Goal => self.face_off(),
            Save => self.face_off(),
            EndOfPeriod => self.face_off(),
            Rebound => {
                let player_pos = get_random_position_after_rebound();
                battle_by_position(player_pos, self);

                generate_an_event(Battle, self);
            },
             _ => action.do_random_action(self)
        };

        self.turns += 1;

        if self.user1.team.need_change() {
            self.user1.team.change_active_five();

            generate_an_event(FirstTeamChangeActiveFive, self);
        }
        if self.user2.team.need_change() {
            self.user2.team.change_active_five();

            generate_an_event(SecondTeamChangeActiveFive, self);
        }

        if [30, 60, 90].contains(&self.turns) {
            generate_an_event(EndOfPeriod, self);
            self.zone_number = 2;
        }

        let state = if self.is_game_over() {
            generate_an_event(GameFinished, self);
            GameState::GameOver { winner_id: self.get_winner_id() }
        } else {
            GameState::InProgress
        };

        if state == GameState::InProgress && self.turns == 90 {
            generate_an_event(Overtime, self);
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

    fn get_last_action(&self) -> &ActionTypes {
        if self.events.len() == 0 {
            &EndOfPeriod
        } else {
            &self.events[self.events.len() - 1].action
        }
    }

    pub fn get_user_info(&mut self, user_id: usize) -> &mut UserInfo {
        if user_id == 1 {
            &mut self.user1
        } else {
            &mut self.user2
        }
    }
}

fn get_random_position_after_rebound() -> PlayerPosition {
    let rnd = Game::get_random_in_range(0, 10);

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

fn battle_by_position(pos: PlayerPosition, game: &mut Game) {
    let player1 = &game.user1.team.active_five.field_players.get(&pos.to_string());
    let player2 = &game.user2.team.active_five.field_players.get(&pos.to_string());

    let player1_stat = match player1 {
        Some(player) => get_relative_field_player_stat(player, player.stats.strength),
        _ => panic!("Player not found")
    };

    let player2_stat = match player2 {
        Some(player) => get_relative_field_player_stat(player, player.stats.strength),
        _ => panic!("Player not found")
    };

    if has_won(player1_stat, player2_stat) {
        match *player1 {
            Some(player) => game.player_with_puck = Option::from(player.clone()),
            _ => panic!("Player not found")
        }
    } else {
        match *player2 {
            Some(player) => game.player_with_puck = Option::from(player.clone()),
            _ => panic!("Player not found")
        }
    }
}
// #[cfg(test)]
// mod tests {
//     use crate::{Game, TokenBalance};
//     use crate::PlayerPosition::Center;
//
//     fn get_new_game() -> Game {
//         Game::new("alice".into(),
//                   "bob".into(),
//                   TokenBalance{ token_id: Some("NEAR".into()), balance: 1 }
//         )
//     }
//
//     #[test]
//     fn step() {
//         let mut game = get_new_game();
//         let player1 = &game.user1.field_players.get(&Center);
//
//         let player1_stat = match player1 {
//             Some(player) => assert_eq!(player.stats.strength, 60.0, "not 60.0"),
//             _ => panic!("Player not found")
//         };
//         //game.step();
//     }
// }