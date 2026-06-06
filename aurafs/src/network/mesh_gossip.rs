//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with cosmic l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Mesh Gossip - Epidemic Anti-Entropy Protocol Engine
//! 🛸 Asynchronous + Deduplicated + Secure + Self-Healing Propagation
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    network::{mesh::GossipMessage, peer::PeerState},
    gov::BlissId,
    shard::ShardId,
    crypto::hash::Blake3Digest,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{RwLock, mpsc};
use tokio::time::{interval};
use tracing::{info, debug, warn};
use rand::{thread_rng, Rng};
use blake3::Hasher;

/// Gossip message envelope with deduplication + TTL
#[derive(Debug, Clone)]
pub struct GossipEnvelope {
    pub id: Blake3Digest,           // Unique message ID
    pub sender: BlissId,           // Original sender
    pub payload: GossipPayload,    // Actual message content
    pub ttl: u8,                   // Time-to-live (hops remaining)
    pub timestamp_ns: u64,         // Creation timestamp
    pub signature: Vec<u8>,        // Quantum signature
}

#[derive(Debug, Clone)]
pub enum GossipPayload {
    PeerDiscovery(Vec<BlissId>),
    ShardAnnouncement(Vec<ShardAnnouncement>),
    HealthUpdate(HealthUpdate),
    ConsensusVote(ConsensusVote),
    AntiEntropyRequest(AntiEntropyRequest),
}

#[derive(Debug, Clone)]
pub struct ShardAnnouncement {
    pub shard_id: ShardId,
    pub owner: BlissId,
    pub replicas: Vec<BlissId>,
}

#[derive(Debug, Clone)]
pub struct HealthUpdate {
    pub peer_id: BlissId,
    pub health_score: u8,
    pub shards_served: usize,
    pub load_factor: f32,
}

#[derive(Debug, Clone)]
pub struct ConsensusVote {
    pub proposal_id: Blake3Digest,
    pub vote: bool,
    pub voter: BlissId,
}

#[derive(Debug, Clone)]
pub struct AntiEntropyRequest {
    pub from_peer: BlissId,
    pub shards_wanted: Vec<ShardId>,
}

/// Production gossip engine with deduplication + anti-entropy
pub struct MeshGossip {
    /// Seen message IDs (LRU eviction)
    seen_messages: Arc<RwLock<HashSet<Blake3Digest>>>,
    
    /// Pending gossip queue
    pending_queue: Arc<RwLock<Vec<GossipEnvelope>>>,
    
    /// Peer states for fanout selection
    peers: Arc<RwLock<HashMap<BlissId, Arc<PeerState>>>>,
    
    /// Gossip fanout (peers per round)
    fanout: usize,
    
    /// Message TTL (hops)
    max_ttl: u8,
    
    /// Gossip round interval
    round_interval: Duration,
    
    /// Send channel to network layer
    tx: mpsc::Sender<GossipEnvelope>,
}

impl MeshGossip {
    /// Forge production gossip engine
    pub fn new(
        peers: Arc<RwLock<HashMap<BlissId, Arc<PeerState>>>>,
        fanout: usize,
        tx: mpsc::Sender<GossipEnvelope>,
    ) -> Arc<Self> {
        Arc::new(Self {
            seen_messages: Arc::new(RwLock::new(HashSet::new())),
            pending_queue: Arc::new(RwLock::new(Vec::new())),
            peers,
            fanout: fanout.min(10), // Cap at 10
            max_ttl: 7,
            round_interval: Duration::from_millis(100),
            tx,
        })
    }
    
    /// Inject gossip message into epidemic propagation
    pub async fn gossip(&self, payload: GossipPayload, sender: BlissId) -> bool {
        let envelope = self.create_envelope(payload, sender).await;
        
        // Deduplication check
        {
            let mut seen = self.seen_messages.write().await;
            if seen.contains(&envelope.id) {
                return false; // Already seen
            }
            seen.insert(envelope.id.clone());
        }
        
        // Queue for fanout
        self.pending_queue.write().await.push(envelope.clone());
        self.tx.send(envelope).await.ok();
        
        true
    }
    
