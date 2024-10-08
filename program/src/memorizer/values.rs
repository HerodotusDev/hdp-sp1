use crate::memorizer::cl_header::BeaconHeader;
use alloy_consensus::Account;
use alloy_primitives::{Bytes, B256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountMemorizerValue {
    pub account: Account,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeaderMemorizerValue {
    pub header: U256,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconHeaderMemorizerValue {
    pub header: BeaconHeader,
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
    Account(AccountMemorizerValue),
    Header(HeaderMemorizerValue),
    Storage(StorageMemorizerValue),
    BeaconHeader(BeaconHeaderMemorizerValue),
}
