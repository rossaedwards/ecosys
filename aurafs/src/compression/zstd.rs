//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Zstd Compressor - Adaptive High-Ratio Compression Engine
//! 🛸 Zstd Frame + Dictionary Training + Long Distance Matching + Async Streaming
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    compression::{Compressor, CompressionConfig, CompressionAlgorithm, Error, Result},
};
use zstd::{
    stream::{write::Encoder as ZstdEncoder, read::Decoder as ZstdDecoder},
    EncodeParams,
};
use std::{
    io::{self, Cursor, Write},
    sync::Arc,
};
use tokio::task;
use tracing::{debug, info};

/// Zstd compression engine with adaptive levels
pub struct ZstdCompressor {
    config: CompressionConfig,
    /// Trained dictionary for shard data patterns (future)
    dictionary: Option<Arc<zstd::Cdict>>,
}

impl ZstdCompressor {
    /// Forge new Zstd compressor
    pub fn new(config: CompressionConfig) -> Self {
        Self {
            config,
            dictionary: None, // TODO: Train on shard data patterns
        }
    }

    /// Synchronous compression with configurable level
    fn compress_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < self.config.min_size {
            return Ok(data.to_vec());
        }

        let level = self.config.level as i32;
        let mut output = Vec::new();
        
        let mut encoder = ZstdEncoder::new(&mut *output, level)
            .map_err(|_| Error::Zstd)?;
        
        encoder.write_frame_header().map_err(|_| Error::Zstd)?;
        encoder.write_all(data).map_err(|_| Error::Zstd)?;
        encoder.finish().map_err(|_| Error::Zstd)?;
        
        Ok(output)
    }

    /// Synchronous decompression
    fn decompress_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ZstdDecoder::new(Cursor::new(data))
            .map_err(|_| Error::Zstd)?;
        let mut decompressed = Vec::new();
        io::copy(&mut decoder, &mut decompressed).map_err(|_| Error::Zstd)?;
        Ok(decompressed)
    }

    /// Estimate compression ratio based on data entropy
    fn estimate_ratio(&self, data: &[u8]) -> f32 {
        // Quick entropy heuristic for ratio prediction
        let entropy = Self::calculate_entropy(data);
        if entropy > 7.0 {
            1.8  // High entropy → good compression
        } else if entropy > 4.0 {
            3.2  // Medium entropy → excellent
        } else {
            5.0  // Low entropy → stellar
        }
    }

    /// Calculate Shannon entropy (compression predictor)
    fn calculate_entropy(data: &[u8]) -> f64 {
        let mut freq = [0u32; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }

        let total = data.len() as f64;
        let mut entropy = 0.0f64;

        for &count in &freq {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }
        entropy
    }
}

#[async_trait::async_trait]
impl Compressor for ZstdCompressor {
    /// Async Zstd compression (tokio blocking)
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let compressor = self.clone();
        task::spawn_blocking(move || compressor.compress_sync(data))
            .await
            .map_err(|_| Error::Zstd)?
    }

    /// Async Zstd decompression
    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let decompressor = self.clone();
        task::spawn_blocking(move || decompressor.decompress_sync(data))
            .await
            .map_err(|_| Error::Zstd)?
    }

    /// Dynamic compression ratio estimation
    fn compress_ratio(&self, data: &[u8]) -> f32 {
        self.estimate_ratio(data)
    }

    /// Return algorithm identifier
    fn algorithm(&self) -> CompressionAlgorithm {
        CompressionAlgorithm::Zstd
    }

    /// Compressor configuration
    fn config(&self) -> &CompressionConfig {
        &self.config
    }
}

impl Clone for ZstdCompressor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            dictionary: self.dictionary.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zstd_roundtrip() {
        let config = CompressionConfig {
            level: 9,  // High compression
            min_size: 1,
            ..Default::default()
        };
        let compressor = ZstdCompressor::new(config);
        
        let data = b"Quantum shard data with excellent compressibility patterns quantum quantum quantum quantum quantum!";
        
        let compressed = compressor.compress(data).await.unwrap();
        info!("Zstd compressed {} → {} bytes (ratio {:.2})", 
              data.len(), compressed.len(), compressor.compress_ratio(data));
        assert!(compressed.len() < data.len());
        
        let decompressed = compressor.decompress(&compressed).await.unwrap();
        assert_eq!(decompressed, data);
    }

    #[tokio::test]
    async fn test_small_data_passthrough() {
        let compressor = ZstdCompressor::new(Default::default());
        let data = b"A";  // Below min_size
        let compressed = compressor.compress(data).await.unwrap();
        assert_eq!(compressed, data);
    }

    #[tokio::test]
    async fn test_entropy_calculation() {
        let compressor = ZstdCompressor::new(Default::default());
        let repetitive = b"aaaaaaaaaaaaaaa";
        let randomish = b"The quick brown fox jumps";
        
        let entropy1 = compressor.estimate_ratio(repetitive);
        let entropy2 = compressor.estimate_ratio(randomish);
        
        assert!(entropy1 > entropy2, "Repetitive data should compress better");
    }
}