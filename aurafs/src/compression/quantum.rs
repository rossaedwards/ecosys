//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Quantum Compressor - Post-Quantum Wavelet + Entropy Encoding
//! 🛸 Quantum Wavelet Transforms + Lattice-aware Sparse Encoding + Neural Compression
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    compression::{Compressor, CompressionConfig, CompressionAlgorithm, Error, Result},
    compression::lattice::LatticeCompressor,
};
use std::sync::Arc;
use tokio::task;
use tracing::{debug, info};

/// High-level quantum compressor combining wavelet + lattice + neural entropies
pub struct QuantumCompressor {
    config: CompressionConfig,
    lattice_compressor: LatticeCompressor,
    // Future: Neural entropy model, wavelet transform parameters
}

impl QuantumCompressor {
    /// Forge new quantum compressor with lattice base
    pub fn new(config: CompressionConfig) -> Self {
        let lattice_compressor = LatticeCompressor::new(config.clone());
        Self {
            config,
            lattice_compressor,
        }
    }

    /// Post-quantum wavelet plus lattice compression logic
    fn compress_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < self.config.min_size {
            return Ok(data.to_vec());
        }

        // Step 1: Wavelet transform (Haar/Quantum Wavelet)
        let wavelet_data = self.quantum_wavelet_transform(data);

        // Step 2: Lattice compression on transformed data
        let compressed = self.lattice_compressor.compress_sync(&wavelet_data)?;
        
        // Step 3: Optional neural entropy coding (future)
        Ok(compressed)
    }

    /// Decompression with inverse wavelet after lattice decode
    fn decompress_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Step 1: Lattice decompression
        let decompressed = self.lattice_compressor.decompress_sync(data)?;
        
        // Step 2: Inverse wavelet transform
        let inverse_data = self.inverse_quantum_wavelet(&decompressed);
        
        // Step 3: Neural entropy decoding (future)
        Ok(inverse_data)
    }

    /// Simplified Haar wavelet transform (1D discrete)
    fn quantum_wavelet_transform(&self, data: &[u8]) -> Vec<u8> {
        let mut transformed = Vec::with_capacity(data.len());
        
        let mut i = 0;
        while i + 1 < data.len() {
            let avg = ((data[i] as u16 + data[i+1] as u16) / 2) as u8;
            let diff = (((data[i] as i16) - (data[i+1] as i16)) / 2).abs() as u8;
            transformed.push(avg);
            transformed.push(diff);
            i += 2;
        }
        if i < data.len() {
            transformed.push(data[i]);
        }
        transformed
    }

    /// Inverse Haar wavelet transform
    fn inverse_quantum_wavelet(&self, data: &[u8]) -> Vec<u8> {
        let mut original = Vec::with_capacity(data.len());
        
        let mut i = 0;
        while i + 1 < data.len() {
            let avg = data[i] as i16;
            let diff = data[i+1] as i16;
            let left = avg + diff;
            let right = avg - diff;
            original.push(left as u8);
            original.push(right as u8);
            i += 2;
        }
        if i < data.len() {
            original.push(data[i]);
        }
        original
    }
}

#[async_trait::async_trait]
impl Compressor for QuantumCompressor {
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let compressor = self.clone();
        task::spawn_blocking(move || compressor.compress_sync(data))
            .await
            .map_err(|_| Error::Quantum)?
    }

    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let decompressor = self.clone();
        task::spawn_blocking(move || decompressor.decompress_sync(data))
            .await
            .map_err(|_| Error::Quantum)?
    }

    fn compress_ratio(&self, data: &[u8]) -> f32 {
        self.lattice_compressor.compress_ratio(data)
    }

    fn algorithm(&self) -> CompressionAlgorithm {
        CompressionAlgorithm::Quantum
    }

    fn config(&self) -> &CompressionConfig {
        &self.config
    }
}

impl Clone for QuantumCompressor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            lattice_compressor: self.lattice_compressor.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quantum_roundtrip() {
        let config = CompressionConfig {
            min_size: 1,
            ..Default::default()
        };
        let compressor = QuantumCompressor::new(config);
        
        let data = b"Quantum state data with high entropy and structure, ideal for wavelet compression!";
        
        let compressed = compressor.compress(data).await.unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = compressor.decompress(&compressed).await.unwrap();
        assert_eq!(decompressed[..data.len()], data[..]);
    }
    
    #[test]
    fn test_wavelet_transform() {
        let compressor = QuantumCompressor::new(Default::default());
        let data = vec![10, 20, 30, 40];
        let transformed = compressor.quantum_wavelet_transform(&data);
        let reconstructed = compressor.inverse_quantum_wavelet(&transformed);
        assert_eq!(reconstructed[..data.len()], data[..]);
    }
}