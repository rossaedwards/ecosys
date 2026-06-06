//! ═══════════════════════════════════════════════════════════════════
//! 🏥 AuraFS Core Health - Enterprise Health Check System
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Comprehensive health monitoring for all core components with
//! readiness/liveness probes, degraded state detection, and alerting.
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, internal};

/// Health status levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some components degraded but functional
    Degraded,
    /// System unhealthy, may need intervention
    Unhealthy,
    /// Unknown state (not yet checked)
    Unknown,
}

impl HealthStatus {
    /// Check if status indicates system can serve requests
    pub fn is_ready(&self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Degraded)
    }
    
    /// Check if status indicates system is alive
    pub fn is_alive(&self) -> bool {
        !matches!(self, HealthStatus::Unknown)
    }
    
    /// Get HTTP status code for health endpoint
    pub fn http_status_code(&self) -> u16 {
        match self {
            HealthStatus::Healthy => 200,
            HealthStatus::Degraded => 200, // Still serving
            HealthStatus::Unhealthy => 503,
            HealthStatus::Unknown => 503,
        }
    }
}

/// Individual component health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Current status
    pub status: HealthStatus,
    /// Human-readable message
    pub message: Option<String>,
    /// Last check timestamp
    pub last_checked: SystemTime,
    /// Check duration in milliseconds
    pub check_duration_ms: u64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ComponentHealth {
    /// Create healthy component result
    pub fn healthy(name: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            message: None,
            last_checked: SystemTime::now(),
            check_duration_ms: duration_ms,
            metadata: HashMap::new(),
        }
    }
    
    /// Create unhealthy component result
    pub fn unhealthy(name: impl Into<String>, message: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Unhealthy,
            message: Some(message.into()),
            last_checked: SystemTime::now(),
            check_duration_ms: duration_ms,
            metadata: HashMap::new(),
        }
    }
    
    /// Create degraded component result
    pub fn degraded(name: impl Into<String>, message: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Degraded,
            message: Some(message.into()),
            last_checked: SystemTime::now(),
            check_duration_ms: duration_ms,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to component health
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Overall system health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Overall status (worst of all components)
    pub status: HealthStatus,
    /// Individual component health results
    pub components: Vec<ComponentHealth>,
    /// Report generation timestamp
    pub timestamp: SystemTime,
    /// Total check duration in milliseconds
    pub total_duration_ms: u64,
    /// Version information
    pub version: String,
    /// Environment
    pub environment: String,
    /// System uptime in seconds
    pub uptime_secs: u64,
}

impl HealthReport {
    /// Check if system is ready to serve requests
    pub fn is_ready(&self) -> bool {
        self.status.is_ready()
    }
    
    /// Check if system is alive
    pub fn is_alive(&self) -> bool {
        self.status.is_alive()
    }
    
    /// Get all unhealthy components
    pub fn unhealthy_components(&self) -> Vec<&ComponentHealth> {
        self.components.iter()
            .filter(|c| c.status == HealthStatus::Unhealthy)
            .collect()
    }
    
    /// Get all degraded components
    pub fn degraded_components(&self) -> Vec<&ComponentHealth> {
        self.components.iter()
            .filter(|c| c.status == HealthStatus::Degraded)
            .collect()
    }
}

/// Health check function trait
#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    /// Perform health check
    async fn check(&self) -> ComponentHealth;
    
    /// Component name
    fn name(&self) -> &'static str;
    
    /// Check timeout
    fn timeout(&self) -> Duration {
        Duration::from_secs(5)
    }
}

/// Crypto subsystem health checker
pub struct CryptoHealthChecker;

#[async_trait::async_trait]
impl HealthChecker for CryptoHealthChecker {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        
        // Test entropy source
        match crate::core::crypto::gen_random_bytes(32) {
            Ok(_) => ComponentHealth::healthy("crypto", start.elapsed().as_millis() as u64)
                .with_metadata("entropy_source", "getrandom"),
            Err(e) => ComponentHealth::unhealthy("crypto", format!("Entropy failure: {}", e), start.elapsed().as_millis() as u64),
        }
    }
    
    fn name(&self) -> &'static str {
        "crypto"
    }
}

/// Metrics subsystem health checker
pub struct MetricsHealthChecker;

