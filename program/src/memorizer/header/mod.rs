use std::error::Error;

use super::keys::HeaderKey;
use alloy_consensus::Header;
use cfg_if::cfg_if;

pub trait HeaderMemorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, Box<dyn Error>>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
