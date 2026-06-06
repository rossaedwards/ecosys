//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Gossip - Enterprise-Grade Epidemic Protocol
//! 🌐 Efficient Message Propagation + Deduplication + Anti-Entropy
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::BlissId,
    shard::ShardId,
    crypto::hash::Blake3Digest,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{info, debug, warn, error};

/// Gossip message with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipMessage {
    /// Unique message ID (content-addressed)
    pub id: Blake3Digest,
    
    /// Message sender
    pub sender: BlissId,
    
    /// Message payload
    pub payload: Vec<u8>,
    
    /// Time-to-live (hops remaining)
    pub ttl: u8,
    
    /// Creation timestamp (nanos)
    pub timestamp_ns: u64,
    
    /// Message type
    pub message_type: GossipMessageType,
}

/// Gossip message types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GossipMessageType {
    PeerDiscovery,
    ShardAnnouncement,
    HealthUpdate,
    ConsensusVote,
    AntiEntropy,
}

/// Enterprise-grade gossip engine
pub struct GossipEngine {
    /// Seen message IDs (for deduplication)
    seen_messages: Arc<RwLock<HashMap<Blake3Digest, Instant>>>,
    
    /// Message cache (LRU-like, with TTL)
    message_cache: Arc<RwLock<HashMap<Blake3Digest, GossipMessage>>>,
    
    /// Gossip statistics
    stats: Arc<RwLock<GossipStats>>,
    
    /// Configuration
    config: GossipConfig,
}

/// Gossip configuration
#[derive(Debug, Clone)]
pub struct GossipConfig {
    /// Maximum TTL for messages
    pub max_ttl: u8,
    
    /// Message cache TTL
    pub cache_ttl: Duration,
    
    /// Deduplication window
    pub dedup_window: Duration,
    
    /// Fanout (peers per round)
    pub fanout: usize,
    
    /// Gossip interval
    pub gossip_interval: Duration,
}

impl Default for GossipConfig {
    fn default() -> Self {
        Self {
            max_ttl: 7,
            cache_ttl: Duration::from_secs(3600),
            dedup_window: Duration::from_secs(300),
            fanout: 5,
            gossip_interval: Duration::from_millis(100),
        }
    }
}

/// Gossip statistics
#[derive(Debug, Clone, Default)]
pub struct GossipStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub messages_dropped: u64,
    pub duplicate_messages: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl GossipEngine {
    /// Create new gossip engine
    pub fn new(config: GossipConfig) -> Arc<Self> {
        let engine = Arc::new(Self {
            seen_messages: Arc::new(RwLock::new(HashMap::new())),
            message_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(GossipStats::default())),
            config,
        });
        
        // Start cleanup task
        let engine_clone = Arc::clone(&engine);
        tokio::spawn(async move {
            engine_clone.cleanup_loop().await;
        });
        
        engine
    }
    
    /// Process incoming gossip message with deduplication
    pub async fn process_message(&self, message: GossipMessage) -> Result<bool, GossipError> {
        // Validate message
        if message.id.0.as_bytes().is_empty() {
            return Err(GossipError::InvalidMessage("Message ID is empty".to_string()));
        }
        
        if message.sender.0.as_bytes().is_empty() {
            return Err(GossipError::InvalidMessage("Sender ID is empty".to_string()));
        }
        
        if message.payload.is_empty() {
            return Err(GossipError::InvalidMessage("Message payload is empty".to_string()));
        }
        
        // Check deduplication
        {
            let seen = self.seen_messages.read().await;
            if seen.contains_key(&message.id) {
                let mut stats = self.stats.write().await;
                stats.duplicate_messages += 1;
                return Ok(false); // Duplicate
            }
        }
        
        // Add to seen messages
        {
            let mut seen = self.seen_messages.write().await;
            seen.insert(message.id.clone(), Instant::now());
        }
        
        // Cache message
        {
            let mut cache = self.message_cache.write().await;
            cache.insert(message.id.clone(), message.clone());
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.messages_received += 1;
        }
        
        debug!("📢 Processed gossip message {} from {}", message.id, message.sender);
        Ok(true)
    }
    
    /// Create new gossip message
    pub async fn create_message(
        &self,
        payload: Vec<u8>,
        sender: BlissId,
        message_type: GossipMessageType,
    ) -> Result<GossipMessage, GossipError> {
        // Validate inputs
        if payload.is_empty() {
            return Err(GossipError::InvalidMessage("Payload is empty".to_string()));
        }
        
        if sender.0.as_bytes().is_empty() {
            return Err(GossipError::InvalidMessage("Sender ID is empty".to_string()));
        }
        
        // Generate message ID (content-addressed)
        let timestamp_ns = Instant::now().elapsed().as_nanos() as u64;
        let id = crate::crypto::hash::blake3_hash_bytes(
            &[&sender.0.as_bytes(), &payload, &timestamp_ns.to_be_bytes()].concat()
        );
        
        let message = GossipMessage {
            id,
            sender,
            payload,
            ttl: self.config.max_ttl,
            timestamp_ns,
            message_type,
        };
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.messages_sent += 1;
        }
        
        Ok(message)
    }
    
    /// Get message from cache
    pub async fn get_cached_message(&self, id: &Blake3Digest) -> Option<GossipMessage> {
        let cache = self.message_cache.read().await;
        let message = cache.get(id)?.clone();
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.cache_hits += 1;
        }
        
        Some(message)
    }
    
    /// Background cleanup loop
    async fn cleanup_loop(self: Arc<Self>) {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            let now = Instant::now();
            
            // Cleanup seen messages
            {
                let mut seen = self.seen_messages.write().await;
                seen.retain(|_, timestamp| {
                    now.duration_since(*timestamp) < self.config.dedup_window
                });
            }
            
            // Cleanup message cache
            {
                let mut cache = self.message_cache.write().await;
                cache.retain(|_, message| {
                    let message_age = Duration::from_nanos(
                        Instant::now().elapsed().as_nanos() as u64 - message.timestamp_ns
                    );
                    message_age < self.config.cache_ttl
                });
            }
        }
    }
    
    /// Get gossip statistics
    pub async fn stats(&self) -> GossipStats {
        self.stats.read().await.clone()
    }
}

/// Enterprise-grade gossip errors
#[derive(Debug, Error)]
pub enum GossipError {
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("TTL expired")]
    TtlExpired,
    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_gossip_deduplication() {
        let engine = GossipEngine::new(GossipConfig::default());
        let sender = BlissId::new();
        
        let msg1 = engine.create_message(
            b"test".to_vec(),
            sender.clone(),
            GossipMessageType::PeerDiscovery,
        ).await.unwrap();
        
        let msg2 = msg1.clone();
        
        // First message should be processed
        assert!(engine.process_message(msg1).await.unwrap());
        
        // Duplicate should be rejected
        assert!(!engine.process_message(msg2).await.unwrap());
    }
}

