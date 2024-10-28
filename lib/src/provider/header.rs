use crate::{chain::ChainId, mmr::MmrMeta};
use alloy_consensus::Header;
use alloy_primitives::{
    hex::{self, FromHex},
    B256,
};
use alloy_rlp::Decodable;
use serde::{Deserialize, Serialize};

/// Indexer RPC
/// Detail documentation: https://rs-indexer.api.herodotus.cloud/swagger/#/accumulators/get_proofs
const INDEXER_RPC_URL: &str = "https://rs-indexer.api.herodotus.cloud/accumulators/proofs";

/// The response structure from the Indexer API.
/// Contains a list of [`IndexerRpc`] entries.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerResponse {
    data: Vec<IndexerRpc>,
}

/// Represents a complete response for a block from the Indexer,
/// containing MMR metadata and a list of header proofs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexerRpc {
    /// Metadata related to the MMR for the block.
    pub meta: MmrRpc,
    /// List of header proofs.
    pub proofs: Vec<HeaderRpc>,
}

/// Metadata from the MMR related to a specific block, as provided by the Indexer API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MmrRpc {
    mmr_size: u128,
    mmr_id: String,
    mmr_root: String,
    mmr_peaks: Vec<String>,
    contract_address: String,
}

/// Contains the header information for a specific block as well as
/// associated MMR proof data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeaderRpc {
    /// The element index of the block header in the MMR.
    pub element_index: u128,
    /// The hash of the block header element.
    pub element_hash: B256,
    /// The block number.
    pub block_number: u128,
    /// The RLP-encoded block header.
    pub rlp_block_header: RlpBlockHeader,
    /// Merkle Mountain Range inclusion proof.
    pub siblings_hashes: Vec<B256>,
}

/// Encapsulates an RLP-encoded block header as a single hex string.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RlpBlockHeader {
    /// The RLP-encoded block header in hex string format.
    pub string: String,
}

/// convert to block header
impl From<RlpBlockHeader> for Header {
    fn from(rlp_block_header: RlpBlockHeader) -> Self {
        let rlp_bytes = hex::decode(rlp_block_header.string).expect("Failed to decode hex");
        let header: Header =
            Header::decode(&mut rlp_bytes.as_slice()).expect("Failed to decode rlp");
        header
    }
}

impl From<MmrRpc> for MmrMeta {
    fn from(mmr_rpc: MmrRpc) -> Self {
        let mmr_peaks: Vec<B256> = mmr_rpc
            .mmr_peaks
            .into_iter()
            .map(|peak| B256::from_hex(peak).expect("Failed to parse hex"))
            .collect();
        Self::new(
            B256::from_hex(mmr_rpc.mmr_root).expect("Failed to parse hex"),
            mmr_rpc.mmr_size,
            mmr_peaks,
        )
    }
}

/// Client for interacting with the Indexer API to fetch block headers
/// and MMR-related data.
#[derive(Debug)]
pub struct IndexerClient {
    client: reqwest::Client,
    deployed_on_chain: u128,
    accumulates_chain: u128,
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
    /// Creates a new [`IndexerClient`] with specified source and destination chains.
    pub fn new(from_chain_id: ChainId, to_chain_id: ChainId) -> Self {
        Self {
            client: reqwest::Client::new(),
            deployed_on_chain: to_chain_id.to_numeric_id(),
            accumulates_chain: from_chain_id.to_numeric_id(),
        }
    }

    /// Fetches a block header and associated MMR proof data from the Indexer for a specific block.
    ///
    /// # Examples
    ///
    /// ```rust
    ///  use hdp_lib::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = IndexerClient::default();
    ///     match client.get_header(665200).await {
    ///         Ok(header_rpc) => println!("{:?}", header_rpc),
    ///         Err(e) => eprintln!("Error fetching header: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_header(&self, block_number: u64) -> Result<IndexerRpc, reqwest::Error> {
        let url = format!(
            "{INDEXER_RPC_URL}?deployed_on_chain={}&accumulates_chain={}&hashing_function=keccak&contract_type=AGGREGATOR&block_numbers={}&is_meta_included=true&is_whole_tree=true&is_rlp_included=true",
            self.deployed_on_chain, self.accumulates_chain, block_number
        );
        let res = self.client.get(url).send().await?;
        let indexer_rpc: IndexerResponse = res.json().await?;
        Ok(indexer_rpc.data.first().expect("Invalid response").clone())
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
