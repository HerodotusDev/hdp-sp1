use super::{BeaconHeader, ClHeaderMemorizer};
use crate::memorizer::{
    keys::{BeaconHeaderKey, HeaderKey, MemorizerKey},
    values::MemorizerValue,
    Memorizer, MemorizerError,
};
use ssz_rs::HashTreeRoot;

impl ClHeaderMemorizer for Memorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> Result<BeaconHeader, MemorizerError> {
        let header_key: MemorizerKey = HeaderKey {
            block_number: key.block_number + 1,
            chain_id: key.chain_id,
        }
        .into();

        if let Some(MemorizerValue::Header(header_value)) = self.map.get(&header_key) {
            let beacon_root = header_value.header.parent_beacon_block_root.unwrap();

            if let Some(MemorizerValue::BeaconHeader(beacon_header_value)) =
                self.map.get(&MemorizerKey::from(key))
            {
                let ssz_root = beacon_header_value.header.hash_tree_root().unwrap();
                if beacon_root == ssz_root {
                    return Ok(beacon_header_value.header.clone());
                } else {
                    println!(
                        "Mismatched beacon root: beacon root: {:?}, ssz root: {:?}",
                        beacon_root, ssz_root
                    );
                }
            }
        } else {
            println!("Missing header, {:?}", key.block_number);
            return Err(MemorizerError::MissingHeader);
        }

        println!("zkvm run");
        Ok(BeaconHeader::default())
    }
}
