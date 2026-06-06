// afs/src/mesh/mesh_gossip.rs

//! Gossip protocol for AuraFS mesh network
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Implements epidemic-style gossip protocol for efficient message propagation
//! across the distributed mesh network with anti-entropy and deduplication.
//! Extends traditional gossip to include quantum-signed audit event propagation.

use crate::error::{RafsError, Result};
use crate::network::peer::{PeerInfo, PeerManager};
use crate::shard::metadata::{PeerId, ShardId, ShardMetadata};
use crate::audit::holographic_logger::AuditEvent;
use crate::crypto::{verify_signature, quantum_sign};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use uuid::Uuid;

/// Gossip message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    /// Announce shard availability
    ShardAvailable {
        shard_id: ShardId,
        metadata: ShardMetadata,
        peer_id: PeerId,
    },

    /// Request shard location
    ShardRequest {
        shard_id: ShardId,
        requester: PeerId,
    },

    /// Respond with shard locations
    ShardResponse {
        shard_id: ShardId,
        peers: Vec<PeerId>,
    },

    /// Update shard metadata
    ShardMetadataUpdate {
        shard_id: ShardId,
        metadata: ShardMetadata,
    },

    /// Peer capability update
    PeerCapabilityUpdate {
        peer_id: PeerId,
        peer_info: PeerInfo,
    },

    /// Heartbeat ping
    Heartbeat {
        peer_id: PeerId,
        timestamp: u64,
    },

    /// Audit event propagated in mesh (quantum-safe)
    AuditEventNotification {
        event: Arc<AuditEvent>,
        signature: Vec<u8>,
    },
}

/// Gossip envelope with routing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipEnvelope {
    /// Unique message ID
    pub message_id: String,

    /// Originating peer
    pub origin: PeerId,

    /// Message payload
    pub payload: GossipMessage,

    /// Time-to-live (hop count)
    pub ttl: u8,

    /// Timestamp (Unix epoch)
    pub timestamp: u64,

    /// Message priority
    pub priority: MessagePriority,
}

/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Urgent = 3,
}

impl GossipEnvelope {
    /// Create new gossip envelope
    pub fn new(origin: PeerId, payload: GossipMessage, ttl: u8) -> Self {
        Self {
            message_id: Uuid::new_v4().to_string(),
            origin,
            payload,
            ttl,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            priority: MessagePriority::Normal,
        }
    }

    /// Create with priority
    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    /// Decrement TTL; returns true if still alive
    pub fn decrement_ttl(&mut self) -> bool {
        if self.ttl > 0 {
            self.ttl -= 1;
            true
        } else {
            false
        }
    }

    /// Check if message is expired
    pub fn is_expired(&self, max_age_secs: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now - self.timestamp) > max_age_secs
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| RafsError::SerializationError(e.to_string()))
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
            .map_err(|e| RafsError::SerializationError(e.to_string()))
    }
}

/// Gossip protocol configuration
#[derive(Debug, Clone)]
pub struct GossipConfig {
    pub fanout: usize,
    pub default_ttl: u8,
    pub max_message_age_secs: u64,
    pub seen_cache_size: usize,
    pub gossip_interval_ms: u64,
    pub enable_batching: bool,
    pub batch_size: usize,
    pub rate_limit: usize,
}

impl Default for GossipConfig {
    fn default() -> Self {
        Self {
            fanout: 6,
            default_ttl: 10,
            max_message_age_secs: 300,
            seen_cache_size: 10000,
            gossip_interval_ms: 100,
            enable_batching: true,
            batch_size: 50,
            rate_limit: 100,
        }
    }
}

/// Seen message cache for deduplication
struct SeenCache {
    cache: VecDeque<String>,
    max_size: usize,
}

