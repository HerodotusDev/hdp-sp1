use super::StorageMemorizer;
use crate::memorizer::{keys::StorageKey, Memorizer, MemorizerError};
use alloy_primitives::U256;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError> {
        todo!("zkvm implement");
    }
}