use super::{keys::HeaderKey, MemorizerError};
use alloy_consensus::Header;
use cfg_if::cfg_if;

pub trait HeaderMemorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
