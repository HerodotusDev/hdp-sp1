use cfg_if::cfg_if;

pub mod chain;
pub mod memorizer;
pub mod mmr;
pub mod mpt;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
    } else {
        pub mod utils;
        pub mod provider;

        pub use provider::*;
    }
}
