use alloy_primitives::{keccak256, Address, B256};
use serde::{Deserialize, Serialize};

pub type MemorizerKey = B256;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeaderKey {
    pub chain_id: u64,
    pub block_number: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountKey {
    pub chain_id: u64,
    pub block_number: u64,
    pub address: Address,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorageKey {
    pub chain_id: u64,
    pub block_number: u64,
    pub address: Address,
    pub storage_slot: B256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionKey {
    pub chain_id: u64,
    pub block_number: u64,
    pub transaction_index: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconHeaderKey {
    pub chain_id: u64,
    pub block_number: u64,
}

impl From<HeaderKey> for MemorizerKey {
    fn from(value: HeaderKey) -> Self {
        Self(*keccak256(bincode::serialize(&value).unwrap()))
    }
}

impl From<AccountKey> for MemorizerKey {
    fn from(value: AccountKey) -> Self {
        Self(*keccak256(bincode::serialize(&value).unwrap()))
    }
}

impl From<StorageKey> for MemorizerKey {
    fn from(value: StorageKey) -> Self {
        Self(*keccak256(bincode::serialize(&value).unwrap()))
    }
}

impl From<TransactionKey> for MemorizerKey {
    fn from(value: TransactionKey) -> Self {
        Self(*keccak256(bincode::serialize(&value).unwrap()))
    }
}

impl From<BeaconHeaderKey> for MemorizerKey {
    fn from(value: BeaconHeaderKey) -> Self {
        let mut data = bincode::serialize(&value).unwrap();
        data.extend("BeaconHeaderKey".as_bytes());
        Self(*keccak256(data))
    }
}
