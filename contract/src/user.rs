use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Clone)]
pub struct User {
    id: usize,
    score: u8,
}