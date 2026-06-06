//! ═══════════════════════════════════════════════════════════════════
//! 🌐 AuraFS Core Network Orchestrator - Distributed Coordination
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Network orchestration with:
//! - Peer discovery and management
//! - Shard replication coordination
//! - Health monitoring
//! - Load balancing
//! ═══════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, broadcast};

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, internal, client, ShardId, BlissId};
use crate::core::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::core::health::{HealthStatus, ComponentHealth, HealthChecker};

/// Node ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Node role in the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeRole {
    /// Full storage node
    Storage,
    /// Gateway node (API access)
    Gateway,
    /// Coordinator node (orchestration)
    Coordinator,
    /// Relay node (mesh network)
    Relay,
    /// Observer node (read-only)
    Observer,
}

/// Node health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl From<NodeHealth> for HealthStatus {
    fn from(h: NodeHealth) -> Self {
        match h {
            NodeHealth::Healthy => HealthStatus::Healthy,
            NodeHealth::Degraded => HealthStatus::Degraded,
            NodeHealth::Unhealthy => HealthStatus::Unhealthy,
            NodeHealth::Unknown => HealthStatus::Unknown,
        }
    }
}

/// Peer node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerNode {
    /// Node ID
    pub id: NodeId,
    /// Node role
    pub role: NodeRole,
    /// Network address
    pub address: String,
    /// Port
    pub port: u16,
    /// Health status
    pub health: NodeHealth,
    /// Last heartbeat
    pub last_heartbeat: SystemTime,
    /// Latency to peer (microseconds)
    pub latency_us: Option<u64>,
    /// Available storage (bytes)
    pub available_storage_bytes: u64,
    /// Shard count
    pub shard_count: u64,
    /// Capabilities
    pub capabilities: HashSet<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl PeerNode {
    pub fn new(id: NodeId, role: NodeRole, address: String, port: u16) -> Self {
        Self {
            id,
            role,
            address,
            port,
            health: NodeHealth::Unknown,
            last_heartbeat: SystemTime::now(),
            latency_us: None,
            available_storage_bytes: 0,
            shard_count: 0,
            capabilities: HashSet::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Get socket address
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let addr_str = format!("{}:{}", self.address, self.port);
        addr_str.parse().map_err(|e| client(
            AuraFSError::Network {
                transient: false,
                message: format!("Invalid address '{}': {}", addr_str, e),
            },
            ErrorPhase::Network,
            ErrorCode::InvalidInput,
        ))
    }
    
    /// Check if node is available
    pub fn is_available(&self) -> bool {
        self.health == NodeHealth::Healthy || self.health == NodeHealth::Degraded
    }
    
    /// Update heartbeat
    pub fn heartbeat(&mut self) {
        self.last_heartbeat = SystemTime::now();
    }
    
    /// Check if heartbeat is stale
    pub fn is_stale(&self, timeout: Duration) -> bool {
        self.last_heartbeat.elapsed().map(|d| d > timeout).unwrap_or(true)
    }
}

/// Network event
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// Node joined the network
    NodeJoined(NodeId),
    /// Node left the network
    NodeLeft(NodeId),
    /// Node health changed
    NodeHealthChanged { node_id: NodeId, old: NodeHealth, new: NodeHealth },
    /// Shard replicated
    ShardReplicated { shard_id: ShardId, target_nodes: Vec<NodeId> },
    /// Shard lost (below replication threshold)
    ShardUnderReplicated { shard_id: ShardId, current_replicas: u32, target_replicas: u32 },
}

/// Replication strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationStrategy {
    /// Replicate to N random healthy nodes
    Random { count: u32 },
    /// Replicate to specific nodes
    Targeted { nodes: Vec<NodeId> },
    /// Replicate based on locality (same region/rack)
    Locality { same_region: bool, min_racks: u32 },
    /// Erasure coding
    ErasureCoding { data_shards: u32, parity_shards: u32 },
}

impl Default for ReplicationStrategy {
    fn default() -> Self {
        Self::Random { count: 3 }
    }
}

