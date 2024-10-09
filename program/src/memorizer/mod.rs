pub mod account;
pub mod header;
pub mod keys;
pub mod storage;
pub mod values;

use hdp_lib::mmr::MmrMeta;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};
use url::Url;
use values::MemorizerValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct Memorizer {
    pub rpc_url: Option<Url>,
    pub mmr_meta: Option<MmrMeta>,
    pub map: HashMap<MemorizerKey, MemorizerValue>,
}

impl Memorizer {
    pub fn new(rpc_url: Option<Url>, mmr_meta: Option<MmrMeta>) -> Self {
        Self {
            rpc_url,
            mmr_meta,
            map: Default::default(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<bincode::ErrorKind>> {
        bincode::deserialize(bytes)
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        bincode::serialize(self)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MemorizerKey(pub [u8; 32]);

impl Deref for MemorizerKey {
    type Target = [u8; 32];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
