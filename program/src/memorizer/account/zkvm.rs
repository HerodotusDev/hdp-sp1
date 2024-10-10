use super::AccountMemorizer;
use crate::memorizer::{keys::AccountKey, Memorizer, MemorizerError};
use alloy_consensus::Account;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError> {
        println!("zkvm run");
        Ok(Account::default())
    }
}
