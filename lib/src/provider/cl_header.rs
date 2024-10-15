use alloy_primitives::hex;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Deserialize;
use ssz_rs::Vector;

use crate::memorizer::{BeaconHeader, BeaconHeaderKey};

#[derive(Deserialize, Debug)]
struct BeaconHeaderApiResponse {
    data: Vec<BeaconHeaderData>,
}

#[derive(Deserialize, Debug)]
struct BeaconHeaderData {
    header: BeaconHeaderDetail,
}

#[derive(Deserialize, Debug)]
struct BeaconHeaderDetail {
    message: BeaconHeaderMessage,
}

#[derive(Deserialize, Debug)]
struct BeaconHeaderMessage {
    slot: String,
    proposer_index: String,
    parent_root: String,
    state_root: String,
    body_root: String,
}

pub struct BeaconHeaderClient {
    pub client: reqwest::Client,
}

impl Default for BeaconHeaderClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BeaconHeaderClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub async fn get_cl_header(
        &self,
        rpc_url: String,
        key: &BeaconHeaderKey,
    ) -> Result<BeaconHeader, reqwest::Error> {
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

        println!("slot: {}", slot);

        // if let Some(element) = document.select(&selector).next() {
        //     let slot_text = element.text().collect::<Vec<_>>().concat();
        //     println!("Block proposed on slot: {}", slot_text);
        // } else {
        //     println!("Could not find the block slot information.");
        // }

        // let slot = key.block_number / 32; // TODO fix this

        let url = format!("{}/eth/v1/beacon/headers?slot={}", rpc_url.clone(), slot);
        println!("url: {}", url);

        // Sending GET request to the specified URL
        let response = self
            .client
            .get(&url)
            .header("accept", "application/json")
            .send()
            .await
            .expect("Failed to send request")
            .json::<BeaconHeaderApiResponse>()
            .await
            .expect("Failed to parse response");

        println!("response: {:?}", response.data);

        // Extracting the first header in the `data` array
        let header_data = &response.data[0].header.message;

        let parent_root =
            Vector::<u8, 32>::try_from(hex::decode(&header_data.parent_root).unwrap()).unwrap();
        let state_root =
            Vector::<u8, 32>::try_from(hex::decode(&header_data.state_root).unwrap()).unwrap();
        let body_root =
            Vector::<u8, 32>::try_from(hex::decode(&header_data.body_root).unwrap()).unwrap();
        // Converting response data to BeaconHeader struct
        Ok(BeaconHeader {
            slot: header_data.slot.parse().unwrap(),
            proposer_index: header_data.proposer_index.parse().unwrap(),
            parent_root,
            state_root,
            body_root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "CL support public rpc"]
    async fn test_get_cl_header() {
        let beacon_header_client = BeaconHeaderClient::default();

        let key = BeaconHeaderKey {
            chain_id: 11155111,
            block_number: 5244652,
        };

        // TODO: need public rpc url for beacon chain https://ethereum-sepolia-beacon-api.publicnode.com/eth/v1/beacon/headers?slot=4304885
        let res = beacon_header_client
            .get_cl_header(
                "https://ethereum-sepolia-beacon-api.publicnode.com".to_string(),
                &key,
            )
            .await
            .unwrap();
        println!("{:?}", res);
    }
}
