#![cfg_attr(target_os = "zkvm", no_main)]

use cfg_if::cfg_if;
use hdp_lib::memorizer::{
    header::HeaderMemorizer,
    keys::{HeaderKey, TransactionKey},
    transaction::TransactionMemorizer,
    Memorizer,
};

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);
    } else {
        use std::{env, fs, path::Path, str::FromStr};
        use url::Url;
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
            println!("Hello, world! from online mode");
            let rpc_url: String = env::var("RPC_URL").expect("RPC_URL not set");
            let mut memorizer = Memorizer::new(Some(Url::from_str(&rpc_url).unwrap()));
        }
    }

    let block_number = 5244652;

    let header_key = HeaderKey {
        block_number,
        ..Default::default()
    };

    let _ = memorizer.get_header(header_key).unwrap();

    let tx_key = TransactionKey {
        block_number,
        transaction_index: 0,
        ..Default::default()
    };

    let _ = memorizer.get_transaction(tx_key);

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
            fs::write(path, bincode::serialize(&memorizer).unwrap()).unwrap()
        }
    }
}
