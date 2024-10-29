use super::ReceiptMemorizer;
use crate::memorizer::{
    keys::HeaderKey, keys::MemorizerKey, values::MemorizerValue, HeaderMemorizer, Memorizer,
    MemorizerError,
};
use crate::mpt::Mpt;
use alloy_consensus::ReceiptEnvelope;
use alloy_rlp::Decodable;

impl ReceiptMemorizer for Memorizer {
    fn get_receipt(
        &mut self,
        key: crate::memorizer::keys::ReceiptKey,
    ) -> Result<ReceiptEnvelope, MemorizerError> {
        // 1. Header
        let header_key = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        };
        let header = self.get_header(header_key)?;

        // 2. Receipt
        let receipt_root = header.receipts_root;
        let receipt_key: MemorizerKey = key.into();

        if let Some((MemorizerValue::Receipt(receipt_value), is_verified)) =
            self.map.get_mut(&receipt_key)
        {
            if *is_verified {
                println!("Receipt MPT already verified");
                let tx_encoded = receipt_value.receipt_encoded.clone();
                Ok(ReceiptEnvelope::decode(&mut tx_encoded.as_ref())?)
            } else {
                let mpt = Mpt { root: receipt_root };
                println!("cycle-tracker-start: mpt (receipt)");
                mpt.verify_receipt(receipt_value.tx_index, receipt_value.proof.clone())?;
                println!("cycle-tracker-end: mpt (receipt)");
                *is_verified = true;
                let tx_encoded = receipt_value.receipt_encoded.clone();
                Ok(ReceiptEnvelope::decode(&mut tx_encoded.as_ref())?)
            }
        } else {
            Err(MemorizerError::MissingReceipt)
        }
    }
}
