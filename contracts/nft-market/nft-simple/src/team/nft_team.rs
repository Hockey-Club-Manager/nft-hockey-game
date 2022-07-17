use crate::*;
use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json;
use crate::{TokenId, TokenMetadata};
use crate::extra::player_position::PlayerPosition;
use crate::team::ice_time_priority::IceTimePriority;
use crate::team::nft_team::IceTimePriority::*;
use crate::team::number_five::NumberFive;
use crate::team::number_goalie::NumberGoalie;


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
}

// #[near_bindgen]
// impl Contract {
//     pub fn insert_nft_field_players(&mut self, fives: Vec<(Fives, Vec<(PlayerPosition, TokenId)>)>) {
//         let account_id = env::predecessor_account_id();
//         let user_tokens = self.tokens_per_owner.get(&account_id).unwrap();
//
//         match &mut self.nft_team_per_owner.get(&account_id) {
//             Some(nft_team) => {
//                 for (num, five) in fives {
//                     match nft_team.fives.get_mut(&num) {
//                         Some(nft_five) => {
//                             for (player_position, token_id) in five {
//                                 if !user_tokens.contains(&token_id){
//                                     panic!("Token id not found");
//                                 }
//
//                                 nft_five.field_players.insert(player_position, token_id);
//                             }
//                         },
//                         None => panic!("Five not found")
//                     }
//                 }
//
//                 self.nft_team_per_owner.insert(&account_id, nft_team);
//             },
//             None => panic!("Team not found")
//         };
//     }
//
//     pub fn insert_nft_goalies(&mut self, goalies: Vec<(Goalies, TokenId)>) {
//         let account_id = env::predecessor_account_id();
//         let user_tokens = self.tokens_per_owner.get(&account_id).unwrap();
//
//         match &mut self.nft_team_per_owner.get(&account_id) {
//             Some(nft_team) => {
//                 for (priority, token_id) in goalies {
//                     if !user_tokens.contains(&token_id){
//                         panic!("Token id not found");
//                     }
//
//                     nft_team.goalies.insert(priority, token_id);
//                 }
//
//                 self.nft_team_per_owner.insert(&account_id, nft_team);
//             },
//             None => panic!("Team not found")
//         };
//     }
//
//     pub fn get_teams(&mut self, account_id_1: AccountId, account_id_2: AccountId) -> (TeamMetadata, TeamMetadata) {
//         (self.get_owner_team(account_id_1), self.get_owner_team(account_id_2))
//     }
//
//     pub fn get_owner_team(&mut self, account_id: AccountId) -> TeamMetadata {
//         let mut team = match self.free_team_per_owner.get(&account_id) {
//             Some(team) => team,
//             None => self.create_free_team(account_id.clone()),
//         };
//
//         let nft_team = match self.nft_team_per_owner.get(&account_id) {
//             Some(nft_team) => nft_team,
//             None => {
//                 let team = self.create_empty_nft_team();
//                 self.nft_team_per_owner.insert(&account_id, &team);
//                 team
//             }
//         };
//
//         for (fives, five) in nft_team.fives {
//             match team.fives.get_mut(&fives)  {
//                 Some(free_five) => {
//                     free_five.ice_time_priority = five.ice_time_priority;
//                     free_five.number = five.number;
//
//                     for (player_position, toke_id) in five.field_players {
//                         match self.token_metadata_by_id.get(&toke_id){
//                             Some(token_metadata) => {
//                                 free_five.field_players.insert(player_position, token_metadata);
//                             },
//                             None => {}
//                         };
//                     }
//                 }
//                 None => {}
//             };
//         }
//
//         for (goalies, toke_id) in nft_team.goalies {
//             match self.token_metadata_by_id.get(&toke_id) {
//                 Some(token_metadata) => {
//                     team.goalies.insert(goalies, token_metadata);
//                 },
//                 None => {}
//             };
//         }
//
//         team
//     }
//
//     pub fn get_owner_nft_team(&self, account_id: AccountId) -> NftTeam {
//         match self.nft_team_per_owner.get(&account_id) {
//             Some(nft_team) => nft_team,
//             None => panic!("Team not found")
//         }
//     }
//
//     fn create_empty_nft_team(&self) -> NftTeam {
//         let mut empty_fives = HashMap::new();
//
//         empty_fives.insert(First, self.create_empty_five(First));
//         empty_fives.insert(Second, self.create_empty_five(Second));
//         empty_fives.insert(Third, self.create_empty_five(Third));
//         empty_fives.insert(Fourth, self.create_empty_five(Fourth));
//
//         NftTeam {
//             fives: empty_fives,
//             goalies: HashMap::new()
//         }
//     }
//
//     fn create_empty_five(&self, number: Fives) -> NftFive {
//         NftFive {
//             field_players: HashMap::new(),
//             number,
//             ice_time_priority: Normal
//         }
//     }
//
//     fn create_free_team(&mut self, account_id: AccountId) -> TeamMetadata {
//         let mut fives = HashMap::new();
//
//         let first_five = self.create_five(First, SuperHighPriority, 65, 75, 88, 76, 69);
//         fives.insert(First, first_five.clone());
//         fives.insert(Second, self.create_five(Second, HighPriority, 71, 73, 68, 90, 72));
//         fives.insert(Third, self.create_five(Third, Normal, 66, 81, 84, 67, 69));
//         fives.insert(Fourth, self.create_five(Fourth, LowPriority, 87, 75, 89, 65, 81));
//
//         let mut goalies = HashMap::new();
//
//         let main_goalkeeper = self.create_goalie(String::from("Bakin"), 1, Wall, 67, 76, 86, 81, 90);
//         goalies.insert(Goalies::MainGoalkeeper, main_goalkeeper.clone());
//         goalies.insert(Goalies::SubstituteGoalkeeper, self.create_goalie(String::from("Noname"), 2, Post2Post, 83, 75, 88, 75, 67));
//
//
//         let free_team = TeamMetadata {
//             fives,
//             goalies,
//         };
//
//         self.free_team_per_owner.insert(&account_id, &free_team);
//
//         free_team
//     }
//
//     fn create_five(&self, number: Fives, ice_time_priority: IceTimePriority, strength: u128, iq: u128, morale: u128, skating: u128, shooting: u128) -> FiveMetadata {
//         FiveMetadata {
//             field_players: self.create_field_players(strength, iq, morale, skating, shooting),
//             number,
//             ice_time_priority,
//         }
//     }
//
//     fn create_field_players(&self, strength: u128, iq: u128, morale: u128, skating: u128, shooting: u128) -> HashMap<PlayerPosition, TokenMetadata> {
//         let mut field_players = HashMap::new();
//
//         let center = self.create_field_player(String::from("Schukin"), 10,Shooter, Center,
//                                                                 strength, iq, morale, skating, shooting);
//         let right_wind = self.create_field_player(String::from("Antipov"), 77,TryHarder, RightWing,
//                                                                     strength, iq, morale, skating, shooting);
//         let left_wind = self.create_field_player(String::from("Kislyak"), 99, Dangler, LeftWing,
//                                                                    strength, iq, morale, skating, shooting);
//         let right_defender = self.create_field_player(String::from("Ponomarev"), 27,Goon, RightDefender,
//                                                                         strength, iq, morale, skating, shooting);
//         let left_defender = self.create_field_player(String::from("Tsarev"), 31, Professor, LeftDefender,
//                                                                        strength, iq, morale, skating, shooting);
//
//         field_players.insert(Center, center);
//         field_players.insert(RightWing, right_wind);
//         field_players.insert(LeftWing, left_wind);
//         field_players.insert(RightDefender, right_defender);
//         field_players.insert(LeftDefender, left_defender);
//         field_players
//     }
//
//     fn create_field_player(&self, name: String, number: u8, role: PlayerRole, position: PlayerPosition,
//                                              strength: u128, iq: u128, morale: u128, skating: u128, shooting: u128) -> TokenMetadata {
//         let stats = vec![strength, iq, skating, shooting, morale];
//
//         let extra = Extra {
//             number,
//             player_role: role,
//             player_position: position,
//             player_type: PlayerType::FieldPlayer,
//             stats,
//         };
//
//         self.free_token_metadata(Option::from(name),
//                                  Option::from(serde_json::to_string(&extra).unwrap()))
//     }
//
//     fn create_goalie(&self, name: String, number: u8, role: PlayerRole,
//                                        glove_and_blocker: u128, pads:u128, stand: u128, stretch: u128, morale: u128) -> TokenMetadata {
//         let stats = vec![glove_and_blocker, pads, stand, stretch, morale];
//
//         let extra = Extra {
//             number,
//             player_role: role,
//             player_position: GoaliePos,
//             player_type: PlayerType::Goalie,
//             stats,
//         };
//
//         self.free_token_metadata(Option::from(name),
//                                  Option::from(serde_json::to_string(&extra).unwrap()))
//     }
//
//     fn free_token_metadata(&self, title: Option<String>, extra: Option<String>) -> TokenMetadata {
//         TokenMetadata {
//             title,
//             extra,
//             description: None,
//             media: None,
//             media_hash: None,
//             issued_at: None,
//             expires_at: None,
//             starts_at: None,
//             updated_at: None,
//         }
//     }
// }