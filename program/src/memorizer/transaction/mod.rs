use alloy_rpc_types::Transaction;
use cfg_if::cfg_if;
use std::error::Error;

use super::keys::TransactionKey;

pub trait TransactionMemorizer {
    fn get_storage(&mut self, key: TransactionKey) -> Result<Transaction, Box<dyn Error>>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
