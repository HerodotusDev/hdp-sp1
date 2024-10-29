use crate::memorizer::cl_header::BeaconHeader;
use alloy_consensus::serde_bincode_compat;
use alloy_consensus::{Account, Header};
use alloy_primitives::{Bytes, B256, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Represents a memorized header with associated metadata.
#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaderMemorizerValue {
    #[serde_as(as = "serde_bincode_compat::Header")]
    /// Header.
    pub header: Header,
    /// Index of the element in the MMR.
    pub element_index: u128,
    /// Hash of the element.
    pub element_hash: B256,
    /// RLP-encoded string of the header.
    pub rlp: String,
    /// Proof elements for verification.
    pub proof: Vec<B256>,
}

/// Stores an account with associated proof data.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountMemorizerValue {
    /// account data.
    pub account: Account,
    /// Proof elements for account verification.
    pub proof: Vec<Bytes>,
}

/// Represents a memorized Beacon chain header.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct BeaconHeaderMemorizerValue {
    /// Beacon chain header.
    pub header: BeaconHeader,
}

/// Stores a storage value in the state with its proof.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageMemorizerValue {
    /// Storage value.
    pub value: U256,
    /// Proof elements for storage verification.
    pub proof: Vec<Bytes>,
}

/// Represents a memorized transaction with associated proof.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionMemorizerValue {
    /// Encoded transaction data.
    pub transaction_encoded: Bytes,
    /// Transaction index within the block.
    pub tx_index: u64,
    /// Proof elements for transaction verification.
    pub proof: Vec<Bytes>,
}

/// Represents a memorized transaction receipt with associated proof.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceiptMemorizerValue {
    /// Encoded transaction receipt data.
    pub receipt_encoded: Bytes,
    /// Transaction index within the block.
    pub tx_index: u64,
    /// Proof elements for receipt verification.
    pub proof: Vec<Bytes>,
}

/// Enum encapsulating different types of data that can be memorized.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MemorizerValue {
    /// header and proof.
    Header(HeaderMemorizerValue),
    /// account and proof.
    Account(AccountMemorizerValue),
    /// Storage value and proof.
    Storage(StorageMemorizerValue),
    /// Transaction and proof.
    Transaction(TransactionMemorizerValue),
    /// Transaction receipt and proof.
    Receipt(ReceiptMemorizerValue),
    /// Beacon header.
    BeaconHeader(BeaconHeaderMemorizerValue),
}
