use super::{keys::HeaderKey, MemorizerError};
use alloy_consensus::Header;
use cfg_if::cfg_if;

/// Defines a trait for managing and retrieving block headers from the memorizer.
///
/// ### Online Mode
/// In online mode:
/// - If the requested header is missing from the memorizer, it is fetched and added to the memorizer.
///
/// ### zkVM Mode
/// In zkVM (Zero-Knowledge Virtual Machine) mode:
/// - The header is retrieved from the memorizer if present. If it exists but its `is_verified` flag is `false`, it undergoes verification.
/// - If the `is_verified` flag is `true`, the header is read directly from the memorizer without additional verification.
pub trait HeaderMemorizer {
    /// Retrieves a block header based on the provided [`HeaderKey`].
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
