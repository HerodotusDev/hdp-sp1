use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hdp_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        use cfg_if::cfg_if;
        use hdp_lib::memorizer::{
            header::HeaderMemorizer,
            keys::{HeaderKey, TransactionKey},
            transaction::TransactionMemorizer,
            Memorizer,
        };

        cfg_if! {
            if #[cfg(target_os = "zkvm")] {
                sp1_zkvm::entrypoint!(main);
            } else {
                use std::{env, fs, path::Path, str::FromStr};
                use url::Url;
            }
        }

        #fn_vis #fn_sig {
            cfg_if! {
                if #[cfg(target_os = "zkvm")] {
                    println!("Hello, world! from zkvm");

                    // Read an input to the program.
                    let mut memorizer = sp1_zkvm::io::read::<Memorizer>();
                } else {
                    println!("Hello, world! from online mode");
                    let rpc_url: String = env::var("RPC_URL").expect("RPC_URL not set");
                    let mut memorizer = Memorizer::new(Some(Url::from_str(&rpc_url).unwrap()));
                }
            }

            // User's code block
            #fn_block

            cfg_if! {
                if #[cfg(target_os = "zkvm")] {
                    // Commit to the public values of the program.
                    // TODO:  Add a way to commit to the public values of the program.
                    println!("Done!");
                } else {
                    let manifest_dir: String = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
                    let path = Path::new(&manifest_dir).join("../memorizer.bin");
                    println!("Memorizer saved to {path:?}");
                    fs::write(path, bincode::serialize(&memorizer).unwrap()).unwrap();
                }
            }
        }
    };

    TokenStream::from(expanded)
}
