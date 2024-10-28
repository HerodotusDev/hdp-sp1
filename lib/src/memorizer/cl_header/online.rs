use super::BeaconHeader;
use super::ClHeaderMemorizer;
use crate::chain::ChainId;
use crate::cl_header::BeaconHeaderClient;
use crate::memorizer::values::BeaconHeaderMemorizerValue;
use crate::memorizer::values::MemorizerValue;
use crate::memorizer::MemorizerError;
use crate::memorizer::{keys::BeaconHeaderKey, Memorizer};
use tokio::runtime::Runtime;

const SEPOLIA_POS_TRANSITION_BLOCK_NUMBER: u64 = 1450409;
const MAINNET_POS_TRANSITION_BLOCK_NUMBER: u64 = 15537393;

impl ClHeaderMemorizer for Memorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> Result<BeaconHeader, MemorizerError> {
        // Validate that the block number is greater than the POS transition block number
        match key.chain_id {
            ChainId::EthereumMainnet => {
                if key.block_number < MAINNET_POS_TRANSITION_BLOCK_NUMBER {
                    return Err(MemorizerError::InvalidPoSBlockNumber);
                }
            }
            ChainId::EthereumSepolia => {
                if key.block_number < SEPOLIA_POS_TRANSITION_BLOCK_NUMBER {
                    return Err(MemorizerError::InvalidPoSBlockNumber);
                }
            }
        }

        let rpc_url = self
            .chain_map
            .get(&key.chain_id)
            .ok_or(MemorizerError::MissingRpcUrl(key.chain_id))?;

        let rt = Runtime::new()?;
        let header: BeaconHeader = rt.block_on(async {
            let client = BeaconHeaderClient::default();
            client
                .get_cl_header(rpc_url.to_string(), &key)
                .await
                .unwrap()
        });

        self.map.insert(
            key.into(),
            (
                MemorizerValue::BeaconHeader(BeaconHeaderMemorizerValue {
                    header: header.clone(),
                }),
                false,
            ),
        );

        Ok(header)
    }
}
