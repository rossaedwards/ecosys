//! Comprehensive Metrics & Observability
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Prometheus metrics with distributed tracing support.

use prometheus::{Counter, Histogram, Gauge, Registry, Opts, HistogramOpts};
use std::sync::Arc;
use tracing::{info, error};
use crate::error::Result;

/// Comprehensive AuraFS metrics
pub struct AuraFSMetrics {
    // Operation counters
    pub operations_total: Counter,
    pub operations_errors: Counter,
    
    // Latency histograms
    pub operation_duration: Histogram,
    pub read_latency: Histogram,
    pub write_latency: Histogram,
    
    // Resource gauges
    pub active_connections: Gauge,
    pub cache_size: Gauge,
    pub shard_count: Gauge,
    
    // Custom metrics
    pub dedup_ratio: Gauge,
    pub replication_lag: Histogram,
    
    // Circuit breaker metrics
    pub circuit_breaker_open: Counter,
    pub circuit_breaker_closed: Counter,
    
    // Retry metrics
    pub retry_attempts: Counter,
    pub retry_success: Counter,
}

impl AuraFSMetrics {
    /// Create new metrics instance
    pub fn new(registry: &Registry) -> Result<Self> {
        let operations_total = Counter::with_opts(
            Opts::new("aurafs_operations_total", "Total operations")
                .namespace("aurafs")
        )?;
        
        let operation_duration = Histogram::with_opts(
            HistogramOpts::new("aurafs_operation_duration_seconds", "Operation duration")
                .namespace("aurafs")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0])
        )?;
        
        let read_latency = Histogram::with_opts(
            HistogramOpts::new("aurafs_read_latency_seconds", "Read latency")
                .namespace("aurafs")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
        )?;
        
        let write_latency = Histogram::with_opts(
            HistogramOpts::new("aurafs_write_latency_seconds", "Write latency")
                .namespace("aurafs")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5])
        )?;
        
        let active_connections = Gauge::with_opts(
            Opts::new("aurafs_active_connections", "Active connections")
                .namespace("aurafs")
        )?;
        
        let cache_size = Gauge::with_opts(
            Opts::new("aurafs_cache_size_bytes", "Cache size in bytes")
                .namespace("aurafs")
        )?;
        
        let shard_count = Gauge::with_opts(
            Opts::new("aurafs_shard_count", "Total shard count")
                .namespace("aurafs")
        )?;
        
        let dedup_ratio = Gauge::with_opts(
            Opts::new("aurafs_dedup_ratio", "Deduplication ratio")
                .namespace("aurafs")
        )?;
        
        let replication_lag = Histogram::with_opts(
            HistogramOpts::new("aurafs_replication_lag_seconds", "Replication lag")
                .namespace("aurafs")
                .buckets(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0])
        )?;
        
        let operations_errors = Counter::with_opts(
            Opts::new("aurafs_operations_errors_total", "Operation errors")
                .namespace("aurafs")
        )?;
        
        let circuit_breaker_open = Counter::with_opts(
            Opts::new("aurafs_circuit_breaker_open_total", "Circuit breaker opened")
                .namespace("aurafs")
        )?;
        
        let circuit_breaker_closed = Counter::with_opts(
            Opts::new("aurafs_circuit_breaker_closed_total", "Circuit breaker closed")
                .namespace("aurafs")
        )?;
        
        let retry_attempts = Counter::with_opts(
            Opts::new("aurafs_retry_attempts_total", "Total retry attempts")
                .namespace("aurafs")
        )?;
        
        let retry_success = Counter::with_opts(
            Opts::new("aurafs_retry_success_total", "Successful retries")
                .namespace("aurafs")
        )?;
        
        // Register all metrics
        registry.register(Box::new(operations_total.clone()))?;
        registry.register(Box::new(operation_duration.clone()))?;
        registry.register(Box::new(read_latency.clone()))?;
        registry.register(Box::new(write_latency.clone()))?;
        registry.register(Box::new(active_connections.clone()))?;
        registry.register(Box::new(cache_size.clone()))?;
        registry.register(Box::new(shard_count.clone()))?;
        registry.register(Box::new(dedup_ratio.clone()))?;
        registry.register(Box::new(replication_lag.clone()))?;
        registry.register(Box::new(operations_errors.clone()))?;
        registry.register(Box::new(circuit_breaker_open.clone()))?;
        registry.register(Box::new(circuit_breaker_closed.clone()))?;
        registry.register(Box::new(retry_attempts.clone()))?;
        registry.register(Box::new(retry_success.clone()))?;
        
        Ok(Self {
            operations_total,
            operations_errors,
            operation_duration,
            read_latency,
            write_latency,
            active_connections,
            cache_size,
            shard_count,
            dedup_ratio,
            replication_lag,
            circuit_breaker_open,
            circuit_breaker_closed,
            retry_attempts,
            retry_success,
        })
    }
}

/// Instrument an operation with metrics
pub async fn instrumented_operation<F, T>(
    metrics: &AuraFSMetrics,
    operation_name: &str,
    operation: F,
) -> Result<T>
where
    F: std::future::Future<Output = Result<T>>,
{
    let timer = metrics.operation_duration.start_timer();
    metrics.operations_total.inc();
    
    let span = tracing::span!(
        tracing::Level::INFO,
        "operation",
        name = operation_name
    );
    let _guard = span.enter();
    
    let result = operation.await;
    
    timer.observe_duration();
    
    match &result {
        Ok(_) => {
            tracing::info!("Operation {} succeeded", operation_name);
        }
        Err(e) => {
            metrics.operations_errors.inc();
            tracing::error!(error = %e, "Operation {} failed", operation_name);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::Registry;

    #[test]
    fn test_metrics_creation() {
        let registry = Registry::new();
        let metrics = AuraFSMetrics::new(&registry);
        assert!(metrics.is_ok());
    }
}

