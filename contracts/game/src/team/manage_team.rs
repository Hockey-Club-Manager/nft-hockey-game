use crate::*;
use crate::team::five::{FiveNumber, IceTimePriority, Tactics};
use crate::team::team::swap_positions;

#[near_bindgen]
impl Hockey {
    pub fn take_to(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id {
            if !game.user1.take_to_called {
                self.change_stats_take_to(&mut game.user1, &mut game.user2);
                generate_an_event(TakeTO, &mut game);
            }
        } else if game.user2.account_id == account_id {
            if !game.user2.take_to_called {
                self.change_stats_take_to(&mut game.user2, &mut game.user1);
                generate_an_event(TakeTO, &mut game);
            }
        }
        self.games.insert(&game_id, &game);
    }

    fn change_stats_take_to(&self, user1: &mut UserInfo, user2: &mut UserInfo) {
        for (_player_pos, field_player) in user1.team.fives.get_mut(&user1.team.active_five).unwrap().field_players.iter_mut() {
            field_player.stats.morale += 5;
            field_player.stats.strength += 5;
            //field_player.stats.iq += 3;
        }

        for (_player_pos, field_player) in user2.team.fives.get_mut(&user2.team.active_five).unwrap().field_players.iter_mut() {
            field_player.stats.morale += 3;
            field_player.stats.strength += 3;
        }

        user1.take_to_called = true;
    }

    pub fn coach_speech(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id {
            if !game.user1.coach_speech_called {
                self.change_stats_coach_speech(&mut game.user1);
                generate_an_event(CoachSpeech, &mut game);
            }
        } else if game.user2.account_id == account_id {
            if !game.user2.coach_speech_called {
                self.change_stats_coach_speech(&mut game.user2);
                generate_an_event(CoachSpeech, &mut game);
            }
        }
        self.games.insert(&game_id, &game);
    }

    fn change_stats_coach_speech(&self, user: &mut UserInfo) {
        for (_player_pos, field_player) in user.team.fives.get_mut(&user.team.active_five).unwrap().field_players.iter_mut() {
            field_player.stats.morale += 3;
           // field_player.stats.iq += 2;
        }

        user.coach_speech_called = true;
    }

    pub fn goalie_out(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id && !game.user1.is_goalie_out {
            game.user1.is_goalie_out = true;
            generate_an_event(GoalieOut, &mut game);
        } else if game.user2.account_id == account_id && !game.user2.is_goalie_out {
            game.user2.is_goalie_out = true;
            generate_an_event(GoalieOut, &mut game);
        }
        self.games.insert(&game_id, &game);
    }

    pub fn goalie_back(&mut self, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id  && game.user1.is_goalie_out{
            game.user1.is_goalie_out = false;
            generate_an_event(GoalieBack, &mut game);
        } else if game.user2.account_id == account_id && game.user2.is_goalie_out{
            game.user2.is_goalie_out = false;
            generate_an_event(GoalieBack, &mut game);
        }
        self.games.insert(&game_id, &game);
    }

    pub fn change_tactic(&mut self, tactic: Tactics, game_id: GameId) {
        let account_id = env::predecessor_account_id();
        let mut game: Game = self.internal_get_game(&game_id).into();

        if game.user1.account_id == account_id {
            game.user1.tactic = tactic;
        } else if game.user2.account_id == account_id {
            game.user2.tactic = tactic;
        }

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
            swap_positions(&mut game.user1, number_five, position1, position2);
        } else if game.user2.account_id == account_id {
            swap_positions(&mut game.user2, number_five, position1, position2);
        }

        self.games.insert(&game_id, &game);
    }
}

impl Hockey {
    pub fn swap_positions(user_info: &mut UserInfo, number_five: FiveNumber, position1: PlayerPosition, position2: PlayerPosition) {
        let mut five = user_info.team.fives.get_mut(&number_five).unwrap().clone();
        let mut first_player = five.field_players.get(&position1).unwrap().clone();
        let mut second_player = five.field_players.get(&position2).unwrap().clone();

        first_player.position = second_player.position;
        second_player.position = first_player.position;

        first_player.set_position_coefficient();
        second_player.set_position_coefficient();

        five.field_players.insert(position1, second_player);
        five.field_players.insert(position2, first_player);

        user_info.team.fives.insert(number_five, five);
    }
}