//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS LZ4 Compressor - Ultra-Fast SIMD Compression Engine
//! 🛸 LZ4 Frame Format + Async Compression + Streaming + Zero-copy
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    compression::{Compressor, CompressionConfig, CompressionAlgorithm, Error, Result},
};
use lz4_flex::{block::{compress_prepend_size, decompress_size_prepended}, frame::{FrameCompressor, FrameDecompressor}};
use std::io::{Cursor, Read, Write};
use tokio::task;
use tracing::{debug};

/// LZ4 compression engine implementation
pub struct Lz4Compressor {
    config: CompressionConfig,
}

impl Lz4Compressor {
    /// Forge new LZ4 compressor
    pub fn new(config: CompressionConfig) -> Self {
        Self { config }
    }

    /// Synchronous compress helper (blocking)
    fn compress_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Using frame compression for compatibility and streaming support
        let mut encoder = FrameCompressor::new(Vec::new());
        encoder.write_all(data).map_err(|_| Error::Lz4)?;
        let compressed = encoder.finish().map_err(|_| Error::Lz4)?;
        Ok(compressed)
    }

    /// Synchronous decompress helper (blocking)
    fn decompress_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = FrameDecompressor::new(Cursor::new(data));
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|_| Error::Lz4)?;
        Ok(decompressed)
    }
}

#[async_trait::async_trait]
impl Compressor for Lz4Compressor {
    /// Async compress with tokio blocking task
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < self.config.min_size {
            // Return uncompressed for small data
            return Ok(data.to_vec());
        }
        let compressor = self.clone();
        task::spawn_blocking(move || compressor.compress_sync(data))
            .await
            .map_err(|_| Error::Lz4)?
    }

    /// Async decompress with tokio blocking task
    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let decompressor = self.clone();
        task::spawn_blocking(move || decompressor.decompress_sync(data))
            .await
            .map_err(|_| Error::Lz4)?
    }

    /// Estimate compression ratio (fast heuristic)
    fn compress_ratio(&self, data: &[u8]) -> f32 {
        if data.is_empty() {
            return 1.0;
        }
        // Typical LZ4 compression ratio around 1.5-2.0 for compressible data
        1.5
    }

    /// Compressor algorithm type
    fn algorithm(&self) -> CompressionAlgorithm {
        CompressionAlgorithm::Lz4
    }

    /// Return compressor config
    fn config(&self) -> &CompressionConfig {
        &self.config
    }
}

impl Clone for Lz4Compressor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_lz4_compress_decompress() {
        let config = CompressionConfig {
            min_size: 1,
            ..Default::default()
        };
        let compressor = Lz4Compressor::new(config);
        
        let data = b"Quantum compression test data repeated quantum quantum quantum!";
        
        let compressed = compressor.compress(data).await.unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = compressor.decompress(&compressed).await.unwrap();
        assert_eq!(decompressed, data);
    }
    
    #[tokio::test]
    async fn test_small_data_passthrough() {
        let compressor = Lz4Compressor::new(Default::default());
        let data = b"A";  // 1 byte
        let compressed = compressor.compress(data).await.unwrap();
        assert_eq!(compressed, data);
    }
}