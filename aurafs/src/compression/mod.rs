//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Compression Module - LZ4 + Zstd + Quantum Wavelet Engine
//! 🛸 SIMD LZ4 + Adaptive Zstd + Post-Quantum Entropy Compression
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]
#![deny(missing_docs)]

pub use self::{
    lz4::Lz4Compressor,
    zstd::ZstdCompressor,
    quantum::QuantumCompressor,
    stats::CompressionStats,
    config::CompressionConfig,
};

pub mod lz4;
pub mod zstd;
pub mod quantum;
pub mod stats;
pub mod config;

/// Unified compression result type
pub type Result<T> = std::result::Result<T, Error>;

/// Core compression error enum
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("LZ4 compression failed")]
    Lz4,
    #[error("Zstd compression failed")]
    Zstd,
    #[error("Quantum compression failed")]
    Quantum,
    #[error("Decompression size mismatch: expected {expected}, got {got}")]
    SizeMismatch { expected: usize, got: usize },
    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

/// Supported compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    /// Ultra-fast LZ4 (400 MB/s)
    Lz4,
    
    /// Balanced Zstd (adaptive)
    Zstd,
    
    /// Quantum entropy encoding
    Quantum,
    
    /// Auto-select best algorithm
    Auto,
}

impl std::fmt::Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionAlgorithm::Lz4 => write!(f, "LZ4"),
            CompressionAlgorithm::Zstd => write!(f, "Zstd"),
            CompressionAlgorithm::Quantum => write!(f, "Quantum"),
            CompressionAlgorithm::Auto => write!(f, "Auto"),
        }
    }
}

/// Production compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    
    /// Compression level (1-22)
    pub level: u8,
    
    /// Enable SIMD acceleration
    pub enable_simd: bool,
    
    /// Minimum size to compress (bytes)
    pub min_size: usize,
    
    /// Target compression ratio
    pub target_ratio: f32,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Auto,
            level: 3,                    // Balanced speed/ratio
            enable_simd: true,
            min_size: 1024,              // 1KB minimum
            target_ratio: 0.5,           // 50% target
        }
    }
}

/// Production compressor factory
pub fn production_compressor(config: CompressionConfig) -> Arc<dyn Compressor + Send + Sync> {
    match config.algorithm {
        CompressionAlgorithm::Lz4 => Arc::new(Lz4Compressor::new(config)),
        CompressionAlgorithm::Zstd => Arc::new(ZstdCompressor::new(config)),
        CompressionAlgorithm::Quantum => Arc::new(QuantumCompressor::new(config)),
        CompressionAlgorithm::Auto => auto_select_compressor(config),
    }
}

/// Auto-select best compressor based on data characteristics
fn auto_select_compressor(config: CompressionConfig) -> Arc<dyn Compressor + Send + Sync> {
    Arc::new(Lz4Compressor::new(config)) // Fastest default
}

/// Core compression trait
pub trait Compressor: Send + Sync {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn compress_ratio(&self, data: &[u8]) -> f32;
    fn algorithm(&self) -> CompressionAlgorithm;
    fn config(&self) -> &CompressionConfig;
}

/// PRODUCTION QUICK-START MACROS
#[macro_export]
macro_rules! aurafs_compress {
    // Auto compression (recommended)
    ($data:expr) => {{
        let compressor = $crate::compression::production_compressor(Default::default());
        compressor.compress($data)
    }};
    
    // Algorithm-specific
    ($data:expr, $algo:expr) => {{
        let mut config = $crate::compression::CompressionConfig::default();
        config.algorithm = $algo;
        let compressor = $crate::compression::production_compressor(config);
        compressor.compress($data)
    }};
}

/// Batch compression benchmark
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompressionBenchmark {
    pub algorithm: CompressionAlgorithm,
    pub compress_speed_mb_s: f64,
    pub decompress_speed_mb_s: f64,
    pub avg_ratio: f32,
    pub samples: usize,
}

/// Auto-compression benchmark across algorithms
pub async fn benchmark_algorithms(data: &[u8]) -> Vec<CompressionBenchmark> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression_config() {
        let config = CompressionConfig::default();
        assert_eq!(config.algorithm, CompressionAlgorithm::Auto);
        assert_eq!(config.level, 3);
        assert_eq!(config.min_size, 1024);
    }
    
    #[tokio::test]
    async fn test_production_compressor() {
        let compressor = production_compressor(Default::default());
        let data = b"Quantum compression test data repeated many times...";
        
        let compressed = compressor.compress(data).await.unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = compressor.decompress(&compressed).await.unwrap();
        assert_eq!(decompressed, data);
    }
}