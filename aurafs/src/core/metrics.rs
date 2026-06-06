//! ═══════════════════════════════════════════════════════════════════
//! 🌟 AuraFS Core Metrics - Mythical Grade Telemetry & Observability
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! High-performance async metrics collector with:
//! - Prometheus exposition & pushgateway support
//! - Dynamic tagging (shard, soul, phase)
//! - Histograms, gauges, counters with context
//! - Built-in error & latency tracking
//! - Integration hooks for tracing spans & alerts
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use tokio::task;
use prometheus::{
    Encoder, TextEncoder,
    Registry,
    Counter, IntCounter,
    Histogram, HistogramOpts,
    Gauge, IntGauge,
    Opts, core::{Collector, GenericCounter, GenericGauge},
};
use tracing::{info, error};
use warp::{Filter, Reply};

use crate::core::{ErrorClass, ErrorPhase, ErrorCode};

/// Singleton shared registry for AuraFS metrics.
static METRICS_REGISTRY: Lazy<Arc<Registry>> = Lazy::new(|| {
    let registry = Registry::new_custom(Some("aurafs".into()), None).unwrap();
    Arc::new(registry)
});

/// Enum for metric categories for tagging / filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricCategory {
    /// Shard-specific metrics.
    Shard,
    /// Network & orchestrator metrics.
    Network,
    /// Governance, vote, proposal metrics.
    Governance,
    /// Storage subsystem metrics.
    Storage,
    /// Core system internals.
    Core,
    /// ACL and identity subsystem metrics.
    Identity,
}

/// Core metrics collector singleton.
pub struct AuraFSMetrics {
    pub shard_errors: IntCounter,
    pub network_errors: IntCounter,
    pub governance_errors: IntCounter,
    pub storage_errors: IntCounter,

    pub shard_ops_total: IntCounter,
    pub network_msgs_sent: IntCounter,
    pub governance_votes_cast: IntCounter,

    pub operation_latencies: Histogram,

    pub active_nodes_gauge: IntGauge,
    pub active_souls_gauge: IntGauge,

    pub registry: Arc<Registry>,
}

/// Shared instance.
static mut METRICS_INSTANCE: Option<AuraFSMetrics> = None;

impl AuraFSMetrics {
    /// Initialize all metrics and register them.
    pub async fn init() -> Result<(), prometheus::Error> {
        let registry = Arc::clone(&METRICS_REGISTRY);

        let shard_errors = IntCounter::with_opts(Opts::new("shard_errors_total", "Total shard errors"))?;
        let network_errors = IntCounter::with_opts(Opts::new("network_errors_total", "Total network errors"))?;
        let governance_errors = IntCounter::with_opts(Opts::new("governance_errors_total", "Total governance errors"))?;
        let storage_errors = IntCounter::with_opts(Opts::new("storage_errors_total", "Total storage errors"))?;

        let shard_ops_total = IntCounter::with_opts(Opts::new("shard_operations_total", "Total shard operations"))?;
        let network_msgs_sent = IntCounter::with_opts(Opts::new("network_messages_sent_total", "Total network messages sent"))?;
        let governance_votes_cast = IntCounter::with_opts(Opts::new("governance_votes_cast_total", "Total governance votes cast"))?;

        let operation_latencies = Histogram::with_opts(HistogramOpts::new(
            "operation_latency_seconds",
            "Latency distribution of operations in seconds",
        ).buckets(vec![
            0.001, 0.005, 0.01, 0.025,
            0.05, 0.1, 0.25, 0.5,
            1.0, 2.5, 5.0, 10.0,
        ]))?;

        let active_nodes_gauge = IntGauge::with_opts(Opts::new("active_nodes", "Number of currently active nodes"))?;
        let active_souls_gauge = IntGauge::with_opts(Opts::new("active_souls", "Number of currently active authorized souls"))?;

        registry.register(Box::new(shard_errors.clone()))?;
        registry.register(Box::new(network_errors.clone()))?;
        registry.register(Box::new(governance_errors.clone()))?;
        registry.register(Box::new(storage_errors.clone()))?;

        registry.register(Box::new(shard_ops_total.clone()))?;
        registry.register(Box::new(network_msgs_sent.clone()))?;
        registry.register(Box::new(governance_votes_cast.clone()))?;

        registry.register(Box::new(operation_latencies.clone()))?;

        registry.register(Box::new(active_nodes_gauge.clone()))?;
        registry.register(Box::new(active_souls_gauge.clone()))?;

        unsafe {
            METRICS_INSTANCE = Some(Self {
                shard_errors,
                network_errors,
                governance_errors,
                storage_errors,
                shard_ops_total,
                network_msgs_sent,
                governance_votes_cast,
                operation_latencies,
                active_nodes_gauge,
                active_souls_gauge,
                registry,
            });
        }

        info!("AuraFS metrics initialized");
        Ok(())
    }

