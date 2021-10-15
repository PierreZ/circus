# Circus_test :circus_tent:

![status](https://img.shields.io/badge/status-experimental-red)
[![Crates.io Version](https://img.shields.io/crates/v/circus_test.svg)](https://crates.io/crates/circus_test)
[![Docs.rs](https://img.shields.io/docsrs/circus_test)](https://docs.rs/circus_test)
[![Build status](https://github.com/PierreZ/circus/workflows/Build%20and%20test/badge.svg)](https://github.com/PierreZ/circus/actions)
![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.51.0+-lightgray.svg)](#rust-version-requirements)

Allow injection of a random seed upon a test. Can be overloaded with environment var `DETERMINISTIC_SEED`.

## Example:
```rust
use circus_test::with_random_seed;
#[with_random_seed]
#[test]
fn random_seed(seed: u64) {
    println!("{}", seed);
}
```

## Rust version requirements

The MSRV is Rust 1.51.0.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
