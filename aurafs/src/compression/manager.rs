//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Compression Manager - Intelligent Auto-Selection + Policy Engine
//! 🛸 Adaptive Algorithm Selection + Live Benchmarking + Policy Routing + Streaming
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::compression::{
    Compressor, CompressionAlgorithm, CompressionConfig, CompressionStatsCollector,
    CompressionBenchmark, Error, Result, Lz4Compressor, ZstdCompressor, QuantumCompressor,
    LatticeCompressor,
};
use std::{
    sync::Arc,
    collections::HashMap,
    time::Instant,
};
use tokio::{
    sync::RwLock,
    task,
};
use tracing::{info, debug, warn};

/// Intelligent compression manager with auto-selection and policy routing
pub struct CompressionManager {
    /// Default configuration
    config: CompressionConfig,
    
    /// Active compressors (lazy initialized)
    compressors: RwLock<HashMap<CompressionAlgorithm, Arc<dyn Compressor + Send + Sync>>>,
    
    /// Live performance statistics
    stats: Arc<CompressionStatsCollector>,
    
    /// Benchmark cache (algorithm performance history)
    benchmarks: RwLock<Vec<CompressionBenchmark>>,
    
    /// Adaptive policy engine
    policy: CompressionPolicy,
}

#[derive(Debug, Clone, Default)]
struct CompressionPolicy {
    /// Data type hints → preferred algorithms
    data_patterns: HashMap<DataPattern, CompressionAlgorithm>,
    
    /// Size-based routing rules
    size_rules: Vec<(usize, CompressionAlgorithm)>,
    
    /// Entropy thresholds for algorithm selection
    entropy_rules: Vec<(f64, CompressionAlgorithm)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DataPattern {
    Text,
    Binary,
    Sparse,
    Random,
    Json,
    Image,
    Audio,
}

impl CompressionManager {
    /// Forge production compression manager
    pub fn new(config: CompressionConfig) -> Arc<Self> {
        let stats = CompressionStatsCollector::new();
        let manager = Arc::new(Self {
            config,
            compressors: RwLock::new(HashMap::new()),
            stats,
            benchmarks: RwLock::new(Vec::new()),
            policy: CompressionPolicy::default(),
        });
        
        // Background benchmarking task
        let manager_clone = Arc::clone(&manager);
        tokio::spawn(async move {
            manager_clone.benchmark_background().await;
        });
        
        manager
    }

    /// Compress with intelligent algorithm selection
    pub async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let algorithm = self.select_algorithm(data).await;
        let compressor = self.get_compressor(algorithm).await?;
        let start = Instant::now();
        
        let compressed = compressor.compress(data).await?;
        let duration = start.elapsed();
        
        // Record statistics
        self.stats.record_compress(
            algorithm,
            data.len(),
            compressed.len(),
        ).await;
        
        info!("💎 Compressed {}→{}B ({:.2}x) with {} in {:?}",
              data.len(), compressed.len(),
              data.len() as f64 / compressed.len() as f64,
              algorithm, duration);
        
        Ok(compressed)
    }

    /// Decompress with auto-detection
    pub async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // TODO: Detect algorithm from header/magic bytes
        let algorithm = CompressionAlgorithm::Auto;
        let compressor = self.get_compressor(algorithm).await?;
        
        let start = Instant::now();
        let decompressed = compressor.decompress(data).await?;
        let duration = start.elapsed();
        
        self.stats.record_decompress(data.len(), decompressed.len()).await;
        
        info!("🔓 Decompressed {}→{}B in {:?}", 
              data.len(), decompressed.len(), duration);
        
