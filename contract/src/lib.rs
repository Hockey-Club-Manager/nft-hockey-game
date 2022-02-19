use std::collections::HashMap;
//use std::env;
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{AccountId, Balance, BorshStorageKey, env, log, near_bindgen, init, PanicOnDefault, setup_alloc, Timestamp};

use crate::game::{Event, Game, GameState, Team, UserInfo};
use crate::manager::{GameConfig, TokenBalance, UpdateStatsAction, VGameConfig, VStats};
use crate::player::{PlayerPosition, PlayerRole};
use crate::player_field::FieldPlayer;

mod game;
mod user;
mod player;
mod goalie;
mod player_field;
mod action;
mod manager;

type GameId = u64;

// 0.01 NEAR
const MIN_DEPOSIT: Balance = 10_000_000_000_000_000_000_000;
const ONE_YOCTO: Balance = 1;

setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Games,
    AvailablePlayers,
    Stats,
    AvailableGames,
    Affiliates {account_id: AccountId},
    TotalRewards {account_id: AccountId},
    TotalAffiliateRewards{ account_id: AccountId},
    WhitelistedTokens,
    FieldPlayers,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Hockey {
    games: LookupMap<GameId, Game>,
    available_players: UnorderedMap<AccountId, VGameConfig>,
    stats: UnorderedMap<AccountId, VStats>,
    available_games: UnorderedMap<GameId, (AccountId, AccountId)>,
    whitelisted_tokens: LookupSet<AccountId>,

    next_game_id: GameId,
    service_fee: Balance,
}

#[near_bindgen]
impl Hockey {
    #[init]
    pub fn new() -> Self {
        Self {
            games: LookupMap::new(StorageKey::Games),
            available_players: UnorderedMap::new(StorageKey::AvailablePlayers),
            stats: UnorderedMap::new(StorageKey::Stats),
            available_games: UnorderedMap::new(StorageKey::AvailableGames),
            whitelisted_tokens: LookupSet::new(StorageKey::WhitelistedTokens),

            next_game_id: 0,
            service_fee: 0,
        }
    }
}

#[near_bindgen]
impl Hockey {
    pub(crate) fn internal_check_if_has_game_started(&self, account_id: &AccountId) {
        let games_already_started: Vec<(AccountId, AccountId)> = self.available_games.values_as_vector()
            .iter()
            .filter(|(player_1, player_2)| *player_1 == *account_id || *player_2 == *account_id)
            .collect();
        assert_eq!(games_already_started.len(), 0, "Another game already started");
    }

    pub(crate) fn internal_add_referral(&mut self, account_id: &AccountId, referrer_id: &Option<AccountId>) {
        if self.stats.get(account_id).is_none() && self.is_account_exists(referrer_id) {
            if let Some(referrer_id_unwrapped) = referrer_id.clone() {
                self.internal_update_stats(account_id, UpdateStatsAction::AddReferral, referrer_id.clone(), None);
                self.internal_update_stats(&referrer_id_unwrapped, UpdateStatsAction::AddAffiliate, Some(account_id.clone()), None);
                log!("Referrer {} added for {}", referrer_id_unwrapped, account_id);
            }
        }
    }

    #[payable]
    pub fn make_available(&mut self, config: GameConfig, referrer_id: Option<AccountId>) {
        let account_id: &AccountId = &env::predecessor_account_id();
        assert!(self.available_players.get(account_id).is_none(), "Already in the waiting list the list");
        let deposit: Balance = env::attached_deposit();
        assert!(deposit >= MIN_DEPOSIT, "Deposit is too small. Attached: {}, Required: {}", deposit, MIN_DEPOSIT);

        self.internal_check_if_has_game_started(account_id);

        self.internal_add_referral(account_id, &referrer_id);

        self.available_players.insert(account_id,
                                      &VGameConfig::Current(GameConfig {
                                          deposit: Some(deposit),
                                          opponent_id: config.opponent_id,
                                      }));
    }

