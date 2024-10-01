pub mod keys;

use std::collections::HashMap;

pub type Memorizer = HashMap<MemorizerKey, Vec<u8>>;

#[derive(Debug, Default)]
pub struct MemorizerKey(pub [u8; 32]);
