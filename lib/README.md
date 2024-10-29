# hdp-lib

![CI](https://img.shields.io/github/actions/workflow/status/HerodotusDev/hdp-sp1/prove.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)
[![Crates.io](https://img.shields.io/crates/v/hdp-lib?style=flat-square&logo=lootcrate)](https://crates.io/crates/hdp-lib)
[![Documentation](https://img.shields.io/docsrs/hdp-lib)](https://docs.rs/hdp-lib)

`hdp-lib` is a Rust library that enhances off-chain compute capabilities using zkVMs (Zero-Knowledge Virtual Machines) for verifiable on-chain data integration. It supports headers, accounts, storage, transactions, and receipts using MMR and MPT Merkle proofs.

This crate is designed to work with zkVM environments and supports both zkVM and online modes for pre-processing inclusion proofs and verifying proofs.

## Installation

Add the `hdp-lib` dependency to your project:

```toml
[dependencies]
hdp-lib = "0.1.0"
```

## Usage

Define an HDP program with the `hdp_main` macro and the memorizer provided by `hdp-lib`.

```rust
#![cfg_attr(target_os = "zkvm", no_main)]

use alloy_primitives::{address, U256};
use hdp_lib::memorizer::*;
use hdp_macro::hdp_main;

#[hdp_main(to_chain_id = "ETHEREUM_SEPOLIA")]
pub fn main() {
    // ===============================================
    // Example program start
    // ===============================================

    let storage_key = StorageKey {
        block_number: 5_244_634,
        address: address!("7f2c6f930306d3aa736b3a6c6a98f512f74036d4"),
        chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
        storage_slot: U256::from(1).into(),
    };
    let v = memorizer.get_storage(storage_key).unwrap();

    hdp_commit(&v);

    // ===============================================
    // Example program end
    // ===============================================
}
```
