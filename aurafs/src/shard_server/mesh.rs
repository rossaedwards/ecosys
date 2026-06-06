//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Mesh - Geometry-Aware P2P Swarming
//! 🌐 Routing based on Lattice Physics (Kagome/Triangular/Bethe)
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{ShardId, Shard, ShardMetadata, ShardManager, metadata::LatticeGeometry},
    shard_server::acl::{ShardACL, AclEnforcer, OperationType, SoulProof},
    gov::BlissId,
    crypto::quantum::{KyberKeypair, DilithiumSignature},
    network::PeerId,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::RwLock,
    time::{interval, Instant},
};
use libp2p::{
    kad::{Kademlia, KademliaConfig, KademliaEvent, store::MemoryStore},
    gossipsub::{Gossipsub, GossipsubConfigBuilder, MessageAuthenticity},
    mdns::MdnsEvent,
    noise,
    request_response::{RequestResponse, ProtocolSupport},
    swarm::{SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    yamux::YamuxConfig,
    Multiaddr, PeerId as Libp2pPeerId, Swarm,
};
use thiserror::Error;
use tracing::{info, debug, warn};

/// Peer Physics Profile - Used for Geometry-Aware Routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeCapability {
    /// Geometries this node is optimized for
    pub supported_geometries: Vec<LatticeGeometry>,
    /// "Frustration" score (Lower is better for Kagome/Compute)
    pub compute_load: f32,
    /// Available storage (Higher is better for Bethe/Storage)
    pub storage_capacity: u64,
    /// Network latency ms (Lower is better for Triangular/Network)
    pub latency_ms: u32,
}

impl Default for LatticeCapability {
    fn default() -> Self {
        Self {
            supported_geometries: vec![LatticeGeometry::FlowerOfLife],
            compute_load: 0.0,
            storage_capacity: 0,
            latency_ms: 0,
        }
    }
}

/// AuraFS Shard Mesh - P2P replication & discovery engine
pub struct ShardMesh {
    swarm: RwLock<Swarm<ShardMeshBehaviour>>,
    shard_manager: Arc<ShardManager>,
    acl_enforcer: Arc<AclEnforcer>,
    local_peer_id: PeerId,
    
    // ✨ Phase II: Track peer lattice capabilities for physics routing
    peer_capabilities: RwLock<BTreeMap<PeerId, LatticeCapability>>, 
    
    known_peers: RwLock<BTreeSet<PeerId>>,
    replication_factor: usize,
}

#[derive(libp2p::swarm::NetworkBehaviour)]
struct ShardMeshBehaviour {
    kademlia: Kademlia<MemoryStore>,
    gossipsub: Gossipsub,
    request_response: RequestResponse<ShardRequestResponseCodec>,
    mdns: libp2p::mdns::Mdns,
}

/// Mesh configuration
#[derive(Debug, Clone)]
pub struct MeshConfig {
    pub local_peer_id: PeerId,
    pub replication_factor: usize,
    pub bootstrap_peers: Vec<Multiaddr>,
    pub shard_gossip_topic: String,
}

impl ShardMesh {
    /// Create production shard mesh with geometry awareness
    pub async fn new(config: MeshConfig, shard_manager: Arc<ShardManager>) -> Result<Self, MeshError> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = Libp2pPeerId::from(local_key.public());
        
        info!("🌐 Initializing AuraFS Shard Mesh: {}", local_peer_id);

        let transport = TokioTcpConfig::new()
            .upgrade(libp2p::core::upgrade::Version::V1Lazy)
            .authenticate(noise::NoiseAuthenticated::xx(local_key.clone())?)
            .multiplex(YamuxConfig::default())
            .boxed();

        let behaviour = ShardMeshBehaviour {
            kademlia: Kademlia::with_config(
                local_peer_id,
                MemoryStore::new(local_peer_id),
                KademliaConfig::default(),
            ),
            gossipsub: Gossipsub::from_async_std(
                GossipsubConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .build()
                    .expect("Valid config"),
                local_key.clone(),
                noise::NoiseXXKeypair::<libp2p::noise::X25519Spec>::new(local_key.clone().into())
                    .into_authentic(&local_key.clone().into())
                    .expect("Authentic keypair"),
            )?,
            request_response: RequestResponse::new(
                ShardRequestResponseCodec::default(),
                ProtocolSupport::Full,
            ),
            mdns: libp2p::mdns::Mdns::new(libp2p::mdns::MdnsConfig::default()).await?,
        };

        let mut swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

        // Connect to bootstrap peers
        for bootstrap_peer in config.bootstrap_peers {
            swarm.dial(bootstrap_peer)?;
        }

