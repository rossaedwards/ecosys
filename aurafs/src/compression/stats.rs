//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Compression Stats - Live Metrics + Benchmarking + Analytics
//! 🛸 Real-time Throughput + Ratio Tracking + Algorithm Performance + Histograms
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::compression::{CompressionAlgorithm, CompressionBenchmark, CompressionConfig};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Instant,
};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use tracing::{info, debug};

/// Live compression statistics tracker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompressionStats {
    /// Total bytes compressed
    pub total_compressed_bytes: u64,
    
    /// Total bytes decompressed
    pub total_decompressed_bytes: u64,
    
    /// Total operations
    pub total_operations: u64,
    
    /// Compression operations
    pub compress_operations: u64,
    
    /// Decompression operations
    pub decompress_operations: u64,
    
    /// Cumulative compression ratio
    pub avg_ratio: f64,
    
    /// Per-algorithm statistics
    pub algorithm_stats: HashMap<CompressionAlgorithm, AlgorithmStats>,
    
    /// Live throughput (MB/s)
    pub throughput_mbps: f64,
    
    /// Peak throughput observed
    pub peak_throughput_mbps: f64,
    
    /// Start time
    pub started_at: Instant,
    
    /// Cache hit ratio for compressed blocks
    pub cache_hit_ratio: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgorithmStats {
    /// Operations count
    pub operations: u64,
    
    /// Total input bytes
    pub input_bytes: u64,
    
    /// Total output bytes
    pub output_bytes: u64,
    
    /// Average ratio
    pub avg_ratio: f64,
    
    /// Operations per second
    pub ops_per_sec: f64,
    
    /// Sample ratios (histogram)
    pub ratio_histogram: Vec<f64>,
}

/// Production stats collector for compression operations
pub struct CompressionStatsCollector {
    stats: RwLock<CompressionStats>,
    last_throughput_check: RwLock<Instant>,
}

impl Default for CompressionStatsCollector {
    fn default() -> Self {
        Self {
            stats: RwLock::new(CompressionStats::default()),
            last_throughput_check: RwLock::new(Instant::now()),
        }
    }
}

impl CompressionStatsCollector {
    /// Create new stats collector
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Record compression operation
    pub async fn record_compress(
        &self,
        algorithm: CompressionAlgorithm,
        input_size: usize,
        output_size: usize,
    ) {
        let mut stats = self.stats.write().await;
        let mut algo_stats = stats.algorithm_stats.entry(algorithm).or_default();
        
        stats.total_compressed_bytes += input_size as u64;
        stats.compress_operations += 1;
        stats.total_operations += 1;
        
        algo_stats.operations += 1;
        algo_stats.input_bytes += input_size as u64;
        algo_stats.output_bytes += output_size as u64;
        algo_stats.ratio_histogram.push((input_size as f64 / output_size.max(1) as f64));
        
        // Update average ratio
        algo_stats.avg_ratio = algo_stats.input_bytes as f64 / algo_stats.output_bytes.max(1) as f64;
        stats.avg_ratio = stats.total_compressed_bytes as f64 / stats.total_decompressed_bytes.max(1) as f64;
        
        self.update_throughput(&mut stats).await;
    }

    /// Record decompression operation
    pub async fn record_decompress(&self, input_size: usize, output_size: usize) {
        let mut stats = self.stats.write().await;
        stats.total_decompressed_bytes += output_size as u64;
        stats.decompress_operations += 1;
        stats.total_operations += 1;
        
        stats.avg_ratio = stats.total_compressed_bytes as f64 / stats.total_decompressed_bytes.max(1) as f64;
        self.update_throughput(&mut stats).await;
    }

    /// Get current snapshot of statistics
    pub async fn snapshot(&self) -> CompressionStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Reset all statistics
    pub async fn reset(&self) {
        let mut stats = self.stats.write().await;
        *stats = CompressionStats::default();
        *stats.last_throughput_check.write().await = Instant::now();
    }

