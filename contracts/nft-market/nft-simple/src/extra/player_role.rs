use crate::*;

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
