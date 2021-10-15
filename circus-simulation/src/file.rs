//! File module
use crate::deterministic::fs::file::SimulatedFile;
use enum_dispatch::enum_dispatch;

/// File trait
#[enum_dispatch(File)]
pub trait FileTrait {}

/// Enum for the File trait
#[enum_dispatch]
pub enum File {
    /// A simulated file
    SimulatedFile,
}
