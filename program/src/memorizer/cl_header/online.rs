use super::BeaconHeader;
use super::ClHeaderMemorizer;
use crate::memorizer::values::BeaconHeaderMemorizerValue;
use crate::memorizer::values::MemorizerValue;
use crate::memorizer::{keys::BeaconHeaderKey, Memorizer};
use alloy_primitives::hex;
use alloy_primitives::U256 as U256Alloy;
use ssz_rs::Vector;
use tokio::runtime::Runtime;

use reqwest::Client;
use serde::Deserialize;

impl ClHeaderMemorizer for Memorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> BeaconHeader {
        let rt = Runtime::new().unwrap();
        let header: BeaconHeader = rt.block_on(async {
            let client = Client::new();

            println!("slot: {:?}", key.slot);
            let url = format!(
                "{}/eth/v1/beacon/headers?slot={}",
                self.rpc_url.clone().unwrap(),
                key.slot
            );

            println!("url: {:?}", url);

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

            let parent_root =
                Vector::<u8, 32>::try_from(hex::decode(&header_data.parent_root).unwrap()).unwrap();
            let state_root =
                Vector::<u8, 32>::try_from(hex::decode(&header_data.state_root).unwrap()).unwrap();
            let body_root =
                Vector::<u8, 32>::try_from(hex::decode(&header_data.body_root).unwrap()).unwrap();
            // Converting response data to BeaconHeader struct
            BeaconHeader {
                slot: header_data.slot.parse().unwrap(),
                proposer_index: header_data.proposer_index.parse().unwrap(),
                parent_root,
                state_root,
                body_root,
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
