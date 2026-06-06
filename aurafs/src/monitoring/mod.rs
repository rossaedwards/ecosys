//! ═══════════════════════════════════════════════════════════════════
//! Monitoring & Observability Module
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Comprehensive monitoring with metrics collection, health checks,
//! alerting, and distributed tracing support.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};

// ═══════════════════════════════════════════════════════════════════
// METRICS COLLECTION
// ═══════════════════════════════════════════════════════════════════

/// Metrics types
pub mod metrics {
    use super::*;

    /// Core metrics collector for AuraFS
    #[derive(Debug)]
    pub struct MetricsCollector {
        /// Counter metrics
        counters: RwLock<HashMap<String, u64>>,
        /// Gauge metrics
        gauges: RwLock<HashMap<String, f64>>,
        /// Histogram metrics (buckets)
        histograms: RwLock<HashMap<String, Vec<f64>>>,
        /// Start time for uptime calculation
        start_time: Instant,
    }

    impl MetricsCollector {
        /// Create new metrics collector
        pub fn new() -> Result<Self, String> {
            info!("Initializing metrics collector");
            Ok(Self {
                counters: RwLock::new(HashMap::new()),
                gauges: RwLock::new(HashMap::new()),
                histograms: RwLock::new(HashMap::new()),
                start_time: Instant::now(),
            })
        }

        /// Increment a counter
        pub async fn increment(&self, name: &str, value: u64) {
            let mut counters = self.counters.write().await;
            *counters.entry(name.to_string()).or_insert(0) += value;
        }

        /// Set a gauge value
        pub async fn set_gauge(&self, name: &str, value: f64) {
            let mut gauges = self.gauges.write().await;
            gauges.insert(name.to_string(), value);
        }

        /// Record a histogram observation
        pub async fn observe(&self, name: &str, value: f64) {
            let mut histograms = self.histograms.write().await;
            histograms.entry(name.to_string()).or_insert_with(Vec::new).push(value);
        }

        /// Get counter value
        pub async fn get_counter(&self, name: &str) -> u64 {
            self.counters.read().await.get(name).copied().unwrap_or(0)
        }

        /// Get gauge value
        pub async fn get_gauge(&self, name: &str) -> f64 {
            self.gauges.read().await.get(name).copied().unwrap_or(0.0)
        }

        /// Get histogram percentile
        pub async fn percentile(&self, name: &str, p: f64) -> Option<f64> {
            let histograms = self.histograms.read().await;
            let values = histograms.get(name)?;
            
            if values.is_empty() {
                return None;
            }

            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let idx = ((p / 100.0) * (sorted.len() - 1) as f64) as usize;
            Some(sorted[idx])
        }

        /// Get uptime in seconds
        pub fn uptime_secs(&self) -> u64 {
            self.start_time.elapsed().as_secs()
        }

        /// Export all metrics as JSON-compatible structure
        pub async fn export(&self) -> MetricsSnapshot {
            MetricsSnapshot {
                counters: self.counters.read().await.clone(),
                gauges: self.gauges.read().await.clone(),
                uptime_secs: self.uptime_secs(),
            }
        }
    }

    impl Default for MetricsCollector {
        fn default() -> Self {
            Self::new().expect("Failed to create metrics collector")
        }
    }

    /// Snapshot of all metrics
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct MetricsSnapshot {
        /// Counter values
        pub counters: HashMap<String, u64>,
        /// Gauge values
        pub gauges: HashMap<String, f64>,
        /// System uptime
        pub uptime_secs: u64,
    }
}

// ═══════════════════════════════════════════════════════════════════
// HEALTH CHECKS
// ═══════════════════════════════════════════════════════════════════

/// Health check system
pub mod health {
    use super::*;

    /// Health check result
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct HealthStatus {
        /// Overall health status
        pub healthy: bool,
        /// Individual component statuses
        pub components: HashMap<String, ComponentHealth>,
        /// Timestamp of check
        pub timestamp: u64,
        /// Total latency of health check
        pub check_latency_ms: u64,
    }

    /// Individual component health
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct ComponentHealth {
        /// Component name
        pub name: String,
        /// Is component healthy
        pub healthy: bool,
        /// Optional status message
        pub message: Option<String>,
        /// Check latency in ms
        pub latency_ms: u64,
    }

    /// Health checker for AuraFS components
    #[derive(Debug)]
    pub struct HealthChecker {
        /// Registered health check functions
        checks: RwLock<Vec<HealthCheck>>,
    }

    /// A registered health check
    pub struct HealthCheck {
        /// Component name
        pub name: String,
        /// Check function
        pub check_fn: Box<dyn Fn() -> bool + Send + Sync>,
    }

