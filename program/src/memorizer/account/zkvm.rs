use super::AccountMemorizer;
use crate::memorizer::{keys::AccountKey, Memorizer};
use alloy_consensus::Account;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Account {
        println!("zkvm run");

        Account::default()
    }
}
