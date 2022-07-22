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
pub struct NftTeam {
    pub(crate) fives: HashMap<NumberFive, NftFive>,
    pub(crate) goalies: HashMap<NumberGoalie, TokenId>,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftFive {
    pub(crate) field_players: HashMap<PlayerPosition, TokenId>,
    pub(crate) number: NumberFive,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<NumberFive, FiveMetadata>,
    pub(crate) goalies: HashMap<NumberGoalie, TokenMetadata>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveMetadata {
    pub(crate) field_players: HashMap<PlayerPosition, TokenMetadata>,
    pub(crate) number: NumberFive,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) tactic: Tactics,
}

#[near_bindgen]
impl Contract {
    pub fn manage_team(&mut self, team_ids: NftTeam) {
        let account_id = env::predecessor_account_id();

        self.check_team_ids(&team_ids);
        self.nft_team_per_owner.insert(&account_id, &team_ids);
    }

    pub fn check_team_ids(&self, team_ids: &NftTeam) {
        self.check_fives(&team_ids.fives);
        self.check_goalies(&team_ids.goalies);
    }

    fn check_fives(&self, fives: &HashMap<NumberFive, NftFive>) {
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
        for (position, id) in field_players {
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
        let result: FieldPlayerExtra = match serde_json::from_str(&player_metadata.extra.unwrap()) {
            Ok(field_player_extra) => field_player_extra,
            Err(E) => panic!("Wrong player type")
        };
    }

    fn check_goalies(&self, goalies: &HashMap<NumberGoalie, TokenId>) {
        if goalies.keys().len() != 2 {
            panic!("Wrong number of goalkeepers");
        }

        for (number, id) in goalies {
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
        let result: GoalieExtra = match serde_json::from_str(&player_metadata.extra.unwrap()) {
            Ok(goalie_extra) => goalie_extra,
            Err(E) => panic!("Wrong player type")
        };
    }

    pub fn get_teams(&mut self, account_id_1: AccountId, account_id_2: AccountId) -> (TeamMetadata, TeamMetadata) {
        (self.get_owner_team(account_id_1), self.get_owner_team(account_id_2))
    }

    pub fn get_owner_team(&mut self, account_id: AccountId) -> TeamMetadata {
        TeamMetadata {
            fives: self.get_five_metadata_by_ids(&account_id),
            goalies: self.get_goalie_metadata_by_ids(&account_id),
        }
    }

    fn get_five_metadata_by_ids(&self, account_id: &AccountId) -> HashMap<NumberFive, FiveMetadata> {
        let mut result: HashMap<NumberFive, FiveMetadata> = HashMap::new();

        let team_ids = self.nft_team_per_owner.get(account_id).expect("No team");

        for (fives, five) in team_ids.fives {
            let mut field_player_metadata: HashMap<PlayerPosition, TokenMetadata> = HashMap::new();

            for (player_position, toke_id) in five.field_players {
                let token_metadata = self.token_metadata_by_id.get(&toke_id).expect("Token has no metadata");
                field_player_metadata.insert(player_position, token_metadata);
            }

            let five_metadata = FiveMetadata {
                field_players: field_player_metadata,
                number: fives.clone(),
                ice_time_priority: five.ice_time_priority.clone(),
                tactic: five.tactic
            };

            result.insert(fives, five_metadata);
        };

        result
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

    pub fn get_owner_team_ids(&self, account_id: AccountId) -> NftTeam {
        match self.nft_team_per_owner.get(&account_id) {
            Some(nft_team) => nft_team,
            None => panic!("Team not found")
        }
    }
}