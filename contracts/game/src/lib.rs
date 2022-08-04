use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
use near_sdk::{ext_contract, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, BorshStorageKey, env, serde_json, log, near_bindgen, PanicOnDefault, setup_alloc};
use game::actions::action::ActionTypes::{CoachSpeech, GoalieBack, GoalieOut, TakeTO};
use crate::external::ext_manage_team;
use crate::external::ext_self;

use crate::manager::{GameConfig, TokenBalance, UpdateStatsAction, VGameConfig, VStats};
use team::players::player::PlayerPosition;
use team::players::field_player::FieldPlayer;
use crate::game::game::{Event, Game, GameState};
use crate::team::team_metadata::TeamMetadata;
use crate::user_info::UserInfo;

mod game;
mod user_info;
mod manager;
mod team;
mod external;

const NFT_CONTRACT: &str = "nft_0_0.testnet";

type GameId = u64;
type SRC = String;
pub type TokenId = String;


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
                self.internal_update_stats(account_id, UpdateStatsAction::AddReferral, (*referrer_id).clone(), None);
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
    pub fn start_game(&mut self, opponent_id: AccountId, referrer_id: Option<AccountId>) -> Promise {
        if let Some(opponent_config) = self.available_players.get(&opponent_id) {
            let config: GameConfig = opponent_config.into();
            assert_eq!(env::attached_deposit(), config.deposit.unwrap_or(0), "Wrong deposit");

            let account_id = env::predecessor_account_id();
            assert_ne!(account_id.clone(), opponent_id.clone(), "Find a friend to play");

            self.internal_check_if_has_game_started(&account_id);

            if let Some(ref player_id) = config.opponent_id {
                assert_eq!(*player_id, account_id, "Wrong account");
            }

            self.internal_add_referral(&account_id, &referrer_id);

            ext_manage_team::get_teams(account_id.clone(), opponent_id.clone(), &NFT_CONTRACT, 0, 100_000_000_000_000)
                .then(ext_self::on_get_teams(opponent_id, account_id, config.clone(), &env::current_account_id(), 0, 100_000_000_000_000))
        } else {
            panic!("Your opponent is not ready");
        }
    }

    #[private]
    pub fn on_get_teams(
        &mut self,
        opponent_id: AccountId,
        account_id: AccountId,
        config: GameConfig,
        #[callback] teams: (TeamMetadata, TeamMetadata)
    ) -> GameId {
        // TODO Add FT
        let reward = TokenBalance {
            token_id: Some("NEAR".into()),
            balance: config.deposit.unwrap_or(0) * 2,
        };


        let game = Game::new(teams, account_id.clone(),
                             opponent_id.clone(),
                             reward);

        let game_id = self.next_game_id;

        self.games.insert(&game_id, &game);

        self.available_games.insert(&game_id, &(account_id.clone(), opponent_id.clone()));

        let available_game = match serde_json::to_string(&(game_id.clone(), (account_id.clone(), opponent_id.clone()))) {
            Ok(res) => res,
            Err(err) => panic!("{}", err)
        };
        log!("{}", available_game);

        self.next_game_id += 1;

        self.available_players.remove(&opponent_id);
        self.available_players.remove(&account_id);

        self.internal_update_stats(&account_id, UpdateStatsAction::AddPlayedGame, None, None);
        self.internal_update_stats(&opponent_id, UpdateStatsAction::AddPlayedGame, None, None);

        game_id
    }

    pub fn generate_event(&mut self, game_id: GameId) {
        let game: &mut Game = &mut self.internal_get_game(&game_id);

        assert!(game.winner_index.is_none(), "Game already finished");

        let time = env::block_timestamp();
        if time - game.last_event_generation_time < 1 {
            return;
        }

        game.last_event_generation_time = time;

        match game.step() {
            GameState::GameOver { winner_id: winner_index} => {
                let winner_account = if game.user1.user_id == winner_index {
                    game.user1.account_id.clone()
                } else {
                    game.user2.account_id.clone()
                };

                self.internal_distribute_reward(&game.reward, &winner_account, game_id);
                game.winner_index = Some(winner_index);

                self.internal_stop_game(game_id);
            },
            _ => {}
        };

        self.games.insert(&game_id, &game);
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
mod tests {

}
