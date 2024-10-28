use super::HeaderMemorizer;
use crate::memorizer::{
    keys::HeaderKey,
    values::{HeaderMemorizerValue, MemorizerValue},
    Memorizer, MemorizerError, MemorizerKey,
};
use crate::{header::IndexerRpc, mmr::MmrMeta, provider::header::IndexerClient};
use alloy_consensus::Header;
use tokio::runtime::Runtime;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, MemorizerError> {
        let target_block_number = key.block_number;
        let target_chain_id = key.chain_id;
        let header_key: MemorizerKey = key.into();

        // First check if the target value is already cached
        if let Some((MemorizerValue::Header(header_value), _)) = self.map.get(&header_key) {
            Ok(header_value.header.clone())
        } else {
            // If not, fetch from indexer
            let rt = Runtime::new()?;
            let block: IndexerRpc = rt.block_on(async {
                let client = IndexerClient::new(target_chain_id, target_chain_id);
                client
                    .get_header(target_block_number)
                    .await
                    .map_err(MemorizerError::ReqwestError)
            })?;
            let mmr: MmrMeta = block.meta.into();
            let header: Header = block.proofs[0].rlp_block_header.clone().into();

            self.map.insert(
                header_key,
                (
                    MemorizerValue::Header(HeaderMemorizerValue {
                        header: header.clone(),
                        element_index: block.proofs[0].element_index,
                        element_hash: block.proofs[0].element_hash,
                        rlp: block.proofs[0].rlp_block_header.string.clone(),
                        proof: block.proofs[0].siblings_hashes.clone(),
                    }),
                    false,
                ),
            );

            self.mmr_meta.insert(target_chain_id, mmr);

            Ok(header)
        }
    }
}
