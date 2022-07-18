use near_sdk::serde::de::Unexpected::Str;
use near_sdk::serde_json;
use near_sdk::serde_json::from_str;
use crate::*;
use crate::extra::field_player_extra::FieldPlayerExtra;
use crate::extra::goalie_extra::GoalieExtra;
use crate::extra::player_type::PlayerType;
use crate::extra::stats::Stats;

#[near_bindgen]
impl Contract {
    /// only owner can mint NFT

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: Option<TokenId>,
        metadata: TokenMetadata,
        player_type: PlayerType,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
        receiver_id: Option<ValidAccountId>,
        token_type: Option<TokenType>,
    ) {
        self.assert_owner();
        let rarity = get_rarity(&metadata, &player_type);

        let mut final_token_id = format!("{}", self.token_metadata_by_id.len() + 1);
        if let Some(token_id) = token_id {
            final_token_id = token_id
        }

        let initial_storage_usage = env::storage_usage();
        let mut owner_id = env::predecessor_account_id();
        if let Some(receiver_id) = receiver_id {
            owner_id = receiver_id.into();
        }

        // CUSTOM - create royalty map
        let mut royalty = HashMap::new();
        let mut total_perpetual = 0;
        // user added perpetual_royalties (percentage paid with every transfer)
        if let Some(perpetual_royalties) = perpetual_royalties {
            assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
                total_perpetual += amount;
            }
        }
        // royalty limit for minter capped at 20%
        assert!(total_perpetual <= MINTER_ROYALTY_CAP, "Perpetual royalties cannot be more than 20%");

        let token = Token {
            owner_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
            royalty,
            token_type,
        };
        assert!(
            self.tokens_by_id.insert(&final_token_id, &token).is_none(),
            "Token already exists"
        );

        self.internal_add_token_to_pack(&player_type, &rarity, &final_token_id);
        self.token_metadata_by_id.insert(&final_token_id, &metadata);

        let new_token_size_in_bytes = env::storage_usage() - initial_storage_usage;
        let required_storage_in_bytes =
            self.extra_storage_in_bytes_per_token + new_token_size_in_bytes;

        refund_deposit(required_storage_in_bytes);
    }
}

pub fn get_rarity(metadata: &TokenMetadata, player_type: &PlayerType) -> Rarity {
    let stats = get_stats(metadata, player_type);
    stats.get_rarity()
}

pub fn get_stats(metadata: &TokenMetadata, player_type: &PlayerType) -> Box<dyn Stats> {
    match player_type {
        PlayerType::FieldPlayer => {
            let field_player_extra: FieldPlayerExtra = match from_str(&metadata.extra.as_ref().unwrap()) {
                Ok(extra) => extra,
                Err(err) => panic!("Incorrect stats or card type")
            };

            Box::new(field_player_extra.stats)
        },
        PlayerType::Goalie => {
            let goalie_extra: GoalieExtra = match from_str(&metadata.extra.as_ref().unwrap()) {
                Ok(extra) => extra,
                Err(err) => panic!("Incorrect stats or card type")
            };

            Box::new(goalie_extra.stats)
        },
    }
}