use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct User {
    pub(crate) id: usize,
    pub (crate) score: u8,
}