/// Network orchestrator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    /// Local node ID
    pub node_id: NodeId,
    /// Local node role
    pub role: NodeRole,
    /// Listen address
    pub listen_address: String,
    /// Listen port
    pub listen_port: u16,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
    /// Heartbeat timeout (consider node dead)
    pub heartbeat_timeout: Duration,
    /// Default replication strategy
    pub replication_strategy: ReplicationStrategy,
    /// Maximum concurrent replication tasks
    pub max_concurrent_replications: usize,
    /// Enable peer discovery
    pub enable_discovery: bool,
    /// Bootstrap nodes
    pub bootstrap_nodes: Vec<String>,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            node_id: NodeId::generate(),
            role: NodeRole::Storage,
            listen_address: "0.0.0.0".to_string(),
            listen_port: 7890,
            heartbeat_interval: Duration::from_secs(5),
            heartbeat_timeout: Duration::from_secs(30),
            replication_strategy: ReplicationStrategy::default(),
            max_concurrent_replications: 10,
            enable_discovery: true,
            bootstrap_nodes: Vec::new(),
        }
    }
}

/// Network orchestrator trait
#[async_trait::async_trait]
pub trait NetworkOrchestrator: Send + Sync {
    /// Get local node info
    fn local_node(&self) -> &PeerNode;
    
    /// Get all known peers
    async fn peers(&self) -> Vec<PeerNode>;
    
    /// Get healthy peers for role
    async fn healthy_peers(&self, role: Option<NodeRole>) -> Vec<PeerNode>;
    
    /// Add peer
    async fn add_peer(&self, peer: PeerNode) -> Result<()>;
    
    /// Remove peer
    async fn remove_peer(&self, node_id: &NodeId) -> Result<()>;
    
    /// Get peer by ID
    async fn get_peer(&self, node_id: &NodeId) -> Option<PeerNode>;
    
    /// Update peer health
    async fn update_peer_health(&self, node_id: &NodeId, health: NodeHealth) -> Result<()>;
    
    /// Select nodes for shard replication
    async fn select_replication_targets(
        &self,
        shard_id: &ShardId,
        strategy: &ReplicationStrategy,
        exclude: &[NodeId],
    ) -> Result<Vec<PeerNode>>;
    
    /// Request shard replication
    async fn request_replication(
        &self,
        shard_id: &ShardId,
        data: Vec<u8>,
        targets: &[NodeId],
    ) -> Result<Vec<NodeId>>;
    
    /// Health check
    async fn health_check(&self) -> ComponentHealth;
    
    /// Subscribe to network events
    fn subscribe(&self) -> broadcast::Receiver<NetworkEvent>;
}

/// Default network orchestrator implementation
pub struct DefaultNetworkOrchestrator {
    config: OrchestratorConfig,
    local_node: PeerNode,
    peers: Arc<RwLock<HashMap<NodeId, PeerNode>>>,
    circuit_breakers: Arc<RwLock<HashMap<NodeId, CircuitBreaker>>>,
    event_sender: broadcast::Sender<NetworkEvent>,
    _event_receiver: broadcast::Receiver<NetworkEvent>,
    start_time: Instant,
}

impl DefaultNetworkOrchestrator {
    /// Create new orchestrator
    pub fn new(config: OrchestratorConfig) -> Self {
        let local_node = PeerNode::new(
            config.node_id.clone(),
            config.role,
            config.listen_address.clone(),
            config.listen_port,
        );
        
        let (event_sender, event_receiver) = broadcast::channel(1000);
        
        Self {
            config,
            local_node,
            peers: Arc::new(RwLock::new(HashMap::new())),
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            _event_receiver: event_receiver,
            start_time: Instant::now(),
        }
    }
    
