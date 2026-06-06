//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Mesh Module - FRACTAL P2P ORCHESTRATOR
//! 🕸️ Complete Mesh API + Re-exports + Production Macros + Type Aliases
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]
#![deny(missing_docs)]

pub use self::{
    core::{FractalNode, ChordDht, XorDistance, FingerTable},
    protocol::{MeshProtocol, MeshMessage, PingPong, HandshakeResponse},
    routing::{RouteTable, AdaptiveRouter, ShardRoute, RouteMetrics},
    swarm::{SwarmController, ConnectionState, CircuitBreaker},
};

pub mod core;
pub mod protocol;
pub mod routing;
pub mod swarm;

/// Unified mesh result type
pub type Result<T> = std::result::Result<T, Error>;

/// Core mesh error enum
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Fractal routing error: {0}")]
    Routing(#[from] routing::RoutingError),
    #[error("Protocol error: {0}")]
    Protocol(#[from] protocol::ProtocolError),
    #[error("Swarm error: {0}")]
    Swarm(#[from] swarm::SwarmError),
    #[error("Core DHT error: {0}")]
    Core(#[from] core::CoreError),
    #[error("No route to peer")]
    NoRoute,
    #[error("Mesh timeout")]
    Timeout,
    #[error("Circuit breaker tripped")]
    CircuitBreaker,
}

/// Fractal mesh configuration
#[derive(Debug, Clone)]
pub struct MeshConfig {
    /// Local node ID (BlissId)
    pub local_id: crate::gov::BlissId,
    
    /// Listen address for QUIC/gRPC
    pub listen_addr: std::net::SocketAddr,
    
    /// DHT finger table size (2^n)
    pub finger_table_bits: usize,
    
    /// Maximum concurrent connections
    pub max_connections: usize,
    
    /// Gossip fanout
    pub gossip_fanout: usize,
    
    /// Connection timeout
    pub connection_timeout: std::time::Duration,
    
    /// Stabilization interval
    pub stabilization_interval: std::time::Duration,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            local_id: crate::gov::BlissId::genesis(),
            listen_addr: "0.0.0.0:7000".parse().unwrap(),
            finger_table_bits: 160,  // SHA1-sized IDs
            max_connections: 1000,
            gossip_fanout: 8,
            connection_timeout: std::time::Duration::from_secs(10),
            stabilization_interval: std::time::Duration::from_secs(30),
        }
    }
}

/// PRODUCTION QUICK-START MACROS
#[macro_export]
macro_rules! fractal_mesh {
    // Instant production mesh
    ($addr:expr) => {{
        $crate::mesh::fractal_mesh!(addr: $addr, bits: 160, fanout: 8)
    }};
    
    // Full configurable fractal mesh
    (addr: $addr:expr, bits: $bits:expr, fanout: $fanout:expr) => {{
        let config = $crate::mesh::MeshConfig {
            listen_addr: $addr.parse().unwrap(),
            finger_table_bits: $bits,
            gossip_fanout: $fanout,
            ..Default::default()
        };
        $crate::mesh::production_mesh(config).await.unwrap()
    }};
}

/// Production mesh bundle - Complete fractal stack
pub async fn production_mesh(config: MeshConfig) -> Result<(Arc<SwarmController>, Arc<AdaptiveRouter>)> {
    // 1. Fractal DHT core
    let dht = Arc::new(ChordDht::new(
        config.local_id.clone(),
        config.finger_table_bits,
    ));
    
    // 2. Adaptive routing layer
    let router = Arc::new(AdaptiveRouter::new(
        Arc::clone(&dht),
        config.gossip_fanout,
    ));
    
    // 3. Swarm controller (QUIC + connection mgmt)
    let swarm = Arc::new(SwarmController::new(
        config.clone(),
        Arc::clone(&router),
    ));
    
    // 4. Start stabilization protocol
    swarm.start_stabilization(config.stabilization_interval).await;
    
    Ok((swarm, router))
}

/// Mesh health metrics snapshot
#[derive(Debug, Clone, serde::Serialize)]
pub struct MeshHealth {
    pub node_id: crate::gov::BlissId,
    pub connected_peers: usize,
    pub finger_table_size: usize,
    pub route_success_rate: f32,
    pub avg_latency_ms: f64,
    pub healthy: bool,
}

/// Quick mesh health check
pub async fn mesh_health(swarm: &Arc<SwarmController>) -> Result<MeshHealth> {
    let stats = swarm.metrics().await;
    Ok(MeshHealth {
        node_id: swarm.config().local_id.clone(),
        connected_peers: stats.connected_peers,
        finger_table_size: stats.finger_table_size,
        route_success_rate: stats.route_success_rate,
        avg_latency_ms: stats.avg_latency_ms,
        healthy: stats.healthy,
    })
}

/// Feature-gated production mesh stack
#[cfg(feature = "full")]
pub mod full {
    pub use super::*;
    pub use crate::network::{PeerState, SecureTunnel};
    
    /// Production mesh daemon startup
    pub async fn start_mesh_daemon(addr: std::net::SocketAddr) -> Result<()> {
        let mesh = fractal_mesh!(addr);
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}

/// Chord DHT distance metric (XOR)
pub fn xor_distance(id1: &crate::gov::BlissId, id2: &crate::gov::BlissId) -> u128 {
    crate::mesh::core::xor_distance(&id1.0, &id2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mesh_production_bundle() {
        let config = MeshConfig {
            listen_addr: "127.0.0.1:7001".parse().unwrap(),
            ..Default::default()
        };
        
        let (swarm, router) = production_mesh(config).await.unwrap();
        
        assert_eq!(swarm.config().finger_table_bits, 160);
        let health = mesh_health(&swarm).await.unwrap();
        assert!(health.connected_peers >= 0);
    }
    
    #[tokio::test]
    async fn test_mesh_macros() {
        let mesh = fractal_mesh!("127.0.0.1:7002");
        assert!(mesh.is_ok());
    }
}