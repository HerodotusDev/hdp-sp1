//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![cfg_attr(target_os = "zkvm", no_main)]

pub mod memorizer;

use alloy_primitives::hex::FromHex;
use cfg_if::cfg_if;
use memorizer::{
    account::AccountMemorizer,
    cl_header::BeaconHeader,
    header::HeaderMemorizer,
    keys::{AccountKey, HeaderKey},
    Memorizer,
};
use ssz_rs::HashTreeRoot;
use url::Url;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);
    } else {
        use std::{env, fs, path::Path, str::FromStr};
    }
}

pub fn main() {
    cfg_if! {
        if #[cfg(target_os = "zkvm")] {
            println!("Hello, world! from zkvm");

            // Read an input to the program.
            //
            // Behind the scenes, this compiles down to a custom system call which handles reading inputs
            // from the prover.

            let mut memorizer = sp1_zkvm::io::read::<Memorizer>();
            println!("{memorizer:?}");
        } else {
            println!("Hello, world! from non zkvm");
            let rpc_url: String = env::var("RPC_URL").expect("RPC_URL not set");
            let mut memorizer = Memorizer::new(Some(Url::from_str(&rpc_url).unwrap()));
        }
    }

    let block_number = 5244652;

    let header_key = HeaderKey {
        block_number,
        ..Default::default()
    };

    memorizer.get_header(header_key);

    let account_key = AccountKey {
        block_number,
        address: alloy_primitives::Address::from_hex("0x7f2c6f930306d3aa736b3a6c6a98f512f74036d4")
            .unwrap(),
        ..Default::default()
    };

    let account = memorizer.get_account(account_key);
    println!("account {:?}", account.balance);

    // println!("memoizer is {:?}", memorizer.map);

    // SSZ shit
    let beacon_header = BeaconHeader {
        slot: alloy_primitives::U256::from(6050964),
        proposer_index: 35,
        parent_root: alloy_primitives::U256::from_be_bytes([
            0xa0, 0x62, 0xd3, 0x88, 0xca, 0x08, 0xf0, 0x99, 0xf4, 0x9e, 0x9c, 0xad, 0x64, 0xcb,
            0x62, 0xc1, 0x21, 0xe1, 0x00, 0x4b, 0x7b, 0xc0, 0x11, 0xc8, 0xe2, 0xca, 0x8c, 0x2e,
            0xed, 0x4e, 0x74, 0xdd,
        ]),
        state_root: alloy_primitives::U256::from_be_bytes([
            0x29, 0x63, 0xc1, 0x9c, 0xbd, 0x4b, 0x14, 0x82, 0x76, 0x5f, 0x90, 0xf7, 0x15, 0x48,
            0xf2, 0xf5, 0xd6, 0x82, 0x81, 0x2f, 0x94, 0xeb, 0xc0, 0xd8, 0x14, 0x4a, 0x58, 0xa1,
            0xab, 0x0e, 0xe7, 0x44,
        ]),
        body_root: alloy_primitives::U256::from_be_bytes([
            0xac, 0x4d, 0xc6, 0x37, 0x18, 0x65, 0xa1, 0xe7, 0x0a, 0x07, 0x7d, 0xdf, 0xfe, 0x51,
            0xd7, 0x22, 0xc2, 0xcd, 0x7f, 0xd2, 0x1f, 0x1b, 0x88, 0x85, 0x77, 0xfd, 0x6d, 0x0c,
            0xb5, 0x05, 0xfe, 0x88,
        ]),
    };
    let ssz_root = beacon_header.hash_tree_root().unwrap();

    println!("ssz_root {:?}", ssz_root);

    cfg_if! {
        if #[cfg(target_os = "zkvm")] {
            // Commit to the public values of the program. The final proof will have a commitment to all the
            // bytes that were committed to.

            println!("Done!");
        } else {
            let manifest_dir: String = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
            let path = Path::new(&manifest_dir).join("../memorizer.bin");
            println!("Memorizer saved to {path:?}");
            fs::write(path, memorizer.as_bytes().unwrap()).unwrap()
        }
    }
}
