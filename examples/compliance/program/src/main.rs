#![cfg_attr(target_os = "zkvm", no_main)]

use hdp_lib::memorizer::*;
use hdp_macro::hdp_main;

#[hdp_main]
pub fn main() {
    // ===============================================
    // Example program start
    // ===============================================

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

    let v = memorizer.get_transaction(tx_key).unwrap();

    // This function allow you to commit data to the zkvm.
    // If online, this will do nothing.
    // Note that you can only commit data that is serializable.
    hdp_commit(&v.tx_hash());

    // println!("memorizer is {:?}", memorizer);

    // ===============================================
    // Example program end
    // ===============================================
}
