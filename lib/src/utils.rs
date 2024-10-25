use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use alloy_primitives::map::HashMap;

use crate::chain::ChainId;

pub fn find_workspace_root() -> Option<PathBuf> {
    let mut dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    loop {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(contents) = fs::read_to_string(&cargo_toml) {
                if contents.contains("[workspace]") {
                    return Some(dir);
                }
            }
        }
        if !dir.pop() {
            // Reached the root directory without finding a workspace root
            return None;
        }
    }
}

pub fn get_rpc_urls() -> HashMap<ChainId, url::Url> {
    dotenv::dotenv().ok();
    let mut rpc_urls = HashMap::new();

    for (key, value) in env::vars() {
        if let Some(chain_id_str) = key.strip_prefix("RPC_URL_") {
            if let Ok(chain_id) = ChainId::from_str(chain_id_str) {
                if let Ok(parsed_url) = url::Url::parse(&value) {
                    rpc_urls.insert(chain_id, parsed_url);
                } else {
                    eprintln!("Invalid URL for {}: {}", key, value);
                }
            } else {
                eprintln!("Invalid chain ID in key: {}", key);
            }
        }
    }

    rpc_urls
}
