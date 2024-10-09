use super::HeaderMemorizer;
use crate::memorizer::{keys::HeaderKey, Memorizer};
use alloy_consensus::Header;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Header {
        println!("zkvm run");

        Header::default()
    }
}
