use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        pub mod mmr;
        pub mod mpt;
    } else {
        pub mod mmr;
        pub mod mpt;
        pub mod provider;

        pub use provider::*;
    }
}
