//! Platform module
use crate::deterministic::platform::SimulationPlatform;
use crate::file::File;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::path::Path;
use std::{io, time};

/// Main trait for platform
#[async_trait]
#[enum_dispatch(PlatformProvider)]
pub trait Platform {
    /// open a file
    async fn open(&mut self, path: &Path) -> io::Result<File>;

    /// return the current time
    fn now(&self) -> time::Instant;
}

/// Enum of the available platform providers
#[enum_dispatch]
pub enum PlatformProvider {
    /// Simulated platform
    SimulationPlatform,
}
