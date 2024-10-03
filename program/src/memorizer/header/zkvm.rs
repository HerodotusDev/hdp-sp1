use super::HeaderMemorizer;
use crate::memorizer::{keys::HeaderKey, Memorizer};
use alloy_primitives::U256;

impl HeaderMemorizer for Memorizer {
    fn get_header(&self, key: HeaderKey) -> U256 {
        println!("zkvm run");

        U256::from(0)
    }
}
