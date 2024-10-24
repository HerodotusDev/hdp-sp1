use super::ReceiptMemorizer;
use crate::memorizer::{
    keys::HeaderKey, keys::MemorizerKey, values::MemorizerValue, Memorizer, MemorizerError,
};
use crate::mpt::Mpt;
use alloy_consensus::ReceiptEnvelope;
use alloy_rlp::Decodable;

impl ReceiptMemorizer for Memorizer {
    fn get_receipt(
        &mut self,
        key: crate::memorizer::keys::ReceiptKey,
    ) -> Result<ReceiptEnvelope, MemorizerError> {
        let header_key: MemorizerKey = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        }
        .into();

        if let Some(MemorizerValue::Header(header_value)) = self.map.get(&header_key) {
            let receipt_root = header_value.header.receipts_root;
            let receipt_key: MemorizerKey = key.into();

            if let Some(MemorizerValue::Receipt(receipt_value)) = self.map.get(&receipt_key) {
                let mpt = Mpt { root: receipt_root };
                println!("cycle-tracker-start: mpt (receipt)");
                mpt.verify_receipt(receipt_value.tx_index, receipt_value.proof.clone())?;
                println!("cycle-tracker-end: mpt (receipt)");
                let tx_encoded = receipt_value.receipt_encoded.clone();
                Ok(ReceiptEnvelope::decode(&mut tx_encoded.as_ref())?)
            } else {
                Err(MemorizerError::MissingReceipt)
            }
        } else {
            println!("Missing header, {:?}", key.block_number);
            Err(MemorizerError::MissingHeader)
        }
    }
}
