//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![cfg_attr(target_os = "zkvm", no_main)]

pub mod memorizer;

use std::str::FromStr;

use cfg_if::cfg_if;
use memorizer::{
    account::AccountMemorizer,
    header::HeaderMemorizer,
    keys::{AccountKey, HeaderKey, StorageKey},
    storage::StorageMemorizer,
    Memorizer,
};
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

    memorizer.get_header(HeaderKey::default());
    let my_account = memorizer.get_account(AccountKey::default());
    println!("my account is {:?}", my_account);
    println!("memoizer is {:?}", memorizer.map);
    memorizer.get_storage(StorageKey::default());

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&[10]);
}
