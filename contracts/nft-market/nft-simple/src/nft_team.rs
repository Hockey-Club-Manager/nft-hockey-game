use crate::*;
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json;
use crate::{TokenId, TokenMetadata};
use crate::nft_team::IceTimePriority::{HighPriority, LowPriority, Normal, SuperHighPriority};
use crate::nft_team::PlayerPosition::{Center, LeftDefender, LeftWing, RightDefender, RightWing};
use crate::nft_team::PlayerRole::{Dangler, Goon, Post2Post, Professor, Shooter, TryHarder, Wall};


type SRC = String;

#[derive(Serialize, Deserialize, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum IceTimePriority {
    SuperLowPriority,
    LowPriority,
    Normal,
    HighPriority,
    SuperHighPriority,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Fives {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerType {
    FieldPlayer,
    Goalie,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerRole {
    // Forward
    Passer,
    Shooter,
    TryHarder,
    Dangler,

    // Defender
    Rock,
    Goon,
    Professor,
    ToughGuy,

    // goalie
    Wall,
    Post2Post,
}

#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerPosition {
    Center,
    LeftWing,
    RightWing,
    LeftDefender,
    RightDefender,
    GoaliePos,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct  Extra {
    pub number: u8,
    pub player_role: PlayerRole,
    pub player_position: PlayerPosition,
    pub player_type: PlayerType,
    pub stats: Vec<u128>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub enum Goalies {
    MainGoalkeeper,
    SubstituteGoalkeeper,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftTeam {
    pub(crate) fives: HashMap<Fives, NftFive>,
    pub(crate) goalies: HashMap<Goalies, TokenId>,
    pub(crate) active_five: NftFive,

    pub(crate) active_goalie: TokenId,
    pub(crate) score: u8,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftFive {
    pub(crate) field_players: HashMap<PlayerPosition, TokenId>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) time_field: u8,
}

#[derive(Serialize, Deserialize)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<Fives, FiveMetadata>,
    pub(crate) goalies: HashMap<Goalies, TokenMetadata>,
    pub(crate) active_five: FiveMetadata,

    pub(crate) active_goalie: TokenMetadata,
    pub(crate) score: u8,
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveMetadata {
    pub(crate) field_players: HashMap<PlayerPosition, TokenMetadata>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
    pub(crate) time_field: u8,
}

#[near_bindgen]
impl Contract {
    pub fn get_owner_team(&mut self) {
        let account_id = env::predecessor_account_id();

        let free_team = match self.free_team_per_owner.get(&account_id) {
            Some(team) => team,
            None => self.create_free_team(account_id),
        };
    }

    fn create_free_team(&mut self, account_id: AccountId) -> TeamMetadata {
        let mut fives = HashMap::new();

        let first_five = self.create_five(Fives::First, SuperHighPriority);
        fives.insert(Fives::First, first_five.clone());
        fives.insert(Fives::Second, self.create_five(Fives::Second, HighPriority));
        fives.insert(Fives::Third, self.create_five(Fives::Third, Normal));
        fives.insert(Fives::Fourth, self.create_five(Fives::Fourth, LowPriority));

        let mut goalies = HashMap::new();

        let main_goalkeeper = self.create_goalie_with_random_stats(String::from("Bakin"), 1, Wall);
        goalies.insert(Goalies::MainGoalkeeper, main_goalkeeper.clone());
        goalies.insert(Goalies::SubstituteGoalkeeper, self.create_goalie_with_random_stats(String::from("Noname"), 2, Post2Post));


        let free_team = TeamMetadata {
            fives,
            goalies,
            active_five: first_five.clone(),
            active_goalie: main_goalkeeper.clone(),
            score: 0,
        };

        self.free_team_per_owner.insert(&account_id, &free_team);

        free_team
    }

    fn create_five(&self, number: Fives, ice_time_priority: IceTimePriority) -> FiveMetadata {
        FiveMetadata {
            field_players: self.create_field_players_with_random_stats(),
            number,
            ice_time_priority,
            time_field: 0,
        }
    }

    fn create_field_players_with_random_stats(&self) -> HashMap<PlayerPosition, TokenMetadata> {
        let mut field_players = HashMap::new();

        let center = self.create_field_player_with_random_stats(String::from("Schukin"), 10,Shooter, Center);
        let right_wind = self.create_field_player_with_random_stats(String::from("Antipov"), 77,TryHarder, RightWing);
        let left_wind = self.create_field_player_with_random_stats(String::from("Kislyak"), 99, Dangler, LeftWing);
        let right_defender = self.create_field_player_with_random_stats(String::from("Ponomarev"), 27,Goon, RightDefender);
        let left_defender = self.create_field_player_with_random_stats(String::from("Tsarev"), 31, Professor, LeftDefender);

        field_players.insert(Center, center);
        field_players.insert(RightWing, right_wind);
        field_players.insert(LeftWing, left_wind);
        field_players.insert(RightDefender, right_defender);
        field_players.insert(LeftDefender, left_defender);
        field_players
    }

    fn create_field_player_with_random_stats(&self, name: String, number: u8, role: PlayerRole, position: PlayerPosition) -> TokenMetadata {
        let strength = self.get_random_in_range(60, 90)  as u128;
        let iq = self.get_random_in_range(60, 90)  as u128;
        let morale = self.get_random_in_range(60, 90) as u128;
        let skating = self.get_random_in_range(60, 90) as u128;
        let shooting = self.get_random_in_range(60, 90) as u128;

        let stats = vec![strength, iq, skating, shooting, morale];

        let extra = Extra {
            number,
            player_role: role,
            player_position: position,
            player_type: PlayerType::FieldPlayer,
            stats,
        };

        self.free_token_metadata(Option::from(name),
                                 Option::from(serde_json::to_string(&extra).unwrap()))
    }

    fn create_goalie_with_random_stats(&self, name: String, number: u8, role: PlayerRole) -> TokenMetadata {
        let glove_and_blocker = self.get_random_in_range(60, 90)  as u128;
        let pads = self.get_random_in_range(60, 90)  as u128;
        let stand = self.get_random_in_range(60, 90) as u128;
        let stretch = self.get_random_in_range(60, 90) as u128;
        let morale = self.get_random_in_range(60, 90) as u128;

        let stats = vec![glove_and_blocker, pads, stand, stretch, morale];

        let extra = Extra {
            number,
            player_role: role,
            player_position: PlayerPosition::GoaliePos,
            player_type: PlayerType::Goalie,
            stats,
        };

        self.free_token_metadata(Option::from(name),
                                 Option::from(serde_json::to_string(&extra).unwrap()))
    }

    fn free_token_metadata(&self, title: Option<String>, extra: Option<String>) -> TokenMetadata {
        TokenMetadata {
            title,
            extra,
            description: None,
            media: None,
            media_hash: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
        }
    }

    fn get_random_in_range(&self, min: usize, max: usize) -> usize {
        let random = *env::random_seed().get(0).unwrap();
        let random_in_range = (random as f64 / 256.0) * (max - min) as f64 + min as f64;
        random_in_range.floor() as usize
    }
}