use super::TransactionMemorizer;
use crate::memorizer::{values::TransactionMemorizerValue, Memorizer, MemorizerError};
use alloy_consensus::TxEnvelope;
use alloy_rlp::Encodable;
use hdp_lib::transaction::{TransactionClient, TransactionResponse};
use tokio::runtime::Runtime;

impl TransactionMemorizer for Memorizer {
    fn get_transaction(
        &mut self,
        key: crate::memorizer::keys::TransactionKey,
    ) -> Result<TxEnvelope, MemorizerError> {
        let rpc_url = self.rpc_url.clone().unwrap();
        let rt = Runtime::new().unwrap();
        let transaction: TransactionResponse = rt.block_on(async {
            let client = TransactionClient::default();
            client
                .get_transaction(rpc_url, key.block_number, key.transaction_index)
                .await
                .unwrap()
        });

        let tx = transaction.tx.0;
        let mut out = Vec::new();
        tx.encode(&mut out);

        self.map.insert(
            key.into(),
            crate::memorizer::values::MemorizerValue::Transaction(TransactionMemorizerValue {
                transaction_encoded: out.into(),
                tx_index: transaction.tx_index,
                proof: transaction.proof,
            }),
        );

        Ok(tx)
    }
}
