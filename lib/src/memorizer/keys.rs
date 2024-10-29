use alloy_primitives::{keccak256, Address, B256};
use serde::{Deserialize, Serialize};

use crate::chain::ChainId;

/// Alias for the memorizer key, representing a unique 256-bit identifier.
pub type MemorizerKey = B256;

/// Key for identifying a specific block header within a chain.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeaderKey {
    /// Chain ID of the network.
    pub chain_id: ChainId,
    /// Block number within the chain.
    pub block_number: u64,
}

/// Key for identifying a specific account within a block.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountKey {
    /// Chain ID of the network.
    pub chain_id: ChainId,
    /// Block number at which the account data is relevant.
    pub block_number: u64,
    /// Address of the account.
    pub address: Address,
}

/// Key for identifying a specific storage entry within an account.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageKey {
    /// Chain ID of the network.
    pub chain_id: ChainId,
    /// Block number at which the storage data is relevant.
    pub block_number: u64,
    /// Address of the account holding the storage.
    pub address: Address,
    /// Specific storage slot within the account.
    pub storage_slot: B256,
}

/// Key for identifying a specific transaction within a block.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionKey {
    /// Chain ID of the network.
    pub chain_id: ChainId,
    /// Block number containing the transaction.
    pub block_number: u64,
    /// Index of the transaction within the block.
    pub transaction_index: u64,
}

/// Key for identifying a specific receipt within a block.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReceiptKey {
    /// Chain ID of the network.
    pub chain_id: ChainId,
    /// Block number containing the receipt.
    pub block_number: u64,
    /// Index of the transaction receipt within the block.
    pub transaction_index: u64,
}

/// Key for identifying a specific consensus layer (beacon) header.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconHeaderKey {
    /// Chain ID of the network.
    pub chain_id: ChainId,
    /// Block number of the beacon chain header.
    pub block_number: u64,
}

impl From<HeaderKey> for MemorizerKey {
    fn from(value: HeaderKey) -> Self {
        Self(*keccak256(
            bincode::serialize(&value).expect("bincode serde error"),
        ))
    }
}

impl From<AccountKey> for MemorizerKey {
    fn from(value: AccountKey) -> Self {
        Self(*keccak256(bincode::serialize(&value).unwrap()))
    }
}

impl From<StorageKey> for MemorizerKey {
    fn from(value: StorageKey) -> Self {
        Self(*keccak256(
            bincode::serialize(&value).expect("bincode serde error"),
        ))
    }
}

impl From<TransactionKey> for MemorizerKey {
    fn from(value: TransactionKey) -> Self {
        Self(*keccak256(
            bincode::serialize(&value).expect("bincode serde error"),
        ))
    }
}

impl From<ReceiptKey> for MemorizerKey {
    fn from(value: ReceiptKey) -> Self {
        Self(*keccak256(
            bincode::serialize(&value).expect("bincode serde error"),
        ))
    }
}

impl From<BeaconHeaderKey> for MemorizerKey {
    fn from(value: BeaconHeaderKey) -> Self {
        let mut data = bincode::serialize(&value).expect("bincode serde error");
        data.extend("BeaconHeaderKey".as_bytes());
        Self(*keccak256(data))
    }
}
