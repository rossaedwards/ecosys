//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Mesh Swarm - Self-Organizing + Auto-Scaling Mesh Fabric
//! 🕸️ Connection Management + Circuit Breaking + Exponential Backoff + Health Checks
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::BlissId,
    mesh::{
        core::{FractalNode, ChordDht},
        protocol::MeshProtocol,
        routing::{AdaptiveRouter, RouteTable},
    },
};
use std::{
    sync::Arc,
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};
use tokio::{
    sync::RwLock,
    time::{interval, timeout},
};
use tracing::{info, debug, warn, error};

/// Swarm error enum
#[derive(Debug, thiserror::Error)]
pub enum SwarmError {
    #[error("Connection limit exceeded")]
    ConnectionLimit,
    #[error("Circuit breaker tripped")]
    CircuitBreakerTripped,
    #[error("Peer unreachable")]
    PeerUnreachable,
    #[error("Health check failed")]
    HealthCheckFailed,
}

/// Connection lifecycle states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Degraded,
    Disconnected,
    Banned,
}

/// Circuit breaker for failure detection and recovery
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    recovery_timeout: Duration,
    consecutive_failures: usize,
    last_failure: Instant,
    state: CircuitBreakerState,
}

#[derive(Debug, Clone, Copy)]
enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            consecutive_failures: 0,
            last_failure: Instant::now(),
            state: CircuitBreakerState::Closed,
        }
    }
    
    pub fn on_failure(&mut self) {
        self.consecutive_failures += 1;
        self.last_failure = Instant::now();
        
        if self.consecutive_failures >= self.failure_threshold {
            self.state = CircuitBreakerState::Open;
        }
    }
    
    pub fn on_success(&mut self) {
        if matches!(self.state, CircuitBreakerState::HalfOpen) {
            self.state = CircuitBreakerState::Closed;
        }
        self.consecutive_failures = 0;
    }
    
    pub fn allow_request(&self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::HalfOpen => true,
            CircuitBreakerState::Open => {
                Instant::now().duration_since(self.last_failure) > self.recovery_timeout
            }
        }
    }
}

/// Swarm metrics snapshot
#[derive(Debug, Clone, Default)]
pub struct SwarmMetrics {
    pub connected_peers: usize,
    pub total_peers: usize,
    pub healthy_peers: usize,
    pub avg_latency_ms: f64,
    pub connection_success_rate: f32,
    pub circuit_breakers_open: usize,
}

/// Production self-organizing swarm controller
pub struct SwarmController {
    /// Local node identity
    local_node: Arc<FractalNode>,
    
    /// Chord DHT core
    dht: Arc<ChordDht>,
    
    /// Adaptive routing layer
    router: Arc<AdaptiveRouter>,
    
    /// QUIC mesh protocol
    protocol: Arc<MeshProtocol>,
    
    /// Active connections (peer_id → ConnectionInfo)
    connections: RwLock<HashMap<BlissId, ConnectionInfo>>,
    
    /// Circuit breakers per peer
    circuit_breakers: RwLock<HashMap<BlissId, CircuitBreaker>>,
    
    /// Configuration
    config: crate::mesh::MeshConfig,
    
    /// Health metrics
    metrics: RwLock<SwarmMetrics>,
}

#[derive(Debug)]
struct ConnectionInfo {
    peer: Arc<FractalNode>,
    state: ConnectionState,
    last_health_check: Instant,
    latency_ms: f64,
    circuit_breaker: CircuitBreaker,
    backoff_duration: Duration,
}

impl SwarmController {
    /// Forge production swarm controller
    pub fn new(config: crate::mesh::MeshConfig, router: Arc<AdaptiveRouter>) -> Self {
        let local_node = Arc::new(FractalNode::new(
            config.local_id.clone(),
            config.listen_addr,
            Some("Swarm Node".to_string()),
        ));
        
        let dht = Arc::new(ChordDht::new(config.local_id.clone(), config.finger_table_bits));
        let protocol = Arc::new(MeshProtocol::new(local_node.clone(), Default::default()).unwrap());
        
        Self {
            local_node,
            dht,
            router,
            protocol,
            connections: RwLock::new(HashMap::new()),
            circuit_breakers: RwLock::new(HashMap::new()),
            config,
            metrics: RwLock::new(SwarmMetrics::default()),
        }
    }
    
