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
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftFive {
    pub(crate) field_players: HashMap<PlayerPosition, TokenId>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
}

#[derive(Serialize, Deserialize)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TeamMetadata {
    pub(crate) fives: HashMap<Fives, FiveMetadata>,
    pub(crate) goalies: HashMap<Goalies, TokenMetadata>,
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FiveMetadata {
    pub(crate) field_players: HashMap<PlayerPosition, TokenMetadata>,
    pub(crate) number: Fives,
    pub(crate) ice_time_priority: IceTimePriority,
}

#[near_bindgen]
impl Contract {
    pub fn insert_nft_field_player_to_nft_team(&mut self, five: Fives, player_position: PlayerPosition, token_id: TokenId) {
        let account_id = env::predecessor_account_id();

        match &mut self.nft_team_per_owner.get(&account_id) {
            Some(nft_team) => {
               match nft_team.fives.get_mut(&five) {
                   Some(five) => five.field_players.insert(player_position, token_id),
                   None => panic!("Five not found")
               }
            },
            None => panic!("Team not found")
        };
    }

    pub fn get_owner_team(&mut self) -> TeamMetadata {
        let account_id = env::predecessor_account_id();

        let mut team = match self.free_team_per_owner.get(&account_id) {
            Some(team) => team,
            None => self.create_free_team(account_id.clone()),
        };

        let nft_team = match self.nft_team_per_owner.get(&account_id) {
            Some(nft_team) => nft_team,
            None => {
                let team = NftTeam {
                    fives: HashMap::new(),
                    goalies: HashMap::new(),
                };
                self.nft_team_per_owner.insert(&account_id, &team);
                team
            }
        };

        for (fives, five) in nft_team.fives {
            match team.fives.get_mut(&fives)  {
                Some(free_five) => {
                    free_five.ice_time_priority = five.ice_time_priority;
                    free_five.number = five.number;

                    for (player_position, toke_id) in five.field_players {
                        match self.token_metadata_by_id.get(&toke_id){
                            Some(token_metadata) => {
                                free_five.field_players.insert(player_position, token_metadata);
                            },
                            None => {}
                        };
                    }
                }
                None => {}
            };
        }

        for (goalies, toke_id) in nft_team.goalies {
            match self.token_metadata_by_id.get(&toke_id) {
                Some(token_metadata) => {
                    team.goalies.insert(goalies, token_metadata);
                },
                None => {}
            };
        }

        team
    }

    fn create_free_team(&mut self, account_id: AccountId) -> TeamMetadata {
        let mut fives = HashMap::new();

        let first_five = self.create_five(Fives::First, SuperHighPriority, 65, 75, 88, 76, 69);
        fives.insert(Fives::First, first_five.clone());
        fives.insert(Fives::Second, self.create_five(Fives::Second, HighPriority, 71, 73, 68, 90, 72));
        fives.insert(Fives::Third, self.create_five(Fives::Third, Normal, 66, 81, 84, 67, 69));
        fives.insert(Fives::Fourth, self.create_five(Fives::Fourth, LowPriority, 87, 75, 89, 65, 81));

        let mut goalies = HashMap::new();

        let main_goalkeeper = self.create_goalie(String::from("Bakin"), 1, Wall, 67, 76, 86, 81, 90);
        goalies.insert(Goalies::MainGoalkeeper, main_goalkeeper.clone());
        goalies.insert(Goalies::SubstituteGoalkeeper, self.create_goalie(String::from("Noname"), 2, Post2Post, 83, 75, 88, 75, 67));


        let free_team = TeamMetadata {
            fives,
            goalies,
        };

        self.free_team_per_owner.insert(&account_id, &free_team);

        free_team
    }

    fn create_five(&self, number: Fives, ice_time_priority: IceTimePriority, strength: u128, iq: u128, morale: u128, skating: u128, shooting: u128) -> FiveMetadata {
        FiveMetadata {
            field_players: self.create_field_players(strength, iq, morale, skating, shooting),
            number,
            ice_time_priority,
        }
    }

    fn create_field_players(&self, strength: u128, iq: u128, morale: u128, skating: u128, shooting: u128) -> HashMap<PlayerPosition, TokenMetadata> {
        let mut field_players = HashMap::new();

        let center = self.create_field_player(String::from("Schukin"), 10,Shooter, Center,
                                                                strength, iq, morale, skating, shooting);
        let right_wind = self.create_field_player(String::from("Antipov"), 77,TryHarder, RightWing,
                                                                    strength, iq, morale, skating, shooting);
        let left_wind = self.create_field_player(String::from("Kislyak"), 99, Dangler, LeftWing,
                                                                   strength, iq, morale, skating, shooting);
        let right_defender = self.create_field_player(String::from("Ponomarev"), 27,Goon, RightDefender,
                                                                        strength, iq, morale, skating, shooting);
        let left_defender = self.create_field_player(String::from("Tsarev"), 31, Professor, LeftDefender,
                                                                       strength, iq, morale, skating, shooting);

        field_players.insert(Center, center);
        field_players.insert(RightWing, right_wind);
        field_players.insert(LeftWing, left_wind);
        field_players.insert(RightDefender, right_defender);
        field_players.insert(LeftDefender, left_defender);
        field_players
    }

    fn create_field_player(&self, name: String, number: u8, role: PlayerRole, position: PlayerPosition,
                                             strength: u128, iq: u128, morale: u128, skating: u128, shooting: u128) -> TokenMetadata {
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

    fn create_goalie(&self, name: String, number: u8, role: PlayerRole,
                                       glove_and_blocker: u128, pads:u128, stand: u128, stretch: u128, morale: u128) -> TokenMetadata {
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
}