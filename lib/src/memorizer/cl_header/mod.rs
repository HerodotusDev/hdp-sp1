use super::MemorizerError;
use crate::memorizer::keys::BeaconHeaderKey;
use cfg_if::cfg_if;
use ssz_rs::prelude::*;

/// Represents a beacon chain header containing essential fields for consensus verification.
#[derive(
    PartialEq, Eq, Debug, Default, SimpleSerialize, serde::Serialize, serde::Deserialize, Clone,
)]
pub struct BeaconHeader {
    /// The slot number associated with this header.
    pub slot: u64,
    /// Index of the validator proposing this block.
    pub proposer_index: u64,
    /// Hash of the parent block’s root, linking to the previous header.
    pub parent_root: Vector<u8, 32>,
    /// Root hash of the state at this header.
    pub state_root: Vector<u8, 32>,
    /// Root hash of the block body, containing transactions and other data.
    pub body_root: Vector<u8, 32>,
}

/// Defines a trait for managing and retrieving consensus layer (CL) headers from the memorizer.
///
/// ### Online Mode
/// In online mode:
/// - If the dependent parent block header is missing, it is fetched first.
/// - Once the parent header is present, the requested consensus layer header is fetched and returned.
///
/// ### zkVM Mode
/// In zkVM (Zero-Knowledge Virtual Machine) mode:
/// - The parent block header is retrieved first, and if it exists but has an `is_verified` flag of `false`, it undergoes verification.
/// - After verifying the parent header, the requested consensus header is fetched.
/// - If the requested consensus header’s `is_verified` flag is `false`, it is verified as well.
/// - In both cases, if the `is_verified` flag is `true`, the memorized header is read directly without additional verification.
pub trait ClHeaderMemorizer {
    /// Retrieves a consensus layer header based on the provided [`BeaconHeaderKey`].
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> Result<BeaconHeader, MemorizerError>;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