    /// Get static shared instance (panic if not initialized).
    pub fn get() -> &'static Self {
        unsafe {
            METRICS_INSTANCE.as_ref().expect("AuraFSMetrics not initialized")
        }
    }

    /// Increment error count with classification and validation
    pub fn error_occurred(class: ErrorClass, phase: ErrorPhase, code: ErrorCode) {
        // Validate metrics are initialized
        let metrics = match Self::try_get() {
            Some(m) => m,
            None => {
                tracing::warn!("AuraFSMetrics not initialized, skipping error metric");
                return;
            }
        };
        
        // Increment appropriate counter based on class
        match class {
            ErrorClass::Client => {
                // Client errors are expected, but we can track them separately if needed
                // For now, we'll track them in a separate counter if we add one
            }
            ErrorClass::Transient => {
                metrics.network_errors.inc();
            }
            ErrorClass::Internal => {
                metrics.storage_errors.inc();
            }
            ErrorClass::Security => {
                metrics.governance_errors.inc();
            }
            ErrorClass::External => {
                metrics.shard_errors.inc();
            }
        }
        
        // Track errors by phase for better observability
        // TODO: Add phase-specific counters if needed
        let _ = (phase, code); // Suppress unused warnings for now
    }
    
    /// Try to get metrics instance (returns None if not initialized)
    fn try_get() -> Option<&'static Self> {
        unsafe {
            METRICS_INSTANCE.as_ref()
        }
    }

    /// Observe latency measurement in seconds with validation
    pub fn observe_latency(sec: f64) {
        // Validate latency value
        if sec < 0.0 {
            tracing::warn!("Negative latency observed: {}s, ignoring", sec);
            return;
        }
        
        // Cap at reasonable maximum (1 hour)
        const MAX_LATENCY_SEC: f64 = 3600.0;
        if sec > MAX_LATENCY_SEC {
            tracing::warn!("Latency exceeds maximum: {}s, capping at {}s", sec, MAX_LATENCY_SEC);
            Self::get().operation_latencies.observe(MAX_LATENCY_SEC);
            return;
        }
        
        Self::get().operation_latencies.observe(sec);
    }

    /// Increment shard operation counter.
    pub fn increment_shard_ops() {
        Self::get().shard_ops_total.inc();
    }

    /// Increment network messages sent.
    pub fn increment_network_msgs() {
        Self::get().network_msgs_sent.inc();
    }

    /// Increment governance votes cast.
    pub fn increment_governance_votes() {
        Self::get().governance_votes_cast.inc();
    }

    /// Set current number of active nodes gauge with validation
    pub fn set_active_nodes(n: i64) {
        // Validate value
        if n < 0 {
            tracing::warn!("Negative active nodes count: {}, setting to 0", n);
            Self::get().active_nodes_gauge.set(0);
            return;
        }
        
        // Cap at reasonable maximum
        const MAX_NODES: i64 = 1_000_000;
        if n > MAX_NODES {
            tracing::warn!("Active nodes count exceeds maximum: {}, capping at {}", n, MAX_NODES);
            Self::get().active_nodes_gauge.set(MAX_NODES);
            return;
        }
        
        Self::get().active_nodes_gauge.set(n);
    }

    /// Set current number of active souls gauge with validation
    pub fn set_active_souls(n: i64) {
        // Validate value
        if n < 0 {
            tracing::warn!("Negative active souls count: {}, setting to 0", n);
            Self::get().active_souls_gauge.set(0);
            return;
        }
        
        // Cap at reasonable maximum
        const MAX_SOULS: i64 = 10_000_000;
        if n > MAX_SOULS {
            tracing::warn!("Active souls count exceeds maximum: {}, capping at {}", n, MAX_SOULS);
            Self::get().active_souls_gauge.set(MAX_SOULS);
            return;
        }
        
        Self::get().active_souls_gauge.set(n);
    }

    /// Prometheus exposition string with error handling
    pub fn gather_metrics() -> Result<String, prometheus::Error> {
        let metrics = match Self::try_get() {
            Some(m) => m,
            None => {
                return Err(prometheus::Error::Msg("AuraFSMetrics not initialized".to_string()));
            }
        };
        
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let registry = &metrics.registry;
        let metric_families = registry.gather();
        
        // Validate metric families
        if metric_families.is_empty() {
            tracing::warn!("No metrics to gather");
        }
        
        encoder.encode(&metric_families, &mut buffer)
            .map_err(|e| prometheus::Error::Msg(format!("Failed to encode metrics: {}", e)))?;
        
        String::from_utf8(buffer)
            .map_err(|e| prometheus::Error::Msg(format!("Failed to convert metrics to UTF-8: {}", e)))
    }
}

/// Warp filter for the `/metrics` endpoint.
pub fn metrics_route() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .and(warp::get())
        .map(|| match AuraFSMetrics::gather_metrics() {
            Ok(m) => warp::reply::with_header(
                m,
                "Content-Type",
                "text/plain; version=0.0.4; charset=utf-8",
            ),
            Err(e) => {
                error!("Failed to gather metrics: {}", e);
                warp::reply::with_status("Failed to gather metrics", warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ErrorClass, ErrorCode, ErrorPhase};

    #[test]
    fn test_error_counters_increment() {
        let _ = tokio_test::block_on(AuraFSMetrics::init());
        AuraFSMetrics::error_occurred(ErrorClass::Transient, ErrorPhase::Network, ErrorCode::Unavailable);
        AuraFSMetrics::error_occurred(ErrorClass::Internal, ErrorPhase::Storage, ErrorCode::ShardCorrupt);
        AuraFSMetrics::error_occurred(ErrorClass::Security, ErrorPhase::Governance, ErrorCode::InvalidSignature);
    }

    #[test]
    fn test_metrics_exposition() {
        let _ = tokio_test::block_on(AuraFSMetrics::init());
        let metrics = AuraFSMetrics::gather_metrics();
        assert!(metrics.is_ok());
        let output = metrics.unwrap();
        assert!(output.contains("shard_errors_total"));
    }
}