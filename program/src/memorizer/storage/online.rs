use super::StorageMemorizer;
use crate::memorizer::{keys::StorageKey, Memorizer};
use alloy_primitives::U256;

impl StorageMemorizer for Memorizer {
    fn get_storage(&self, key: StorageKey) -> U256 {
        println!("online run");

        U256::from(0)
    }
}
