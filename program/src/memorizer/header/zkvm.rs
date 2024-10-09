use super::HeaderMemorizer;
use crate::memorizer::{keys::HeaderKey, keys::MemorizerKey, values::MemorizerValue, Memorizer};
use alloy_consensus::Header;
use std::error::Error;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, Box<dyn Error>> {
        println!("zkvm run");

        let header_key: MemorizerKey = key.into();

        if let MemorizerValue::Header(header_value) = self.map.get(&header_key).unwrap() {
            println!("Got a HeaderMemorizerValue: {:?}", header_value);
            let mmr = &self.mmr_meta[0];
            mmr.verify_proof(
                header_value.element_index,
                header_value.element_hash,
                header_value.proof.clone(),
            )
            .unwrap();
            Ok(header_value.header.clone())
        } else {
            Err("Header not found".into())
        }
    }
}
