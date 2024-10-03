use super::MemorizerKey;
use alloy_primitives::{keccak256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeaderKey {
    chain_id: u32,
    block_number: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountKey {
    chain_id: u32,
    block_number: u32,
    address: U256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorageKey {
    chain_id: u32,
    block_number: u32,
    address: U256,
    storage_slot: U256,
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
