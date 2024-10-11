use super::StorageMemorizer;
use crate::memorizer::values::StorageMemorizerValue;
use crate::memorizer::MemorizerError;
use crate::memorizer::{keys::StorageKey, Memorizer};
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{B256, U256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use alloy_rpc_types::EIP1186AccountProofResponse;
use tokio::runtime::Runtime;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError> {
        let rt = Runtime::new().unwrap();
        let value: StorageMemorizerValue = rt.block_on(async {
            let client: ReqwestClient =
                ClientBuilder::default().http(self.rpc_url.clone().unwrap());
            let mut batch = client.new_batch();

            let block_header_fut: alloy_rpc_client::Waiter<EIP1186AccountProofResponse> = batch
                .add_call(
                    "eth_getProof",
                    &(
                        key.address,
                        Vec::<B256>::new(),
                        BlockNumberOrTag::from(key.block_number),
                    ),
                )
                .unwrap();

            batch.send().await.unwrap();
            let response: EIP1186AccountProofResponse = block_header_fut.await.unwrap();

            StorageMemorizerValue {
                key: response.storage_proof[0].key.0,
                value: response.storage_proof[0].value,
                proof: response.storage_proof[0].proof.clone(),
            }
        });

        self.map.insert(
            key.into(),
            crate::memorizer::values::MemorizerValue::Storage(value.clone()),
        );

        Ok(value.value)
    }
}
