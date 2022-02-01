use crate::cell::CellData;
use crate::user::User;

// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Clone)]
pub struct UserInfo {
    user: User,
    // pub(crate) account_id: AccountId,
}

// #[derive(BorshDeserialize, BorshSerialize)]
pub struct GameToSave {
    pub(crate) user_1: UserInfo,
    pub(crate) user_2: UserInfo,
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,

    // pub(crate) field: LookupMap<u8, CellData>,
}

pub struct Game {
    pub(crate) players: [UserInfo; 2],
    // pub(crate) reward: TokenBalance,
    pub(crate) winner_index: Option<usize>,
    // pub(crate) total_time_spent: Vec<Timestamp>,
    // pub(crate) board: Board,
}