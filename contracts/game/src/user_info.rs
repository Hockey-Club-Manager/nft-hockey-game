use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::team::five::Tactics;
use crate::team::team::Team;

pub type UserId = usize;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserInfo {
    pub(crate) user_id: UserId,
    pub(crate) team: Team,
    pub(crate) account_id: AccountId,
    pub(crate) take_to_called: bool,
    pub(crate) coach_speech_called: bool,
    pub(crate) is_goalie_out: bool,
}