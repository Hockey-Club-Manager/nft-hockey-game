use near_sdk::collections::{LookupMap};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{BorshStorageKey, near_bindgen, PanicOnDefault, setup_alloc};

use crate::game::Game;

setup_alloc!();


mod game;
mod user;
mod player;
mod goalie;
mod player_field;
mod action;

type GameId = u64;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Games,
    // AvailablePlayers,
    // Stats,
    // AvailableGames,
    // Affiliates {account_id: AccountId},
    // TotalRewards {account_id: AccountId},
    // TotalAffiliateRewards{ account_id: AccountId},
    // WhitelistedTokens
}

#[near_bindgen]
#[derive(PanicOnDefault)]
struct Hockey {
    games: LookupMap<GameId, Game>,
    // available_players: UnorderedMap<AccountId, VGameConfig>,
    // stats: UnorderedMap<AccountId, VStats>,
    // available_games: UnorderedMap<GameId, (AccountId, AccountId)>,
    // whitelisted_tokens: LookupSet<AccountId>,
    //
    // next_game_id: GameId,
    // service_fee: Balance,
}

#[near_bindgen]
impl Hockey {
    pub fn new() -> Self {
        Self {
            games: LookupMap::new(StorageKey::Games),
            // available_players: UnorderedMap::new(StorageKey::AvailablePlayers),
            // stats: UnorderedMap::new(StorageKey::Stats),
            // available_games: UnorderedMap::new(StorageKey::AvailableGames),
            // whitelisted_tokens: LookupSet::new(StorageKey::WhitelistedTokens),
            //
            // next_game_id: 0,
            // service_fee: 0,
        }
    }
}

#[cfg(test)]
mod tests {
}
