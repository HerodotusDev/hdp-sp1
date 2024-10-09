use cfg_if::cfg_if;
use ssz_rs::prelude::*;

use crate::memorizer::keys::BeaconHeaderKey;

#[derive(
    PartialEq, Eq, Debug, Default, SimpleSerialize, serde::Serialize, serde::Deserialize, Clone,
)]
pub struct BeaconHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: Vector<u8, 32>,
    pub state_root: Vector<u8, 32>,
    pub body_root: Vector<u8, 32>,
}

pub trait ClHeaderMemorizer {
    fn get_cl_header(&mut self, key: BeaconHeaderKey) -> BeaconHeader;
}

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        mod zkvm;
    } else {
        mod online;
    }
}
