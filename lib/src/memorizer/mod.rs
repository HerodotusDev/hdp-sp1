/// account memorizer
pub mod account;
/// consensus layer header memorizer
pub mod cl_header;
/// header memorizer
pub mod header;
/// memorizer keys
pub mod keys;
/// receipt memorizer
pub mod receipt;
/// storage memorizer
pub mod storage;
/// transaction memorizer
pub mod transaction;
/// memorizer values
pub mod values;

pub use account::*;
use alloy_sol_types::sol;
pub use cl_header::*;
pub use header::*;
pub use keys::*;
pub use receipt::*;
pub use storage::*;
pub use transaction::*;
pub use values::*;

use crate::{
    chain::ChainId,
    mmr::{MmrError, MmrMeta},
    mpt::MptError,
};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror_no_std::Error;
use url::Url;

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct PublicValuesStruct {
        /// @dev The id of the MMR.
        uint256 mmrId;
        /// @dev The size of the MMR.
        uint256 mmrSize;
        /// @dev The root of the MMR.
        bytes32 mmrRoot;
    }
}

/// Represents a main structure for managing and memorizing various components such as headers, accounts, and receipts.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Memorizer {
    /// Maps chain IDs to their respective RPC URLs.
    #[serde(skip)]
    pub chain_map: HashMap<ChainId, Url>,
    /// Target chain ID for verification.
    pub to_chain_id: ChainId,
    /// Metadata for the Merkle Mountain Range (MMR).
    pub mmr_meta: HashMap<ChainId, MmrMeta>,
    /// Maps memorizer keys to their values and a boolean flag for is already verified value.
    pub map: HashMap<MemorizerKey, (MemorizerValue, bool)>,
}

impl Memorizer {
    /// Creates a new [`Memorizer`] instance.
    pub fn new<S: AsRef<str>>(chain_map: HashMap<ChainId, Url>, to_chain_id: S) -> Self {
        Self {
            chain_map,
            to_chain_id: ChainId::from_str(to_chain_id.as_ref()).unwrap(),
            mmr_meta: Default::default(),
            map: Default::default(),
        }
    }
}

/// Defines errors that may occur within the memorizer.
#[derive(Debug, Error)]
pub enum MemorizerError {
    /// Indicates a missing or invalid header in the memorizer.
    #[error("Header is missing or invalid")]
    MissingHeader,

    /// Indicates a missing or invalid account in the memorizer.
    #[error("Account is missing or invalid")]
    MissingAccount,

    /// Indicates a missing or invalid storage entry in the memorizer.
    #[error("Storage is missing or invalid")]
    MissingStorage,

    /// Indicates a missing or invalid transaction in the memorizer.
    #[error("Transaction is missing or invalid")]
    MissingTransaction,

    /// Indicates a missing or invalid receipt in the memorizer.
    #[error("Receipt is missing or invalid")]
    MissingReceipt,

    /// Indicates a missing consensus layer beacon header in the memorizer.
    #[error("Beacon header is missing")]
    MissingBeaconRoot,

    /// Indicates an invalid consensus layer beacon header in the memorizer.
    #[error("Beacon header is invalid")]
    InvalidBeaconRoot,

    /// Indicates that the RPC URL could not be fetched for a specified `ChainId`.
    #[error("Failed to fetch RPC URL for chainId: {0}")]
    MissingRpcUrl(ChainId),

    /// Represents an I/O error, typically arising from file operations.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Indicates a failure in MPT proof verification.
    #[error(transparent)]
    MptProofFailed(#[from] MptError),

    /// Indicates a failure in MMR proof verification.
    #[error(transparent)]
    MmrProofFailed(#[from] MmrError),

    /// Represents an error in decoding RLP data.
    #[error(transparent)]
    RlpDecodeFailed(#[from] alloy_rlp::Error),

    /// Represents an error in transport operations, such as network failures.
    #[cfg(not(target_os = "zkvm"))]
    #[error(transparent)]
    TransportError(#[from] alloy_transport::TransportError),

    /// Represents an error in HTTP requests.
    #[cfg(not(target_os = "zkvm"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// Indicates an error in Ethereum trie proof verification.
    #[cfg(not(target_os = "zkvm"))]
    #[error(transparent)]
    EthTrieError(#[from] eth_trie_proofs::EthTrieError),

    /// Indicates that the given block number belongs to the pre-PoS (Proof of Stake) era.
    #[error("The given execution layer block number was produced before the PoS transition")]
    InvalidPoSBlockNumber,

    /// Indicates an unknown base chain ID in the memorizer.
    #[error("Unknown base chain chainId")]
    UnknownBaseChainId,
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Bytes, B256};
    use std::fs;
    use tempdir::TempDir;
    use values::{HeaderMemorizerValue, TransactionMemorizerValue};

    use super::*;

    #[test]
    fn test_memorizer() {
        let binding = TempDir::new("test").unwrap();
        let path = binding.path().join("memorizer.bin");

        let mut original_mem = Memorizer::new(HashMap::default(), "ETHEREUM_SEPOLIA");
        original_mem.mmr_meta = HashMap::default();
        original_mem.map.insert(
            B256::ZERO,
            (
                MemorizerValue::Header(HeaderMemorizerValue::default()),
                false,
            ),
        );
        let raw_tx = alloy_primitives::hex::decode("02f86f0102843b9aca0085029e7822d68298f094d9e1459a7a482635700cbc20bbaf52d495ab9c9680841b55ba3ac080a0c199674fcb29f353693dd779c017823b954b3c69dffa3cd6b2a6ff7888798039a028ca912de909e7e6cdef9cdcaf24c54dd8c1032946dfa1d85c206b32a9064fe8").unwrap();
        // let res = TxEnvelope::decode(&mut raw_tx.as_slice()).unwrap();
        original_mem.map.insert(
            B256::ZERO,
            (
                MemorizerValue::Transaction(TransactionMemorizerValue {
                    transaction_encoded: Bytes::from(raw_tx),
                    tx_index: 0,
                    proof: Default::default(),
                }),
                false,
            ),
        );

        fs::write(&path, bincode::serialize(&original_mem).unwrap()).unwrap();
        let mem = bincode::deserialize(&fs::read(path).unwrap()).unwrap();

        assert_eq!(original_mem, mem);
    }
}
