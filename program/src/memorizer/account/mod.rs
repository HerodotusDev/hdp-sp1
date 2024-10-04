use super::keys::AccountKey;
use alloy::consensus::Account;
use cfg_if::cfg_if;

pub trait AccountMemorizer {
    fn get_account(&mut self, key: AccountKey) -> Account;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
