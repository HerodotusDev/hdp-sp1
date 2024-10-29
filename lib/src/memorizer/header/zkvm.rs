use super::HeaderMemorizer;
use crate::memorizer::{
    keys::{HeaderKey, MemorizerKey},
    values::MemorizerValue,
    Memorizer, MemorizerError,
};
use alloy_consensus::Header;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, MemorizerError> {
        let target_chain_id = key.chain_id;
        let header_key: MemorizerKey = key.into();

        if let Some((MemorizerValue::Header(header_value), is_verified)) =
            self.map.get_mut(&header_key)
        {
            if *is_verified {
                println!("Header MMR already verified");
                Ok(header_value.header.clone())
            } else {
                let mmr = &self.mmr_meta.get(&target_chain_id).unwrap();
                println!("cycle-tracker-start: mmr");
                mmr.verify_proof(
                    header_value.element_index,
                    header_value.element_hash,
                    header_value.proof.clone(),
                )?;
                println!("cycle-tracker-end: mmr");
                *is_verified = true;
                Ok(header_value.header.clone())
            }
        } else {
            Err(MemorizerError::MissingHeader)
        }
    }
}
