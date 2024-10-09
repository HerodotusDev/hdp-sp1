use alloy_consensus::{Account, Header};
use alloy_primitives::{Bytes, B256, U256};
use hdp_lib::mmr::MmrMeta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeaderMemorizerValue {
    pub header: Header,
    pub mmr: MmrMeta,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountMemorizerValue {
    pub account: Account,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageMemorizerValue {
    pub key: B256,
    pub value: U256,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MemorizerValue {
    Header(HeaderMemorizerValue),
    Account(AccountMemorizerValue),
    Storage(StorageMemorizerValue),
}
