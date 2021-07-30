# circus :circus_tent:
A toolkit to develop distributed systems

[![Build status](https://github.com/PierreZ/circus/workflows/Build%20and%20test/badge.svg)](https://github.com/PierreZ/circus/actions)
[![Coverage Status](https://coveralls.io/repos/github/PierreZ/circus/badge.svg?branch=main)](https://coveralls.io/github/PierreZ/circus?branch=main)
[![Dependency Status](https://deps.rs/repo/github/PierreZ/circus/status.svg)](https://deps.rs/repo/github/PierreZ/circus)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

A crate that will provides a toolbox to create distributed systems in Rust.
Highly experimental for now, but the end-goal is to provide some of the helpers that the [FoundationDB's developers](https://www.foundationdb.org/) have:
* a [simulator](https://www.youtube.com/watch?v=4fFDFbi3toc)
* a [rpc framework](https://forums.foundationdb.org/t/why-was-flow-developed/1711/2) built using the simulator.
  
It will be compatible with both `async-std` and `Tokio`, allowing you to use Circus during development, then switch to your favorite runtime in production.

## Examples

