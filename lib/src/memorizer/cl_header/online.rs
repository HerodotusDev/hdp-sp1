use super::BeaconHeader;
use super::ClHeaderMemorizer;
use crate::memorizer::values::BeaconHeaderMemorizerValue;
use crate::memorizer::values::MemorizerValue;
use crate::memorizer::MemorizerError;
use crate::memorizer::{keys::BeaconHeaderKey, Memorizer};
use alloy_primitives::hex;
use ssz_rs::Vector;
use tokio::runtime::Runtime;

use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;

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

        let rt = Runtime::new().unwrap();
        let header: BeaconHeader = rt.block_on(async {
            let client = Client::new();

            let etherscan_url = format!(
                "https://{}etherscan.io/block/{}#consensusinfo",
                if key.chain_id == 11155111 {
                    "sepolia."
                } else {
                    ""
                },
                key.block_number
            );

            let response = reqwest::get(etherscan_url)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let document = Html::parse_document(&response);

            // Define a selector to target the div containing "Block proposed on slot"
            let selector = Selector::parse("#ContentPlaceHolder1_divhSlotEpoch .col-md-9").unwrap();

            let slot_text = document
                .select(&selector)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .concat();

            let slot = Regex::new(r"slot (\d+),")
                .unwrap()
                .captures(slot_text.as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            let slot = slot.parse::<u64>().unwrap();

            // if let Some(element) = document.select(&selector).next() {
            //     let slot_text = element.text().collect::<Vec<_>>().concat();
            //     println!("Block proposed on slot: {}", slot_text);
            // } else {
            //     println!("Could not find the block slot information.");
            // }

            // let slot = key.block_number / 32; // TODO fix this

            let url = format!(
                "{}/eth/v1/beacon/headers?slot={}",
                self.rpc_url.clone().unwrap(),
                slot
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
            }),
        );

        Ok(header)
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
