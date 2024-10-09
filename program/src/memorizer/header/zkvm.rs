use super::HeaderMemorizer;
use crate::memorizer::{keys::HeaderKey, values::MemorizerValue, Memorizer};
use alloy_consensus::Header;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Header {
        println!("zkvm run");

        if let MemorizerValue::Header(header_value) = self.map.get(&key.into()).unwrap() {
            println!("Got a HeaderMemorizerValue: {:?}", header_value);
        } else {
            println!("MemorizerValue is not a Header variant");
        };

        Header::default()
    }
}
