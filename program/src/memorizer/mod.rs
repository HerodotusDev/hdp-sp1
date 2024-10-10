pub mod account;
pub mod header;
pub mod keys;
pub mod storage;
pub mod transaction;
pub mod values;

use hdp_lib::mmr::MmrMeta;
use keys::MemorizerKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use values::MemorizerValue;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Memorizer {
    #[serde(skip)]
    pub rpc_url: Option<Url>,
    pub mmr_meta: Vec<MmrMeta>,
    pub map: HashMap<MemorizerKey, MemorizerValue>,
}

impl Memorizer {
    pub fn new(rpc_url: Option<Url>) -> Self {
        Self {
            rpc_url,
            mmr_meta: Vec::new(),
            map: Default::default(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<bincode::ErrorKind>> {
        bincode::deserialize(bytes)
    }

    pub fn as_bytes(&mut self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        bincode::serialize(&self)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Bytes, B256};
    use std::fs;
    use tempdir::TempDir;
    use values::{HeaderMemorizerValue, TransactionMemorizerValue};

    use super::*;

    #[test]
    fn test_memorizer() {
        let binding = TempDir::new("test").unwrap();
        let path = binding.path().join("memorizer.bin");

        let mut original_mem = Memorizer::new(None);
        original_mem.mmr_meta = vec![MmrMeta::default()];
        original_mem.map.insert(
            B256::ZERO,
            MemorizerValue::Header(HeaderMemorizerValue::default()),
        );
        let raw_tx = alloy_primitives::hex::decode("02f86f0102843b9aca0085029e7822d68298f094d9e1459a7a482635700cbc20bbaf52d495ab9c9680841b55ba3ac080a0c199674fcb29f353693dd779c017823b954b3c69dffa3cd6b2a6ff7888798039a028ca912de909e7e6cdef9cdcaf24c54dd8c1032946dfa1d85c206b32a9064fe8").unwrap();
        // let res = TxEnvelope::decode(&mut raw_tx.as_slice()).unwrap();
        original_mem.map.insert(
            B256::ZERO,
            MemorizerValue::Transaction(TransactionMemorizerValue {
                transaction_encoded: Bytes::from(raw_tx),
                tx_index: 0,
                proof: Default::default(),
            }),
        );

        fs::write(&path, original_mem.as_bytes().unwrap()).unwrap();

        let bytes = fs::read(path).unwrap();
        let mem = Memorizer::from_bytes(&bytes).unwrap();

        assert_eq!(original_mem, mem);
    }
}
