use super::keys::AccountKey;
use alloy_primitives::U256;
use cfg_if::cfg_if;

pub trait AccountMemorizer {
    fn get_account(&mut self, key: AccountKey) -> U256;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
