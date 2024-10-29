use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use alloy_primitives::map::HashMap;

use crate::chain::ChainId;

/// Attempts to find the root directory of the current Cargo workspace by searching
/// upward from the directory specified in `CARGO_MANIFEST_DIR`.
///
/// This function will check for the presence of a `Cargo.toml` file containing a `[workspace]`
/// section, indicating the root of the workspace.
///
/// # Returns
/// `Some(PathBuf)` containing the path to the workspace root if found, or `None` if the
/// root cannot be determined.
///
/// # Panics
/// This function will panic if the `CARGO_MANIFEST_DIR` environment variable is not set.
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

/// Retrieves RPC URLs from environment variables and associates each URL with a specific [`ChainId`].
///
/// The function searches for environment variables prefixed with `RPC_URL_`
/// and parses them to identify which [`ChainId`] they correspond to.
/// If a valid [`ChainId`] is found and the URL is valid, the URL is added to the returned HashMap.
/// Invalid URLs or unrecognized `ChainId` strings result in an error message but are not added.
///
/// # Returns
/// A `HashMap<ChainId, url::Url>` containing each recognized [`ChainId`] as a key
/// and the corresponding RPC URL as the value.
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
