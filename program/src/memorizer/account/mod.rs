use super::{keys::AccountKey, MemorizerError};
use alloy_consensus::Account;
use cfg_if::cfg_if;

pub trait AccountMemorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
