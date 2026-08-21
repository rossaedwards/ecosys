//! Quantum I/O - Sacred Geometry File Operations
pub mod fux;
pub mod quantum_stream;

pub use fux::{read_fux, write_fux, FuxFile};
pub use quantum_stream::QuantumStream;

use crate::core::lattice::Lattice;
use std::path::Path;

pub fn quantum_read<T>(path: impl AsRef<Path>) -> Result<Lattice<T>, IoError> {
    let data = std::fs::read(path)?;
    let lattice: Lattice<T> = bincode::deserialize(&data)?;
    Ok(lattice)
}

pub fn quantum_write<T>(lattice: &Lattice<T>, path: impl AsRef<Path>) -> Result<(), IoError> {
    let data = bincode::serialize(lattice)?;
    std::fs::write(path, data)?;
    Ok(())
}

#[derive(Debug)]
pub enum IoError {
    Bincode(bincode::Error),
    Io(std::io::Error),
}
impl From<bincode::Error> for IoError {
    fn from(err: bincode::Error) -> Self {
        IoError::Bincode(err)
    }
}
impl From<std::io::Error> for IoError {
    fn from(err: std::io::Error) -> Self {
        IoError::Io(err)
    }
}
Compare this snippet from fuxyez/fuxrt/core/lattice.rs:
}
