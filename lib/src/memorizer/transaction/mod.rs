use super::{keys::TransactionKey, MemorizerError};
use alloy_consensus::TxEnvelope;
use cfg_if::cfg_if;

/// Defines a trait for managing and retrieving transactions from the memorizer.
///
/// ### Online Mode
/// In online mode, if a requested transaction's dependent header is missing from the memorizer,
/// it is fetched automatically. After ensuring the header is present, the requested transaction
/// is fetched and returned.
///
/// ### zkVM Mode
/// In zkVM (Zero-Knowledge Virtual Machine) mode:
/// - If the dependent header exists but its `is_verified` flag is `false`, it is verified first.
/// - Similarly, if the requested transaction is present but has an `is_verified` flag of `false`,
///   it undergoes verification.
/// - In both cases, if the `is_verified` flag is `true`, the memorized header or transaction
///   is read directly without additional verification.
pub trait TransactionMemorizer {
    /// Retrieves a transaction based on the provided [`TransactionKey`].
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
