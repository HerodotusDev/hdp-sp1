use super::keys::StorageKey;
use alloy::primitives::U256;
use cfg_if::cfg_if;

pub trait StorageMemorizer {
    fn get_storage(&mut self, key: StorageKey) -> U256;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
