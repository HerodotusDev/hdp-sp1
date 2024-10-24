# Herodotus Data Processor (HDP) with SP1 Backend

> [!WARNING]
> This codebase is experimental and not production-ready 🚧

The Herodotus Data Processor (HDP) enables you to access verified on-chain data by verifying Merkle Mountain Range (MMR) and Merkle Patricia Tree (MPT) proofs in a zkVM environment. Learn more about HDP [here](https://docs.herodotus.dev/herodotus-docs/developers/data-processor).

## Architecture

- [`hdp-macro`](./hdp-macro/): A macro to simplify HDP programs for SP1's online and zkVM modes.
- [`hdp-lib`](./lib/): The core library for HDP programs, including providers, memorizer, verifier, etc.
- [`hdp-sdk`](./hdp-sdk/): The `DataProcessorClient`, which wraps the SP1 client and handles HDP's full flow.

## Supported Memorizers

- [x] Header
- [x] Transaction
- [x] Consensus Header
- [x] Account
- [x] Storage
- [x] Receipt

## Performance

M2 MAX / 12 core - (todo: will update numbers with proper metrics)

| Operation                          | Clock Cycles | Code                                            |
| ---------------------------------- | ------------ | ----------------------------------------------- |
| **MMR Verification (header)**      | 625,471      | [code](./lib/src/memorizer/header/zkvm.rs)      |
| **MPT Verification (transaction)** | 136,951      | [code](./lib/src/memorizer/transaction/zkvm.rs) |
| **MPT Verification (receipt)**     | 172,726      | [code](./lib/src/memorizer/receipt/zkvm.rs)     |
| **MPT Verification (account)**     | 514,710      | [code](./lib/src/memorizer/account/zkvm.rs)     |
| **MPT Verification (storage)**     | 18,790       | [code](./lib/src/memorizer/storage/zkvm.rs)     |

We also checked other operations (wip):

| Operation                            | Clock Cycles | Code                                              |
| ------------------------------------ | ------------ | ------------------------------------------------- |
| **Bloom Filter - 3 address (Set)**   | 17,656       | [code](./examples/compliance/program/src/main.rs) |
| **Bloom Filter - 3 address (Check)** | 20,119       | [code](./examples/compliance/program/src/main.rs) |

## Running Examples

Before running the examples, ensure that you have set the necessary environment variables for online mode to fetch proofs.

For example, use the following format to define RPC providers in a `.env` file:

```
# Required for online mode in .env
RPC_URL_ETHEREUM_SEPOLIA=
RPC_URL_ETHEREUM_MAINNET=
```

The following command runs the [simple example](./examples/simple/README.md). It first runs the HDP program in online mode to retrieve proofs, and then runs the HDP program in zkVM mode to generate an ELF file. This ELF file is used to generate a proof and verify it.

```
cargo run --package simple --bin simple --release +nightly
```

## Example Program

We provide the `#[hdp_main]` macro to simplify writing HDP programs. This macro handles conditional compilation based on the target OS and supports conditional commits.

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
        chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
    };
    let v = memorizer.get_transaction(tx_key).unwrap();

    // This function commits data to the zkVM.
    // If online, it will do nothing.
    // Only serializable data can be committed.
    commit(&v.tx_hash());
}
```

## HDP SDK

We provide an SDK that wraps the SP1 client and abstracts the process of running SP1 programs in online mode (to retrieve proofs) and zkVM mode (to verify proofs). You can use it like a regular SP1 client, but in the program path, you provide an HDP program that utilizes the `#[hdp_main]` macro.

```rust
use hdp_sdk::DataProcessorClient;

fn main() {
    let client = DataProcessorClient::new();
    let (proof, vk) = client.prove("./program".into()).unwrap();
    client.verify(&proof, &vk).expect("Failed to verify proof");
}
```

### Passing Input to an HDP Program

To pass input to the HDP program, use the `write` method:

```rust
use hdp_sdk::DataProcessorClient;

fn main() {
    let mut client = DataProcessorClient::new();
    client.write(5_244_652_u64);
    let (proof, vk) = client.prove("./program".into()).unwrap();
    client.verify(&proof, &vk).expect("Failed to verify proof");
}
```

You need to read the input within the HDP program as follows:

```rust
#[hdp_main]
pub fn main() {
    let block_number: u64 = hdp::read();
    println!("Received block number: {:?}", block_number);
}
```
