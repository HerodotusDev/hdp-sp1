use super::ReceiptMemorizer;
use crate::memorizer::ReceiptMemorizerValue;
use crate::memorizer::{Memorizer, MemorizerError};
use crate::transaction::{ReceiptResponse, TransactionClient};
use alloy_consensus::ReceiptEnvelope;
use alloy_rlp::Encodable;
use tokio::runtime::Runtime;

impl ReceiptMemorizer for Memorizer {
    fn get_receipt(
        &mut self,
        key: crate::memorizer::keys::ReceiptKey,
    ) -> Result<ReceiptEnvelope, MemorizerError> {
        let rpc_url = self.chain_map.get(&key.chain_id).unwrap().to_owned();
        let rt = Runtime::new().unwrap();
        let transaction: ReceiptResponse = rt.block_on(async {
            let client = TransactionClient::default();
            client
                .get_receipt(rpc_url, key.block_number, key.transaction_index)
                .await
                .unwrap()
        });

        let tx = transaction.receipt.0;
        let mut out = Vec::new();
        tx.encode(&mut out);

        self.map.insert(
            key.into(),
            crate::memorizer::values::MemorizerValue::Receipt(ReceiptMemorizerValue {
                receipt_encoded: out.into(),
                tx_index: transaction.tx_index,
                proof: transaction.proof,
            }),
        );

        Ok(tx)
    }
}