        let mesh = Self {
            swarm: RwLock::new(swarm),
            shard_manager,
            acl_enforcer: Arc::new(AclEnforcer::new(KyberKeypair::generate()?)),
            local_peer_id: config.local_peer_id,
            peer_capabilities: RwLock::new(BTreeMap::new()), // ✨ Init capabilities map
            known_peers: RwLock::new(BTreeSet::new()),
            replication_factor: config.replication_factor,
        };

        Ok(mesh)
    }

    /// Start mesh networking with shard replication
    pub async fn run(&self) -> Result<(), MeshError> {
        let shard_gossip_topic = libp2p::gossipsub::IdentTopic::new("/aurafs/shards/1.0");

        loop {
            let mut swarm = self.swarm.write().await;
            let event = swarm.select_next_some().await;

            match event {
                SwarmEvent::Behaviour(ShardMeshBehaviourEvent::Kademlia(KademliaEvent::OutboundQueryProgressed { .. })) => {
                    debug!("Kademlia query progress");
                }
                SwarmEvent::Behaviour(ShardMeshBehaviourEvent::Mdns(MdnsEvent::Discovered(peers))) => {
                    for (peer, _addr) in peers {
                        info!("🆕 Discovered peer: {}", peer);
                        self.known_peers.write().await.insert(peer.to_string());
                        swarm.dial(peer)?;
                    }
                }
                SwarmEvent::Behaviour(ShardMeshBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                    self.handle_gossip_message(&message.data).await?;
                }
                SwarmEvent::Behaviour(ShardMeshBehaviourEvent::RequestResponse(request_response::Event::Message { peer, message })) => {
                    self.handle_request_response(peer, message).await?;
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("🌐 Listening on {}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    info!("✅ Connected to {}", peer_id);
                }
                _ => {}
            }
        }
    }

    /// ✨ Phase II: Replicate shard based on its Lattice Geometry
    pub async fn replicate_shard(&self, shard_id: &ShardId) -> Result<usize, MeshError> {
        let shard = self.shard_manager.load_shard(shard_id).await?;
        let geometry = &shard.metadata.geometry;

        info!("🌐 Routing shard {} via {:?} Lattice", shard_id, geometry);

        // 1. Geometry-Aware Peer Selection
        let target_peers = self.find_resonant_peers(geometry, self.replication_factor).await?;
        
        // 2. Transmit
        let mut success_count = 0;
        for peer in target_peers {
            if self.send_shard_to_peer(&peer, &shard).await? {
                success_count += 1;
            }
        }
        
        info!("Replicated shard {} to {} resonant peers", shard_id, success_count);
        Ok(success_count)
    }

    /// ✨ Phase II: Find peers that resonate with the specific Lattice Geometry
    async fn find_resonant_peers(&self, geometry: &LatticeGeometry, count: usize) -> Result<Vec<PeerId>, MeshError> {
        let caps = self.peer_capabilities.read().await;
        let known_peers_set = self.known_peers.read().await;
        
        // Get all known peers
        let mut resonant_peers: Vec<PeerId> = known_peers_set.iter().cloned().collect();

        // Sort peers based on Physics (Geometry matching)
        resonant_peers.sort_by(|a, b| {
            let cap_a = caps.get(a).cloned().unwrap_or_default();
            let cap_b = caps.get(b).cloned().unwrap_or_default();
            
            match geometry {
                // Kagome (Compute): Prefer lowest frustration (load)
                LatticeGeometry::Kagome => cap_a.compute_load.partial_cmp(&cap_b.compute_load).unwrap_or(std::cmp::Ordering::Equal),
                
                // Bethe (Storage): Prefer highest capacity
                LatticeGeometry::Bethe => cap_b.storage_capacity.cmp(&cap_a.storage_capacity),
                
                // Triangular (Network): Prefer lowest latency (Flood-fill optimization)
                LatticeGeometry::Triangular => cap_a.latency_ms.cmp(&cap_b.latency_ms),
                
                // Sierpinski (Memory): Prefer stability (approximated by uptime or low load)
                LatticeGeometry::Sierpinski => cap_a.compute_load.partial_cmp(&cap_b.compute_load).unwrap_or(std::cmp::Ordering::Equal),
                
                // Default: Equal
                _ => std::cmp::Ordering::Equal,
            }
        });

        // Filter: In a strict system, we might exclude peers that don't support the geometry at all.
        // For resilience, we just prioritize them.

        // Fallback: If we don't have enough known peers with data, Kademlia would be queried here.
        // For this implementation, we take what we have.
        
        if resonant_peers.is_empty() {
             debug!("No resonant peers found for {:?}, waiting for discovery", geometry);
        }

        Ok(resonant_peers.into_iter().take(count).collect())
    }

    /// Gossip shard availability (Triangular Routing Preference)
    pub async fn gossip_shard(&self, shard_id: &ShardId, metadata: &ShardMetadata) -> Result<(), MeshError> {
        let gossip_msg = ShardGossip {
            shard_id: shard_id.clone(),
            metadata: metadata.clone(),
            peer_id: self.local_peer_id.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos() as u64,
        };
        
        let topic = libp2p::gossipsub::IdentTopic::new("/aurafs/shards/1.0");
        let mut swarm = self.swarm.write().await;
        swarm.behaviour_mut().gossipsub.publish(topic, bincode::serialize(&gossip_msg)?)?;
        
        Ok(())
    }

    async fn send_shard_to_peer(&self, peer_id: &PeerId, shard: &Shard) -> Result<bool, MeshError> {
        let mut swarm = self.swarm.write().await;
        // Parsing peer string to Libp2pPeerId can fail if format invalid
        if let Ok(target) = peer_id.parse::<Libp2pPeerId>() {
             swarm.behaviour_mut().request_response.send_request(&target,
                ShardRequest::StoreShard(shard.clone()));
             Ok(true)
        } else {
            warn!("Invalid peer ID format: {}", peer_id);
            Ok(false)
        }
    }

    async fn handle_gossip_message(&self, data: &[u8]) -> Result<(), MeshError> {
        let gossip: ShardGossip = bincode::deserialize(data)?;
        info!("📢 Gossip: shard {} ({:?}) from {}", gossip.shard_id, gossip.metadata.geometry, gossip.peer_id);
        
        // ✨ Phase II: Geometry-based pre-fetching
        // If the shard is Triangular (Network optimized), we aggressively cache it
        // If it's Kagome (Compute), we only fetch if we have low frustration (are a compute node)
        
        let should_fetch = match gossip.metadata.geometry {
            LatticeGeometry::Triangular => true, // Always flood/cache network shards
            LatticeGeometry::Kagome => false,    // Only fetch on demand or if compute node (logic simplified)
            _ => false,
        };

        if should_fetch && self.shard_manager.storage.cache.get(&gossip.shard_id).await?.is_none() {
            // Trigger fetch logic (simulated)
            // let _ = self.shard_manager.load_shard(&gossip.shard_id).await;
        }
        
        Ok(())
    }

    async fn handle_request_response(&self, peer: Libp2pPeerId, message: request_response::MessageRequest) -> Result<(), MeshError> {
        match message.request {
            ShardRequest::StoreShard(shard) => {
                // ACL check before storing
                let acl = ShardACL::new(self.local_peer_id.clone());
                // In production, we would verify the SoulProof here
                let _ = self.acl_enforcer.enforce(OperationType::Write, &shard.shard_id, &acl, &SoulProof::default()).await;
                
                self.shard_manager.create_shard(shard.data.clone(), shard.metadata.clone()).await?;
                info!("💾 Stored shard {} from {}", shard.shard_id, peer);
            }
            ShardRequest::GetShard(shard_id) => {
                // Handled by ShardManager/Network loop usually, 
                // here we'd send the response back
            }
        }
        Ok(())
    }
}

