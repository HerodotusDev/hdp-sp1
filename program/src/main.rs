//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![cfg_attr(target_os = "zkvm", no_main)]

pub mod memorizer;

use cfg_if::cfg_if;
use memorizer::{
    account::AccountMemorizer,
    header::HeaderMemorizer,
    keys::{AccountKey, HeaderKey, StorageKey},
    storage::StorageMemorizer,
    Memorizer,
};

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);
    }
}

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // let _memorizer = sp1_zkvm::io::read::<Memorizer>();

    let memorizer = Memorizer::default();
    memorizer.get_header(HeaderKey::default());
    memorizer.get_account(AccountKey::default());
    memorizer.get_storage(StorageKey::default());

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&[10]);
}
