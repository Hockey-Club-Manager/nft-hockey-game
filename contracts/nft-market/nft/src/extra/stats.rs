use crate::Rarity;
use crate::Rarity::*;

pub trait Stats {
    fn get_rarity(&self) -> Rarity;
}

pub fn calculate_rarity(average_stats: f32) -> Rarity {
    if average_stats >= 95 as f32 {
        Exclusive
    } else if average_stats >= 85 as f32 {
        Unique
    } else if average_stats >= 75 as f32 {
        Rare
    } else if average_stats >= 60 as f32 {
        Uncommon
    } else {
        Common
    }
}