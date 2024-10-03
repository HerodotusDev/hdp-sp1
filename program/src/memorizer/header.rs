use super::{keys::HeaderKey, Memorizer};
use alloy_primitives::U256;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        // PLACEHOLDER
    } else {
        use reqwest::blocking;
    }
}

pub trait HeaderMemorizer {
    fn get_header(self, key: HeaderKey) -> U256;
}

impl HeaderMemorizer for Memorizer {
    fn get_header(self, key: HeaderKey) -> U256 {
        cfg_if! {
            if #[cfg(target_os = "zkvm")] {
                println!("zkvm run");
            } else {
                println!("online run");
                 // Define the URL you want to request
                let url = "https://jsonplaceholder.typicode.com/posts/1";

                // Make a GET request
                let response = blocking::get(url).unwrap();

                // Check if the request was successful
                if response.status().is_success() {
                    // Parse and print the response body
                    let body = response.text().unwrap();
                    println!("Response: {}", body);
                } else {
                    println!("Request failed with status: {}", response.status());
                }
            }
        }

        U256::from(0)
    }
}
