//! # hdp-lib
//!
//! header, transactions, receipt, accounts, and storage using Merkle proofs, MMR, MPT, and more.
//!
//! This crate is designed to work with zkVM environments and supports both zkVM and online modes,
//! Merkle proofs, and related structures.

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
