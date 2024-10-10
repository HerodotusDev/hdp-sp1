use super::{BeaconHeader, ClHeaderMemorizer};
use crate::memorizer::{keys::BeaconHeaderKey, Memorizer, MemorizerError};

impl ClHeaderMemorizer for Memorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> Result<BeaconHeader, MemorizerError> {
        println!("zkvm run");
        Ok(BeaconHeader::default())
    }
}
