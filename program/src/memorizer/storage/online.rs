use super::StorageMemorizer;
use crate::memorizer::{keys::StorageKey, Memorizer};
use alloy_primitives::U256;
use alloy_rpc_client::{ClientBuilder, ReqwestClient};

impl StorageMemorizer for Memorizer {
    fn get_storage(&self, key: StorageKey) -> U256 {
        let client: ReqwestClient = ClientBuilder::default().http(self.rpc_url.to_owned().unwrap());
        U256::from(0)
    }
}
