use super::HeaderMemorizer;
use crate::memorizer::{
    keys::HeaderKey,
    values::{HeaderMemorizerValue, MemorizerValue},
    Memorizer,
};
use alloy_consensus::Header;
use hdp_lib::{header::IndexerRpc, mmr::MmrMeta, provider::header::IndexerClient};
use tokio::runtime::Runtime;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Header {
        let rt = Runtime::new().unwrap();
        let block: IndexerRpc = rt.block_on(async {
            let client = IndexerClient::default();
            client.get_header(key.block_number).await.unwrap()
        });
        let mmr: MmrMeta = block.meta.into();
        let header: Header = block.proofs[0].rlp_block_header.clone().into();

        self.map.insert(
            key.into(),
            MemorizerValue::Header(HeaderMemorizerValue {
                mmr,
                header: header.clone(),
                proof: block.proofs[0].siblings_hashes.clone(),
            }),
        );
        header
    }
}
