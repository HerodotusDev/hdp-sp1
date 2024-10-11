# Herodotus Data Processor (HDP) with SP1 Backend

> **Warning:** This codebase is experimental and not production ready 🚧

The Herodotus Data Processor (HDP) allows you to access verified on-chain data by verifying MMR (Merkle Mountain Range) and MPT (Merkle Patricia Tree) proofs in a zkVM environment. More about HDP can be found [here](https://docs.herodotus.dev/herodotus-docs/developers/data-processor).

## Example Program

We provide the `#[hdp_main]` macro to simplify the process of writing HDP programs. This macro handles conditional compilation based on the target OS and also supports conditional commits.

```rust
#![cfg_attr(target_os = "zkvm", no_main)]

use hdp_lib::memorizer::*;
use hdp_macro::hdp_main;

#[hdp_main]
pub fn main() {
    let block_number = 5_244_652;

    // Access header, account, storage, or transaction via key type
    let tx_key = TransactionKey {
        block_number,
        transaction_index: 0,
        ..Default::default()
    };
    let v = memorizer.get_transaction(tx_key).unwrap();

    // This function allows you to commit data to the zkVM.
    // If online, this will do nothing.
    // Note that you can only commit data that is serializable.
    commit(&v.tx_hash());
}
```

### hdp-sdk

We provide an SDK that wraps the SP1 client and abstracts away running SP1 programs in online mode (to get proofs) and zkVM mode (to verify proofs). You can use it like a normal SP1 client; however, in the program path, you provide an HDP program that uses the `#[hdp_main]` macro.

```rust
use hdp_sdk::DataProcessorClient;

fn main() {
    let client = DataProcessorClient::new();
    let (proof, vk) = client.prove("./program".into()).unwrap();
    client.verify(&proof, &vk).expect("failed to verify proof");
}
```

## Demo

![](.github/demo.gif)
