use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, CryptoHash, env, Promise};
use crate::external::{ext_manage_team};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::env::{attached_deposit, predecessor_account_id};
use crate::{Hockey, StorageKey};
use crate::team::team::Team;

pub type UserId = usize;
pub const USER_ID1: usize = 1;
pub const USER_ID2: usize = 2;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserInfo {
    pub(crate) user_id: UserId, // 1 or 2
    pub(crate) team: Team,
    pub(crate) account_id: AccountId,
    pub(crate) take_to_called: bool,
    pub(crate) coach_speech_called: bool,
    pub(crate) is_goalie_out: bool,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub(crate) user_id: AccountId,
    pub(crate) friends: UnorderedSet<AccountId>,
    pub(crate) sent_friend_requests: UnorderedSet<AccountId>,
    pub(crate) friend_requests_received: UnorderedSet<AccountId>,
    pub(crate) sent_requests_play: UnorderedMap<AccountId, Balance>,
    pub(crate) requests_play_received: UnorderedMap<AccountId, Balance>,
}

#[near_bindgen]
impl Hockey {
    pub fn register_account(&mut self) {
        let account_id = predecessor_account_id();

        if self.accounts.get(&account_id).is_some() {
            panic!("Account already registered");
        }

        let account = Account {
            user_id: account_id.clone(),
            friends: UnorderedSet::new(
                StorageKey::Friends {
                    account_id: hash_account_id(account_id.as_str())
                }
                    .try_to_vec().unwrap()),
            sent_friend_requests: UnorderedSet::new(
                StorageKey::SentFriendRequests {
                    account_id: hash_account_id(account_id.as_str())
                }
                    .try_to_vec().unwrap()),
            friend_requests_received: UnorderedSet::new(
                StorageKey::FriendRequestsReceived {
                    account_id: hash_account_id(account_id.as_str())
                }
                    .try_to_vec().unwrap()),
            sent_requests_play: UnorderedMap::new(
                StorageKey::SentFriendRequests {
                    account_id: hash_account_id(account_id.as_str())
                }
                    .try_to_vec().unwrap()),
            requests_play_received: UnorderedMap::new(
                StorageKey::RequestsPlayReceived {
                    account_id: hash_account_id(account_id.as_str())
                }
                    .try_to_vec().unwrap()),
        };

        self.accounts.insert(&account_id, &account);
    }

    pub fn set_team_logo(
        &mut self,
        logo_json: String
    ) { }

    pub fn remove_friend(&mut self, friend_id: AccountId) {
        let account_id = predecessor_account_id();
        
        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        assert_ne!(account_id.clone(), friend_id.clone(), "Wrong friend id");

        account.friends.remove(&friend_id);
        friend.friends.remove(&account_id);

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);
    }

    pub fn send_friend_request(&mut self, friend_id: AccountId) {
        let account_id = predecessor_account_id();

        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        assert_ne!(account_id.clone(), friend_id.clone(), "Wrong friend id");

        account.sent_friend_requests.insert(&friend_id);
        friend.friend_requests_received.insert(&account_id);

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);
    }

    pub fn accept_friend_request(&mut self, friend_id: AccountId) {
        let account_id = predecessor_account_id();

        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        if !account.friend_requests_received.remove(&friend_id) {
            panic!("Friend id not found")
        }

        if !friend.sent_friend_requests.remove(&account_id) {
            panic!("Account id not found")
        }

        account.friends.insert(&friend_id);
        friend.friends.insert(&account_id);

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);
    }

    pub fn decline_friend_request(&mut self, friend_id: AccountId) {
        let account_id = predecessor_account_id();

        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        account.friend_requests_received.remove(&friend_id);
        account.sent_friend_requests.remove(&friend_id);
        friend.sent_friend_requests.remove(&account_id);
        friend.friend_requests_received.remove(&account_id);

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);
    }

    #[payable]
    pub fn send_request_play(&mut self, friend_id: AccountId) {
        let account_id = predecessor_account_id();
        let deposit = attached_deposit();

        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        assert_ne!(account_id.clone(), friend_id.clone(), "Wrong friend id");

        account.sent_requests_play.insert(&friend_id, &deposit);
        friend.requests_play_received.insert(&account_id, &deposit);

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);
    }

    pub fn accept_request_play(&mut self, friend_id: AccountId) -> Promise {
        let account_id = predecessor_account_id();
        let deposit = attached_deposit();

        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        let friend_deposit = friend.sent_requests_play.get(&account_id).expect("Request to play not found");

        assert_eq!(deposit, friend_deposit, "Wrong deposit");

        if account.requests_play_received.remove(&friend_id).is_none() {
            panic!("Friend id not found");
        }

        if !friend.sent_requests_play.remove(&account_id).is_none() {
            panic!("Account id not found");
        }

        self.internal_check_if_has_game_started(&account_id);
        self.internal_check_if_has_game_started(&friend_id);

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);

        let config = GameConfig {
            deposit: Some(deposit),
            opponent_id: Some(friend_id.clone())
        };

        ext_manage_team::ext(AccountId::new_unchecked("hcm.parh.testnet".parse().unwrap()))
            .with_static_gas(Gas(100_000_000_000_000))
            .get_teams(account_id.clone(), friend_id.clone())
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(100_000_000_000_000))
                    .on_get_teams(friend_id.clone(), account_id, config.clone())
            )
    }

    #[private]
    pub fn on_get_teams(&mut self,
                    opponent_id: AccountId,
                    account_id: AccountId,
                    config: GameConfig,
                    #[callback_result] call_result: Result<(TeamMetadata, TeamMetadata), PromiseError>
    ) -> Option<Game> {
        if call_result.is_err() {
            log!("The team is incomplete");
            Promise::new(account_id).transfer(config.deposit.unwrap());
            Promise::new(config.opponent_id.unwrap()).transfer(config.deposit.unwrap());
            return None;
        }
        let teams = call_result.unwrap();
        Some(self.init_game(opponent_id, account_id, config, teams))
    }

    pub fn decline_request_play(&mut self, friend_id: AccountId) {
        let account_id = predecessor_account_id();

        let mut account = self.accounts.get(&account_id).expect("You are not registered");
        let mut friend = self.accounts.get(&friend_id).expect(&format!("Account not found {}", friend_id.clone()));

        account.requests_play_received.remove(&friend_id);
        account.sent_requests_play.remove(&friend_id);

        if let Some(deposit) = friend.requests_play_received.remove(&account_id) {
            account.sent_requests_play.remove(&friend_id);
            Promise::new(account_id.clone()).transfer(deposit);
        }

        if let Some(deposit) = account.requests_play_received.remove(&account_id) {
            friend.sent_requests_play.remove(&account_id);
            Promise::new(friend_id.clone()).transfer(deposit);
        }

        self.accounts.insert(&account_id, &account);
        self.accounts.insert(&friend_id, &friend);
    }
}

pub fn hash_account_id(account_id: &str) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}
