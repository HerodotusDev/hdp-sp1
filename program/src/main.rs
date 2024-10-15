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
        chain_id: 11155111,
    };

    let _ = memorizer.get_header(header_key).unwrap();

    let header_key_plus_one = HeaderKey {
        block_number: block_number + 1,
        chain_id: 11155111,
    };
    let _ = memorizer.get_header(header_key_plus_one).unwrap();

    // TODO: to use CL header, provide RPC that support beacon header
    // let cl_header_key = BeaconHeaderKey {
    //     block_number,
    //     chain_id: 11155111,
    // };
    // let _ = memorizer.get_cl_header(cl_header_key).unwrap();

    println!("memorizer is {:?}", memorizer);

    // ===============================================
    // Example program end
    // ===============================================
}