    /// Start background tasks
    pub async fn start(&self) -> Result<()> {
        // Start heartbeat task
        let peers = Arc::clone(&self.peers);
        let config = self.config.clone();
        let event_sender = self.event_sender.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(config.heartbeat_interval).await;
                
                // Check for stale peers
                let mut stale_peers = Vec::new();
                {
                    let peers_guard = peers.read().await;
                    for (id, peer) in peers_guard.iter() {
                        if peer.is_stale(config.heartbeat_timeout) {
                            stale_peers.push(id.clone());
                        }
                    }
                }
                
                // Update stale peers as unhealthy
                for id in stale_peers {
                    let mut peers_guard = peers.write().await;
                    if let Some(peer) = peers_guard.get_mut(&id) {
                        if peer.health != NodeHealth::Unhealthy {
                            let old_health = peer.health;
                            peer.health = NodeHealth::Unhealthy;
                            let _ = event_sender.send(NetworkEvent::NodeHealthChanged {
                                node_id: id.clone(),
                                old: old_health,
                                new: NodeHealth::Unhealthy,
                            });
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Get or create circuit breaker for peer
    async fn get_circuit_breaker(&self, node_id: &NodeId) -> CircuitBreaker {
        let mut cbs = self.circuit_breakers.write().await;
        
        if let Some(cb) = cbs.get(node_id) {
            return CircuitBreaker::new(
                format!("peer_{}", node_id),
                CircuitBreakerConfig::default(),
            );
        }
        
        let cb = CircuitBreaker::new(
            format!("peer_{}", node_id),
            CircuitBreakerConfig::default(),
        );
        cbs.insert(node_id.clone(), CircuitBreaker::new(
            format!("peer_{}", node_id),
            CircuitBreakerConfig::default(),
        ));
        
        cb
    }
}

#[async_trait::async_trait]
impl NetworkOrchestrator for DefaultNetworkOrchestrator {
    fn local_node(&self) -> &PeerNode {
        &self.local_node
    }
    
    async fn peers(&self) -> Vec<PeerNode> {
        self.peers.read().await.values().cloned().collect()
    }
    
    async fn healthy_peers(&self, role: Option<NodeRole>) -> Vec<PeerNode> {
        self.peers.read().await
            .values()
            .filter(|p| {
                p.is_available() && role.map(|r| p.role == r).unwrap_or(true)
            })
            .cloned()
            .collect()
    }
    
    async fn add_peer(&self, peer: PeerNode) -> Result<()> {
        let node_id = peer.id.clone();
        let is_new = {
            let mut peers = self.peers.write().await;
            let is_new = !peers.contains_key(&node_id);
            peers.insert(node_id.clone(), peer);
            is_new
        };
        
        if is_new {
            let _ = self.event_sender.send(NetworkEvent::NodeJoined(node_id));
        }
        
        Ok(())
    }
    
    async fn remove_peer(&self, node_id: &NodeId) -> Result<()> {
        let removed = {
            let mut peers = self.peers.write().await;
            peers.remove(node_id).is_some()
        };
        
        if removed {
            let _ = self.event_sender.send(NetworkEvent::NodeLeft(node_id.clone()));
        }
        
        Ok(())
    }
    
    async fn get_peer(&self, node_id: &NodeId) -> Option<PeerNode> {
        self.peers.read().await.get(node_id).cloned()
    }
    
    async fn update_peer_health(&self, node_id: &NodeId, health: NodeHealth) -> Result<()> {
        let old_health = {
            let mut peers = self.peers.write().await;
            if let Some(peer) = peers.get_mut(node_id) {
                let old = peer.health;
                peer.health = health;
                peer.heartbeat();
                Some(old)
            } else {
                None
            }
        };
        
        if let Some(old) = old_health {
            if old != health {
                let _ = self.event_sender.send(NetworkEvent::NodeHealthChanged {
                    node_id: node_id.clone(),
                    old,
                    new: health,
                });
            }
        }
        
        Ok(())
    }
    
    async fn select_replication_targets(
        &self,
        _shard_id: &ShardId,
        strategy: &ReplicationStrategy,
        exclude: &[NodeId],
    ) -> Result<Vec<PeerNode>> {
        let healthy = self.healthy_peers(Some(NodeRole::Storage)).await;
        let exclude_set: HashSet<_> = exclude.iter().collect();
        
        let candidates: Vec<PeerNode> = healthy
            .into_iter()
            .filter(|p| !exclude_set.contains(&p.id))
            .collect();
        
        match strategy {
            ReplicationStrategy::Random { count } => {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                let mut selected = candidates;
                selected.shuffle(&mut rng);
                Ok(selected.into_iter().take(*count as usize).collect())
            }
            ReplicationStrategy::Targeted { nodes } => {
                let node_set: HashSet<_> = nodes.iter().collect();
                Ok(candidates.into_iter()
                    .filter(|p| node_set.contains(&p.id))
                    .collect())
            }
            ReplicationStrategy::Locality { .. } => {
                // Simplified: just return random selection
                // In production, implement rack/region awareness
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                let mut selected = candidates;
                selected.shuffle(&mut rng);
                Ok(selected.into_iter().take(3).collect())
            }
            ReplicationStrategy::ErasureCoding { data_shards, parity_shards } => {
                let total = *data_shards + *parity_shards;
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                let mut selected = candidates;
                selected.shuffle(&mut rng);
                Ok(selected.into_iter().take(total as usize).collect())
            }
        }
    }
    
    async fn request_replication(
        &self,
        shard_id: &ShardId,
        _data: Vec<u8>,
        targets: &[NodeId],
    ) -> Result<Vec<NodeId>> {
        // In production, this would send the actual data to target nodes
        // For now, we simulate success
        let mut successful = Vec::new();
        
        for node_id in targets {
            let cb = self.get_circuit_breaker(node_id).await;
            
            // Simulate network request with circuit breaker
            let result: std::result::Result<(), String> = Ok(()); // Simulated success
            
            match cb.execute(async { result }).await {
                Ok(_) => {
                    successful.push(node_id.clone());
                }
                Err(e) => {
                    tracing::warn!(
                        shard_id = %shard_id,
                        node_id = %node_id,
                        error = %e,
                        "Replication failed"
                    );
                }
            }
        }
        
        if !successful.is_empty() {
            let _ = self.event_sender.send(NetworkEvent::ShardReplicated {
                shard_id: shard_id.clone(),
                target_nodes: successful.clone(),
            });
        }
        
        Ok(successful)
    }
    
    async fn health_check(&self) -> ComponentHealth {
        let start = Instant::now();
        let peers = self.peers().await;
        let healthy_count = peers.iter().filter(|p| p.is_available()).count();
        let total_count = peers.len();
        
        let status = if healthy_count == total_count {
            HealthStatus::Healthy
        } else if healthy_count > 0 {
            HealthStatus::Degraded
        } else if total_count == 0 {
            HealthStatus::Healthy // No peers required
        } else {
            HealthStatus::Unhealthy
        };
        
        let mut health = ComponentHealth {
            name: "network_orchestrator".to_string(),
            status,
            message: Some(format!("{}/{} peers healthy", healthy_count, total_count)),
            last_checked: SystemTime::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: HashMap::new(),
        };
        
        health.metadata.insert("total_peers".to_string(), total_count.to_string());
        health.metadata.insert("healthy_peers".to_string(), healthy_count.to_string());
        health.metadata.insert("uptime_secs".to_string(), self.start_time.elapsed().as_secs().to_string());
        
        health
    }
    
    fn subscribe(&self) -> broadcast::Receiver<NetworkEvent> {
        self.event_sender.subscribe()
    }
}

#[async_trait::async_trait]
impl HealthChecker for DefaultNetworkOrchestrator {
    async fn check(&self) -> ComponentHealth {
        self.health_check().await
    }
    
    fn name(&self) -> &'static str {
        "network_orchestrator"
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_orchestrator_creation() {
        let config = OrchestratorConfig::default();
        let orch = DefaultNetworkOrchestrator::new(config);
        
        assert!(orch.peers().await.is_empty());
    }
    
    #[tokio::test]
    async fn test_peer_management() {
        let config = OrchestratorConfig::default();
        let orch = DefaultNetworkOrchestrator::new(config);
        
        let peer = PeerNode::new(
            NodeId::new("peer1"),
            NodeRole::Storage,
            "127.0.0.1".to_string(),
            7891,
        );
        
        orch.add_peer(peer.clone()).await.unwrap();
        
        assert_eq!(orch.peers().await.len(), 1);
        assert!(orch.get_peer(&peer.id).await.is_some());
        
        orch.remove_peer(&peer.id).await.unwrap();
        assert!(orch.get_peer(&peer.id).await.is_none());
    }
    
    #[tokio::test]
    async fn test_health_update() {
        let config = OrchestratorConfig::default();
        let orch = DefaultNetworkOrchestrator::new(config);
        
        let mut peer = PeerNode::new(
            NodeId::new("peer1"),
            NodeRole::Storage,
            "127.0.0.1".to_string(),
            7891,
        );
        peer.health = NodeHealth::Healthy;
        
        orch.add_peer(peer.clone()).await.unwrap();
        
        let healthy = orch.healthy_peers(None).await;
        assert_eq!(healthy.len(), 1);
        
        orch.update_peer_health(&peer.id, NodeHealth::Unhealthy).await.unwrap();
        
        let healthy = orch.healthy_peers(None).await;
        assert!(healthy.is_empty());
    }
    
    #[tokio::test]
    async fn test_replication_target_selection() {
        let config = OrchestratorConfig::default();
        let orch = DefaultNetworkOrchestrator::new(config);
        
        // Add healthy storage peers
        for i in 0..5 {
            let mut peer = PeerNode::new(
                NodeId::new(format!("peer{}", i)),
                NodeRole::Storage,
                format!("192.168.1.{}", i),
                7891,
            );
            peer.health = NodeHealth::Healthy;
            orch.add_peer(peer).await.unwrap();
        }
        
        let shard_id = ShardId::new(b"test").unwrap();
        let strategy = ReplicationStrategy::Random { count: 3 };
        
        let targets = orch.select_replication_targets(&shard_id, &strategy, &[]).await.unwrap();
        assert_eq!(targets.len(), 3);
    }
}
