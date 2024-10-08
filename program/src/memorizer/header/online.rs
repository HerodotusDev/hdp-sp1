use super::HeaderMemorizer;
use crate::memorizer::{
    keys::HeaderKey,
    values::{HeaderMemorizerValue, MemorizerValue},
    Memorizer,
};
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::U256;
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use alloy_rpc_types::Block;
use serde::Serialize;
use tokio::runtime::Runtime;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> U256 {
        let rt = Runtime::new().unwrap();
        let block: Block = rt.block_on(async {
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

            batch.send().await.unwrap();

            block_header_fut.await.unwrap()
        });
        let header: U256 = block.header.hash.into();

        self.map.insert(
            key.into(),
            MemorizerValue::Header(HeaderMemorizerValue {
                header,
                proof: vec![],
            }),
        );
        header
    }
}
