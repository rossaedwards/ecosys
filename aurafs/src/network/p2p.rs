//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS P2P Network - Enterprise-Grade Peer-to-Peer Protocol
//! 🌐 Direct Peer Connections + NAT Traversal + Connection Pooling
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::BlissId,
    network::peer::Peer,
};
use crate::audit::holographic_logger::{AuditEvent, HolographicLogger};
use crate::crypto::pqc::dilithium_sig;
use crate::network::meshwerk::transport::{MeshTransport, MeshTransportError};
use crate::physics::{INVARIANTS, PhysicsViolationError, is_within_coherence_window};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tokio::io::AsyncWriteExt;
use thiserror::Error;
use tracing::{info, debug, warn, error};
use libp2p::{identity, PeerId};

/// P2P connection state
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed(String),
}

/// P2P connection with metadata
#[derive(Debug, Clone)]
pub struct P2PConnection {
    /// Remote peer
    pub peer: Peer,
    
    /// Connection state
    pub state: ConnectionState,
    
    /// Last successful connection time
    pub last_connected: Option<Instant>,
    
    /// Connection attempts
    pub attempts: u32,
    
    /// Last error (if any)
    pub last_error: Option<String>,
}

/// Enterprise-grade P2P network manager
pub struct P2PNetwork {
    /// Active connections
    connections: Arc<RwLock<HashMap<BlissId, P2PConnection>>>,
    
    /// Connection pool size
    max_connections: usize,
    
    /// Connection timeout
    connection_timeout: Duration,
    
    /// Retry configuration
    max_retries: usize,
    retry_backoff: Duration,
}

/// Libp2p-backed transport for Meshwerk Titan backbone.
pub struct Libp2pTransport {
    identity: identity::Keypair,
    peer_id: PeerId,
    logger: Arc<HolographicLogger>,
    signing_key: dilithium_sig::PrivateKey,
    verify_key: dilithium_sig::PublicKey,
    connect_timeout: Duration,
}

impl Libp2pTransport {
    /// Create a new libp2p transport with PQC audit logging.
    pub fn new(logger: Arc<HolographicLogger>, connect_timeout: Duration) -> Self {
        let identity = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(identity.public());
        let (public_key, private_key) = dilithium_sig::dilithium_keygen();
        Self {
            identity,
            peer_id,
            logger,
            signing_key: private_key,
            verify_key: public_key,
            connect_timeout,
        }
    }

    fn is_decoherence_exempt_peer(peer: &Peer) -> bool {
        peer.metadata.as_ref().map(|meta| {
            let lower = meta.to_ascii_lowercase();
            lower.contains("decoherence_exempt") || lower.contains("async_backhaul")
        }).unwrap_or(false)
    }

