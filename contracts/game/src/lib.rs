use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{CryptoHash, ext_contract, Gas, Promise, PromiseError};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, BorshStorageKey, env, serde_json, log, near_bindgen, PanicOnDefault};
use near_sdk::env::{predecessor_account_id};
use game::actions::action::ActionData::{CoachSpeech, GoalieBack, GoalieOut, TakeTO};

use crate::external::{ext_manage_team};
use crate::manager::{GameConfig, TokenBalance, UpdateStatsAction, VGameConfig, VStats};
use team::players::player::PlayerPosition;
use team::players::field_player::FieldPlayer;
use crate::game::actions::action::{ActionData, ActionTypes};
use crate::game::game::{Event, Game, GameState};
use crate::team::team_metadata::TeamMetadata;
use crate::user_info::{Account, hash_account_id, UserInfo};

mod game;
mod user_info;
mod manager;
mod team;
mod external;

const NFT_CONTRACT: &str = "hcm.parh.testnet";

type GameId = u64;
type SRC = String;
pub type TokenId = String;

// 1 second in nanoseconds
const SECOND: u64 = 1000000000;

// 1 NEAR
const MIN_DEPOSIT: Balance = 1_000_000_000_000_000_000_000_000;
const ONE_YOCTO: Balance = 1;
const NUMBER_OF_STEPS: u128 = 75;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Games,
    Teams,
    Deposit,
    AvailablePlayers {deposit: CryptoHash},
    Stats,
    AvailableGames,
    Affiliates {account_id: AccountId},
    TotalRewards {account_id: AccountId},
    TotalAffiliateRewards{ account_id: AccountId},
    WhitelistedTokens,
    FieldPlayers,
    Account,
    Friends { account_id: CryptoHash},
    SentFriendRequests { account_id: CryptoHash},
    SentFriendPlay{ account_id: CryptoHash},
    FriendRequestsReceived { account_id: CryptoHash},
    RequestsPlayReceived { account_id: CryptoHash},
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Hockey {
    games: LookupMap<GameId, Game>,
    teams: LookupMap<AccountId, TeamMetadata>,
    available_players: UnorderedMap<Balance, UnorderedMap<AccountId, VGameConfig>>,
    stats: UnorderedMap<AccountId, VStats>,
    available_games: UnorderedMap<GameId, (AccountId, AccountId)>,

    accounts: UnorderedMap<AccountId, Account>,

    next_game_id: GameId,
    service_fee: Balance,
}

#[near_bindgen]
impl Hockey {
    #[init]
    pub fn new() -> Self {
        Self {
            games: LookupMap::new(StorageKey::Games),
            teams: LookupMap::new(StorageKey::Teams),
            available_players: UnorderedMap::new(StorageKey::Deposit),
            stats: UnorderedMap::new(StorageKey::Stats),
            available_games: UnorderedMap::new(StorageKey::AvailableGames),

            accounts: UnorderedMap::new(StorageKey::Account),
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

    pub(crate) fn internal_add_referral(&mut self,
                                        account_id: &AccountId,
                                        referrer_id: &Option<AccountId>
    ) {
        if self.stats.get(account_id).is_none() && self.is_account_exists(referrer_id) {
            if let Some(referrer_id_unwrapped) = referrer_id.clone() {
                self.internal_update_stats(account_id,
                                           UpdateStatsAction::AddReferral,
                                           (*referrer_id).clone(),
                                           None);
                self.internal_update_stats(&referrer_id_unwrapped,
                                           UpdateStatsAction::AddAffiliate,
                                           Some(account_id.clone()),
                                           None);
                log!("Referrer {} added for {}", referrer_id_unwrapped, account_id);
            }
        }
    }

    #[payable]
    pub fn make_available(&mut self, config: GameConfig) -> Promise {
        let account_id: &AccountId = &predecessor_account_id();
        let deposit: Balance = env::attached_deposit();
        assert!(deposit >= MIN_DEPOSIT,
                "Deposit is too small. Attached: {}, Required: {}",
                deposit,
                MIN_DEPOSIT
        );

        ext_manage_team::ext(AccountId::new_unchecked("hcm.parh.testnet".parse().unwrap()))
            .with_static_gas(Gas(100_000_000_000_000))
            .get_owner_team(account_id.clone())
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(100_000_000_000_000))
                    .on_get_team(account_id.clone(), deposit, config)
            )
    }

