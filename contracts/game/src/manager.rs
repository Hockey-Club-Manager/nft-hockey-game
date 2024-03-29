use near_sdk::{AccountId, Balance, log, Promise, PromiseOrValue};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use crate::{Game, GameId, Hockey, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use crate::*;


#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenBalance {
    pub(crate) token_id: Option<String>,
    pub(crate) balance: Balance,
}

#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameConfig {
    pub(crate) deposit: Option<Balance>,
    pub(crate) opponent_id: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VGameConfig {
    Current(GameConfig)
}

impl From<VGameConfig> for GameConfig {
    fn from(v_game_config: VGameConfig) -> Self {
        match v_game_config {
            VGameConfig::Current(game_config) => game_config,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameConfigOutput {
    deposit: U128,
    opponent_id: Option<AccountId>,
}

impl From<GameConfig> for GameConfigOutput {
    fn from(config: GameConfig) -> Self {
        GameConfigOutput {
            deposit: U128::from(config.deposit.unwrap_or(0)),
            opponent_id: config.opponent_id,
        }
    }
}

#[derive(PartialEq)]
pub enum UpdateStatsAction {
    AddPlayedGame,
    AddReferral,
    AddAffiliate,
    AddWonGame,
    AddTotalReward,
    AddAffiliateReward,
    AddPenaltyGame,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Stats {
    referrer_id: Option<AccountId>,
    affiliates: UnorderedSet<AccountId>,
    games_num: u64,
    victories_num: u64,
    penalties_num: u64,
    total_reward: UnorderedMap<Option<AccountId>, Balance>,
    total_affiliate_reward: UnorderedMap<Option<AccountId>, Balance>,
}

impl Stats {
    pub fn new(account_id: &AccountId) -> Stats {
        Stats {
            referrer_id: None,
            affiliates: UnorderedSet::new(StorageKey::Affiliates { account_id: account_id.clone() }),
            games_num: 0,
            victories_num: 0,
            penalties_num: 0,
            total_reward: UnorderedMap::new(StorageKey::TotalRewards { account_id: account_id.clone() }),
            total_affiliate_reward: UnorderedMap::new(StorageKey::TotalAffiliateRewards { account_id: account_id.clone() }),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VStats {
    Current(Stats),
}

impl From<VStats> for Stats {
    fn from(v_stats: VStats) -> Self {
        match v_stats {
            VStats::Current(stats) => stats,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StatsOutput {
    referrer_id: Option<AccountId>,
    affiliates: Vec<AccountId>,
    games_num: u64,
    victories_num: u64,
    penalties_num: u64,
    total_reward: U128,
    total_affiliate_reward: U128
}

impl From<Stats> for StatsOutput {
    fn from(stats: Stats) -> Self {
        StatsOutput {
            referrer_id: stats.referrer_id,
            affiliates: stats.affiliates.to_vec(),
            games_num: stats.games_num,
            victories_num: stats.victories_num,
            penalties_num: stats.penalties_num,
            // TODO Add FT
            total_reward: U128::from(stats.total_reward.get(&None).unwrap_or(0)),
            total_affiliate_reward: U128::from(stats.total_affiliate_reward.get(&None).unwrap_or(0)),
        }
    }
}

impl Hockey {
    pub(crate) fn internal_distribute_reward(
        &mut self,
        token_balance: &TokenBalance,
        winner_id: &AccountId,
        game_id: GameId
    ) -> Balance {
        // TODO add for FT
        let amount = token_balance.balance;
        let fee = amount / 10;
        let winner_reward: Balance = amount - fee;
        Promise::new(winner_id.clone()).transfer(winner_reward);

        let reward = match serde_json::to_string(&(game_id, (winner_id.clone(), winner_reward.clone()))) {
            Ok(res) => res,
            Err(e) => panic!("{}", e)
        };
        log!("{}", reward);

        let stats = self.internal_get_stats(winner_id);
        let referrer_fee = if let Some(referrer_id) = stats.referrer_id {
            let referrer_fee = fee / 2;
            log!("Affiliate reward for {} is {}", referrer_id, referrer_fee);
            self.internal_update_stats(&referrer_id, UpdateStatsAction::AddAffiliateReward, None, Some(referrer_fee));
            Promise::new(referrer_id.clone()).transfer(referrer_fee);
            referrer_fee
        } else {
            0
        };

        self.service_fee += fee - referrer_fee;

        self.internal_update_stats(winner_id, UpdateStatsAction::AddWonGame, None   , None);
        self.internal_update_stats(winner_id, UpdateStatsAction::AddTotalReward, None, Some(winner_reward));

        // finish
        // TODO add to stats
        winner_reward
    }

    pub(crate) fn internal_update_stats(&mut self,
                                        account_id: &AccountId,
                                        action: UpdateStatsAction,
                                        additional_account_id: Option<AccountId>,
                                        balance: Option<Balance>) {
        let mut stats = self.internal_get_stats(account_id);

        if action == UpdateStatsAction::AddPlayedGame {
            stats.games_num += 1
        } else if action == UpdateStatsAction::AddReferral {
            if additional_account_id.is_some() {
                stats.referrer_id = additional_account_id;
            }
        } else if action == UpdateStatsAction::AddAffiliate {
            if let Some(additional_account_id_unwrapped) = additional_account_id {
                stats.affiliates.insert(&additional_account_id_unwrapped);
            }
        } else if action == UpdateStatsAction::AddWonGame {
            stats.victories_num += 1;
        } else if action == UpdateStatsAction::AddTotalReward {
            if let Some(balance_unwrapped) = balance {
                // TODO Add FT
                let total_reward = stats.total_reward.get(&None).unwrap_or(0);
                stats.total_reward.insert(&None, &(total_reward + balance_unwrapped));
            }
        } else if action == UpdateStatsAction::AddAffiliateReward {
            if let Some(balance_unwrapped) = balance {
                // TODO Add FT
                let total_affiliate_reward = stats.total_affiliate_reward.get(&None).unwrap_or(0);
                stats.total_affiliate_reward.insert(&None, &(total_affiliate_reward + balance_unwrapped));
            }
        } else if action == UpdateStatsAction::AddPenaltyGame {
            stats.penalties_num += 1;
        }

        self.stats.insert(account_id, &VStats::Current(stats));
    }

    pub(crate) fn internal_get_stats(&self, account_id: &AccountId) -> Stats {
        if let Some(stats) = self.stats.get(account_id) {
            stats.into()
        } else {
            Stats::new(&account_id)
        }
    }

    pub(crate) fn internal_get_game(&self, game_id: &GameId) -> Game {
        self.games.get(game_id).expect("Game not found")
    }

    pub(crate) fn is_account_exists(&self, account_id: &Option<AccountId>) -> bool {
        if let Some(account_id_unwrapped) = account_id {
            self.stats.get(account_id_unwrapped).is_some()
        } else {
            false
        }
    }
}

#[near_bindgen]
impl Hockey {
    pub fn make_unavailable(&mut self, bid: String) -> PromiseOrValue<bool> {
        let account_id = predecessor_account_id();
        let deposit: Balance = serde_json::from_str(&bid).expect("Wrong deposit");

        let mut available_players_by_deposit = self.available_players.get(&deposit).expect("Deposit not found");
        if let Some(v_game_config) = available_players_by_deposit.get(&account_id) {
            let config: GameConfig = v_game_config.into();
            available_players_by_deposit.remove(&account_id);
            self.available_players.insert(&deposit, &available_players_by_deposit);
            PromiseOrValue::Promise(Promise::new(account_id).transfer(config.deposit.unwrap_or(0)))
        } else {
            PromiseOrValue::Value(false)
        }
    }

    pub(crate) fn get_available_players(&self, from_index: u64, limit: u64, available_players: &UnorderedMap<AccountId, VGameConfig>) -> Vec<(AccountId, GameConfigOutput)> {
        let keys = available_players.keys_as_vector();
        let values = available_players.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| {
                let config: GameConfig = values.get(index).unwrap().into();
                (keys.get(index).unwrap(), config.into())
            })
            .collect()
    }

    pub fn get_available_games(&self, from_index: u64, limit: u64) -> Vec<(GameId, (AccountId, AccountId))> {
        let keys = self.available_games.keys_as_vector();
        let values = self.available_games.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| {
                let accounts: (AccountId, AccountId) = values.get(index).unwrap().into();
                (keys.get(index).unwrap(), accounts)
            })
            .collect()
    }

    pub fn get_stats(&self, account_id: AccountId) -> StatsOutput {
        self.internal_get_stats(&account_id).into()
    }

    pub fn get_service_fee(&self) -> U128 {
        U128::from(self.service_fee)
    }

    pub fn is_already_in_the_waiting_list(&self, account_id: AccountId, deposit: Balance) -> bool {
        let available_players_by_deposit = self.available_players.get(&deposit).expect("Deposit not found");
        !available_players_by_deposit.get(&account_id).is_none()
    }

    pub fn get_game_config(&self, account_id: AccountId, deposit: Balance) -> GameConfig {
        let available_players_by_deposit = self.available_players.get(&deposit).expect("Deposit not found");
        available_players_by_deposit.get(&account_id).unwrap().into()
    }
}