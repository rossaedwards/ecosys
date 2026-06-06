//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Deduplication Module - Quantum Content Deduplication Engine
//! 🛸 Merkle Tree Dedup + SIMD Similarity + Post-Quantum Fingerprints + Cache
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]
#![deny(missing_docs)]

pub use self::{
    dedup_engine::{DedupEngine, DedupConfig, DedupStats},
    fingerprint::{ContentFingerprint, FingerprintType},
    cache::{DedupCache, CacheEntry},
    similarity::{SimilarityScore, SimilarityEngine},
};

pub mod dedup_engine;
pub mod fingerprint;
pub mod cache;
pub mod similarity;

/// Unified deduplication result type
pub type Result<T> = std::result::Result<T, Error>;

/// Core deduplication error enum
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Fingerprint computation failed: {0}")]
    Fingerprint(String),
    #[error("Cache lookup failed")]
    CacheMiss,
    #[error("Similarity threshold not met: {0}")]
    SimilarityThreshold(f32),
    #[error("Duplicate not found")]
    NoDuplicate,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SIMD acceleration unavailable")]
    SimdUnavailable,
}

/// Production deduplication configuration
#[derive(Debug, Clone)]
pub struct DedupConfig {
    /// Minimum similarity threshold (0.0-1.0)
    pub similarity_threshold: f32,
    
    /// Fingerprint type (SHA3, BLAKE3, PostQuantum)
    pub fingerprint_type: FingerprintType,
    
    /// Cache size (entries)
    pub cache_size: usize,
    
    /// Chunk size for rolling hash (bytes)
    pub chunk_size: usize,
    
    /// Enable SIMD similarity comparison
    pub enable_simd: bool,
    
    /// Maximum dedup candidates to check
    pub max_candidates: usize,
}

impl Default for DedupConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.95,  // 95% similarity = dedup
            fingerprint_type: FingerprintType::Blake3,
            cache_size: 1_000_000,       // 1M entries
            chunk_size: 64 * 1024,       // 64KB chunks
            enable_simd: true,
            max_candidates: 32,
        }
    }
}

/// PRODUCTION QUICK-START MACROS
#[macro_export]
macro_rules! aurafs_dedup {
    // Instant deduplication engine
    () => {{
        $crate::dedup::DedupEngine::new(Default::default())
    }};
    
    // Configurable production dedup
    (threshold: $threshold:expr) => {{
        let mut config = $crate::dedup::DedupConfig::default();
        config.similarity_threshold = $threshold;
        $crate::dedup::DedupEngine::new(config)
    }};
}

/// Production deduplication bundle - Complete stack
pub async fn production_dedup(config: DedupConfig) -> Result<(Arc<DedupEngine>, Arc<DedupCache>)> {
    let cache = Arc::new(DedupCache::new(config.cache_size));
    let engine = Arc::new(DedupEngine::new_with_cache(config, Arc::clone(&cache)));
    
    Ok((engine, cache))
}

/// Quick content deduplication check
pub async fn find_duplicate(
    engine: &Arc<DedupEngine>,
    data: &[u8],
) -> Result<Option<ContentFingerprint>> {
    engine.find_similar(data).await
}

/// Batch deduplication statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct DedupReport {
    pub total_bytes: u64,
    pub dedup_bytes: u64,
    pub savings_ratio: f32,
    pub duplicate_count: usize,
    pub cache_hits: u64,
}

/// Feature-gated production dedup stack
#[cfg(feature = "full")]
pub mod full {
    pub use super::*;
    
    /// Production daemon startup
    pub async fn start_dedup_daemon(config: DedupConfig) -> Result<()> {
        let (engine, cache) = production_dedup(config).await?;
        // Background cache maintenance
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dedup_config() {
        let config = DedupConfig::default();
        assert_eq!(config.similarity_threshold, 0.95);
        assert_eq!(config.cache_size, 1_000_000);
        assert_eq!(config.fingerprint_type, FingerprintType::Blake3);
    }
    
    #[tokio::test]
    async fn test_production_bundle() {
        let (engine, cache) = production_dedup(Default::default()).await.unwrap();
        assert!(!engine.is_empty().await);
    }
}