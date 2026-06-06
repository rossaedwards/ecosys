//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Module - Enterprise-Grade Quantum Mesh Networking
//! 🌐 Production P2P Mesh + Discovery + Replication + Security + Monitoring
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]
#![deny(missing_docs)]

// Core network components
pub mod peer;
pub mod mesh;
pub mod mesh_gossip;
pub mod gossip;
pub mod p2p;
pub mod node_manager;
pub mod orchestrator;
pub mod discovery;
pub mod replication;
pub mod secure_tunnel;
pub mod firewall;
pub mod autoheal_daemon;

// Subsystems
pub mod meshwerk;
pub mod meshtastic_integration;
pub mod transport;
pub mod monitoring;
pub mod defense;
pub mod integration;
pub mod reticulum_bridge;
pub mod packet;
pub mod rns_client;
pub mod rns_bridge;

// Re-exports for convenience
pub use peer::{Peer, PeerState};
pub use mesh::{Mesh, MeshConfig, GossipMessage};
pub use node_manager::{NodeManager, PeerNode};
pub use orchestrator::{Orchestrator, OrchestratorConfig};
pub use discovery::{DiscoveryEngine, DiscoveryConfig};
pub use replication::{ReplicationEngine, ReplicationConfig, ReplicationStatus};
pub use secure_tunnel::{SecureTunnel, TunnelConfig};
pub use firewall::{Firewall, FirewallConfig, FirewallRules, Operation as FirewallOperation};
pub use reticulum_bridge::{ReticulumBridge, NetworkLane, LaneStatus, ReplicationType};
pub use packet::{NetworkPacket, PacketType, PacketHeader, HandshakePacket, PacketError};
pub use rns_client::{RNSClient, BridgeStats, PacketHandlerStats};
pub use rns_bridge::{RNSBridgeManager, ProcessStatus};

/// Enterprise-grade network errors
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Peer error: {0}")]
    PeerError(String),
    #[error("Mesh error: {0}")]
    MeshError(String),
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
    #[error("Replication error: {0}")]
    ReplicationError(String),
    #[error("Security error: {0}")]
    SecurityError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Operation timeout")]
    Timeout,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type NetworkResult<T> = std::result::Result<T, NetworkError>;

/// Network module initialization
pub async fn initialize_network() -> NetworkResult<()> {
    tracing::info!("🌐 Initializing AuraFS Network Module");
    Ok(())
}
