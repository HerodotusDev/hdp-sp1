use super::{keys::ReceiptKey, MemorizerError};
use alloy_consensus::ReceiptEnvelope;
use cfg_if::cfg_if;

/// Defines a trait for managing and retrieving transaction receipts from the memorizer.
///
/// ### Online Mode
/// In online mode, if a requested receipt's dependent header is missing from the memorizer,
/// it is fetched automatically. After ensuring the header is present, the requested receipt
/// is fetched and returned.
///
/// ### zkVM Mode
/// In zkVM (Zero-Knowledge Virtual Machine) mode:
/// - If the dependent header exists but its `is_verified` flag is `false`, it is verified first.
/// - Similarly, if the requested receipt is present but has an `is_verified` flag of `false`,
///   it undergoes verification.
/// - In both cases, if the `is_verified` flag is `true`, the memorized header or receipt
///   is read directly without additional verification.
pub trait ReceiptMemorizer {
    /// Retrieves a transaction receipt based on the provided [`ReceiptKey`].
    fn get_receipt(&mut self, key: ReceiptKey) -> Result<ReceiptEnvelope, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
