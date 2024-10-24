use super::{keys::ReceiptKey, MemorizerError};
use alloy_consensus::ReceiptEnvelope;
use cfg_if::cfg_if;

pub trait ReceiptMemorizer {
    fn get_receipt(&mut self, key: ReceiptKey) -> Result<ReceiptEnvelope, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
