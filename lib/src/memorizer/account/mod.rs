use super::{keys::AccountKey, MemorizerError};
use alloy_consensus::Account;
use cfg_if::cfg_if;

/// Defines a trait for managing and retrieving account data from the memorizer.
///
/// ### Online Mode
/// In online mode:
/// - If the requested account’s dependent header is missing from the memorizer, it is fetched first.
/// - Once the header is present, an `eth_getProof` request retrieves the account data.
/// - The account data is then saved to the memorizer for future access.
///
/// ### zkVM Mode
/// In zkVM (Zero-Knowledge Virtual Machine) mode:
/// - The header is checked and verified first. If it exists but has an `is_verified` flag of `false`, it is verified.
/// - After verifying the header, the account is checked, and if the `is_verified` flag is `false`, it undergoes verification.
/// - If either the header or account has an `is_verified` flag of `true`, it is read directly from the memorizer without re-verification.
pub trait AccountMemorizer {
    /// Retrieves account data based on the provided [`AccountKey`].
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
