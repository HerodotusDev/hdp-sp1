use super::BeaconHeader;
use super::ClHeaderMemorizer;
use crate::memorizer::values::BeaconHeaderMemorizerValue;
use crate::memorizer::values::MemorizerValue;
use crate::memorizer::{keys::BeaconHeaderKey, Memorizer};
use alloy_primitives::U256 as U256Alloy;
use tokio::runtime::Runtime;

use reqwest::Client;
use serde::Deserialize;

impl ClHeaderMemorizer for Memorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> BeaconHeader {
        let rt = Runtime::new().unwrap();
        let header: BeaconHeader = rt.block_on(async {
            let client = Client::new();
            let url = format!(
                "{}/eth/v1/beacon/headers?slot={}",
                self.rpc_url.clone().unwrap(),
                key.slot
            );

            // Sending GET request to the specified URL
            let response = client
                .get(&url)
                .header("accept", "application/json")
                .send()
                .await
                .expect("Failed to send request")
                .json::<BeaconHeaderApiResponse>()
                .await
                .expect("Failed to parse response");

            // Extracting the first header in the `data` array
            let header_data = &response.data[0].header.message;

            // Converting response data to BeaconHeader struct
            BeaconHeader {
                slot: U256Alloy::from_str_radix(&header_data.slot, 10).unwrap(),
                proposer_index: header_data.proposer_index.parse().unwrap(),
                parent_root: U256Alloy::from_str_radix(&header_data.parent_root[2..], 16).unwrap(),
                state_root: U256Alloy::from_str_radix(&header_data.state_root[2..], 16).unwrap(),
                body_root: U256Alloy::from_str_radix(&header_data.body_root[2..], 16).unwrap(),
            }
        });
        self.map.insert(
            key.into(),
            MemorizerValue::BeaconHeader(BeaconHeaderMemorizerValue {
                header: header.clone(),
                proof: vec![],
            }),
        );
        header
    }
}

#[derive(Deserialize)]
struct BeaconHeaderApiResponse {
    data: Vec<BeaconHeaderData>,
}

#[derive(Deserialize)]
struct BeaconHeaderData {
    header: BeaconHeaderDetail,
}

#[derive(Deserialize)]
struct BeaconHeaderDetail {
    message: BeaconHeaderMessage,
}

#[derive(Deserialize)]
struct BeaconHeaderMessage {
    slot: String,
    proposer_index: String,
    parent_root: String,
    state_root: String,
    body_root: String,
}
