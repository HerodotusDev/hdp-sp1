use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    to_chain_id: String,
}

#[proc_macro_attribute]
pub fn hdp_main(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    let hdp_read_fn = quote! {
        mod hdp {
            use serde::{Serialize, de::DeserializeOwned};
            cfg_if::cfg_if! {
                if #[cfg(target_os = "zkvm")] {
                    pub fn read<T: DeserializeOwned>() -> T {
                        sp1_zkvm::io::read::<T>()
                    }
                } else {
                    use std::io::{self, Read};
                    use std::fmt::Debug;
                    pub fn read<T: DeserializeOwned + Debug>() -> T {
                        let stdin = io::stdin();
                        let mut reader = stdin.lock();
                        let deserialized_value: T = bincode::deserialize_from(&mut reader).expect("Failed to deserialize input");
                        deserialized_value
                    }
                }
            }
        }
    };

    let to_chain_id = args.to_chain_id;

    let expanded = quote! {
        use cfg_if::cfg_if;
        use hdp_lib::memorizer::Memorizer;
        use serde::Serialize;
        use alloy_primitives::Bytes;

        cfg_if! {
            if #[cfg(target_os = "zkvm")] {
                sp1_zkvm::entrypoint!(main);

                use hdp_lib::chain::ChainId;
                use hdp_lib::memorizer::PublicValuesStruct;
                use core::str::FromStr;
                use serde::Deserialize;
                use alloy_sol_types::SolValue;

            } else {
                use hdp_lib::utils::find_workspace_root;
                use hdp_lib::utils::get_rpc_urls;
                use std::{env, fs, path::Path, str::FromStr};
                use url::Url;
            }
        }

        #hdp_read_fn

        #fn_vis #fn_sig {
            cfg_if! {
                if #[cfg(target_os = "zkvm")] {
                    println!("Hello, world! from zkvm");

                    // Read an input to the program.
                    let mut memorizer = sp1_zkvm::io::read::<Memorizer>();
                } else {
                    println!("Hello, world! from online mode");
                    let chain_map = get_rpc_urls();
                    let mut memorizer = Memorizer::new(chain_map, #to_chain_id);
                }
            }

             // Variable to store the value passed to hdp_commit
             let mut hdp_commit_value: Option<Bytes> = None;

             // Define hdp_commit closure
             let mut hdp_commit = |value: &[u8]| {
                 cfg_if! {
                     if #[cfg(target_os = "zkvm")] {
                         hdp_commit_value = Some(Bytes::from(value.to_vec()));
                     } else {
                         // No-op in online mode
                     }
                 }
             };

            // User's code block
            #fn_block

            cfg_if! {
                if #[cfg(target_os = "zkvm")] {
                    if let Some(result_value) = hdp_commit_value {
                        let mmr_meta = memorizer.mmr_meta.get(&ChainId::from_str(#to_chain_id).unwrap()).expect("MMR metadata not found");

                        let public_values = PublicValuesStruct {
                            mmrId: mmr_meta.mmr_id,
                            mmrSize: mmr_meta.mmr_size,
                            mmrRoot: mmr_meta.root_hash,
                            result: result_value.into(),
                        };

                        // Commit the public values
                        sp1_zkvm::io::commit_slice(&public_values.abi_encode());
                    } else {
                        panic!("hdp_commit was not called with a value");
                    }
                } else {
                    let workspace_root = find_workspace_root().expect("Workspace root not found");
                    let path = workspace_root.join("memorizer.bin");
                    println!("Memorizer saved to {path:?}");
                    if cfg!(debug_assertions) {
                        println!("Memorizer: {:#?}", memorizer);
                    }
                    fs::write(path, bincode::serialize(&memorizer).unwrap()).unwrap();
                }
            }
        }
    };

    TokenStream::from(expanded)
}