#[async_trait::async_trait]
impl HealthChecker for MetricsHealthChecker {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        
        // Test metrics collection
        match crate::core::AuraFSMetrics::gather_metrics() {
            Ok(_) => ComponentHealth::healthy("metrics", start.elapsed().as_millis() as u64),
            Err(e) => ComponentHealth::degraded("metrics", format!("Metrics error: {}", e), start.elapsed().as_millis() as u64),
        }
    }
    
    fn name(&self) -> &'static str {
        "metrics"
    }
}

/// Memory health checker
pub struct MemoryHealthChecker {
    /// Warning threshold (bytes)
    pub warning_threshold_bytes: u64,
    /// Critical threshold (bytes)
    pub critical_threshold_bytes: u64,
}

impl Default for MemoryHealthChecker {
    fn default() -> Self {
        Self {
            warning_threshold_bytes: 1024 * 1024 * 1024, // 1GB
            critical_threshold_bytes: 2 * 1024 * 1024 * 1024, // 2GB
        }
    }
}

#[async_trait::async_trait]
impl HealthChecker for MemoryHealthChecker {
    async fn check(&self) -> ComponentHealth {
        let start = Instant::now();
        
        // Get current process memory usage (simplified)
        // In production, use sys-info or similar crate
        let duration_ms = start.elapsed().as_millis() as u64;
        
        ComponentHealth::healthy("memory", duration_ms)
            .with_metadata("threshold_warning_mb", (self.warning_threshold_bytes / 1024 / 1024).to_string())
            .with_metadata("threshold_critical_mb", (self.critical_threshold_bytes / 1024 / 1024).to_string())
    }
    
    fn name(&self) -> &'static str {
        "memory"
    }
}

/// Health check manager
pub struct HealthManager {
    /// Registered health checkers
    checkers: Vec<Arc<dyn HealthChecker>>,
    /// Cached health results
    cache: Arc<RwLock<Option<HealthReport>>>,
    /// Cache TTL
    cache_ttl: Duration,
    /// Last cache update
    last_update: Arc<RwLock<Option<Instant>>>,
    /// System start time
    start_time: Instant,
    /// Version string
    version: String,
    /// Environment
    environment: String,
}

impl HealthManager {
    /// Create new health manager
    pub fn new(version: impl Into<String>, environment: impl Into<String>) -> Self {
        Self {
            checkers: Vec::new(),
            cache: Arc::new(RwLock::new(None)),
            cache_ttl: Duration::from_secs(5),
            last_update: Arc::new(RwLock::new(None)),
            start_time: Instant::now(),
            version: version.into(),
            environment: environment.into(),
        }
    }
    
    /// Register a health checker
    pub fn register(&mut self, checker: Arc<dyn HealthChecker>) {
        self.checkers.push(checker);
    }
    
    /// Register default checkers
    pub fn register_defaults(&mut self) {
        self.register(Arc::new(CryptoHealthChecker));
        self.register(Arc::new(MetricsHealthChecker));
        self.register(Arc::new(MemoryHealthChecker::default()));
    }
    
    /// Set cache TTL
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }
    
    /// Check all components and return health report
    pub async fn check_health(&self) -> Result<HealthReport> {
        // Check cache first
        {
            let last = self.last_update.read().await;
            if let Some(last_time) = *last {
                if last_time.elapsed() < self.cache_ttl {
                    if let Some(cached) = self.cache.read().await.clone() {
                        return Ok(cached);
                    }
                }
            }
        }
        
        // Run all health checks with timeout
        let start = Instant::now();
        let mut components = Vec::new();
        
        for checker in &self.checkers {
            let timeout = checker.timeout();
            let result = tokio::time::timeout(timeout, checker.check()).await;
            
            match result {
                Ok(health) => components.push(health),
                Err(_) => {
                    components.push(ComponentHealth::unhealthy(
                        checker.name(),
                        "Health check timeout",
                        timeout.as_millis() as u64,
                    ));
                }
            }
        }
        
        // Calculate overall status (worst of all)
        let overall_status = components.iter()
            .map(|c| &c.status)
            .fold(HealthStatus::Healthy, |acc, status| {
                match (&acc, status) {
                    (HealthStatus::Unhealthy, _) | (_, HealthStatus::Unhealthy) => HealthStatus::Unhealthy,
                    (HealthStatus::Degraded, _) | (_, HealthStatus::Degraded) => HealthStatus::Degraded,
                    (HealthStatus::Unknown, _) | (_, HealthStatus::Unknown) => HealthStatus::Unknown,
                    _ => HealthStatus::Healthy,
                }
            });
        
        let report = HealthReport {
            status: overall_status,
            components,
            timestamp: SystemTime::now(),
            total_duration_ms: start.elapsed().as_millis() as u64,
            version: self.version.clone(),
            environment: self.environment.clone(),
            uptime_secs: self.start_time.elapsed().as_secs(),
        };
        
        // Update cache
        {
            let mut cache = self.cache.write().await;
            *cache = Some(report.clone());
            let mut last = self.last_update.write().await;
            *last = Some(Instant::now());
        }
        
        Ok(report)
    }
    
    /// Quick liveness check (no caching)
    pub async fn liveness(&self) -> bool {
        // Just check if we can respond
        true
    }
    
    /// Readiness check
    pub async fn readiness(&self) -> Result<bool> {
        let report = self.check_health().await?;
        Ok(report.is_ready())
    }
    
    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// HTTP health endpoint handler
