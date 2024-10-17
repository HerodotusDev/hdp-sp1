#![cfg_attr(target_os = "zkvm", no_main)]

use hdp_lib::memorizer::*;
use hdp_macro::hdp_main;

#[hdp_main]
pub fn main() {
    // ===============================================
    // Example program start
    // ===============================================

    let block_number: u64 = hdp::read();
    let chain_id: u64 = hdp::read();
    println!("Received block_number: {:?}", block_number);
    println!("Received chain_id: {:?}", chain_id);

    let header_key = HeaderKey {
        block_number,
        chain_id,
    };

    let _ = memorizer.get_header(header_key).unwrap();

    let header_key_plus_one = HeaderKey {
        block_number: block_number + 1,
        chain_id,
    };
    let v = memorizer.get_header(header_key_plus_one).unwrap();

    // TODO: to use CL header, provide RPC that support beacon header
    // let cl_header_key = BeaconHeaderKey {
    //     block_number,
    //     chain_id: 11155111,
    // };
    // let _ = memorizer.get_cl_header(cl_header_key).unwrap();

    hdp_commit(&v.beneficiary);

    // ===============================================
    // Example program end
    // ===============================================
}
