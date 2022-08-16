use near_sdk::env::{attached_deposit, predecessor_account_id};
use near_sdk::{log, serde_json};
use crate::*;
use crate::extra::player_type::PlayerType;
use crate::extra::player_type::PlayerType::{FieldPlayer, Goalie};

pub(crate) const  BRONZE_PACK_COST: Balance = 7_0000_0000_0000_0000_0000_0000;
pub(crate) const SILVER_PACK_COST: Balance = 10_0000_0000_0000_0000_0000_0000;
pub(crate) const GOLD_PACK_COST: Balance = 13_0000_0000_0000_0000_0000_0000;
pub(crate) const PLATINUM_PACK_COST: Balance = 15_0000_0000_0000_0000_0000_0000;
pub(crate) const BRILLIANT_PACK_COST: Balance = 20_0000_0000_0000_0000_0000_0000;

const NUMBER_OF_CARDS_IN_PACK: usize = 3;
const FIELD_PLAYER_PROBABILITY: u8 = 80;

pub enum Pack {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Brilliant
}

fn get_rarity_by_index(index: usize) -> Rarity {
    let rarities: Vec<Rarity> = vec![Rarity::Common, Rarity::Uncommon, Rarity::Rare, Rarity::Unique, Rarity::Exclusive];
    rarities[index]
}

pub fn get_pack_probabilities(pack: Pack) -> Vec<u8> {
    // usual -> rare -> super_rare ->  myth -> exclusive
    let bronze_probabilities: Vec<u8> = vec![80, 20, 0, 0, 0];
    let silver_probabilities: Vec<u8> = vec![50, 45, 5, 0, 0];
    let gold_probabilities : Vec<u8> = vec![30, 40, 25, 5, 0];
    let platinum_probabilities: Vec<u8> = vec![10, 30, 30, 20, 10];
    let brilliant_probabilities: Vec<u8> = vec![0, 15, 30, 35, 20];

    match pack {
        Pack::Bronze => bronze_probabilities,
        Pack::Silver => silver_probabilities,
        Pack::Gold => gold_probabilities,
        Pack::Platinum => platinum_probabilities,
        Pack::Brilliant => brilliant_probabilities,
    }
}

#[near_bindgen]
impl Contract {
    pub fn nft_register_account(&mut self, receiver_id: AccountId) -> Vec<TokenMetadata> {
        if !self.is_account_registered() {
            panic!("Account already registered");
        }

        let mut pack_probabilities = get_pack_probabilities(Pack::Silver);

        let mut tokens: Vec<TokenId> = Vec::new();

        let mut result: Vec<TokenMetadata> = Vec::new();
        for i in 0.. 6 {
            let player_type = if i == 0 {
                Goalie
            } else {
                FieldPlayer
            };

            let rnd = self.get_random_in_range(1, 100, i);
            let random_rarity = self.get_random_rarity(pack_probabilities, rnd);
            let token_id = self.get_random_token_by_rarity(&player_type, &random_rarity);

            tokens.push(token_id.clone());

            result.push(self.internal_transfer_token_from_pack(&receiver_id, &token_id, &player_type, &random_rarity));

            pack_probabilities = get_pack_probabilities(Pack::Silver);
        }

        let json_tokens = match serde_json::to_string(&tokens) {
            Ok(res) => res,
            Err(e) => panic!("{}", e)
        };

        log!("{}", json_tokens);

        result
    }

    pub fn is_account_registered(&self) -> bool {
        self.registered_accounts.contains(&predecessor_account_id())
    }

    #[payable]
    pub fn nft_buy_pack(&mut self, receiver_id: AccountId) -> Vec<TokenMetadata> {
        let mut tokens: Vec<TokenId> = Vec::new();

        let mut result: Vec<TokenMetadata> = Vec::new();
        for i in 0..NUMBER_OF_CARDS_IN_PACK {
            let pack_probabilities = self.internal_get_pack_probabilities();

            let rnd = self.get_random_in_range(1, 100, i);
            let random_rarity = self.get_random_rarity(pack_probabilities, rnd);
            let random_player_type = self.get_random_player_type(rnd);
            let token_id = self.get_random_token_by_rarity(&random_player_type, &random_rarity);

            tokens.push(token_id.clone());

            result.push(self.internal_transfer_token_from_pack(&receiver_id, &token_id, &random_player_type, &random_rarity));
        }

        let json_tokens = match serde_json::to_string(&tokens) {
            Ok(res) => res,
            Err(e) => panic!("{}", e)
        };

        log!("{}", json_tokens);

        result
    }

    pub fn get_random_in_range(&self, min: usize, max: usize, index: usize) -> u8 {
        let random = *env::random_seed().get(index).unwrap();
        let random_in_range = (random as f64 / 256.0) * (max - min) as f64 + min as f64;
        random_in_range.floor() as u8
    }

    pub fn get_random_rarity(&self, probabilities: Vec<u8>, rnd: u8) -> Rarity {
        let mut percent: u8 = 100;

        for i in 0..probabilities.len() {
            percent -= probabilities[i];

            if rnd >= percent {
                return get_rarity_by_index(i);
            }
        }

        panic!("Get random rarity failed :(");
    }

    pub fn get_random_player_type(&self, rnd: u8) -> PlayerType {
        if rnd >= FIELD_PLAYER_PROBABILITY {
            Goalie
        } else {
            FieldPlayer
        }
    }

    pub fn get_random_token_by_rarity(&self, player_type: &PlayerType, rarity: &Rarity) -> TokenId {
        match player_type {
            FieldPlayer => {
                self.field_players.get(rarity).unwrap().as_vector().get(0).unwrap()
            },
            Goalie => {
                self.goalies.get(rarity).unwrap().as_vector().get(0).unwrap()
            }
        }
    }
}