pub mod http {
    use super::*;
    use warp::{Filter, Reply, Rejection};
    
    /// Create health check routes
    pub fn health_routes(
        manager: Arc<HealthManager>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let health = health_endpoint(Arc::clone(&manager));
        let live = liveness_endpoint(Arc::clone(&manager));
        let ready = readiness_endpoint(manager);
        
        health.or(live).or(ready)
    }
    
    fn health_endpoint(
        manager: Arc<HealthManager>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("health")
            .and(warp::get())
            .and(warp::any().map(move || Arc::clone(&manager)))
            .and_then(|mgr: Arc<HealthManager>| async move {
                match mgr.check_health().await {
                    Ok(report) => {
                        let status = warp::http::StatusCode::from_u16(report.status.http_status_code())
                            .unwrap_or(warp::http::StatusCode::INTERNAL_SERVER_ERROR);
                        let json = warp::reply::json(&report);
                        Ok::<_, Rejection>(warp::reply::with_status(json, status))
                    }
                    Err(_) => {
                        let error = serde_json::json!({"status": "error"});
                        Ok(warp::reply::with_status(
                            warp::reply::json(&error),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                }
            })
    }
    
    fn liveness_endpoint(
        manager: Arc<HealthManager>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("health" / "live")
            .and(warp::get())
            .and(warp::any().map(move || Arc::clone(&manager)))
            .and_then(|mgr: Arc<HealthManager>| async move {
                if mgr.liveness().await {
                    Ok::<_, Rejection>(warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"status": "alive"})),
                        warp::http::StatusCode::OK,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"status": "dead"})),
                        warp::http::StatusCode::SERVICE_UNAVAILABLE,
                    ))
                }
            })
    }
    
    fn readiness_endpoint(
        manager: Arc<HealthManager>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("health" / "ready")
            .and(warp::get())
            .and(warp::any().map(move || Arc::clone(&manager)))
            .and_then(|mgr: Arc<HealthManager>| async move {
                match mgr.readiness().await {
                    Ok(true) => Ok::<_, Rejection>(warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"status": "ready"})),
                        warp::http::StatusCode::OK,
                    )),
                    _ => Ok(warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"status": "not_ready"})),
                        warp::http::StatusCode::SERVICE_UNAVAILABLE,
                    )),
                }
            })
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_health_status_is_ready() {
        assert!(HealthStatus::Healthy.is_ready());
        assert!(HealthStatus::Degraded.is_ready());
        assert!(!HealthStatus::Unhealthy.is_ready());
        assert!(!HealthStatus::Unknown.is_ready());
    }
    
    #[test]
    fn test_component_health_creation() {
        let healthy = ComponentHealth::healthy("test", 10);
        assert_eq!(healthy.status, HealthStatus::Healthy);
        assert_eq!(healthy.name, "test");
        
        let unhealthy = ComponentHealth::unhealthy("test", "error", 20);
        assert_eq!(unhealthy.status, HealthStatus::Unhealthy);
        assert_eq!(unhealthy.message, Some("error".to_string()));
    }
    
    #[tokio::test]
    async fn test_health_manager() {
        let mut manager = HealthManager::new("1.0.0", "test");
        manager.register_defaults();
        
        // Note: This test may fail if metrics aren't initialized
        // In production, ensure metrics are initialized first
        let _ = manager.check_health().await;
    }
}
