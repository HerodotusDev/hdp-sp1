use super::HeaderMemorizer;
use crate::memorizer::{keys::HeaderKey, Memorizer};
use alloy_primitives::U256;
use alloy_rpc_client::{ClientBuilder, ReqwestClient};

impl HeaderMemorizer for Memorizer {
    fn get_header(&self, key: HeaderKey) -> U256 {
        let client: ReqwestClient = ClientBuilder::default().http(self.rpc_url.to_owned().unwrap());
        U256::from(0)
    }
}
