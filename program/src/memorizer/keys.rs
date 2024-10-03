use super::MemorizerKey;
use alloy_primitives::{keccak256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeaderKey {
    pub chain_id: u32,
    pub block_number: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountKey {
    pub chain_id: u32,
    pub block_number: u32,
    pub address: U256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorageKey {
    pub chain_id: u32,
    pub block_number: u32,
    pub address: U256,
    pub storage_slot: U256,
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
