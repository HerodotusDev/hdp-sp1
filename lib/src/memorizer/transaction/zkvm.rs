use super::TransactionMemorizer;
use crate::memorizer::{
    keys::HeaderKey, keys::MemorizerKey, keys::TransactionKey, values::MemorizerValue,
    HeaderMemorizer, Memorizer, MemorizerError,
};
use crate::mpt::Mpt;
use alloy_consensus::TxEnvelope;
use alloy_rlp::Decodable;

impl TransactionMemorizer for Memorizer {
    fn get_transaction(&mut self, key: TransactionKey) -> Result<TxEnvelope, MemorizerError> {
        // 1. Header
        let header_key = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        };
        let header = self.get_header(header_key)?;

        // 2. Transaction
        let tx_root = header.transactions_root;
        let tx_key: MemorizerKey = key.into();

        if let Some((MemorizerValue::Transaction(tx_value), is_verified)) =
            self.map.get_mut(&tx_key)
        {
            if *is_verified {
                println!("Transaction MPT already verified");
                let tx_encoded = tx_value.transaction_encoded.clone();
                Ok(TxEnvelope::decode(&mut tx_encoded.as_ref())?)
            } else {
                let mpt = Mpt { root: tx_root };
                println!("cycle-tracker-start: mpt (transaction)");
                mpt.verify_transaction(tx_value.tx_index, tx_value.proof.clone())?;
                println!("cycle-tracker-end: mpt (transaction)");
                *is_verified = true;
                let tx_encoded = tx_value.transaction_encoded.clone();
                Ok(TxEnvelope::decode(&mut tx_encoded.as_ref())?)
            }
        } else {
            Err(MemorizerError::MissingTransaction)
        }
    }
}