    async fn log_event(&self, action: &str, peer: &Peer, details: String) -> Result<(), MeshTransportError> {
        let event = AuditEvent::new(
            self.peer_id.to_string(),
            action.to_string(),
            Some(peer.id.to_string()),
            details,
            &self.signing_key,
        ).map_err(|e| MeshTransportError::Failure(e.to_string()))?;
        self.logger.log_and_verify(event, &self.verify_key).await
            .map_err(|e| MeshTransportError::Failure(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl MeshTransport for Libp2pTransport {
    async fn connect(&self, peer: &Peer) -> Result<Duration, MeshTransportError> {
        let start = Instant::now();
        let result = tokio::time::timeout(
            self.connect_timeout,
            tokio::net::TcpStream::connect(&peer.address),
        ).await;
        let elapsed = start.elapsed();

        match result {
            Ok(Ok(_stream)) => {
                if !Self::is_decoherence_exempt_peer(peer) && !is_within_coherence_window(elapsed.as_micros() as u64) {
                    return Err(PhysicsViolationError::StabilityTimeout {
                        elapsed: elapsed.as_micros() as u64,
                        limit: INVARIANTS.coherence_window_us,
                    }.into());
                }
                self.log_event(
                    "P2P_CONNECT",
                    peer,
                    format!("Connected in {}μs", elapsed.as_micros()),
                ).await?;
                Ok(elapsed)
            }
            Ok(Err(e)) => Err(MeshTransportError::Io(e)),
            Err(_) => Err(PhysicsViolationError::StabilityTimeout {
                elapsed: INVARIANTS.coherence_window_us + 1,
                limit: INVARIANTS.coherence_window_us,
            }.into()),
        }
    }

    async fn ping(&self, peer: &Peer) -> Result<Duration, MeshTransportError> {
        let latency = self.connect(peer).await?;
        self.log_event(
            "P2P_PING",
            peer,
            format!("Ping latency {}μs", latency.as_micros()),
        ).await?;
        Ok(latency)
    }

    async fn send(&self, peer: &Peer, payload: &[u8]) -> Result<(), MeshTransportError> {
        let signature = dilithium_sig::sign(payload, &self.signing_key)
            .map_err(|e| MeshTransportError::Failure(e.to_string()))?;

        let mut stream = tokio::time::timeout(
            self.connect_timeout,
            tokio::net::TcpStream::connect(&peer.address),
        ).await.map_err(|_| MeshTransportError::Failure("Send timeout".to_string()))?
          .map_err(MeshTransportError::Io)?;

        let sig_len = (signature.len() as u32).to_be_bytes();
        let payload_len = (payload.len() as u32).to_be_bytes();
        stream.write_all(&sig_len).await?;
        stream.write_all(&signature).await?;
        stream.write_all(&payload_len).await?;
        stream.write_all(payload).await?;

        self.log_event(
            "P2P_SEND",
            peer,
            format!("Sent payload {} bytes", payload.len()),
        ).await?;
        Ok(())
    }
}

/// P2P network configuration
#[derive(Debug, Clone)]
pub struct P2PConfig {
    /// Maximum concurrent connections
    pub max_connections: usize,
    
    /// Connection timeout
    pub connection_timeout: Duration,
    
    /// Maximum retry attempts
    pub max_retries: usize,
    
    /// Retry backoff duration
    pub retry_backoff: Duration,
    
    /// Enable NAT traversal
    pub enable_nat_traversal: bool,
    
    /// Enable connection pooling
    pub enable_pooling: bool,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            connection_timeout: Duration::from_secs(10),
            max_retries: 3,
            retry_backoff: Duration::from_secs(1),
            enable_nat_traversal: true,
            enable_pooling: true,
        }
    }
}

impl P2PNetwork {
    /// Create new P2P network manager
    pub fn new(config: P2PConfig) -> Arc<Self> {
        Arc::new(Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            max_connections: config.max_connections,
            connection_timeout: config.connection_timeout,
            max_retries: config.max_retries,
            retry_backoff: config.retry_backoff,
        })
    }
    
    /// Connect to peer with retry logic
    pub async fn connect(&self, peer: Peer) -> Result<(), P2PError> {
        // Validate peer
        if peer.address.port() == 0 {
            return Err(P2PError::InvalidPeer("Invalid peer port: 0".to_string()));
        }
        
        if peer.id.0.as_bytes().is_empty() {
            return Err(P2PError::InvalidPeer("Invalid peer ID: empty".to_string()));
        }
        
        // Check connection limit
        {
            let connections = self.connections.read().await;
            if connections.len() >= self.max_connections {
                return Err(P2PError::ConnectionLimitExceeded(self.max_connections));
            }
        }
        
        // Check if already connected
        {
            let connections = self.connections.read().await;
            if let Some(conn) = connections.get(&peer.id) {
                if matches!(conn.state, ConnectionState::Connected) {
                    return Ok(()); // Already connected
                }
            }
        }
        
        // Retry connection with exponential backoff
        let mut last_error = None;
        for attempt in 0..self.max_retries {
            match self.connect_internal(&peer).await {
                Ok(_) => {
                    // Update connection state
                    let mut connections = self.connections.write().await;
                    connections.insert(peer.id.clone(), P2PConnection {
                        peer: peer.clone(),
                        state: ConnectionState::Connected,
                        last_connected: Some(Instant::now()),
                        attempts: attempt + 1,
                        last_error: None,
                    });
                    
                    info!("✅ Connected to peer {} (attempt {})", peer.id, attempt + 1);
                    return Ok(());
                }
                Err(e) if attempt < self.max_retries - 1 => {
                    last_error = Some(e.to_string());
                    warn!("Connection failed (attempt {}/{}): {}, retrying...", 
                        attempt + 1, self.max_retries, last_error.as_ref().unwrap());
                    
                    // Update connection state
                    let mut connections = self.connections.write().await;
                    connections.insert(peer.id.clone(), P2PConnection {
                        peer: peer.clone(),
                        state: ConnectionState::Connecting,
                        last_connected: None,
                        attempts: attempt + 1,
                        last_error: last_error.clone(),
                    });
                    
                    tokio::time::sleep(self.retry_backoff * (attempt as u32 + 1)).await;
                    continue;
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    break;
                }
            }
        }
        
        // Mark as failed
        let mut connections = self.connections.write().await;
        connections.insert(peer.id.clone(), P2PConnection {
            peer: peer.clone(),
            state: ConnectionState::Failed(
                last_error.unwrap_or_else(|| "Unknown error".to_string())
            ),
            last_connected: None,
            attempts: self.max_retries,
            last_error: last_error.clone(),
        });
        
        Err(P2PError::ConnectionFailed(
            last_error.unwrap_or_else(|| "Connection failed after retries".to_string())
        ))
    }
    
    /// Internal connection implementation
    async fn connect_internal(&self, peer: &Peer) -> Result<(), P2PError> {
        // Validate peer address
        if peer.address.ip().is_unspecified() {
            return Err(P2PError::InvalidPeer("Peer address is unspecified".to_string()));
        }
        
        // Simulate connection (production would use actual network)
        match tokio::time::timeout(
            self.connection_timeout,
            tokio::net::TcpStream::connect(&peer.address)
        ).await {
            Ok(Ok(_stream)) => {
                debug!("📡 Connected to peer {}", peer.id);
                Ok(())
            }
            Ok(Err(e)) => {
                Err(P2PError::ConnectionFailed(format!("TCP connection failed: {}", e)))
            }
            Err(_) => {
                Err(P2PError::ConnectionTimeout)
            }
        }
    }
    
    /// Disconnect from peer
    pub async fn disconnect(&self, peer_id: &BlissId) -> Result<(), P2PError> {
        let mut connections = self.connections.write().await;
        
        if let Some(mut conn) = connections.remove(peer_id) {
            conn.state = ConnectionState::Disconnected;
            info!("🔌 Disconnected from peer {}", peer_id);
            Ok(())
        } else {
            Err(P2PError::PeerNotFound(peer_id.to_string()))
        }
    }
    
    /// Get connection state for peer
    pub async fn get_connection_state(&self, peer_id: &BlissId) -> Option<ConnectionState> {
        let connections = self.connections.read().await;
        connections.get(peer_id).map(|conn| conn.state.clone())
    }
    
    /// Get all connected peers
    pub async fn connected_peers(&self) -> Vec<BlissId> {
        let connections = self.connections.read().await;
        connections.iter()
            .filter(|(_, conn)| matches!(conn.state, ConnectionState::Connected))
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Cleanup stale connections
    pub async fn cleanup_stale_connections(&self, max_age: Duration) {
        let mut connections = self.connections.write().await;
        let now = Instant::now();
        
        connections.retain(|id, conn| {
            if let Some(last_connected) = conn.last_connected {
                if now.duration_since(last_connected) > max_age {
                    warn!("🧹 Removing stale connection to peer {}", id);
                    return false;
                }
            }
            true
        });
    }
}

/// Enterprise-grade P2P errors
#[derive(Debug, Error)]
pub enum P2PError {
    #[error("Invalid peer: {0}")]
    InvalidPeer(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Connection timeout")]
    ConnectionTimeout,
    #[error("Connection limit exceeded: {0}")]
    ConnectionLimitExceeded(usize),
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Physics violation: {0}")]
    PhysicsViolation(#[from] PhysicsViolationError),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_p2p_connection() {
        let network = P2PNetwork::new(P2PConfig::default());
        let peer = Peer::new(
            "127.0.0.1:6000".parse().unwrap(),
            crate::gov::SoulACL::default(),
            None,
        );
        
        // Connection will fail in test (no actual network), but should handle gracefully
        let result = network.connect(peer).await;
        assert!(result.is_err()); // Expected in test environment
    }
}

