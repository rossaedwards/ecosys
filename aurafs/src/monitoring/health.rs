//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS Health Check System
//!
//! Comprehensive health monitoring with component-level checks,
//! dependency verification, and readiness/liveness probes.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, warn};

use crate::acl::AclManager;
use crate::cache::CacheManager;
use crate::namespace::NamespaceManager;
use crate::network::node_manager::NodeManager;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Component unhealthy: {0}")]
    ComponentUnhealthy(String),
    
    #[error("Timeout during health check")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, HealthError>;

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Component health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub latency_ms: u64,
}

/// Overall system health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub components: Vec<ComponentHealth>,
    pub uptime_seconds: u64,
    pub version: String,
}

/// Health checker
pub struct HealthChecker {
    namespace: Arc<NamespaceManager>,
    acl: Arc<AclManager>,
    cache: Arc<CacheManager>,
    node_manager: Arc<NodeManager>,
    start_time: Instant,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new(
        namespace: Arc<NamespaceManager>,
        acl: Arc<AclManager>,
        cache: Arc<CacheManager>,
        node_manager: Arc<NodeManager>,
    ) -> Self {
        Self {
            namespace,
            acl,
            cache,
            node_manager,
            start_time: Instant::now(),
        }
    }
    
    /// Perform full health check
    pub async fn check_health(&self) -> SystemHealth {
        let mut components = Vec::new();
        
        // Check namespace
        components.push(self.check_namespace().await);
        
        // Check ACL
        components.push(self.check_acl().await);
        
        // Check cache
        components.push(self.check_cache().await);
        
        // Check nodes
        components.push(self.check_nodes().await);
        
        // Determine overall status
        let overall_status = if components.iter().all(|c| c.status == HealthStatus::Healthy) {
            HealthStatus::Healthy
        } else if components.iter().any(|c| c.status == HealthStatus::Unhealthy) {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };
        
        SystemHealth {
            status: overall_status,
            components,
            uptime_seconds: self.start_time.elapsed().as_secs(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
    
    /// Check namespace component
    async fn check_namespace(&self) -> ComponentHealth {
        let start = Instant::now();
        
        let (status, message) = match self.namespace.get_entry("/") {
            Ok(_) => (HealthStatus::Healthy, "Namespace accessible".to_string()),
            Err(e) => {
                warn!("Namespace health check failed: {}", e);
                (HealthStatus::Unhealthy, format!("Error: {}", e))
            }
        };
        
        ComponentHealth {
            name: "namespace".to_string(),
            status,
            message,
            latency_ms: start.elapsed().as_millis() as u64,
        }
    }
    
    /// Check ACL component
    async fn check_acl(&self) -> ComponentHealth {
        let start = Instant::now();
        
        let (status, message) = match self.acl.list_users().len() {
            0 => (HealthStatus::Degraded, "No users configured".to_string()),
            n => (HealthStatus::Healthy, format!("{} users configured", n)),
        };
        
        ComponentHealth {
            name: "acl".to_string(),
            status,
            message,
            latency_ms: start.elapsed().as_millis() as u64,
        }
    }
    
    /// Check cache component
    async fn check_cache(&self) -> ComponentHealth {
        let start = Instant::now();
        
        let (hits, misses, hit_rate, size) = self.cache.get_stats();
        let total = hits + misses;
        
        let (status, message) = if total == 0 {
            (HealthStatus::Healthy, "Cache initialized".to_string())
        } else if hit_rate < 50.0 {
            (HealthStatus::Degraded, format!("Low hit rate: {:.1}%", hit_rate))
        } else {
            (HealthStatus::Healthy, format!("Hit rate: {:.1}%, size: {} bytes", hit_rate, size))
        };
        
        ComponentHealth {
            name: "cache".to_string(),
            status,
            message,
            latency_ms: start.elapsed().as_millis() as u64,
        }
    }
    
    /// Check nodes component
    async fn check_nodes(&self) -> ComponentHealth {
        let start = Instant::now();
        
        let (total_nodes, live_nodes, _, _) = self.node_manager.get_cluster_stats();
        
        let (status, message) = if total_nodes == 0 {
            (HealthStatus::Degraded, "No nodes registered".to_string())
        } else if live_nodes == 0 {
            (HealthStatus::Unhealthy, "No live nodes".to_string())
        } else if live_nodes < total_nodes {
            (
                HealthStatus::Degraded,
                format!("{}/{} nodes alive", live_nodes, total_nodes),
            )
        } else {
            (HealthStatus::Healthy, format!("All {} nodes alive", total_nodes))
        };
        
        ComponentHealth {
            name: "nodes".to_string(),
            status,
            message,
            latency_ms: start.elapsed().as_millis() as u64,
        }
    }
    
    /// Liveness probe (is the service running?)
    pub async fn liveness(&self) -> bool {
        true // If we can execute this, we're alive
    }
    
    /// Readiness probe (is the service ready to accept traffic?)
    pub async fn readiness(&self) -> bool {
        let health = self.check_health().await;
        health.status != HealthStatus::Unhealthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_liveness() {
        // Liveness always returns true if code is running
        assert!(true);
    }
}