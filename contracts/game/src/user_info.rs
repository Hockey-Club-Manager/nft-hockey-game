use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::team::five::Tactics;
use crate::team::team::Team;


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub(crate) user_id: usize,
    pub(crate) team: Team,
    pub(crate) account_id: AccountId,
    pub(crate) take_to_called: bool,
    pub(crate) coach_speech_called: bool,
    pub(crate) is_goalie_out: bool,
    pub(crate) tactic: Tactics,
}