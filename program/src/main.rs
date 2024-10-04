//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![cfg_attr(target_os = "zkvm", no_main)]

pub mod memorizer;

use alloy_primitives::{hex::FromHex, B256};
use cfg_if::cfg_if;
use memorizer::{
    account::AccountMemorizer,
    header::HeaderMemorizer,
    keys::{AccountKey, HeaderKey, StorageKey},
    storage::StorageMemorizer,
    Memorizer,
};
use std::str::FromStr;
use url::Url;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);
    } else {
        use std::env;
    }
}

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // let _memorizer = sp1_zkvm::io::read::<Memorizer>();

    cfg_if! {
        if #[cfg(target_os = "zkvm")] {
            println!("Hello, world! from zkvm");
            let mut memorizer = Memorizer::new(None);
        } else {
            println!("Hello, world! from non zkvm");
            let mut memorizer = Memorizer::new(Some(Url::from_str(env::var("RPC_URL").expect("RPC_URL not set").as_ref()).unwrap()));
        }
    }

    let block_number = 100;

    let header_key = HeaderKey {
        block_number,
        ..Default::default()
    };

    memorizer.get_header(header_key);

    let account_key = AccountKey {
        block_number,
        address: alloy_primitives::Address::from_hex("0x0").unwrap(),
        ..Default::default()
    };

    let account = memorizer.get_account(account_key);
    println!("account {:?}", account);

    let storage_key = StorageKey {
        block_number,
        address: alloy_primitives::Address::from_hex("0x0").unwrap(),
        storage_slot: B256::from_hex("0x0").unwrap(),
        ..Default::default()
    };

    let slot = memorizer.get_storage(storage_key);
    println!("slot {:?}", slot);

    println!("memoizer is {:?}", memorizer.map);

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&[10]);
}
