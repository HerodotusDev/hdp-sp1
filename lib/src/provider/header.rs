use alloy_consensus::Header;
use alloy_primitives::{
    hex::{self, FromHex},
    B256,
};
use alloy_rlp::Decodable;
use serde::{Deserialize, Serialize};

use crate::mmr::MmrMeta;

/// Indexer RPC
/// Detail documentation: https://rs-indexer.api.herodotus.cloud/swagger/#/accumulators/get_proofs
const INDEXER_RPC_URL: &str = "https://rs-indexer.api.herodotus.cloud/accumulators/proofs";

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerResponse {
    pub data: Vec<IndexerRpc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexerRpc {
    pub meta: MmrRpc,
    pub proofs: Vec<HeaderRpc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MmrRpc {
    pub mmr_size: u128,
    pub mmr_id: String,
    pub mmr_root: String,
    pub mmr_peaks: Vec<String>,
    pub contract_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeaderRpc {
    pub element_index: u128,
    pub element_hash: B256,
    pub block_number: u128,
    pub rlp_block_header: RlpBlockHeader,
    pub siblings_hashes: Vec<B256>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RlpBlockHeader {
    pub string: String,
}

/// convert to block header
impl From<RlpBlockHeader> for Header {
    fn from(rlp_block_header: RlpBlockHeader) -> Self {
        let rlp_bytes = hex::decode(rlp_block_header.string).unwrap();
        let header: Header = Header::decode(&mut rlp_bytes.as_slice()).unwrap();
        header
    }
}

impl From<MmrRpc> for MmrMeta {
    fn from(mmr_rpc: MmrRpc) -> Self {
        let mmr_peaks: Vec<B256> = mmr_rpc
            .mmr_peaks
            .into_iter()
            .map(|peak| B256::from_hex(peak).unwrap())
            .collect();
        Self {
            root_hash: B256::from_hex(mmr_rpc.mmr_root).unwrap(),
            mmr_size: mmr_rpc.mmr_size,
            peaks: mmr_peaks,
        }
    }
}

pub struct IndexerClient {
    pub client: reqwest::Client,
    pub deployed_on_chain: u128,
    pub accumulates_chain: u128,
}

impl Default for IndexerClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            deployed_on_chain: 11155111,
            accumulates_chain: 11155111,
        }
    }
}

impl IndexerClient {
    pub async fn get_header(&self, block_number: u64) -> Result<IndexerRpc, reqwest::Error> {
        let url = format!(
            "{INDEXER_RPC_URL}?deployed_on_chain={}&accumulates_chain={}&hashing_function=keccak&contract_type=AGGREGATOR&block_numbers={}&is_meta_included=true&is_whole_tree=true&is_rlp_included=true",
            self.deployed_on_chain, self.accumulates_chain, block_number
        );
        let res = self.client.get(url).send().await?;
        let indexer_rpc: IndexerResponse = res.json().await?;
        Ok(indexer_rpc.data.first().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_header() {
        let client = IndexerClient::default();
        let indexer_rpc = client.get_header(665200).await.unwrap();
        let header: Header = indexer_rpc
            .proofs
            .first()
            .unwrap()
            .rlp_block_header
            .clone()
            .into();
        println!("{:#?}", header);
    }
}
