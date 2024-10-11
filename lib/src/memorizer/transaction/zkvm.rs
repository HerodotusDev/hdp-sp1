use super::TransactionMemorizer;
use crate::memorizer::{
    keys::HeaderKey, keys::MemorizerKey, keys::TransactionKey, values::MemorizerValue, Memorizer,
    MemorizerError,
};
use crate::mpt::Mpt;
use alloy_consensus::TxEnvelope;
use alloy_rlp::Decodable;

impl TransactionMemorizer for Memorizer {
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, MemorizerError> {
        let header_key: MemorizerKey = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        }
        .into();

        if let Some(MemorizerValue::Header(header_value)) = self.map.get(&header_key) {
            let tx_root = header_value.header.transactions_root;
            let tx_key: MemorizerKey = key.into();

            if let Some(MemorizerValue::Transaction(tx_value)) = self.map.get(&tx_key) {
                let mpt = Mpt { root: tx_root };
                println!("cycle-tracker-start: mpt");
                mpt.verify_transaction(tx_value.tx_index, tx_value.proof.clone())?;
                println!("cycle-tracker-end: mpt");
                let tx_encoded = tx_value.transaction_encoded.clone();
                Ok(TxEnvelope::decode(&mut tx_encoded.as_ref())?)
            } else {
                Err(MemorizerError::MissingTransaction)
            }
        } else {
            Err(MemorizerError::MissingHeader)
        }
    }
}
