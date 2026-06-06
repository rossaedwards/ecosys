//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with cosmic l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Network Mesh - Mystical Quantum Mesh Orchestrator
//! 🛸 Peer Gossip + Fractal Routing + Quantum Consensus + Auto-Healing Swarm
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    network::peer::{Peer, PeerState},
    network::node_manager::NodeManager,
    shard::ShardId,
    gov::BlissId,
};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{RwLock, broadcast};
use tokio::time::{interval, sleep};
use tracing::{info, debug, warn, error};
use futures::future::join_all;

/// Network mesh - mystical fractal peer-to-peer orchestrator
pub struct Mesh {
    /// Local node's peer state
    local_peer: Arc<PeerState>,
    
    /// Node manager for lifecycle + heartbeat + health
    node_manager: Arc<NodeManager>,
    
    /// Known peers map (peer id → PeerState)
    peers: Arc<RwLock<HashMap<BlissId, Arc<PeerState>>>>,
    
    /// Gossip message channel transmitter
    gossip_tx: broadcast::Sender<GossipMessage>,
    
    /// Gossip message receiver
    gossip_rx: broadcast::Receiver<GossipMessage>,
    
    /// Mesh config parameters
    config: MeshConfig,
}

/// Mesh configuration parameters
#[derive(Debug, Clone)]
pub struct MeshConfig {
    /// Heartbeat interval duration
    pub heartbeat_interval: Duration,
    
    /// Gossip broadcast interval
    pub gossip_interval: Duration,
    
    /// Peer inactivity timeout
    pub peer_timeout: Duration,
    
    /// Maximum gossip fanout (peers to gossip to)
    pub max_gossip_fanout: usize,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(10),
            gossip_interval: Duration::from_secs(20),
            peer_timeout: Duration::from_secs(90),
            max_gossip_fanout: 5,
        }
    }
}

/// Gossip message for peer-to-peer state sharing
#[derive(Debug, Clone)]
pub struct GossipMessage {
    pub sender: BlissId,
    pub known_peers: Vec<BlissId>,
    pub shard_announcements: Vec<ShardAnnouncement>,
}

/// Announced shard ownership for gossip
#[derive(Debug, Clone)]
pub struct ShardAnnouncement {
    pub shard_id: ShardId,
    pub owner_peer: BlissId,
}

impl Mesh {
    /// Forge the mystical quantum mesh orchestrator
    pub fn new(local_peer: Arc<PeerState>, node_manager: Arc<NodeManager>, config: MeshConfig) -> Arc<Self> {
        let (tx, rx) = broadcast::channel(1000);
        Arc::new(Self {
            local_peer,
            node_manager,
            peers: Arc::new(RwLock::new(HashMap::new())),
            gossip_tx: tx,
            gossip_rx: rx,
            config,
        })
    }
    
    /// Start mesh event loops - heartbeats, gossip, cleanup
    pub async fn start(self: Arc<Self>) {
        // Spawn heartbeat to node manager
        let nm = Arc::clone(&self.node_manager);
        tokio::spawn(async move { nm.start_heartbeat().await });
        
        // Spawn rebalance monitor
        let nm2 = Arc::clone(&self.node_manager);
        tokio::spawn(async move { nm2.monitor_and_rebalance().await });
        
        // Spawn gossip broadcaster
        let mesh = Arc::clone(&self);
        tokio::spawn(async move { mesh.gossip_loop().await });
        
        // Spawn peer cleanup task
        let mesh = Arc::clone(&self);
        tokio::spawn(async move { mesh.peer_cleanup_loop().await });
        
        info!("🕸️  Quantum mesh started with config {:?}", self.config);
    }
    
    /// Add or update peer in mesh
    pub async fn add_peer(&self, peer_state: Arc<PeerState>) {
        let mut peers = self.peers.write().await;
        peers.insert(peer_state.snapshot().await.id.clone(), peer_state);
        info!("🌐 Peer added to mesh");
    }
    
