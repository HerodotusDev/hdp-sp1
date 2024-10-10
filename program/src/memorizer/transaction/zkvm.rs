use super::TransactionMemorizer;
use crate::memorizer::{keys::TransactionKey, Memorizer};
use alloy_consensus::TxEnvelope;
use alloy_rlp::Decodable;
use std::error::Error;

impl TransactionMemorizer for Memorizer {
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, Box<dyn Error>> {
        println!("zkvm run");

        let raw_tx = alloy_primitives::hex::decode("02f86f0102843b9aca0085029e7822d68298f094d9e1459a7a482635700cbc20bbaf52d495ab9c9680841b55ba3ac080a0c199674fcb29f353693dd779c017823b954b3c69dffa3cd6b2a6ff7888798039a028ca912de909e7e6cdef9cdcaf24c54dd8c1032946dfa1d85c206b32a9064fe8").unwrap();
        let res = TxEnvelope::decode(&mut raw_tx.as_slice()).unwrap();
        Ok(res)
    }
}
