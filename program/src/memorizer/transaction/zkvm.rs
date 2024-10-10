use super::TransactionMemorizer;
use crate::memorizer::{
    keys::HeaderKey, keys::MemorizerKey, keys::TransactionKey, values::MemorizerValue, Memorizer,
};
use alloy_consensus::TxEnvelope;
use alloy_rlp::Decodable;
use hdp_lib::mpt::Mpt;
use std::error::Error;

impl TransactionMemorizer for Memorizer {
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, Box<dyn Error>> {
        let header_key: MemorizerKey = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        }
        .into();

        if let MemorizerValue::Header(header_value) = self.map.get(&header_key).unwrap() {
            let tx_root = header_value.header.transactions_root;
            let tx_key: MemorizerKey = key.into();

            if let MemorizerValue::Transaction(tx_value) = self.map.get(&tx_key).unwrap() {
                let mpt = Mpt { root: tx_root };
                println!("cycle-tracker-start: mpt");
                mpt.verify(tx_value.tx_index, tx_value.proof.clone());
                println!("cycle-tracker-end: mpt");
                let tx_encoded = tx_value.transaction_encoded.clone();
                Ok(TxEnvelope::decode(&mut tx_encoded.as_ref()).unwrap())
            } else {
                Err("Transaction not found".into())
            }
        } else {
            Err("Header not found".into())
        }
    }
}
