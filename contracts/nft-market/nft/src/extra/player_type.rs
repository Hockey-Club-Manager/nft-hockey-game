use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerType {
    FieldPlayer,
    Goalie,
}
