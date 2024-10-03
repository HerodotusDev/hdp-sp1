use super::AccountMemorizer;
use crate::memorizer::{keys::AccountKey, Memorizer};
use alloy_primitives::U256;

impl AccountMemorizer for Memorizer {
    fn get_account(&self, key: AccountKey) -> U256 {
        println!("zkvm run");

        U256::from(0)
    }
}
