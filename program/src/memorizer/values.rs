use alloy_consensus::{Account, Header};
use alloy_primitives::{Bytes, B256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct HeaderMemorizerValue {
    pub header: Header,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountMemorizerValue {
    pub account: Account,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageMemorizerValue {
    pub key: B256,
    pub value: U256,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MemorizerValue {
    Header(HeaderMemorizerValue),
    Account(AccountMemorizerValue),
    Storage(StorageMemorizerValue),
}