    /// Connect to peer with exponential backoff + circuit breaking
    pub async fn connect_peer(&self, peer: Arc<FractalNode>) -> Result<(), SwarmError> {
        let mut connections = self.connections.write().await;
        
        if connections.len() >= self.config.max_connections {
            return Err(SwarmError::ConnectionLimit);
        }
        
        let peer_id = peer.id.clone();
        let circuit = self.get_or_create_circuit_breaker(&peer_id).await;
        
        if !circuit.allow_request() {
            return Err(SwarmError::CircuitBreakerTripped);
        }
        
        // Exponential backoff
        let backoff = connections.entry(peer_id.clone())
            .or_insert_with(|| ConnectionInfo {
                peer: peer.clone(),
                state: ConnectionState::Connecting,
                last_health_check: Instant::now(),
                latency_ms: f64::MAX,
                circuit_breaker: circuit.clone(),
                backoff_duration: Duration::from_millis(100),
            })
            .backoff_duration;
        
        tokio::time::sleep(backoff).await;
        
        // Attempt QUIC handshake
        match timeout(Duration::from_secs(10), self.protocol.handshake(&peer)).await {
            Ok(Ok(_)) => {
                let conn_info = connections.get_mut(&peer_id).unwrap();
                conn_info.state = ConnectionState::Connected;
                conn_info.backoff_duration = Duration::from_millis(100); // Reset backoff
                self.router.record_success(peer_id.clone()).await;
                info!("✅ Connected to peer {}", peer.id);
                Ok(())
            }
            _ => {
                let conn_info = connections.entry(peer_id).or_default();
                conn_info.circuit_breaker.on_failure();
                conn_info.backoff_duration *= 2; // Exponential backoff
                conn_info.state = ConnectionState::Degraded;
                self.router.record_failure(peer_id.clone()).await;
                Err(SwarmError::PeerUnreachable)
            }
        }
    }
    
    /// Periodic health checks + peer pruning
    pub async fn health_check_loop(&self) {
        let mut interval = interval(Duration::from_secs(15));
        
        loop {
            interval.tick().await;
            self.perform_health_checks().await;
            self.update_metrics().await;
            self.prune_dead_peers().await;
        }
    }
    
    /// Perform health checks on all connections
    async fn perform_health_checks(&self) {
        let connections = self.connections.read().await;
        let mut health_checks = Vec::new();
        
        for (peer_id, conn_info) in connections.iter() {
            if Instant::now().duration_since(conn_info.last_health_check) > Duration::from_secs(30) {
                let protocol = Arc::clone(&self.protocol);
                let peer_node = Arc::clone(&conn_info.peer);
                health_checks.push(async move {
                    match timeout(Duration::from_secs(5), protocol.ping(&peer_node)).await {
                        Ok(Ok(rtt_ns)) => {
                            let rtt_ms = rtt_ns as f64 / 1_000_000.0;
                            (peer_id.clone(), Ok(rtt_ms))
                        }
                        _ => (peer_id.clone(), Err(SwarmError::HealthCheckFailed)),
                    }
                });
            }
        }
        
        let results = futures::future::join_all(health_checks).await;
        for (peer_id, result) in results {
            match result {
                Ok(latency) => {
                    if let Some(conn_info) = self.connections.write().await.get_mut(&peer_id) {
                        conn_info.latency_ms = latency;
                        conn_info.last_health_check = Instant::now();
                        conn_info.state = ConnectionState::Connected;
                        conn_info.circuit_breaker.on_success();
                    }
                }
                Err(_) => {
                    if let Some(conn_info) = self.connections.write().await.get_mut(&peer_id) {
                        conn_info.circuit_breaker.on_failure();
                        conn_info.state = ConnectionState::Degraded;
                    }
                }
            }
        }
    }
    
    /// Prune permanently dead peers
    async fn prune_dead_peers(&self) {
        let mut connections = self.connections.write().await;
        let mut to_remove = Vec::new();
        
        for (peer_id, conn_info) in connections.iter() {
            if matches!(conn_info.state, ConnectionState::Banned) ||
               (matches!(conn_info.state, ConnectionState::Degraded) &&
                Instant::now().duration_since(conn_info.last_health_check) > Duration::from_secs(300)) {
                to_remove.push(peer_id.clone());
            }
        }
        
        for peer_id in to_remove {
            connections.remove(&peer_id);
            self.circuit_breakers.write().await.remove(&peer_id);
            warn!("🗑️ Pruned dead peer {}", peer_id);
        }
    }
    
    /// Start DHT stabilization protocol
    pub async fn start_stabilization(&self, interval: Duration) {
        let dht = Arc::clone(&self.dht);
        tokio::spawn(async move {
            let mut stab_interval = interval(interval);
            loop {
                stab_interval.tick().await;
                dht.stabilize().await;
            }
        });
    }
    
    /// Get swarm metrics snapshot
    pub async fn metrics(&self) -> SwarmMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    async fn get_or_create_circuit_breaker(&self, peer_id: &BlissId) -> CircuitBreaker {
        let mut breakers = self.circuit_breakers.write().await;
        breakers.entry(peer_id.clone())
            .or_insert_with(|| CircuitBreaker::new(5, Duration::from_secs(60)))
            .clone()
    }
    
    async fn update_metrics(&self) {
        let connections = self.connections.read().await;
        let mut metrics = self.metrics.write().await;
        
        metrics.connected_peers = connections.values()
            .filter(|c| matches!(c.state, ConnectionState::Connected))
            .count();
        metrics.total_peers = connections.len();
        metrics.healthy_peers = connections.values()
            .filter(|c| c.latency_ms < 100.0)
            .count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(10));
        
        // Simulate 3 failures → open
        breaker.on_failure();
        breaker.on_failure();
        breaker.on_failure();
        assert!(!breaker.allow_request());
        
        // Simulate recovery timeout
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(breaker.allow_request()); // Still open until timeout
    }
}