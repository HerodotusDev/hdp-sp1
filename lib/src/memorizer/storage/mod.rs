use super::{keys::StorageKey, MemorizerError};
use alloy_primitives::U256;
use cfg_if::cfg_if;

/// Defines a trait for managing and retrieving storage values from the memorizer.
///
/// ### Online Mode
/// In online mode:
/// - If the requested storage's dependent header is missing from the memorizer, it is fetched automatically.
/// - Once the header is present, an `eth_getProof` request retrieves both the account and storage data.
/// - If the account is missing, both the account and storage entries are saved to the memorizer for future access.
///
/// ### zkVM Mode
/// In zkVM (Zero-Knowledge Virtual Machine) mode:
/// - The header is checked and verified first. If it exists but has an `is_verified` flag of `false`, it is verified.
/// - After verifying the header, the account is checked similarly, and if not verified, it undergoes verification.
/// - Finally, the storage is checked and verified if needed.
/// - If any element (header, account, or storage) has an `is_verified` flag of `true`, it is read directly from the memorizer without re-verification.
pub trait StorageMemorizer {
    /// Retrieves a storage value based on the provided [`StorageKey`].
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
