use std::cmp::min;
use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128, U64, ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    AccountId, Balance, BorshStorageKey, CryptoHash, env, near_bindgen, PanicOnDefault, Promise,
    PromiseOrValue, StorageUsage,
};

pub use crate::enumerable::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
use team::nft_team::{NftTeam, TeamMetadata};
use crate::extra::hand::Hand;
pub use crate::token::*;

mod burn;
mod enumerable;
mod internal;
mod metadata;
mod mint;
mod nft_core;
mod token;
mod extra;
mod team;
mod pack;

// CUSTOM types
pub type TokenType = String;
pub type TypeSupplyCaps = HashMap<TokenType, U64>;
pub const CONTRACT_ROYALTY_CAP: u32 = 1000;
pub const MINTER_ROYALTY_CAP: u32 = 2000;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub nft_team_per_owner: LookupMap<AccountId, NftTeam>,

    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    pub goalies: LookupMap<Rarity, UnorderedSet<TokenId>>,

    pub field_players: LookupMap<Rarity, UnorderedSet<TokenId>>,

    pub registered_accounts: UnorderedSet<AccountId>,

    pub tokens_by_id: LookupMap<TokenId, Token>,

    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    pub owner_id: AccountId,

    /// The storage size in bytes for one account.
    pub extra_storage_in_bytes_per_token: StorageUsage,

    pub metadata: LazyOption<NFTContractMetadata>,

    /// CUSTOM fields
    pub contract_royalty: u32,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NftContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    NftTeamPerOwner,
    GoaliesInner { goalies_hash: CryptoHash },
    Goalies,
    FieldPlayersInner { field_player_hash: CryptoHash },
    FieldPlayers,
    RegisterAccounts,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        metadata: NFTContractMetadata,
    ) -> Self {
        let mut this = Self {
            nft_team_per_owner: LookupMap::new(
                StorageKey::NftTeamPerOwner.try_to_vec().unwrap()
            ),
            tokens_per_owner: LookupMap::new(
                StorageKey::TokensPerOwner.try_to_vec().unwrap()
            ),
            goalies: LookupMap::new(
                StorageKey::Goalies.try_to_vec().unwrap()
            ),
            field_players: LookupMap::new(
                StorageKey::FieldPlayers.try_to_vec().unwrap()
            ),
            registered_accounts: UnorderedSet::new(
                StorageKey::RegisterAccounts.try_to_vec().unwrap(),
            ),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            owner_id: owner_id.into(),
            extra_storage_in_bytes_per_token: 0,
            metadata: LazyOption::new(
                StorageKey::NftContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),

            contract_royalty: 0,
        };

        this.measure_min_token_storage_cost();

        this
    }

    // TODO: remove on release
    pub fn delete_data(&mut self) {
        self.assert_owner();
        self.nft_team_per_owner = LookupMap::new(
            StorageKey::NftTeamPerOwner.try_to_vec().unwrap()
        );
        self.tokens_per_owner = LookupMap::new(
            StorageKey::TokensPerOwner.try_to_vec().unwrap()
        );
        self.goalies = LookupMap::new(
            StorageKey::Goalies.try_to_vec().unwrap()
        );
        self.field_players = LookupMap::new(
            StorageKey::FieldPlayers.try_to_vec().unwrap()
        );
        self.registered_accounts =  UnorderedSet::new(
            StorageKey::RegisterAccounts.try_to_vec().unwrap(),
        );
        self.tokens_by_id = LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap());
        self.token_metadata_by_id =  UnorderedMap::new(
            StorageKey::TokenMetadataById.try_to_vec().unwrap(),
        );
        self.extra_storage_in_bytes_per_token = 0;

        self.contract_royalty = 0;
    }

    fn measure_min_token_storage_cost(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = "a".repeat(64);
        let u = UnorderedSet::new(
            StorageKey::TokenPerOwnerInner {
                account_id_hash: hash_account_id(&tmp_account_id),
            }
                .try_to_vec()
                .unwrap(),
        );
        self.tokens_per_owner.insert(&tmp_account_id, &u);

        let tokens_per_owner_entry_in_bytes = env::storage_usage() - initial_storage_usage;
        let owner_id_extra_cost_in_bytes = (tmp_account_id.len() - self.owner_id.len()) as u64;

        self.extra_storage_in_bytes_per_token =
            tokens_per_owner_entry_in_bytes + owner_id_extra_cost_in_bytes;

        self.tokens_per_owner.remove(&tmp_account_id);
    }

    /// CUSTOM - setters for owner

    pub fn set_contract_royalty(&mut self, contract_royalty: u32) {
        self.assert_owner();
        assert!(contract_royalty <= CONTRACT_ROYALTY_CAP, "Contract royalties limited to 10% for owner");
        self.contract_royalty = contract_royalty;
    }
}
