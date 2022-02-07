use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Clone)]
pub struct User {
    pub(crate) id: usize,
    pub (crate) score: u8,
}