    /// Generate benchmark report
    pub async fn benchmark_report(&self) -> Vec<CompressionBenchmark> {
        let stats = self.stats.read().await;
        stats.algorithm_stats.iter()
            .map(|(algo, a_stats)| CompressionBenchmark {
                algorithm: *algo,
                compress_speed_mb_s: (a_stats.input_bytes as f64 / 1024.0 / 1024.0) / 
                    stats.started_at.elapsed().as_secs_f64(),
                decompress_speed_mb_s: 0.0, // TODO: Track separately
                avg_ratio: a_stats.avg_ratio as f32,
                samples: a_stats.operations as usize,
            })
            .collect()
    }

    /// Update live throughput metrics
    async fn update_throughput(&self, stats: &mut CompressionStats) {
        let now = Instant::now();
        let mut last_check = self.last_throughput_check.write().await;
        
        let elapsed = now.duration_since(*last_check).as_secs_f64();
        if elapsed > 0.1 { // Update every 100ms
            let total_bytes = stats.total_compressed_bytes + stats.total_decompressed_bytes;
            let throughput = (total_bytes as f64 / 1024.0 / 1024.0) / elapsed;
            
            stats.throughput_mbps = throughput;
            if throughput > stats.peak_throughput_mbps {
                stats.peak_throughput_mbps = throughput;
            }
            
            *last_check = now;
        }
    }

    /// Pretty-print live dashboard
    pub async fn print_dashboard(&self) {
        let stats = self.snapshot().await;
        let elapsed = stats.started_at.elapsed().as_secs_f64();
        
        info!("🛸 AuraFS Compression Dashboard");
        info!("═══════════════════════════════════════════════════════════════");
        info!("📊 Total Ops: {:>8} | Ratio: {:.2}x | Throughput: {:.1} MB/s",
              stats.total_operations, stats.avg_ratio, stats.throughput_mbps);
        info!("📈 Peak: {:.1} MB/s | Cache Hit: {:.1}%", 
              stats.peak_throughput_mbps, stats.cache_hit_ratio * 100.0);
        
        for (algo, a_stats) in &stats.algorithm_stats {
            info!("  {}: {:.2}x ({:.1} MB/s, {} ops)", 
                  algo, a_stats.avg_ratio, 
                  (a_stats.input_bytes as f64 / elapsed / 1024.0 / 1024.0), 
                  a_stats.operations);
        }
    }
}

/// Global stats collector (singleton-like)
pub static GLOBAL_STATS: std::sync::OnceLock<Arc<CompressionStatsCollector>> = 
    std::sync::OnceLock::new();

/// Initialize global stats collector
pub fn init_global_stats() -> Arc<CompressionStatsCollector> {
    GLOBAL_STATS.get_or_init(|| Arc::new(CompressionStatsCollector::new())).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stats_collection() {
        let stats = CompressionStatsCollector::new();
        
        stats.record_compress(CompressionAlgorithm::Lz4, 1024, 512).await;
        stats.record_compress(CompressionAlgorithm::Zstd, 2048, 600).await;
        stats.record_decompress(600, 2048).await;
        
        let snapshot = stats.snapshot().await;
        assert_eq!(snapshot.compress_operations, 2);
        assert_eq!(snapshot.decompress_operations, 1);
        assert!(snapshot.avg_ratio > 1.0);
    }

    #[tokio::test]
    async fn test_benchmark_report() {
        let stats = CompressionStatsCollector::new();
        stats.record_compress(CompressionAlgorithm::Lz4, 1024 * 1024, 512 * 1024).await;
        
        let report = stats.benchmark_report().await;
        assert!(!report.is_empty());
        
        if let Some(bench) = report.first() {
            assert_eq!(bench.algorithm, CompressionAlgorithm::Lz4);
            assert!(bench.avg_ratio > 1.0);
        }
    }
}