    /// Periodic gossip loop broadcasting peer state
    async fn gossip_loop(self: Arc<Self>) {
        let mut interval = interval(self.config.gossip_interval);
        
        loop {
            interval.tick().await;
            
            let local_snapshot = self.local_peer.snapshot().await;
            let peers = self.peers.read().await;
            
            // Prepare gossip message
            let known_peers: Vec<_> = peers.keys().cloned().collect();
            let shard_anns: Vec<_> = peers.values()
                .flat_map(|p| p.snapshot().await.shards.clone().into_iter()
                    .map(|shard_id| ShardAnnouncement {
                        shard_id,
                        owner_peer: p.snapshot().await.id.clone(),
                    }))
                .collect();
            
            let gossip_msg = GossipMessage {
                sender: local_snapshot.id.clone(),
                known_peers,
                shard_announcements: shard_anns,
            };
            
            // Fanout gossip to max_gossip_fanout peers randomly
            let selected_peers: Vec<_> = peers.values()
                .take(self.config.max_gossip_fanout)
                .cloned()
                .collect();
            
            for peer in selected_peers {
                // Simulated gossip send, replace with network send
                debug!("📡 Gossiping to peer {}", peer.snapshot().await.id);
            }
            
            // Broadcast locally for listeners
            let _ = self.gossip_tx.send(gossip_msg);
        }
    }
    
    /// Peer inactivity cleanup task
    async fn peer_cleanup_loop(self: Arc<Self>) {
        let mut interval = interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            let now = Instant::now();
            let mut peers = self.peers.write().await;
            
            peers.retain(|id, peer_state| {
                let peer = futures::executor::block_on(peer_state.snapshot());
                let elapsed = Duration::from_nanos(now.elapsed().as_nanos() as u64).as_secs();
                let active = peer.is_active && (now.elapsed() < self.config.peer_timeout);
                
                if !active {
                    warn!("🧹 Removing inactive peer {}", id);
                }
                active
            });
        }
    }
    
    /// Subscribe to gossip messages locally
    pub fn subscribe(&self) -> broadcast::Receiver<GossipMessage> {
        self.gossip_tx.subscribe()
    }
    
    /// Get connected peers count
    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }
    
    /// Find healthy peers serving a shard (for routing)
    pub async fn find_peers_for_shard(&self, shard_id: &ShardId) -> Vec<PeerState> {
        let peers = self.peers.read().await;
        peers.values()
            .filter_map(|p| {
                let peer = futures::executor::block_on(p.snapshot());
                if peer.shards.contains(shard_id) && peer.is_active && peer.health_score() > 70 {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Peer {
    /// Calculate health score based on activity and latency (placeholder)
    pub fn health_score(&self) -> u8 {
        if !self.is_active {
            0
        } else {
            100 // Simplified; replace with real metrics
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_mesh_peer_add_and_count() {
        let local_peer = Arc::new(PeerState::new(Peer::new(
            "127.0.0.1:6000".parse().unwrap(),
            SoulACL::root(),
            Some("Local Node".to_string()),
        )));
        
        let node_manager = Arc::new(NodeManager::new(Arc::new(ShardStore::default())));
        let mesh = Mesh::new(local_peer.clone(), node_manager.clone(), MeshConfig::default());
        
        mesh.add_peer(local_peer).await;
        assert_eq!(mesh.peer_count().await, 1);
    }
    
    #[tokio::test]
    async fn test_gossip_subscribe() {
        let local_peer = Arc::new(PeerState::new(Peer::new(
            "127.0.0.1:6000".parse().unwrap(),
            SoulACL::root(),
            None,
        )));
        
        let node_manager = Arc::new(NodeManager::new(Arc::new(ShardStore::default())));
        let mesh = Mesh::new(local_peer, node_manager, MeshConfig::default());
        
        let mut rx = mesh.subscribe();
        let send_result = mesh.gossip_tx.send(GossipMessage {
            sender: BlissId::new(),
            known_peers: vec![],
            shard_announcements: vec![],
        });
        assert!(send_result.is_ok());
        
        let msg = timeout(Duration::from_secs(1), rx.recv()).await;
        assert!(msg.is_ok());
    }
}