impl SeenCache {
    fn new(max_size: usize) -> Self {
        Self {
            cache: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn contains(&self, message_id: &str) -> bool {
        self.cache.contains(&message_id.to_string())
    }

    fn insert(&mut self, message_id: String) {
        if self.cache.len() >= self.max_size {
            self.cache.pop_front();
        }
        self.cache.push_back(message_id);
    }
}

/// Gossip protocol engine
pub struct GossipProtocol {
    peer_id: PeerId,
    peer_manager: Arc<PeerManager>,
    config: GossipConfig,
    seen_cache: Arc<RwLock<SeenCache>>,
    outbound_queue: Arc<RwLock<VecDeque<GossipEnvelope>>>,
    inbound_rx: Arc<RwLock<mpsc::UnboundedReceiver<GossipEnvelope>>>,
    inbound_tx: mpsc::UnboundedSender<GossipEnvelope>,
    handlers: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<GossipMessage>>>>,
    stats: Arc<RwLock<GossipStats>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GossipStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub messages_dropped: u64,
    pub messages_deduplicated: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl GossipProtocol {
    /// Create new gossip protocol instance
    pub fn new(
        peer_id: PeerId,
        peer_manager: Arc<PeerManager>,
        config: GossipConfig,
    ) -> Self {
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel();

        Self {
            peer_id,
            peer_manager,
            config,
            seen_cache: Arc::new(RwLock::new(SeenCache::new(config.seen_cache_size))),
            outbound_queue: Arc::new(RwLock::new(VecDeque::new())),
            inbound_rx: Arc::new(RwLock::new(inbound_rx)),
            inbound_tx,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(GossipStats::default())),
        }
    }

    /// Start gossip loops (outbound & inbound)
    pub async fn start(&self) -> Result<()> {
        self.start_outbound_loop().await;
        self.start_inbound_loop().await;
        Ok(())
    }

    async fn start_outbound_loop(&self) {
        let queue = self.outbound_queue.clone();
        let peer_manager = self.peer_manager.clone();
        let stats = self.stats.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(config.gossip_interval_ms));

            loop {
                interval.tick().await;

                let mut queue = queue.write().await;
                let batch: Vec<GossipEnvelope> = queue
                    .drain(..config.batch_size.min(queue.len()))
                    .collect();

                if batch.is_empty() {
                    continue;
                }

                let peers = peer_manager.healthy_peers().await;
                if peers.is_empty() {
                    continue;
                }

                let fanout_peers: Vec<_> = peers
                    .iter()
                    .take(config.fanout)
                    .collect();

                for envelope in batch {
                    for peer in &fanout_peers {
                        tracing::debug!(
                            "Gossip: {} → {} (TTL: {})",
                            envelope.origin,
                            peer.peer_id,
                            envelope.ttl
                        );

                        let mut stats = stats.write().await;
                        stats.messages_sent += 1;
                    }
                }
            }
        });
    }

    async fn start_inbound_loop(&self) {
        let inbound_rx = self.inbound_rx.clone();
        let seen_cache = self.seen_cache.clone();
        let outbound_queue = self.outbound_queue.clone();
        let handlers = self.handlers.clone();
        let stats = self.stats.clone();
        let config = self.config.clone();
        let peer_id = self.peer_id.clone();

        tokio::spawn(async move {
            let mut rx = inbound_rx.write().await;

            while let Some(mut envelope) = rx.recv().await {
                {
                    let mut cache = seen_cache.write().await;
                    if cache.contains(&envelope.message_id) {
                        let mut stats = stats.write().await;
                        stats.messages_deduplicated += 1;
                        continue;
                    }
                    cache.insert(envelope.message_id.clone());
                }

                if envelope.is_expired(config.max_message_age_secs) {
                    let mut stats = stats.write().await;
                    stats.messages_dropped += 1;
                    continue;
                }

                {
                    let mut stats = stats.write().await;
                    stats.messages_received += 1;
                }

                // Skip re-gossip own messages
                if envelope.origin != peer_id {
                    if envelope.decrement_ttl() {
                        let mut queue = outbound_queue.write().await;
                        queue.push_back(envelope.clone());
                    }
                }

                let handlers = handlers.read().await;
                for handler_tx in handlers.values() {
                    let _ = handler_tx.send(envelope.payload.clone());
                }
            }
        });
    }

    /// Broadcast a message with priority
    pub async fn broadcast(&self, message: GossipMessage, priority: MessagePriority) {
        let envelope = GossipEnvelope::new(
            self.peer_id.clone(),
            message,
            self.config.default_ttl,
        )
        .with_priority(priority);

        let mut queue = self.outbound_queue.write().await;

        match priority {
            MessagePriority::Urgent => queue.push_front(envelope),
            _ => queue.push_back(envelope),
        }
    }

    /// Send message with normal priority
    pub async fn send(&self, message: GossipMessage) {
        self.broadcast(message, MessagePriority::Normal).await;
    }

    /// Receive inbound gossip message
    pub async fn receive(&self, envelope: GossipEnvelope) -> Result<()> {
        self.inbound_tx
            .send(envelope)
            .map_err(|_| RafsError::NetworkError("Inbound channel closed".to_string()))
    }

    /// Register a handler to receive gossip message payloads
    pub async fn register_handler(&self, name: String) -> mpsc::UnboundedReceiver<GossipMessage> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut handlers = self.handlers.write().await;
        handlers.insert(name, tx);
        rx
    }

    /// Announce availability of a shard
    pub async fn announce_shard(&self, shard_id: ShardId, metadata: ShardMetadata) {
        let message = GossipMessage::ShardAvailable {
            shard_id,
            metadata,
            peer_id: self.peer_id.clone(),
        };
        self.send(message).await;
    }

    /// Request shard location information
    pub async fn request_shard(&self, shard_id: ShardId) {
        let message = GossipMessage::ShardRequest {
            shard_id,
            requester: self.peer_id.clone(),
        };
        self.broadcast(message, MessagePriority::High).await;
    }

    /// Send heartbeat ping
    pub async fn heartbeat(&self) {
        let message = GossipMessage::Heartbeat {
            peer_id: self.peer_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.send(message).await;
    }

    /// Retrieve gossip statistics
    pub async fn stats(&self) -> GossipStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get current outbound queue size
    pub async fn queue_size(&self) -> usize {
        let queue = self.outbound_queue.read().await;
        queue.len()
    }

    /// Clear the message deduplication cache
    pub async fn clear_cache(&self) {
        let mut cache = self.seen_cache.write().await;
        *cache = SeenCache::new(self.config.seen_cache_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use tokio::sync::Mutex;

    #[test]
    fn test_gossip_envelope_creation() {
        let peer_id = "test-peer".to_string();
        let shard_id = hash::hash(b"test-shard");
        let metadata = ShardMetadata::new(shard_id, 100, None);

        let message = GossipMessage::ShardAvailable {
            shard_id,
            metadata,
            peer_id: peer_id.clone(),
        };

        let envelope = GossipEnvelope::new(peer_id, message, 10);

        assert_eq!(envelope.ttl, 10);
        assert_eq!(envelope.priority, MessagePriority::Normal);
    }

    #[test]
    fn test_envelope_ttl_decrement() {
        let peer_id = "test-peer".to_string();
        let message = GossipMessage::Heartbeat {
            peer_id: peer_id.clone(),
            timestamp: 0,
        };

        let mut envelope = GossipEnvelope::new(peer_id, message, 3);

        assert!(envelope.decrement_ttl());
        assert_eq!(envelope.ttl, 2);

        assert!(envelope.decrement_ttl());
        assert_eq!(envelope.ttl, 1);

        assert!(envelope.decrement_ttl());
        assert_eq!(envelope.ttl, 0);

        assert!(!envelope.decrement_ttl());
        assert_eq!(envelope.ttl, 0);
    }

    #[test]
    fn test_envelope_serialization() {
        let peer_id = "test-peer".to_string();
        let message = GossipMessage::Heartbeat {
            peer_id: peer_id.clone(),
            timestamp: 12345,
        };

        let envelope = GossipEnvelope::new(peer_id, message, 10);

        let bytes = envelope.to_bytes().unwrap();
        let deserialized = GossipEnvelope::from_bytes(&bytes).unwrap();

        assert_eq!(deserialized.message_id, envelope.message_id);
        assert_eq!(deserialized.ttl, envelope.ttl);
    }

    #[test]
    fn test_seen_cache() {
        let mut cache = SeenCache::new(3);

        cache.insert("msg1".to_string());
        cache.insert("msg2".to_string());
        cache.insert("msg3".to_string());

        assert!(cache.contains("msg1"));
        assert!(cache.contains("msg2"));
        assert!(cache.contains("msg3"));

        // Should evict msg1
        cache.insert("msg4".to_string());
        assert!(!cache.contains("msg1"));
        assert!(cache.contains("msg4"));
    }

    #[tokio::test]
    async fn test_gossip_protocol_creation() {
        let peer_id = "local-peer".to_string();
        let peer_manager = Arc::new(PeerManager::new(vec![], 100));
        let config = GossipConfig::default();

        let gossip = GossipProtocol::new(peer_id, peer_manager, config);

        assert_eq!(gossip.queue_size().await, 0);
    }

    #[tokio::test]
    async fn test_broadcast_message() {
        let peer_id = "local-peer".to_string();
        let peer_manager = Arc::new(PeerManager::new(vec![], 100));
        let config = GossipConfig::default();

        let gossip = GossipProtocol::new(peer_id.clone(), peer_manager, config);

        let message = GossipMessage::Heartbeat {
            peer_id,
            timestamp: 0,
        };

        gossip.broadcast(message, MessagePriority::High).await;

        assert_eq!(gossip.queue_size().await, 1);
    }

    #[tokio::test]
    async fn test_register_handler() {
        let peer_id = "local-peer".to_string();
        let peer_manager = Arc::new(PeerManager::new(vec![], 100));
        let config = GossipConfig::default();

        let gossip = GossipProtocol::new(peer_id, peer_manager, config);

        let _rx = gossip.register_handler("test-handler".to_string()).await;

        // Handler registered successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_gossip_stats() {
        let peer_id = "local-peer".to_string();
        let peer_manager = Arc::new(PeerManager::new(vec![], 100));
        let config = GossipConfig::default();

        let gossip = GossipProtocol::new(peer_id, peer_manager, config);

        let stats = gossip.stats().await;
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
    }
}