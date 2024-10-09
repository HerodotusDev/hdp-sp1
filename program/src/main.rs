#![cfg_attr(target_os = "zkvm", no_main)]

pub mod memorizer;

use alloy_primitives::hex::FromHex;
use cfg_if::cfg_if;
use memorizer::{
    account::AccountMemorizer,
    header::HeaderMemorizer,
    keys::{AccountKey, HeaderKey},
    Memorizer,
};
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
        } else {
            println!("Hello, world! from non zkvm");
            // let rpc_url: String = env::var("RPC_URL").expect("RPC_URL not set");
            let mut memorizer = Memorizer::new(Some(Url::from_str("https://sepolia.ethereum.iosis.tech/").unwrap()));
        }
    }

    let block_number = 5244652;

    let header_key = HeaderKey {
        block_number,
        ..Default::default()
    };

    let _ = memorizer.get_header(header_key).unwrap();
    let account_key = AccountKey {
        block_number,
        address: alloy_primitives::Address::from_hex("0x7f2c6f930306d3aa736b3a6c6a98f512f74036d4")
            .unwrap(),
        ..Default::default()
    };

    let _ = memorizer.get_account(account_key);
    //  println!("account {:?}", account.balance);

    println!("memoizer is {:?}", memorizer);

    cfg_if! {
        if #[cfg(target_os = "zkvm")] {
            // Commit to the public values of the program. The final proof will have a commitment to all the
            // bytes that were committed to.
            // TODO: need to properly commit arbitrary data from program
            println!("Done!");
        } else {
            let manifest_dir: String = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
            let path = Path::new(&manifest_dir).join("../memorizer.bin");
            println!("Memorizer saved to {path:?}");
            fs::write(path, memorizer.as_bytes().unwrap()).unwrap()
        }
    }
}
