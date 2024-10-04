use super::HeaderMemorizer;
use crate::memorizer::{keys::HeaderKey, Memorizer};
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::U256;
use alloy::rpc::client::{ClientBuilder, ReqwestClient};
use tokio::runtime::Runtime;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> U256 {
        let rt = Runtime::new().unwrap();
        let (_, _): (Vec<u8>, Vec<u8>) = rt.block_on(async {
            let client: ReqwestClient =
                ClientBuilder::default().http(self.rpc_url.clone().unwrap());
            let mut batch = client.new_batch();

            // TODO: Check and correct the parameters in these calls if necessary
            let block_header_fut = batch
                .add_call(
                    "eth_getBlockByNumber",
                    &(BlockNumberOrTag::from(key.block_number), false),
                )
                .unwrap();
            let proof_fut = batch
                .add_call(
                    "eth_getBlockByNumber",
                    &(BlockNumberOrTag::from(key.block_number), false),
                )
                .unwrap();

            batch.send().await.unwrap();

            (block_header_fut.await.unwrap(), proof_fut.await.unwrap())
        });
        //self.map.insert(key.into(), Proof(proof));
        U256::ZERO
    }
}
