#![cfg_attr(target_os = "zkvm", no_main)]

use alloy_primitives::{address, U256};
use hdp_lib::memorizer::*;
use hdp_macro::hdp_main;

#[hdp_main(to_chain_id = "ETHEREUM_SEPOLIA")]
pub fn main() {
    // ===============================================
    // Example program start
    // ===============================================

    // let block_number: u64 = hdp::read();
    // let chain_id: u64 = hdp::read();
    // println!("Received block_number: {:?}", block_number);
    // println!("Received chain_id: {:?}", chain_id);

    let header_key = HeaderKey {
        block_number: 5244634,
        chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
    };

    let v = memorizer.get_header(header_key).unwrap();

    let account_key = AccountKey {
        block_number: 5244634,
        address: address!("7f2c6f930306d3aa736b3a6c6a98f512f74036d4"),
        chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
    };
    let _ = memorizer.get_account(account_key).unwrap();

    let storage_key = StorageKey {
        block_number: 5244634,
        address: address!("7f2c6f930306d3aa736b3a6c6a98f512f74036d4"),
        chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
        storage_slot: U256::from(1).into(),
    };
    let _ = memorizer.get_storage(storage_key).unwrap();

    // let tx_key = TransactionKey {
    //     block_number: 5244634,
    //     chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
    //     transaction_index: 2,
    // };
    // let _ = memorizer.get_transaction(tx_key).unwrap();

    // let receipt_key = ReceiptKey {
    //     block_number: 5244634,
    //     chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
    //     transaction_index: 2,
    // };
    // let _ = memorizer.get_receipt(receipt_key).unwrap();
    // TODO: to use CL header, provide RPC that support beacon header
    // let cl_header_key = BeaconHeaderKey {
    //     block_number,
    //     chain_id: hdp_lib::chain::ChainId::EthereumSepolia,
    // };
    // let _ = memorizer.get_cl_header(cl_header_key).unwrap();

    // commit the value would like to get onchain
    hdp_commit(&v.hash_slow().0);

    // ===============================================
    // Example program end
    // ===============================================
}
