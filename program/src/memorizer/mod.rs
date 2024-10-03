pub mod account;
pub mod header;
pub mod keys;
pub mod storage;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Memorizer {
    pub rpc_url: Option<Url>,
    pub map: HashMap<MemorizerKey, Proof>,
}

impl Memorizer {
    pub fn new(rpc_url: Option<Url>) -> Self {
        Self {
            rpc_url,
            map: Default::default(),
        }
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

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Proof(pub Vec<u8>);

impl Deref for Proof {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