        Ok(decompressed)
    }

    /// Intelligent algorithm selection based on data characteristics
    async fn select_algorithm(&self, data: &[u8]) -> CompressionAlgorithm {
        let pattern = self.analyze_data_pattern(data);
        let entropy = self.calculate_entropy(data);
        let size = data.len();
        
        // Policy-based selection
        if let Some(algo) = self.policy.data_patterns.get(&pattern) {
            return *algo;
        }
        
        // Size-based rules
        for (threshold, algo) in &self.policy.size_rules {
            if size >= *threshold {
                return *algo;
            }
        }
        
        // Entropy-based selection
        for (entropy_threshold, algo) in &self.policy.entropy_rules {
            if entropy >= *entropy_threshold {
                return *algo;
            }
        }
        
        // Benchmark-based fallback
        self.select_best_benchmarked().await
    }

    /// Get or create compressor for algorithm
    async fn get_compressor(
        &self,
        algorithm: CompressionAlgorithm,
    ) -> Result<Arc<dyn Compressor + Send + Sync>> {
        let mut compressors = self.compressors.write().await;
        
        if !compressors.contains_key(&algorithm) {
            let compressor = match algorithm {
                CompressionAlgorithm::Lz4 => Arc::new(Lz4Compressor::new(self.config.clone())),
                CompressionAlgorithm::Zstd => Arc::new(ZstdCompressor::new(self.config.clone())),
                CompressionAlgorithm::Quantum => Arc::new(QuantumCompressor::new(self.config.clone())),
                CompressionAlgorithm::Auto => Arc::new(Lz4Compressor::new(self.config.clone())),
            };
            compressors.insert(algorithm, compressor);
        }
        
        compressors.get(&algorithm)
            .cloned()
            .ok_or(Error::UnsupportedAlgorithm(algorithm.to_string()))
    }

    /// Background benchmarking task
    async fn benchmark_background(self: Arc<Self>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5min
        
        loop {
            interval.tick().await;
            if let Err(e) = self.run_benchmark().await {
                warn!("Benchmark failed: {}", e);
            }
        }
    }

    /// Run full algorithm benchmark
    async fn run_benchmark(&self) -> Result<()> {
        let test_data = self.generate_benchmark_data();
        let mut benchmarks = self.benchmarks.write().await;
        
        for &algo in &[CompressionAlgorithm::Lz4, CompressionAlgorithm::Zstd, CompressionAlgorithm::Quantum] {
            let compressor = self.get_compressor(algo).await?;
            let start = Instant::now();
            
            let compressed = task::spawn_blocking({
                let compressor = Arc::clone(&compressor);
                let data = test_data.clone();
                move || compressor.compress(&data)
            }).await??;
            
            let ratio = test_data.len() as f64 / compressed.len() as f64;
            let duration = start.elapsed();
            
            benchmarks.push(CompressionBenchmark {
                algorithm: algo,
                compress_speed_mb_s: (test_data.len() as f64 / 1024.0 / 1024.0) / duration.as_secs_f64(),
                decompress_speed_mb_s: 0.0,
                avg_ratio: ratio as f32,
                samples: 1,
            });
        }
        
        Ok(())
    }

    /// Live dashboard
    pub async fn dashboard(&self) {
        self.stats.print_dashboard().await;
        info!("🤖 Adaptive Policy Active | Benchmarks: {} cached", self.benchmarks.read().await.len());
    }
}

impl CompressionManager {
    /// Data pattern analysis
    fn analyze_data_pattern(&self, data: &[u8]) -> DataPattern {
        let zeros = data.iter().filter(|&&b| b == 0).count();
        let printable = data.iter().filter(|&&b| b.is_ascii() && b.is_printable()).count();
        
        if zeros as f32 / data.len() as f32 > 0.5 {
            DataPattern::Sparse
        } else if printable as f32 / data.len() as f32 > 0.8 {
            DataPattern::Text
        } else {
            DataPattern::Binary
        }
    }

    fn calculate_entropy(&self, data: &[u8]) -> f64 {
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
    
    async fn select_best_benchmarked(&self) -> CompressionAlgorithm {
        let benchmarks = self.benchmarks.read().await;
        benchmarks.first()
            .map(|b| b.algorithm)
            .unwrap_or(CompressionAlgorithm::Lz4)
    }
    
    fn generate_benchmark_data(&self) -> Vec<u8> {
        b"Quantum benchmark data with mixed entropy patterns for compression testing repeated many times...".to_vec()
    }
}

/// Production quick-start
pub fn production_manager() -> Arc<CompressionManager> {
    CompressionManager::new(Default::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_compress() {
        let manager = production_manager();
        let data = b"Quantum test data for compression manager";
        
        let compressed = manager.compress(data).await.unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = manager.decompress(&compressed).await.unwrap();
        assert_eq!(decompressed, data);
    }
}