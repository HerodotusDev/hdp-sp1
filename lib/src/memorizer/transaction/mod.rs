use super::{keys::TransactionKey, MemorizerError};
use alloy_consensus::TxEnvelope;
use cfg_if::cfg_if;

pub trait TransactionMemorizer {
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
