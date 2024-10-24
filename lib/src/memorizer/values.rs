use crate::memorizer::cl_header::BeaconHeader;
use alloy_consensus::serde_bincode_compat;
use alloy_consensus::{Account, Header};
use alloy_primitives::{Bytes, B256, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaderMemorizerValue {
    #[serde_as(as = "serde_bincode_compat::Header")]
    pub header: Header,
    pub element_index: u128,
    pub element_hash: B256,
    pub rlp: String,
    pub proof: Vec<B256>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountMemorizerValue {
    pub account: Account,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct BeaconHeaderMemorizerValue {
    pub header: BeaconHeader,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageMemorizerValue {
    pub value: U256,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionMemorizerValue {
    pub transaction_encoded: Bytes,
    pub tx_index: u64,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceiptMemorizerValue {
    pub receipt_encoded: Bytes,
    pub tx_index: u64,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MemorizerValue {
    Header(HeaderMemorizerValue),
    Account(AccountMemorizerValue),
    Storage(StorageMemorizerValue),
    Transaction(TransactionMemorizerValue),
    Receipt(ReceiptMemorizerValue),
    BeaconHeader(BeaconHeaderMemorizerValue),
}
