# Herodotus Data Processor (HDP) with SP1 Backend

> [!WARNING]
> This codebase is experimental and not production-ready 🚧

The Herodotus Data Processor (HDP) allows you to access verified on-chain data by verifying MMR (Merkle Mountain Range) and MPT (Merkle Patricia Tree) proofs in a zkVM environment. More about HDP can be found [here](https://docs.herodotus.dev/herodotus-docs/developers/data-processor).

## Architecture

- [`hdp-macro`](./hdp-macro/): A macro to simplify HDP programs for SP1's online and zkVM modes.
- [`hdp-lib`](./lib/): The core library for HDP programs, including providers, memorizer, verifier, etc.
- [`hdp-sdk`](./hdp-sdk/): `DataProcessorClient` to wrap the SP1 client and handle HDP's full flow.

## Supported Memorizers

- [x] Header
- [x] Transaction
- [x] Consensus Header
- [x] Account
- [x] Storage
- [x] Receipt

## Performance

| Operation                     | Clock Cycle | Code                                              |
| ----------------------------- | ----------- | ------------------------------------------------- |
| **MMR Verification(header)**  | 625,471     | [code](./lib/src/memorizer/header/zkvm.rs)        |
| **MPT Verification(tx)**      | 136,951     | [code](./lib/src/memorizer/transaction/zkvm.rs)   |
| **MPT Verification(receipt)** | 172,726     | [code](./lib/src/memorizer/receipt/zkvm.rs)       |
| **MPT Verification(account)** | 514,710     | [code](./lib/src/memorizer/account/zkvm.rs)       |
| **MPT Verification(storage)** | 18,790      | [code](./lib/src/memorizer/storage/zkvm.rs)       |
| **Bloom Filter (Set)**        | 17,656      | [code](./examples/compliance/program/src/main.rs) |
| **Bloom Filter (Check)**      | 20,119      | [code](./examples/compliance/program/src/main.rs) |

## Run Example

This command will run the [simple example](./examples/simple/README.md). It will first run the HDP program in online mode to get proofs. Then it will run the HDP program in zkVM mode to generate an ELF file. This ELF file will then be used to generate a proof and verify it.

```
cargo run --package simple --bin simple --release +nightly
```

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

## hdp-sdk

We provide an SDK that wraps the SP1 client and abstracts away running SP1 programs in online mode (to get proofs) and zkVM mode (to verify proofs). You can use it like a normal SP1 client; however, in the program path, you provide an HDP program that uses the `#[hdp_main]` macro.

```rust
use hdp_sdk::DataProcessorClient;

fn main() {
    let client = DataProcessorClient::new();
    let (proof, vk) = client.prove("./program".into()).unwrap();
    client.verify(&proof, &vk).expect("failed to verify proof");
}
```

### Pass input to HDP program

If you want to pass some input to the HDP program, you can use `write` method :

```rust
use hdp_sdk::DataProcessorClient;

fn main() {
    let mut client = DataProcessorClient::new();
    client.write(5244652_u64);
    let (proof, vk) = client.prove("./program".into()).unwrap();
    client.verify(&proof, &vk).expect("failed to verify proof");
}
```

Note that you need to read the input from the HDP program if you want to use it like this:

```rust
#[hdp_main]
pub fn main() {
   let block_number: u64 = hdp::read();
   println!("Received block_number: {:?}", block_number);
}

```
