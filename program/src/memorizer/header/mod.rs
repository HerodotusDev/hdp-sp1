use super::keys::HeaderKey;
use alloy_consensus::Header;
use cfg_if::cfg_if;

pub trait HeaderMemorizer {
    fn get_header(&mut self, key: HeaderKey) -> Header;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
