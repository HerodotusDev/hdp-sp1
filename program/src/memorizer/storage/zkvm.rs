use super::StorageMemorizer;
use crate::memorizer::{keys::StorageKey, Memorizer};
use alloy_primitives::U256;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> U256 {
        println!("zkvm run");

        U256::from(0)
    }
}
