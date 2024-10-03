//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![cfg_attr(target_os = "zkvm", no_main)]
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);
    } else {
        use reqwest::blocking;
    }
}

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // let _memorizer = sp1_zkvm::io::read::<Memorizer>();

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

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&[10]);
}
