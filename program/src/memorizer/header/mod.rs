use super::keys::HeaderKey;
use alloy_primitives::U256;
use cfg_if::cfg_if;

pub trait HeaderMemorizer {
    fn get_header(&mut self, key: HeaderKey) -> U256;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
