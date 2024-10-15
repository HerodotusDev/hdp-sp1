use super::BeaconHeader;
use super::ClHeaderMemorizer;
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
            1 => {
                if key.block_number < MAINNET_POS_TRANSITION_BLOCK_NUMBER {
                    return Err(MemorizerError::InvalidPoSBlockNumber);
                }
            }
            11155111 => {
                if key.block_number < SEPOLIA_POS_TRANSITION_BLOCK_NUMBER {
                    return Err(MemorizerError::InvalidPoSBlockNumber);
                }
            }
            _ => {
                return Err(MemorizerError::UnknownBaseChainId);
            }
        }

        let rpc_url = self.rpc_url.clone().unwrap();

        let rt = Runtime::new().unwrap();
        let header: BeaconHeader = rt.block_on(async {
            let client = BeaconHeaderClient::default();
            client
                .get_cl_header(rpc_url.to_string(), &key)
                .await
                .unwrap()
        });

        self.map.insert(
            key.into(),
            MemorizerValue::BeaconHeader(BeaconHeaderMemorizerValue {
                header: header.clone(),
            }),
        );

        Ok(header)
    }
}