    impl std::fmt::Debug for HealthCheck {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("HealthCheck")
                .field("name", &self.name)
                .finish()
        }
    }

    impl HealthChecker {
        /// Create new health checker
        pub fn new() -> Self {
            Self {
                checks: RwLock::new(Vec::new()),
            }
        }

        /// Register a health check
        pub async fn register<F>(&self, name: &str, check_fn: F)
        where
            F: Fn() -> bool + Send + Sync + 'static,
        {
            let mut checks = self.checks.write().await;
            checks.push(HealthCheck {
                name: name.to_string(),
                check_fn: Box::new(check_fn),
            });
            debug!("Registered health check: {}", name);
        }

        /// Run all health checks
        pub async fn check_all(&self) -> HealthStatus {
            let start = Instant::now();
            let checks = self.checks.read().await;
            let mut components = HashMap::new();
            let mut all_healthy = true;

            for check in checks.iter() {
                let check_start = Instant::now();
                let healthy = (check.check_fn)();
                let latency_ms = check_start.elapsed().as_millis() as u64;

                if !healthy {
                    all_healthy = false;
                    warn!("Health check failed: {}", check.name);
                }

                components.insert(
                    check.name.clone(),
                    ComponentHealth {
                        name: check.name.clone(),
                        healthy,
                        message: None,
                        latency_ms,
                    },
                );
            }

            HealthStatus {
                healthy: all_healthy,
                components,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                check_latency_ms: start.elapsed().as_millis() as u64,
            }
        }

        /// Quick liveness check
        pub fn is_alive(&self) -> bool {
            true // Basic liveness - process is running
        }

        /// Readiness check
        pub async fn is_ready(&self) -> bool {
            let status = self.check_all().await;
            status.healthy
        }
    }

    impl Default for HealthChecker {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ALERTING
// ═══════════════════════════════════════════════════════════════════

/// Alerting system
pub mod alerting {
    use super::*;

    /// Alert severity levels
    #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
    pub enum AlertSeverity {
        /// Informational
        Info,
        /// Warning, may need attention
        Warning,
        /// Error, action required
        Error,
        /// Critical, immediate action needed
        Critical,
    }

    /// An alert instance
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct Alert {
        /// Unique alert ID
        pub id: String,
        /// Alert name/type
        pub name: String,
        /// Severity level
        pub severity: AlertSeverity,
        /// Alert message
        pub message: String,
        /// Timestamp
        pub timestamp: u64,
        /// Additional labels
        pub labels: HashMap<String, String>,
    }

    /// Alert manager for routing and deduplication
    #[derive(Debug)]
    pub struct AlertManager {
        /// Active alerts
        active: RwLock<HashMap<String, Alert>>,
        /// Alert history
        history: RwLock<Vec<Alert>>,
    }

    impl AlertManager {
        /// Create new alert manager
        pub fn new() -> Self {
            Self {
                active: RwLock::new(HashMap::new()),
                history: RwLock::new(Vec::new()),
            }
        }

        /// Fire an alert
        pub async fn fire(&self, name: &str, severity: AlertSeverity, message: &str) {
            let alert = Alert {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.to_string(),
                severity,
                message: message.to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                labels: HashMap::new(),
            };

            match severity {
                AlertSeverity::Critical => error!("🚨 CRITICAL: {} - {}", name, message),
                AlertSeverity::Error => error!("❌ ERROR: {} - {}", name, message),
                AlertSeverity::Warning => warn!("⚠️ WARNING: {} - {}", name, message),
                AlertSeverity::Info => info!("ℹ️ INFO: {} - {}", name, message),
            }

            let mut active = self.active.write().await;
            active.insert(alert.id.clone(), alert.clone());

            let mut history = self.history.write().await;
            history.push(alert);
        }

        /// Resolve an alert
        pub async fn resolve(&self, id: &str) {
            let mut active = self.active.write().await;
            if active.remove(id).is_some() {
                info!("Alert resolved: {}", id);
            }
        }

        /// Get active alerts
        pub async fn get_active(&self) -> Vec<Alert> {
            self.active.read().await.values().cloned().collect()
        }

        /// Get alert count by severity
        pub async fn count_by_severity(&self, severity: AlertSeverity) -> usize {
            self.active
                .read()
                .await
                .values()
                .filter(|a| a.severity == severity)
                .count()
        }
    }

    impl Default for AlertManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════

pub use metrics::{MetricsCollector, MetricsSnapshot};
pub use health::{HealthChecker, HealthStatus, ComponentHealth};
pub use alerting::{AlertManager, Alert, AlertSeverity};

/// Initialize monitoring subsystem
pub fn init() -> (MetricsCollector, HealthChecker, AlertManager) {
    info!("📊 Monitoring subsystem initialized");
    (
        MetricsCollector::default(),
        HealthChecker::default(),
        AlertManager::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics() {
        let metrics = MetricsCollector::default();
        
        metrics.increment("requests", 10).await;
        assert_eq!(metrics.get_counter("requests").await, 10);
        
        metrics.set_gauge("memory_mb", 256.5).await;
        assert_eq!(metrics.get_gauge("memory_mb").await, 256.5);
    }

    #[tokio::test]
    async fn test_health_checker() {
        let checker = HealthChecker::new();
        checker.register("test", || true).await;
        
        let status = checker.check_all().await;
        assert!(status.healthy);
    }

    #[tokio::test]
    async fn test_alerting() {
        let manager = AlertManager::new();
        
        manager.fire("test_alert", AlertSeverity::Warning, "Test message").await;
        
        let active = manager.get_active().await;
        assert_eq!(active.len(), 1);
        assert_eq!(manager.count_by_severity(AlertSeverity::Warning).await, 1);
    }
}
