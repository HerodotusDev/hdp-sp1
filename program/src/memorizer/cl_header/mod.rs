use ssz_rs::prelude::*;

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize, serde::Serialize, serde::Deserialize)]
pub struct BeaconHeader {
    pub slot: U256,
    pub proposer_index: u64,
    pub parent_root: U256,
    pub state_root: U256,
    pub body_root: U256,
}