    #[private]
    pub fn on_get_team(&mut self,
                   account_id: AccountId,
                   deposit: Balance,
                   config: GameConfig,
                   #[callback_result] call_result: Result<TeamMetadata, PromiseError>
    ) -> Option<Game> {
        if call_result.is_err() {
            Promise::new(account_id).transfer(deposit);
            log!("The team is incomplete");
            return None;
        }

        let team = call_result.unwrap();

        let mut available_players_by_deposit = self.available_players.get(&deposit).unwrap_or_else(|| {
            UnorderedMap::new(StorageKey::AvailablePlayers {deposit: hash_account_id(&serde_json::to_string(&deposit).expect(""))}.try_to_vec().unwrap())
        });

        return if available_players_by_deposit.len() == 0 {
            available_players_by_deposit.insert(&account_id, &VGameConfig::Current(GameConfig {
                deposit: Some(deposit),
                opponent_id: config.opponent_id
            }));

            self.internal_check_if_has_game_started(&account_id);
            self.available_players.insert(&deposit, &available_players_by_deposit);
            self.teams.insert(&account_id, &team);
            None
        } else {
            assert!(available_players_by_deposit.get(&account_id).is_none(), "Already in the waiting list the list");
            let available_players = self.get_available_players(0, 1, &available_players_by_deposit);
            self.internal_check_if_has_game_started(&account_id);

            let opponent_id = available_players.get(0).expect("Cannot find opponent id");
            self.teams.insert(&account_id, &team);

            Some(self.start_game(opponent_id.0.clone(), deposit, account_id))
        }
    }

    pub fn start_game(&mut self, opponent_id: AccountId, deposit: Balance, account_id: AccountId) -> Game {
        if let Some(opponent_config) = self.available_players.get(&deposit).expect("Deposit not found").get(&opponent_id) {
            let config: GameConfig = opponent_config.into();
            assert_eq!(deposit, config.deposit.unwrap_or(0), "Wrong deposit");

            assert_ne!(account_id.clone(), opponent_id.clone(), "Find a friend to play");

            self.internal_check_if_has_game_started(&account_id);

            if let Some(ref player_id) = config.opponent_id {
                assert_eq!(*player_id, account_id, "Wrong account");
            }

            let team = self.teams.remove(&account_id).expect("Team not found");
            let opponent_team = self.teams.remove(&opponent_id).expect("Team not found");

            self.init_game(opponent_id, account_id.clone(), config.clone(),  (team, opponent_team))
        } else {
            panic!("Your opponent is not ready");
        }
    }

    pub fn init_game(
        &mut self,
        opponent_id: AccountId,
        account_id: AccountId,
        config: GameConfig,
        teams: (TeamMetadata, TeamMetadata)
    ) -> Game {
        let reward = TokenBalance {
            token_id: Some("NEAR".into()),
            balance: config.deposit.unwrap_or(0) * 2,
        };

        let game_id = self.next_game_id;

        let game = Game::new(teams, account_id.clone(),
                             opponent_id.clone(),
                             reward, &game_id);

        self.games.insert(&game_id, &game);

        self.available_games.insert(&game_id, &(account_id.clone(), opponent_id.clone()));

        self.next_game_id += 1;

        let mut available_players_by_deposit = self.available_players.get(&config.deposit.expect("Incorrect game config")).expect("Deposit not found");
        available_players_by_deposit.remove(&opponent_id);
        available_players_by_deposit.remove(&account_id);

        self.available_players.insert(&config.deposit.unwrap(), &available_players_by_deposit);

        self.internal_update_stats(&account_id, UpdateStatsAction::AddPlayedGame, None, None);
        self.internal_update_stats(&opponent_id, UpdateStatsAction::AddPlayedGame, None, None);

        game
    }

    pub fn generate_event(&mut self, game_id: GameId) -> Event {
        let game: &mut Game = &mut self.internal_get_game(&game_id);

        assert!(game.winner_index.is_none(), "Game already finished");

        let time = env::block_timestamp();
        let d_time = time - game.last_event_generation_time;

        if  d_time == 0
            && game.number_of_generated_events_in_current_block == game.max_number_of_generated_events_in_block
            || d_time < game.event_generation_delay {
            panic!("Events are generated too often")
        }

        if d_time == 0 {
            game.number_of_generated_events_in_current_block += 1;
            if game.number_of_generated_events_in_current_block == game.max_number_of_generated_events_in_block {
                game.event_generation_delay = SECOND;
            }
        } else {
            game.event_generation_delay = 0;
            game.number_of_generated_events_in_current_block = 1;
        }

        game.last_event_generation_time = time;

        let mut generated_actions = game.step();

        let game_state = game.get_game_state();
        if game_state.1.is_some(){
            generated_actions.push(game_state.1.unwrap());
        }

        match game_state.0 {
            GameState::GameOver { winner_id: winner_index} => {
                let winner_account = if game.user1.user_id == winner_index {
                    game.user1.account_id.clone()
                } else {
                    game.user2.account_id.clone()
                };

                let reward = self.internal_distribute_reward(
                    &game.reward, &winner_account, game_id);

                let winner_account = game.get_user_info(winner_index);
                generated_actions.push(ActionData::GameFinished {
                    action_type: ActionTypes::GameFinished,
                    winner_account_id: winner_account.account_id.clone(),
                    reward
                });

                game.winner_index = Some(winner_index);

                self.internal_stop_game(game_id);
            },
            _ => {}
        };

        let generated_event = game.generate_event(&generated_actions);

        self.games.insert(&game_id, &game);

        generated_event
    }

    // TODO make private on release
    pub fn internal_stop_game(&mut self, game_id: GameId) {
        self.available_games.remove(&game_id);
        log!{"{}", game_id};
    }

    pub fn get_next_game_id(&self) -> GameId {
        self.next_game_id
    }
}


#[cfg(test)]
mod tests {}