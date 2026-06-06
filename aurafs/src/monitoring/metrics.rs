//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS Metrics & Monitoring
//!
//! Prometheus-compatible metrics collection with custom gauges, counters,
//! histograms for latency, and performance tracking across all components.

use prometheus::{
    Encoder, TextEncoder, Registry, Counter, Gauge, Histogram, HistogramOpts,
    IntCounter, IntGauge, Opts,
};
use std::sync::Arc;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("Failed to register metric: {0}")]
    RegistrationError(String),
    
    #[error("Failed to encode metrics: {0}")]
    EncodingError(String),
}

pub type Result<T> = std::result::Result<T, MetricsError>;

/// Metrics collector for AuraFS
pub struct MetricsCollector {
    registry: Registry,
    
    // File operations
    pub file_reads: IntCounter,
    pub file_writes: IntCounter,
    pub file_deletes: IntCounter,
    
    // Cache metrics
    pub cache_hits: IntCounter,
    pub cache_misses: IntCounter,
    pub cache_size_bytes: IntGauge,
    
    // Shard metrics
    pub shards_total: IntGauge,
    pub shards_replicated: IntCounter,
    pub shard_size_bytes: Histogram,
    
    // Node metrics
    pub nodes_total: IntGauge,
    pub nodes_alive: IntGauge,
    
    // Network metrics
    pub bytes_sent: Counter,
    pub bytes_received: Counter,
    pub network_latency_ms: Histogram,
    
    // System metrics
    pub cpu_usage_percent: Gauge,
    pub memory_usage_bytes: IntGauge,
    pub disk_usage_bytes: IntGauge,
    pub disk_capacity_bytes: IntGauge,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Result<Self> {
        let registry = Registry::new();
        
        // File operations
        let file_reads = IntCounter::with_opts(
            Opts::new("aurafs_file_reads_total", "Total file read operations")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let file_writes = IntCounter::with_opts(
            Opts::new("aurafs_file_writes_total", "Total file write operations")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let file_deletes = IntCounter::with_opts(
            Opts::new("aurafs_file_deletes_total", "Total file delete operations")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        // Cache metrics
        let cache_hits = IntCounter::with_opts(
            Opts::new("aurafs_cache_hits_total", "Total cache hits")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let cache_misses = IntCounter::with_opts(
            Opts::new("aurafs_cache_misses_total", "Total cache misses")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let cache_size_bytes = IntGauge::with_opts(
            Opts::new("aurafs_cache_size_bytes", "Current cache size in bytes")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        // Shard metrics
        let shards_total = IntGauge::with_opts(
            Opts::new("aurafs_shards_total", "Total number of shards")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let shards_replicated = IntCounter::with_opts(
            Opts::new("aurafs_shards_replicated_total", "Total shard replications")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let shard_size_bytes = Histogram::with_opts(
            HistogramOpts::new("aurafs_shard_size_bytes", "Shard size distribution")
                .buckets(vec![1024.0, 10240.0, 102400.0, 1048576.0, 10485760.0])
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        // Node metrics
        let nodes_total = IntGauge::with_opts(
            Opts::new("aurafs_nodes_total", "Total number of nodes")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let nodes_alive = IntGauge::with_opts(
            Opts::new("aurafs_nodes_alive", "Number of alive nodes")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        // Network metrics
        let bytes_sent = Counter::with_opts(
            Opts::new("aurafs_bytes_sent_total", "Total bytes sent")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let bytes_received = Counter::with_opts(
            Opts::new("aurafs_bytes_received_total", "Total bytes received")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let network_latency_ms = Histogram::with_opts(
            HistogramOpts::new("aurafs_network_latency_ms", "Network latency in milliseconds")
                .buckets(vec![1.0, 5.0, 10.0, 50.0, 100.0, 500.0, 1000.0])
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        // System metrics
        let cpu_usage_percent = Gauge::with_opts(
            Opts::new("aurafs_cpu_usage_percent", "CPU usage percentage")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let memory_usage_bytes = IntGauge::with_opts(
            Opts::new("aurafs_memory_usage_bytes", "Memory usage in bytes")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let disk_usage_bytes = IntGauge::with_opts(
            Opts::new("aurafs_disk_usage_bytes", "Disk usage in bytes")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        let disk_capacity_bytes = IntGauge::with_opts(
            Opts::new("aurafs_disk_capacity_bytes", "Disk capacity in bytes")
        ).map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        // Register all metrics
        registry.register(Box::new(file_reads.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(file_writes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(file_deletes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(cache_hits.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(cache_misses.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(cache_size_bytes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(shards_total.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(shards_replicated.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(shard_size_bytes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(nodes_total.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(nodes_alive.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(bytes_sent.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(bytes_received.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(network_latency_ms.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(cpu_usage_percent.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(memory_usage_bytes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(disk_usage_bytes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        registry.register(Box::new(disk_capacity_bytes.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;
        
        info!("Initialized metrics collector");
        
        Ok(Self {
            registry,
            file_reads,
            file_writes,
            file_deletes,
            cache_hits,
            cache_misses,
            cache_size_bytes,
            shards_total,
            shards_replicated,
            shard_size_bytes,
            nodes_total,
            nodes_alive,
            bytes_sent,
            bytes_received,
            network_latency_ms,
            cpu_usage_percent,
            memory_usage_bytes,
            disk_usage_bytes,
            disk_capacity_bytes,
        })
    }
    
    /// Export metrics in Prometheus format
    pub fn export(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)
            .map_err(|e| MetricsError::EncodingError(e.to_string()))?;
        
        String::from_utf8(buffer)
            .map_err(|e| MetricsError::EncodingError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_collection() {
        let collector = MetricsCollector::new().unwrap();
        
        collector.file_reads.inc();
        collector.cache_hits.inc_by(5);
        collector.nodes_total.set(10);
        
        let metrics = collector.export().unwrap();
        assert!(metrics.contains("aurafs_file_reads_total"));
        assert!(metrics.contains("aurafs_cache_hits_total"));
    }
}