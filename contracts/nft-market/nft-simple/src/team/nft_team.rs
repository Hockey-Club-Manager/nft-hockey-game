use crate::*;
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, Gas, log};
use near_sdk::serde_json;
use crate::{TokenId, TokenMetadata};
use crate::extra::field_player_extra::FieldPlayerExtra;
use crate::extra::goalie_extra::GoalieExtra;
use crate::extra::player_position::PlayerPosition;
use crate::team::ice_time_priority::IceTimePriority;
use crate::team::nft_team::IceTimePriority::*;
use crate::team::number_five::*;
use crate::team::number_goalie::NumberGoalie;

const GAS_FOR_CHECK_TOKENS_SALES: Gas = 10_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Tactics {
    Safe,
    Defensive,
    Neutral,
    Offensive,
    Aggressive,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamIds {
    pub(crate) fives: HashMap<NumberFive, FiveIds>,
    pub(crate) goalies: HashMap<NumberGoalie, TokenId>,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveIds {
    pub(crate) field_players: HashMap<PlayerPosition, TokenId>,
    pub(crate) number: NumberFive,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<NumberFive, FiveIds>,
    pub(crate) goalies: HashMap<NumberGoalie, TokenMetadata>,
    pub(crate) field_players_metadata: HashMap<TokenId, TokenMetadata>,
}

#[ext_contract(ext_check_tokens_sales)]
pub trait ExtTokensSales{
    fn check_tokens_sales(&self, token_ids: Vec<TokenId>, nft_contract_id: AccountId);
}

#[near_bindgen]
impl Contract {
    pub fn manage_team(&mut self, team_ids: TeamIds, nft_contact_id: AccountId, market_contract_id: AccountId) {
        let account_id = predecessor_account_id();

        let token_ids = self.check_team_ids(&team_ids);
        self.nft_team_per_owner.insert(&account_id, &team_ids);

        ext_check_tokens_sales::check_tokens_sales(
            token_ids,
            nft_contact_id,
            &market_contract_id,
            NO_DEPOSIT,
            GAS_FOR_CHECK_TOKENS_SALES);
    }

    pub fn check_team_ids(&self, team_ids: &TeamIds) -> Vec<TokenId> {
        let mut token_ids: Vec<TokenId> = Vec::new();

        token_ids.append(&mut self.check_fives(&team_ids.fives));
        token_ids.append(&mut self.check_goalies(&team_ids.goalies));

        token_ids
    }

    fn check_fives(&self, fives: &HashMap<NumberFive, FiveIds>) -> Vec<TokenId> {
        if fives.keys().len() != NUMBER_OF_FIVES {
            panic!("Wrong number of fives");
        }

        let mut result: Vec<TokenId> = Vec::new();

        for (number, five) in fives {
            let number_of_players = five.field_players.keys().len();
            match number {
                NumberFive::PenaltyKill1 | NumberFive::PenaltyKill2 => {
                    self.check_number_of_field_players(number_of_players, 4);
                },
                _ => self.check_number_of_field_players(number_of_players, 5)
            };

            result.append(&mut self.check_field_players(&five.field_players));
        }

        result
    }

    fn check_number_of_field_players(&self, number_of_players: usize, right_amount: usize) {
        if number_of_players != right_amount {
            panic!("Wrong number of field players: {}", number_of_players);
        }
    }

    fn check_field_players(&self, field_players: &HashMap<PlayerPosition, TokenId>) -> Vec<TokenId> {
        let mut result: Vec<TokenId> = Vec::new();
        for (_position, id) in field_players {
            result.push(id.clone());

            self.check_field_player(&id);
        }

        result
    }

    fn check_field_player(&self, field_player_id: &TokenId) {
        let account_id = env::predecessor_account_id();
        let user_tokens = self.tokens_per_owner.get(&account_id)
            .expect("You don't have tokens");

        if !user_tokens.contains(&field_player_id) {
            panic!("You are not the owner of the token");
        }

        let player_metadata = self.token_metadata_by_id.get(&field_player_id).expect("Token has no metadata");
        let _result: FieldPlayerExtra = match serde_json::from_str(&player_metadata.extra.unwrap()) {
            Ok(field_player_extra) => field_player_extra,
            Err(E) => panic!("{}", E)
        };
    }

    fn check_goalies(&self, goalies: &HashMap<NumberGoalie, TokenId>) -> Vec<TokenId> {
        if goalies.keys().len() != 2 {
            panic!("Wrong number of goalkeepers");
        }

        let mut result: Vec<TokenId> = Vec::new();

        for (_number, id) in goalies {
            result.push(id.clone());

            self.check_goalie(&id);
        }

        result
    }

    fn check_goalie(&self, goalie_id: &TokenId) {
        let account_id = env::predecessor_account_id();
        let user_tokens = self.tokens_per_owner.get(&account_id)
            .expect("You don't have tokens");

        if !user_tokens.contains(&goalie_id) {
            panic!("You are not the owner of the token");
        }

        let player_metadata = self.token_metadata_by_id.get(&goalie_id).expect("Token has no metadata");
        let _result: GoalieExtra = match serde_json::from_str(&player_metadata.extra.unwrap()) {
            Ok(goalie_extra) => goalie_extra,
            Err(E) => panic!("{}", E)
        };
    }

    pub fn get_teams(&mut self, account_id_1: AccountId, account_id_2: AccountId) -> (TeamMetadata, TeamMetadata) {
        (self.get_owner_team(&account_id_1), self.get_owner_team(&account_id_2))
    }

    pub fn get_owner_team(&mut self, account_id: &AccountId) -> TeamMetadata {
        TeamMetadata {
            fives: self.nft_team_per_owner.get(account_id).unwrap().fives,
            goalies: self.get_goalie_metadata_by_ids(account_id),
            field_players_metadata: self.get_field_players_metadata(account_id),
        }
    }

    fn get_goalie_metadata_by_ids(&self, account_id: &AccountId) -> HashMap<NumberGoalie, TokenMetadata> {
        let mut result: HashMap<NumberGoalie, TokenMetadata> = HashMap::new();

        let team_ids = self.nft_team_per_owner.get(account_id).expect("No team");
        for (goalies, toke_id) in team_ids.goalies.into_iter() {
            let token_metadata = self.token_metadata_by_id.get(&toke_id).expect("Token has no metadata");
            result.insert(goalies, token_metadata);
        }

        result
    }

    fn get_field_players_metadata(&self, account_id: &AccountId) -> HashMap<TokenId, TokenMetadata> {
        let mut result: HashMap<TokenId, TokenMetadata> = HashMap::new();

        for (_number, fives_ids) in self.nft_team_per_owner.get(account_id).unwrap().fives {
            for (_position, token_id) in fives_ids.field_players {
                let token_metadata = self.token_metadata_by_id.get(&token_id).expect("Token has no metadata");
                result.insert(token_id, token_metadata);
            }
        }

        result
    }

    /*
    pub fn get_owner_team_ids(&self, account_id: AccountId) -> TeamIds {
        match self.nft_team_per_owner.get(&account_id) {
            Some(nft_team) => nft_team,
            None => panic!("Team not found")
        }
    }
    */

    pub fn remove_token_from_team(&mut self, token_id: &TokenId) {
        let account_id = predecessor_account_id();

        let mut user_team = self.nft_team_per_owner.get(&account_id).unwrap();

        self.remove_token_from_fives(token_id, &mut user_team);
        self.remove_token_from_goalies(token_id, &mut user_team);

        self.nft_team_per_owner.insert(&account_id, &user_team);
    }

    fn remove_token_from_fives(&self, token_id: &TokenId, user_team: &mut TeamIds) {
        for (_five_number, five) in &mut user_team.fives {
            for (player_position, player_id) in five.field_players.clone() {
                if *token_id == player_id {
                    five.field_players.remove(&player_position);
                }
            }
        }
    }

    fn remove_token_from_goalies(&mut self, token_id: &TokenId, user_team: &mut TeamIds) {
        let mut goalies_to_remove: Vec<NumberGoalie> = Vec::new();

        for (goalie_number, goalie_id) in &user_team.goalies {
            if *token_id == *goalie_id {
                goalies_to_remove.push(goalie_number.clone());
            }
        }

        for goalie_number in &goalies_to_remove {
            user_team.goalies.remove(goalie_number);
        }
    }
}