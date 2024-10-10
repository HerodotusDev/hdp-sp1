use super::HeaderMemorizer;
use crate::memorizer::{
    keys::{HeaderKey, MemorizerKey},
    values::MemorizerValue,
    Memorizer, MemorizerError,
};
use alloy_consensus::Header;

impl HeaderMemorizer for Memorizer {
    fn get_header(&mut self, key: HeaderKey) -> Result<Header, MemorizerError> {
        let header_key: MemorizerKey = key.into();

        if let Some(MemorizerValue::Header(header_value)) = self.map.get(&header_key) {
            let mmr = &self.mmr_meta[0];
            println!("cycle-tracker-start: mmr");
            mmr.verify_proof(
                header_value.element_index,
                header_value.element_hash,
                header_value.proof.clone(),
            )?;
            println!("cycle-tracker-end: mmr");
            println!("Header MMR verified successfully");
            Ok(header_value.header.clone())
        } else {
            Err(MemorizerError::MissingHeader)
        }
    }
}
