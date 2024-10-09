use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        pub mod mmr;
    } else {
        pub mod mmr;
        pub mod provider;

        pub use provider::*;
    }
}
