use super::{BeaconHeader, ClHeaderMemorizer};
use crate::memorizer::{
    keys::{BeaconHeaderKey, HeaderKey, MemorizerKey},
    values::MemorizerValue,
    HeaderMemorizer, Memorizer, MemorizerError,
};
use ssz_rs::HashTreeRoot;

impl ClHeaderMemorizer for Memorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> Result<BeaconHeader, MemorizerError> {
        // 1. Header
        let header_key = HeaderKey {
            block_number: key.block_number + 1,
            chain_id: key.chain_id,
        };
        let header = self.get_header(header_key)?;

        // 2. CL Header
        let beacon_root = header.parent_beacon_block_root.unwrap();

        if let Some((MemorizerValue::BeaconHeader(beacon_header_value), is_verified)) =
            self.map.get_mut(&MemorizerKey::from(key))
        {
            if *is_verified {
                println!("CL Header already verified");
                Ok(beacon_header_value.header.clone())
            } else {
                println!("cycle-tracker-start: beacon header hash");
                let ssz_root = beacon_header_value.header.hash_tree_root().unwrap();
                println!("cycle-tracker-end: beacon header hash");
                if beacon_root == ssz_root {
                    *is_verified = true;
                    Ok(beacon_header_value.header.clone())
                } else {
                    println!(
                        "Mismatched beacon root: beacon root: {:?}, ssz root: {:?}",
                        beacon_root, ssz_root
                    );
                    Err(MemorizerError::InvalidBeaconRoot)
                }
            }
        } else {
            Err(MemorizerError::MissingBeaconRoot)
        }
    }
}
