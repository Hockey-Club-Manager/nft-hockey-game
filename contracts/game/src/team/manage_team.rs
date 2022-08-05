use crate::*;
use crate::team::five::{IceTimePriority, Tactics};
use crate::team::numbers::*;

#[near_bindgen]
impl Hockey {
    pub fn take_to(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id {
            if !game.user1.take_to_called {
                self.change_stats_take_to(&mut game.user1, &mut game.user2);
            } else {
                panic!("You have already used TO")
            }
        } else if game.user2.account_id == account_id {
            if !game.user2.take_to_called {
                self.change_stats_take_to(&mut game.user2, &mut game.user1);
            } else {
                panic!("You have already used TO")
            }
        } else {
            panic!("Account id not found!")
        }

        game.generate_an_event(TakeTO);

        self.games.insert(&game_id, &game);
    }


    pub fn coach_speech(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id {
            if !game.user1.coach_speech_called {
                self.change_stats_coach_speech(&mut game.user1);
                game.generate_an_event(CoachSpeech);
            } else {
                panic!("You have already used Coach speech")
            }
        } else if game.user2.account_id == account_id {
            if !game.user2.coach_speech_called {
                self.change_stats_coach_speech(&mut game.user2);
                game.generate_an_event(CoachSpeech);
            } else {
                panic!("You have already used Coach speech")
            }
        }
        self.games.insert(&game_id, &game);
    }


    pub fn goalie_out(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id && !game.user1.is_goalie_out {
            game.user1.is_goalie_out = true;
            game.generate_an_event(GoalieOut);
        } else if game.user2.account_id == account_id && !game.user2.is_goalie_out {
            game.user2.is_goalie_out = true;
            game.generate_an_event(GoalieOut);
        }
        self.games.insert(&game_id, &game);
    }

    pub fn goalie_back(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id  && game.user1.is_goalie_out{
            game.user1.is_goalie_out = false;
            game.generate_an_event(GoalieBack);
        } else if game.user2.account_id == account_id && game.user2.is_goalie_out{
            game.user2.is_goalie_out = false;
            game.generate_an_event(GoalieBack);
        }
        self.games.insert(&game_id, &game);
    }

    pub fn change_tactic(&mut self, five_number: FiveNumber, tactic: Tactics, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        let user = game.get_user_info_by_acc_id(&account_id);
        let five = user.team.fives.get_mut(&five_number).unwrap();
        five.tactic = tactic;

        self.games.insert(&game_id, &game);
    }

    pub fn change_ice_priority(&mut self, ice_time_priority: IceTimePriority, five: FiveNumber, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id);

        if game.user1.account_id == account_id {
            game.user1.team.fives.get_mut(&five).unwrap().ice_time_priority = ice_time_priority;
        } else if game.user2.account_id == account_id {
            game.user2.team.fives.get_mut(&five).unwrap().ice_time_priority = ice_time_priority;
        }

        self.games.insert(&game_id, &game);
    }

    pub fn change_positions(&mut self, number_five: FiveNumber, game_id: GameId, position1: PlayerPosition, position2: PlayerPosition) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id);

        if game.user1.account_id == account_id {
            self.swap_positions(&mut game.user1, number_five, position1, position2);
        } else if game.user2.account_id == account_id {
            self.swap_positions(&mut game.user2, number_five, position1, position2);
        }

        self.games.insert(&game_id, &game);
    }
}

impl Hockey {
    fn change_stats_take_to(&self, user1: &mut UserInfo, user2: &mut UserInfo) {
        for (_five_number, five_ids) in user1.team.fives.clone() {
            for (_player_pos, field_player) in five_ids.field_players {
                let field_player = user1.team.get_field_player_mut(&field_player);
                field_player.stats.increase_strength(5);
                field_player.stats.increase_iq(3)
            }
        }

        for (_goalie_number, goalie) in user1.team.goalies.iter_mut() {
            goalie.stats.increase_strength(5);
        }

        for (_five_number, five_ids) in user2.team.fives.clone() {
            for (_player_pos, field_player) in five_ids.field_players {
                let field_player = user2.team.get_field_player_mut(&field_player);
                field_player.stats.increase_strength(3);
                field_player.stats.morale += 3;
            }
        }

        for (_goalie_number, goalie) in user1.team.goalies.iter_mut() {
            goalie.stats.increase_strength(3);
        }

        user1.take_to_called = true;
    }

    fn change_stats_coach_speech(&self, user: &mut UserInfo) {
        for (_five_number, five_ids) in user.team.fives.clone() {
            for (_player_pos, field_player) in five_ids.field_players {
                let field_player = user.team.get_field_player_mut(&field_player);
                field_player.stats.increase_strength(5);
            }
        }

        for (_goalie_number, goalie) in user.team.goalies.iter_mut() {
            goalie.stats.increase_strength(5);
        }

        user.coach_speech_called = true;
    }

    fn swap_positions(&mut self, user_info: &mut UserInfo, number_five: FiveNumber, position1: PlayerPosition, position2: PlayerPosition) {
        let five = user_info.team.fives.get_mut(&number_five).unwrap();
        let first_player_id = five.field_players.get(&position1).unwrap().clone();
        let second_player_id = five.field_players.get(&position2).unwrap().clone();

        five.field_players.insert(position1, second_player_id);
        five.field_players.insert(position2, first_player_id);

        // TODO calculate teamwork
    }
}