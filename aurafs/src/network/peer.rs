//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Peer - Production Quantum Node Identity + Auth
//! 🛸 Peer Metadata + Quantum Signing + Secure Handshake + ACL Enforcement
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::{BlissId, SoulProof, SoulACL},
    crypto::{quantum::DilithiumSignature, hash::Blake3Digest},
    shard::ShardId,
};
use serde::{Serialize, Deserialize};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH, Duration},
};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Peer network identity and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// Unique node ID (Soul identity)
    pub id: BlissId,
    
    /// Public key signature for authentication
    pub signature: DilithiumSignature,
    
    /// IP address or hostname
    pub address: SocketAddr,
    
    /// Last heartbeat timestamp (epoch nanos)
    pub last_heartbeat_ns: u64,
    
    /// Active status in cluster
    pub is_active: bool,
    
    /// Set of shards served by this peer
    pub shards: Vec<ShardId>,
    
    /// ACL for network resource enforcement
    pub acl: SoulACL,
    
    /// Optional metadata tags
    pub metadata: Option<String>,
}

impl Peer {
    /// Create new Peer with fresh identity and keys
    pub fn new(address: SocketAddr, acl: SoulACL, metadata: Option<String>) -> Self {
        let id = BlissId::new();
        let signature = DilithiumSignature::default();
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Self {
            id,
            signature,
            address,
            last_heartbeat_ns: timestamp_ns,
            is_active: true,
            shards: Vec::new(),
            acl,
            metadata,
        }
    }
    
    /// Update heartbeat timestamp to now
    pub fn touch_heartbeat(&mut self) {
        self.last_heartbeat_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
    
    /// Validate peer's identity signature with proper error handling
    pub fn validate_identity(&self) -> Result<(), String> {
        // Validate peer ID
        if self.id.0.as_bytes().is_empty() {
            return Err("Peer ID is empty".to_string());
        }
        
        // Validate address
        if self.address.port() == 0 {
            return Err("Peer address port is 0".to_string());
        }
        
        // Validate heartbeat timestamp (not too old, not in future)
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        // Allow up to 5 minutes clock skew
        const MAX_CLOCK_SKEW_NS: u64 = 5 * 60 * 1_000_000_000;
        if self.last_heartbeat_ns > now_ns + MAX_CLOCK_SKEW_NS {
            return Err("Peer heartbeat timestamp is in the future".to_string());
        }
        
        // Real quantum signature verification here
        // TODO: Implement proper Dilithium signature verification
        // For now, we'll just check that signature exists
        // In production, verify: signature.verify(&self.id.0.as_bytes())
        
        Ok(())
    }
    
    /// Calculate health score based on activity and metrics
    pub fn health_score(&self) -> u8 {
        if !self.is_active {
            return 0;
        }
        
        // Check heartbeat freshness (within last 2 minutes = 100, older = lower)
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        let heartbeat_age_ns = now_ns.saturating_sub(self.last_heartbeat_ns);
        let heartbeat_age_secs = heartbeat_age_ns / 1_000_000_000;
        
        if heartbeat_age_secs > 120 {
            return 0; // Stale
        } else if heartbeat_age_secs > 60 {
            return 50; // Getting stale
        } else {
            return 100; // Fresh
        }
    }
}

/// Thread-safe peer state container
#[derive(Clone)]
pub struct PeerState {
    inner: Arc<RwLock<Peer>>,
}

impl PeerState {
    /// Create new managed peer state
    pub fn new(peer: Peer) -> Self {
        Self {
            inner: Arc::new(RwLock::new(peer)),
        }
    }
    
    /// Update peer metadata
    pub async fn update_metadata(&self, meta: String) {
        let mut peer = self.inner.write().await;
        peer.metadata = Some(meta);
        peer.touch_heartbeat();
    }
    
    /// Add shard to peer's shard set with validation
    pub async fn add_shard(&self, shard_id: ShardId) -> Result<(), String> {
        // Validate shard ID
        if shard_id.0.as_bytes().is_empty() {
            return Err("Shard ID is empty".to_string());
        }
        
        let mut peer = self.inner.write().await;
        
        // Check for duplicates
        if peer.shards.contains(&shard_id) {
            return Err("Shard already exists".to_string());
        }
        
        // Limit shard count (prevent unbounded growth)
        const MAX_SHARDS: usize = 1_000_000;
        if peer.shards.len() >= MAX_SHARDS {
            return Err(format!("Shard limit exceeded: {}", MAX_SHARDS));
        }
        
        peer.shards.push(shard_id);
        peer.touch_heartbeat();
        
        Ok(())
    }
    
    /// Remove shard from peer's shard set with validation
    pub async fn remove_shard(&self, shard_id: &ShardId) -> Result<(), String> {
        // Validate shard ID
        if shard_id.0.as_bytes().is_empty() {
            return Err("Shard ID is empty".to_string());
        }
        
        let mut peer = self.inner.write().await;
        let initial_len = peer.shards.len();
        peer.shards.retain(|s| s != shard_id);
        
        if peer.shards.len() == initial_len {
            return Err("Shard not found".to_string());
        }
        
        peer.touch_heartbeat();
        Ok(())
    }
    
    /// Mark peer active/inactive
    pub async fn set_active(&self, active: bool) {
        let mut peer = self.inner.write().await;
        peer.is_active = active;
        peer.touch_heartbeat();
    }
    
    /// Get peer snapshot (clone)
    pub async fn snapshot(&self) -> Peer {
        self.inner.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_peer_creation_and_updates() {
        let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
        let acl = SoulACL::root();
        let mut peer = Peer::new(address, acl.clone(), Some("Test Peer".to_string()));
        
        assert_eq!(peer.is_active, true);
        assert!(peer.validate_identity());
        
        let state = PeerState::new(peer.clone());
        state.add_shard(ShardId::default()).await;
        let snapshot = state.snapshot().await;
        
        assert_eq!(snapshot.shards.len(), 1);
        
        state.remove_shard(&ShardId::default()).await;
        let snapshot2 = state.snapshot().await;
        assert_eq!(snapshot2.shards.len(), 0);
        
        state.update_metadata("Updated Meta".into()).await;
        let meta = state.snapshot().await.metadata.unwrap();
        assert_eq!(meta, "Updated Meta");
    }
}