    #[payable]
    pub fn start_game(&mut self, opponent_id: AccountId, referrer_id: Option<AccountId>) -> GameId {
        if let Some(opponent_config) = self.available_players.get(&opponent_id) {
            let config: GameConfig = opponent_config.into();
            assert_eq!(env::attached_deposit(), config.deposit.unwrap_or(0), "Wrong deposit");

            let account_id = env::predecessor_account_id();
            assert_ne!(account_id.clone(), opponent_id.clone(), "Find a friend to play");

            self.internal_check_if_has_game_started(&account_id);

            if let Some(player_id) = config.opponent_id {
                assert_eq!(player_id, account_id, "Wrong account");
            }

            let game_id = self.next_game_id;

            // TODO Add FT
            let reward = TokenBalance {
                token_id: Some("NEAR".into()),
                balance: config.deposit.unwrap_or(0) * 2,
            };

            let game = Game::new(account_id.clone(),
                                       opponent_id.clone(),
                                                   reward);

            self.games.insert(&game_id, &game);

            self.available_games.insert(&game_id, &(account_id.clone(), opponent_id.clone()));

            self.next_game_id += 1;

            self.available_players.remove(&opponent_id);
            self.available_players.remove(&account_id);

            self.internal_add_referral(&account_id, &referrer_id);

            self.internal_update_stats(&account_id, UpdateStatsAction::AddPlayedGame, None, None);
            self.internal_update_stats(&opponent_id, UpdateStatsAction::AddPlayedGame, None, None);

            game_id
        } else {
            panic!("Your opponent is not ready");
        }
    }

    pub fn generate_event(&mut self, game_id: GameId, number_of_rendered_events: usize) -> Vec<Event> {
        let mut game: Game = self.internal_get_game(&game_id).into();
        assert!(game.winner_index.is_none(), "Game already finished");

        let time = env::block_timestamp();
        if time - game.last_event_generation_time >= 1 {
            game.last_event_generation_time = time;

            match game.step() {
                GameState::GameOver { winner_id: winner_index} => {
                    let winner_account = if game.user1.user.id == winner_index {
                        game.user1.account_id.clone()
                    } else {
                        game.user2.account_id.clone()
                    };

                    self.internal_distribute_reward(&game.reward, &winner_account);
                    game.winner_index = Some(winner_index);

                    self.internal_stop_game(game_id);

                    log!("\nGame over! {} won!", winner_account);
                },
                _ => {}
            };

            self.games.insert(&game_id, &game);
        }

        let teams = if game.user1.account_id == env::predecessor_account_id() {
            (Team {
                field_players: game.user1.field_players.clone(),
                goalie: game.user1.goalie.clone()
            },
             Team{
                 field_players: game.user2.field_players.clone(),
                 goalie: game.user2.goalie.clone()
             })
        } else {
            (Team {
                field_players: game.user2.field_players.clone(),
                goalie: game.user2.goalie.clone()
            },
             Team{
                 field_players: game.user1.field_players.clone(),
                 goalie: game.user1.goalie.clone()
             })
        };


        self.get_events(game.events.len() - number_of_rendered_events, teams.0, teams.1, &game)
    }

    fn get_events(&self, number_of_events: usize, my_team: Team, opponent_team: Team, game: &Game) -> Vec<Event> {
        let mut result: Vec<Event> = vec![];
        for i in 0..number_of_events {
            let event = game.events[game.events.len() - 1 - i];

            result.push(Event {
                my_team: my_team.clone(),
                opponent_team: opponent_team.clone(),
                time: event.time,
                zone_number: event.zone_number,
                action: event.action,
                player_with_puck: event.player_with_puck,
            })
        }

        result
    }

    // fn get_field_player_vec(&self, user: &UserInfo) -> Vec<FieldPlayer> {
    //     let mut field_players = Vec::new();
    //
    //     for (player_pos, field_player) in user.field_players.iter() {
    //         field_players.insert(field_players.len() - 1,field_player);
    //     }
    //
    //     field_players
    // }

    pub fn internal_stop_game(&mut self, game_id: GameId) {
        self.available_games.remove(&game_id);
    }

    pub fn get_next_game_id(&self) -> GameId {
        self.next_game_id
    }
}

#[cfg(test)]
mod tests {
}
