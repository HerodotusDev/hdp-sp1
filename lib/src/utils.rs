use std::env;
use std::fs;
use std::path::PathBuf;

pub fn find_workspace_root() -> Option<PathBuf> {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
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

pub fn get_rpc_url() -> url::Url {
    dotenv::dotenv().ok();
    let url_str = env::var("RPC_URL").expect("RPC_URL must be set");
    url::Url::parse(&url_str).unwrap()
}
