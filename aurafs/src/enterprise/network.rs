//! Connection Pool with Health Checks
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Manages connection pooling with automatic health checks and reconnection.

use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use crate::error::{RafsError, Result};
use tracing::{debug, info, warn};

/// Peer identifier
pub type PeerId = String;

/// Connection trait
#[async_trait::async_trait]
pub trait Connection: Send + Sync {
    async fn ping(&self) -> Result<()>;
    async fn is_healthy(&self) -> bool;
}

/// Connection health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Pooled connection wrapper
struct PooledConnection {
    connection: Arc<dyn Connection>,
    last_used: Instant,
    health_status: ConnectionHealth,
    use_count: usize,
    created_at: Instant,
}

/// Connection pool implementation
pub struct ConnectionPool {
    connections: Arc<RwLock<HashMap<PeerId, PooledConnection>>>,
    max_connections: usize,
    idle_timeout: Duration,
    health_check_interval: Duration,
}

impl ConnectionPool {
    /// Create new connection pool
    pub fn new(
        max_connections: usize,
        idle_timeout: Duration,
        health_check_interval: Duration,
    ) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            max_connections,
            idle_timeout,
            health_check_interval,
        }
    }

    /// Get connection for peer (create if needed)
    pub async fn get_connection<F>(&self, peer_id: &PeerId, create_fn: F) -> Result<Arc<dyn Connection>>
    where
        F: std::future::Future<Output = Result<Arc<dyn Connection>>>,
    {
        let mut connections = self.connections.write().await;
        
        // Check if connection exists and is healthy
        if let Some(pooled) = connections.get_mut(peer_id) {
            if pooled.health_status == ConnectionHealth::Healthy {
                pooled.last_used = Instant::now();
                pooled.use_count += 1;
                return Ok(pooled.connection.clone());
            } else {
                // Remove unhealthy connection
                connections.remove(peer_id);
            }
        }
        
        // Create new connection if under limit
        if connections.len() >= self.max_connections {
            // Evict least recently used unhealthy connection
            self.evict_unhealthy_connection(&mut connections).await;
        }
        
        let connection = create_fn().await?;
        let pooled = PooledConnection {
            connection: connection.clone(),
            last_used: Instant::now(),
            health_status: ConnectionHealth::Healthy,
            use_count: 1,
            created_at: Instant::now(),
        };
        
        connections.insert(peer_id.clone(), pooled);
        info!("Created new connection for peer: {}", peer_id);
        
        Ok(connection)
    }

    /// Start health check background task
    pub async fn start_health_checks(&self) {
        let connections = self.connections.clone();
        let health_check_interval = self.health_check_interval;
        let idle_timeout = self.idle_timeout;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(health_check_interval);
            
            loop {
                interval.tick().await;
                Self::check_all_connections(&connections, idle_timeout).await;
            }
        });
    }

    /// Check all connections health
    async fn check_all_connections(
        connections: &Arc<RwLock<HashMap<PeerId, PooledConnection>>>,
        idle_timeout: Duration,
    ) {
        let mut connections = connections.write().await;
        let mut to_remove = Vec::new();
        
        for (peer_id, pooled) in connections.iter_mut() {
            let health = if pooled.connection.is_healthy().await {
                ConnectionHealth::Healthy
            } else {
                ConnectionHealth::Unhealthy
            };
            
            pooled.health_status = health;
            
            // Remove unhealthy connections that haven't been used recently
            if health == ConnectionHealth::Unhealthy 
                && pooled.last_used.elapsed() > idle_timeout {
                to_remove.push(peer_id.clone());
            }
        }
        
        for peer_id in to_remove {
            connections.remove(&peer_id);
            debug!("Removed unhealthy idle connection: {}", peer_id);
        }
    }

    /// Evict least recently used unhealthy connection
    async fn evict_unhealthy_connection(
        &self,
        connections: &mut HashMap<PeerId, PooledConnection>,
    ) {
        let mut oldest_unhealthy: Option<(PeerId, Instant)> = None;
        
        for (peer_id, pooled) in connections.iter() {
            if pooled.health_status == ConnectionHealth::Unhealthy {
                if let Some((_, oldest_time)) = oldest_unhealthy {
                    if pooled.last_used < oldest_time {
                        oldest_unhealthy = Some((peer_id.clone(), pooled.last_used));
                    }
                } else {
                    oldest_unhealthy = Some((peer_id.clone(), pooled.last_used));
                }
            }
        }
        
        if let Some((peer_id, _)) = oldest_unhealthy {
            connections.remove(&peer_id);
            debug!("Evicted unhealthy connection: {}", peer_id);
        }
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let connections = self.connections.read().await;
        let mut healthy = 0;
        let mut degraded = 0;
        let mut unhealthy = 0;
        
        for pooled in connections.values() {
            match pooled.health_status {
                ConnectionHealth::Healthy => healthy += 1,
                ConnectionHealth::Degraded => degraded += 1,
                ConnectionHealth::Unhealthy => unhealthy += 1,
            }
        }
        
        PoolStats {
            total: connections.len(),
            healthy,
            degraded,
            unhealthy,
        }
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total: usize,
    pub healthy: usize,
    pub degraded: usize,
    pub unhealthy: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    struct MockConnection {
        healthy: Arc<AtomicBool>,
    }

    #[async_trait::async_trait]
    impl Connection for MockConnection {
        async fn ping(&self) -> Result<()> {
            if self.healthy.load(Ordering::SeqCst) {
                Ok(())
            } else {
                Err(RafsError::NetworkError("unhealthy".to_string()))
            }
        }

        async fn is_healthy(&self) -> bool {
            self.healthy.load(Ordering::SeqCst)
        }
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let pool = ConnectionPool::new(
            10,
            Duration::from_secs(60),
            Duration::from_secs(5),
        );

        let peer_id = "peer1".to_string();
        let healthy = Arc::new(AtomicBool::new(true));

        let connection = pool
            .get_connection(&peer_id, async {
                Ok(Arc::new(MockConnection {
                    healthy: healthy.clone(),
                }) as Arc<dyn Connection>)
            })
            .await
            .unwrap();

        assert!(connection.is_healthy().await);

        let stats = pool.stats().await;
        assert_eq!(stats.total, 1);
        assert_eq!(stats.healthy, 1);
    }
}

