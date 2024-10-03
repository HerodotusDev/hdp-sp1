use super::{keys::HeaderKey, Memorizer};
use alloy_primitives::U256;
use cfg_if::cfg_if;

pub trait HeaderMemorizer {
    fn get_header(self, key: HeaderKey) -> U256;
}

impl HeaderMemorizer for Memorizer {
    fn get_header(self, key: HeaderKey) -> U256 {
        cfg_if! {
            if #[cfg(unix)] {
                println!("zkvm run");
                U256::from(0)
            } else {
                println!("online run");
                U256::from(0)
            }
        }
    }
}
