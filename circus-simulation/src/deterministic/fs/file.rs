//! Simulated file module

#[allow(dead_code)]
/// Simulation implementation of a file.
pub struct SimulatedFile {
    file: std::fs::File,
}

impl SimulatedFile {
    /// creates a `SimulatedFile`
    pub fn new(file: std::fs::File) -> Self {
        SimulatedFile { file }
    }
}
