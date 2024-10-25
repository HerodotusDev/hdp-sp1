use super::HeaderMemorizer;
use crate::memorizer::{
    keys::HeaderKey,
    values::{HeaderMemorizerValue, MemorizerValue},
    Memorizer, MemorizerError,
};
use crate::{header::IndexerRpc, mmr::MmrMeta, provider::header::IndexerClient};
use alloy_consensus::Header;
use tokio::runtime::Runtime;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, MemorizerError> {
        let rt = Runtime::new()?;
        let block: IndexerRpc = rt.block_on(async {
            let client = IndexerClient::default();
            client.get_header(key.block_number).await.unwrap()
        });
        let mmr: MmrMeta = block.meta.into();
        let header: Header = block.proofs[0].rlp_block_header.clone().into();

        self.map.insert(
            key.into(),
            MemorizerValue::Header(HeaderMemorizerValue {
                header: header.clone(),
                element_index: block.proofs[0].element_index,
                element_hash: block.proofs[0].element_hash,
                rlp: block.proofs[0].rlp_block_header.string.clone(),
                proof: block.proofs[0].siblings_hashes.clone(),
            }),
        );
        self.mmr_meta = vec![mmr];

        Ok(header)
    }
}