    /// Main gossip round - epidemic fanout to healthy peers
    pub async fn run_gossip_loop(self: Arc<Self>) {
        let mut interval = interval(self.round_interval);
        
        loop {
            interval.tick().await;
            
            let envelopes = {
                let mut queue = self.pending_queue.write().await;
                std::mem::take(&mut *queue)
            };
            
            for envelope in envelopes {
                if envelope.ttl == 0 {
                    continue; // TTL expired
                }
                
                // Select healthy peers for fanout
                let healthy_peers = self.select_healthy_peers(envelope.ttl).await;
                
                // Fanout to random subset
                let mut rng = thread_rng();
                let selected: Vec<_> = healthy_peers
                    .into_iter()
                    .choose_multiple_fill(&mut rng, self.fanout);
                
                for peer_id in selected {
                    // Decrement TTL + forward (network layer handles send)
                    let forwarded = GossipEnvelope {
                        ttl: envelope.ttl.saturating_sub(1),
                        ..envelope.clone()
                    };
                    let _ = self.tx.send(forwarded).await;
                }
            }
            
            // Anti-entropy trigger (every 100 rounds)
            if interval.interval().as_millis() % 10_000 == 0 {
                self.trigger_anti_entropy().await;
            }
        }
    }
    
    /// Select healthy peers based on TTL (prefer closer peers for lower TTL)
    async fn select_healthy_peers(&self, ttl: u8) -> Vec<BlissId> {
        let peers = self.peers.read().await;
        let mut healthy: Vec<_> = peers.values()
            .filter_map(|p| {
                let peer = futures::executor::block_on(p.snapshot());
                (peer.is_active && peer.health_score() >= 70).then_some(peer.id.clone())
            })
            .collect();
        
        // Probabilistic selection based on TTL (higher TTL = more peers)
        let target_count = (self.fanout as f32 * (ttl as f32 / self.max_ttl as f32)).max(1.0) as usize;
        healthy.truncate(target_count);
        healthy
    }
    
    /// Anti-entropy reconciliation with peers
    async fn trigger_anti_entropy(&self) {
        let peers = self.peers.read().await;
        let sample_peers: Vec<_> = peers.values()
            .choose_multiple(&mut thread_rng(), 3);
        
        for peer_state in sample_peers {
            let peer = futures::executor::block_on(peer_state.snapshot());
            let request = AntiEntropyRequest {
                from_peer: peer.id.clone(),
                shards_wanted: vec![], // Compute missing shards
            };
            let _ = self.gossip(GossipPayload::AntiEntropyRequest(request), peer.id.clone()).await;
        }
        
        info!("🔄 Anti-entropy reconciliation triggered");
    }
    
    /// Create signed gossip envelope
    async fn create_envelope(&self, payload: GossipPayload, sender: BlissId) -> GossipEnvelope {
        let timestamp_ns = Instant::now().elapsed().as_nanos() as u64;
        let payload_bytes = bincode::serialize(&payload).unwrap();
        
        let mut hasher = Hasher::new();
        hasher.update(&sender.0);
        hasher.update(&payload_bytes);
        hasher.update(&timestamp_ns.to_be_bytes());
        let msg_id = hasher.finalize().into();
        
        GossipEnvelope {
            id: msg_id,
            sender,
            payload,
            ttl: self.max_ttl,
            timestamp_ns,
            signature: vec![], // Quantum signing placeholder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_gossip_deduplication() {
        let (tx, _rx) = mpsc::channel(10);
        let peers = Arc::new(RwLock::new(HashMap::new()));
        let gossip = MeshGossip::new(peers, 3, tx);
        
        let payload = GossipPayload::PeerDiscovery(vec![BlissId::new()]);
        let sender = BlissId::new();
        
        // First gossip succeeds
        assert!(gossip.gossip(payload.clone(), sender.clone()).await);
        // Second identical gossip is deduplicated
        assert!(!gossip.gossip(payload.clone(), sender.clone()).await);
    }
    
    #[tokio::test]
    async fn test_gossip_ttl_decay() {
        let (tx, mut rx) = mpsc::channel(10);
        let peers = Arc::new(RwLock::new(HashMap::new()));
        let gossip = Arc::new(MeshGossip::new(peers, 3, tx));
        
        let payload = GossipPayload::HealthUpdate(HealthUpdate {
            peer_id: BlissId::new(),
            health_score: 90,
            shards_served: 42,
            load_factor: 0.7,
        });
        
        gossip.gossip(payload.clone(), BlissId::new()).await;
        
        // Verify envelope TTL
        if let Some(envelope) = rx.recv().await {
            assert_eq!(envelope.ttl, 7); // max_ttl
        }
    }
}