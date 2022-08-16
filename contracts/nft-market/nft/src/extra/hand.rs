use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Hand {
    #[serde(alias = "L")]
    Left,
    #[serde(alias = "R")]
    Right,
}