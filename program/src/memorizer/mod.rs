pub mod header;
pub mod keys;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Memorizer(pub HashMap<MemorizerKey, Proof>);

impl Deref for Memorizer {
    type Target = HashMap<MemorizerKey, Proof>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
