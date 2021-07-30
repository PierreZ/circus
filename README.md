# circus :circus_tent:
A toolkit to develop distributed systems

![status](https://img.shields.io/badge/status-experimental-red)
[![Build status](https://github.com/PierreZ/circus/workflows/Build%20and%20test/badge.svg)](https://github.com/PierreZ/circus/actions)
[![Coverage Status](https://coveralls.io/repos/github/PierreZ/circus/badge.svg?branch=main)](https://coveralls.io/github/PierreZ/circus?branch=main)
[![Dependency Status](https://deps.rs/repo/github/PierreZ/circus/status.svg)](https://deps.rs/repo/github/PierreZ/circus)
![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42.0+-lightgray.svg)](#rust-version-requirements)

## Overview

A crate that will provides a toolbox to create distributed systems in Rust.
Highly experimental for now, but the end-goal is to provide some of the helpers that the [FoundationDB's developers](https://www.foundationdb.org/) have:
* a [simulator](https://www.youtube.com/watch?v=4fFDFbi3toc)
* a [rpc framework](https://forums.foundationdb.org/t/why-was-flow-developed/1711/2) built using the simulator.
  
It will be compatible with both `async-std` and `Tokio`, allowing you to use Circus during development, then switch to your favorite runtime in production.

## Rust version requirements

The MSRV is Rust 1.42.0.

## Examples

Examples can be found in the [examples folder](simulation/examples).

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
