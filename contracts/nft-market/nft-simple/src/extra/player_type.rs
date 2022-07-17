use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerType {
    FieldPlayer,
    Goalie,
}
