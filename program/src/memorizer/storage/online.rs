use super::StorageMemorizer;
use crate::memorizer::{keys::StorageKey, Memorizer, Proof};
use alloy_primitives::U256;
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use tokio::runtime::Runtime;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> U256 {
        let rt = Runtime::new().unwrap();
        let (value, proof): (U256, Vec<u8>) = rt.block_on(async {
            let client: ReqwestClient =
                ClientBuilder::default().http(self.rpc_url.clone().unwrap());
            let mut batch = client.new_batch();

            // TODO: Check and correct the parameters in these calls if necessary
            let block_header_fut = batch
                .add_call("eth_blockHeader", &key.block_number)
                .unwrap();
            let proof_fut = batch.add_call("eth_getProof", &key.block_number).unwrap();

            batch.send().await.unwrap();

            (block_header_fut.await.unwrap(), proof_fut.await.unwrap())
        });
        self.map.insert(key.into(), Proof(proof));
        value
    }
}
