use alloy_consensus::TxEnvelope;
use cfg_if::cfg_if;
use std::error::Error;

use super::keys::TransactionKey;

pub trait TransactionMemorizer {
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, Box<dyn Error>>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
