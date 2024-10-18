#![cfg_attr(target_os = "zkvm", no_main)]

use alloy_consensus::Transaction;
use alloy_primitives::keccak256;
use ethbloom::{Bloom, Input};
use hdp_lib::memorizer::*;
use hdp_macro::hdp_main;

const SANCTIONED_ADDRESS: [[u8; 20]; 3] = [
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01,
    ],
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02,
    ],
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x03,
    ],
];

#[hdp_main]
pub fn main() {
    // ===============================================
    // Example program start
    // ===============================================

    let block_number: u64 = hdp::read();
    let transaction_length: u64 = hdp::read();
    println!("Received block_number: {:?}", block_number);
    println!("Received transaction_length: {:?}", transaction_length);

    // ================================================
    // Initialize Bloom Filter
    // ================================================

    println!("cycle-tracker-start: bloom default init");
    let mut my_bloom = Bloom::default();
    println!("cycle-tracker-end: bloom default init");
    for i in SANCTIONED_ADDRESS {
        println!("cycle-tracker-start: bloom accrue init");
        my_bloom.accrue(Input::Raw(&i));
        println!("cycle-tracker-end:  bloom accrue init");
    }

    let header_key = HeaderKey {
        block_number: 5244652,
        ..Default::default()
    };

    let _ = memorizer.get_header(header_key).unwrap();

    for i in 0..transaction_length {
        let tx_key = TransactionKey {
            block_number: 5244652,
            transaction_index: i,
            ..Default::default()
        };
        let tx = memorizer.get_transaction(tx_key).unwrap();
        println!("cycle-tracker-start: bloom check");
        let might_be_sanctioned =
            my_bloom.contains_input(Input::Raw(tx.recover_signer().unwrap().as_ref()));
        println!("cycle-tracker-end: bloom check");
        if might_be_sanctioned {
            if let Some(receiver) = tx.to().to() {
                println!(
                    "Found a sanctioned address in transaction receiver: {:?}",
                    receiver
                );
                println!("cycle-tracker-start: bloom set");
                my_bloom.accrue(Input::Raw(receiver.as_ref()));
                println!("cycle-tracker-end:  bloom set");
            }
        }
    }

    let bloom_commit = keccak256(my_bloom.as_bytes());

    // This function allow you to commit data to the zkvm.
    // If online, this will do nothing.
    // Note that you can only commit data that is serializable.
    hdp_commit(&bloom_commit);

    // println!("memorizer is {:?}", memorizer);

    // ===============================================
    // Example program end
    // ===============================================
}
