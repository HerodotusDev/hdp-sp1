use super::ReceiptMemorizer;
use crate::memorizer::{HeaderKey, HeaderMemorizer, ReceiptMemorizerValue};
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
        let header_key = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        };
        let _ = self.get_header(header_key)?;

        let rt = Runtime::new()?;
        let rpc_url = self
            .chain_map
            .get(&key.chain_id)
            .ok_or(MemorizerError::MissingRpcUrl(key.chain_id))?
            .to_owned();
        let transaction: ReceiptResponse = rt.block_on(async {
            let client = TransactionClient::default();
            client
                .get_receipt(rpc_url, key.block_number, key.transaction_index)
                .await
                .map_err(MemorizerError::EthTrieError)
        })?;

        let tx = transaction.receipt.0;
        let mut out = Vec::new();
        tx.encode(&mut out);

        self.map.insert(
            key.into(),
            (
                crate::memorizer::values::MemorizerValue::Receipt(ReceiptMemorizerValue {
                    receipt_encoded: out.into(),
                    tx_index: transaction.tx_index,
                    proof: transaction.proof,
                }),
                false,
            ),
        );

        Ok(tx)
    }
}
