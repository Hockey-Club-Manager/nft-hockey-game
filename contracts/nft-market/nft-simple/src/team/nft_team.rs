use crate::*;
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json;
use crate::{TokenId, TokenMetadata};
use crate::extra::field_player_extra::FieldPlayerExtra;
use crate::extra::goalie_extra::GoalieExtra;
use crate::extra::player_position::PlayerPosition;
use crate::team::ice_time_priority::IceTimePriority;
use crate::team::nft_team::IceTimePriority::*;
use crate::team::number_five::*;
use crate::team::number_goalie::NumberGoalie;

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
#[derive(Serialize, Deserialize)]
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

#[near_bindgen]
impl Contract {
    pub fn manage_team(&mut self, team_ids: TeamIds) {
        let account_id = env::predecessor_account_id();

        self.check_team_ids(&team_ids);
        self.nft_team_per_owner.insert(&account_id, &team_ids);
    }

    pub fn check_team_ids(&self, team_ids: &TeamIds) {
        self.check_fives(&team_ids.fives);
        self.check_goalies(&team_ids.goalies);
    }

    fn check_fives(&self, fives: &HashMap<NumberFive, FiveIds>) {
        if fives.keys().len() != NUMBER_OF_FIVES {
            panic!("Wrong number of fives");
        }

        for (number, five) in fives {
            let number_of_players = five.field_players.keys().len();
            match number {
                NumberFive::PenaltyKill1 | NumberFive::PenaltyKill2 => {
                    self.check_number_of_field_players(number_of_players, 4);
                },
                _ => self.check_number_of_field_players(number_of_players, 5)
            };

            self.check_field_players(&five.field_players);
        }
    }

    fn check_number_of_field_players(&self, number_of_players: usize, right_amount: usize) {
        if number_of_players != right_amount {
            panic!("Wrong number of field players: {}", number_of_players);
        }
    }

    fn check_field_players(&self, field_players: &HashMap<PlayerPosition, TokenId>) {
        for (_position, id) in field_players {
            self.check_field_player(&id);
        }
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

    fn check_goalies(&self, goalies: &HashMap<NumberGoalie, TokenId>) {
        if goalies.keys().len() != 2 {
            panic!("Wrong number of goalkeepers");
        }

        for (_number, id) in goalies {
            self.check_goalie(&id);
        }
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

    pub fn get_owner_team_ids(&self, account_id: AccountId) -> TeamIds {
        match self.nft_team_per_owner.get(&account_id) {
            Some(nft_team) => nft_team,
            None => panic!("Team not found")
        }
    }
}