use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hdp_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    let commit_fn = quote! {
        #[cfg(target_os = "zkvm")]
        fn hdp_commit<T: serde::Serialize>(value: &T) {
            sp1_zkvm::io::commit(value);
        }

        #[cfg(not(target_os = "zkvm"))]
        fn hdp_commit<T>(_value: &T) {
            // No-op in online mode
        }
    };

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

    let expanded = quote! {
        use cfg_if::cfg_if;
        use hdp_lib::memorizer::Memorizer;
        use serde::Serialize;

        cfg_if! {
            if #[cfg(target_os = "zkvm")] {
                sp1_zkvm::entrypoint!(main);
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
                    let mut memorizer = Memorizer::new(chain_map);
                }
            }

            // Conditional commit
            #commit_fn

            // User's code block
            #fn_block

            cfg_if! {
                if #[cfg(target_os = "zkvm")] {
                } else {
                    let workspace_root = find_workspace_root().expect("Workspace root not found");
                    let path = workspace_root.join("memorizer.bin");
                    println!("Memorizer saved to {path:?}");
                    fs::write(path, bincode::serialize(&memorizer).unwrap()).unwrap();
                }
            }
        }
    };

    TokenStream::from(expanded)
}
