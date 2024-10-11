use super::{keys::StorageKey, MemorizerError};
use alloy_primitives::U256;
use cfg_if::cfg_if;

pub trait StorageMemorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
