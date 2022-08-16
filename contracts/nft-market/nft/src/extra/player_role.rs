use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerRole {
    // Forward
    Playmaker,
    Enforcer,
    Shooter,
    TryHarder,
    DefensiveForward,
    Grinder,

    // Defenseman
    DefensiveDefenseman,
    OffensiveDefenseman,
    TwoWay,
    ToughGuy,

    // goalie
    Standup,
    Butterfly,
    Hybrid,
}