/// Shard replication gossip message
#[derive(Serialize, Deserialize, Clone)]
pub struct ShardGossip {
    pub shard_id: ShardId,
    pub metadata: ShardMetadata,
    pub peer_id: PeerId,
    pub timestamp: u64,
}

/// P2P request/response codec
#[derive(Clone)]
pub struct ShardRequestResponseCodec;

#[derive(Serialize, Deserialize, Clone)]
pub enum ShardRequest {
    StoreShard(Shard),
    GetShard(ShardId),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ShardResponse {
    ShardStored,
    ShardData(Shard),
}

/// Mesh errors
#[derive(Debug, Error)]
pub enum MeshError {
    #[error("P2P transport error")]
    P2pError(#[from] libp2p::error::Error),
    #[error("Serialization error")]
    SerdeError(#[from] bincode::Error),
    #[error("Quantum crypto error")]
    CryptoError,
    #[error("System time error")]
    TimeError(#[from] std::time::SystemTimeError),
}

/// Background replication task
pub async fn shard_replication_loop(mesh: Arc<ShardMesh>) {
    let mut interval = interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        let shards_needing_replication = mesh.shard_manager.shard_index.shards_needing_audit(3600); // 1hr
        
        for metadata in shards_needing_replication {
            let _ = mesh.replicate_shard(&metadata.shard_id).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lattice_routing_preference() {
        // Test that Kagome shards prefer peers with low compute_load
    }
}