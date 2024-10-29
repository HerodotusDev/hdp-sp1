//! # hdp-lib
//!
//! ![CI](https://img.shields.io/github/actions/workflow/status/HerodotusDev/hdp-sp1/prove.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)
//!
//! `hdp-lib` is a Rust library that enhances off-chain compute capabilities using zkVMs (Zero-Knowledge Virtual Machines) for verifiable on-chain data integration. It supports headers, accounts, storage, transactions, and receipts using MMR and MPT Merkle proofs.
//!
//! This crate is designed to work with zkVM environments and supports both zkVM and online modes for pre-processing inclusion proofs and verifying proofs.
//!
//! ## Installation
//!
//! Add the `hdp-lib` dependency to your project:
//!
//! ```toml
//! [dependencies]
//! hdp-lib = { git = "https://github.com/HerodotusDev/hdp-sp1.git"}
//! ```
//!
//! ## Usage
//!
//! Define an HDP program with the `hdp_main` macro and the memorizer provided by `hdp-lib`.
//!
//! ```rust
//! #![cfg_attr(target_os = "zkvm", no_main)]
//!
//! use alloy_primitives::{address, U256};
//! use hdp_lib::memorizer::*;
//! use hdp_macro::hdp_main;
//!
//! #[hdp_main(to_chain_id = "ETHEREUM_SEPOLIA")]
//! pub fn main() {
//!     // ===============================================
//!     // Example program start
//!     // ===============================================
//!
//!     let storage_key = StorageKey {
//!         block_number: 5_244_634,
//!         address: address!("7f2c6f930306d3aa736b3a6c6a98f512f74036d4"),
//!         chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
//!         storage_slot: U256::from(1).into(),
//!     };
//!     let v = memorizer.get_storage(storage_key).unwrap();
//!
//!     hdp_commit(&v);
//!
//!     // ===============================================
//!     // Example program end
//!     // ===============================================
//! }
//! ```

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]

use cfg_if::cfg_if;

/// Defines various chain types, such as different blockchain networks
/// and utilities to identify them.
pub mod chain;

/// Defines the `Memorizer` and related traits for handling various types of data.
/// This includes consensus layer headers, transactions, accounts, and storage proofs.
pub mod memorizer;

/// Defines the MMR (Merkle Mountain Range) type and related utilities,
/// used for managing and verifying data in an append-only tree structure.
pub mod mmr;

/// Defines the MPT (Merkle Patricia Trie) type, primarily used in Ethereum
/// for efficient state and proof management.
pub mod mpt;

// Conditional compilation based on the operating system target.
cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        // zkVM-specific configurations and modules could be defined here.
    } else {
        /// Utility functions for various tasks such as fetching data
        /// from the network or handling environment configurations.
        pub mod utils;

        /// Defines providers for interacting with RPC endpoints and fetching
        /// blockchain-related data such as transactions and state proofs.
        pub mod provider;

        // Exposes provider module contents at the crate level for easier access.
        pub use provider::*;
    }
}
