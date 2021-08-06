#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! A crate that will provides a toolbox to create distributed systems in Rust.
//! Highly experimental for now, but the end-goal is to provide some of the helpers that the [FoundationDB's developers](https://www.foundationdb.org/) have:
//! * a [simulator](https://www.youtube.com/watch?v=4fFDFbi3toc)
//! * a [rpc framework](https://forums.foundationdb.org/t/why-was-flow-developed/1711/2) built using the simulator
//! It will be compatible with both `async-std` and `Tokio`, allowing you to use Circus during development, then switch to your favorite runtime in production.
//!
//! Examples can be found in the [examples folder](https://github.com/PierreZ/circus/tree/main/simulation/examples).

use std::sync::Arc;

pub mod buggify;
pub mod deterministic;
