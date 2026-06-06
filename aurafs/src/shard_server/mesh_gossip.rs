//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Mesh Gossip - Efficient Shard Propagation Engine
//! 🌐 GossipSub PubSub + Geometry-Aware Filtering
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{ShardId, ShardMetadata, ShardManager, metadata::LatticeGeometry},
    shard_server::acl::{AclEnforcer, OperationType, ShardACL},
    gov::BlissId,
};
use libp2p::{
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, IdentTopic, MessageId, MessageAuthenticity},
    identity::Keypair,
    swarm::{NetworkBehaviourEventProcess, SwarmEvent},
    PeerId as Libp2pPeerId,
    Swarm,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Gossip message to advertise shard availability
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShardGossipMessage {
    pub shard_id: ShardId,
    pub metadata: ShardMetadata,
    pub peer_id: BlissId,
    pub timestamp: u64,
}

/// Shard Mesh Gossip engine encapsulating libp2p Gossipsub
pub struct MeshGossip {
    swarm: Arc<RwLock<Swarm<Gossipsub>>>,
    shard_manager: Arc<ShardManager>,
    acl_enforcer: Arc<AclEnforcer>,
    subscribed_topics: Arc<RwLock<HashSet<IdentTopic>>>,
    known_shards: Arc<RwLock<HashMap<ShardId, u64>>>, // timestamp cache
}

impl MeshGossip {
    /// Initialize MeshGossip with keypair and shard manager
    pub async fn new(
        keypair: Keypair,
        shard_manager: Arc<ShardManager>,
        acl_enforcer: Arc<AclEnforcer>,
    ) -> Self {
        let gossipsub_config = GossipsubConfig::default();
        
        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(keypair.clone()),
            gossipsub_config,
        ).expect("Valid Gossipsub config");

        let swarm = Swarm::new(
            libp2p::development_transport(keypair.clone())
                .await
                .expect("Transport setup"),
            gossipsub,
            Libp2pPeerId::from(keypair.public()),
        );

        Self {
            swarm: Arc::new(RwLock::new(swarm)),
            shard_manager,
            acl_enforcer,
            subscribed_topics: Arc::new(RwLock::new(HashSet::new())),
            known_shards: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribe to shard gossip topic
    pub async fn subscribe(&self, topic_str: &str) -> anyhow::Result<()> {
        let topic = IdentTopic::new(topic_str);
        let mut swarm = self.swarm.write().await;
        swarm.behaviour_mut().subscribe(&topic)?;
        self.subscribed_topics.write().await.insert(topic);
        Ok(())
    }

    /// Publish shard availability gossip
    pub async fn publish_shard(&self, shard_id: &ShardId, metadata: &ShardMetadata, peer_id: &BlissId) -> anyhow::Result<()> {
        let topic = {
            let subs = self.subscribed_topics.read().await;
            subs.iter()
                .find(|t| t.topic_str().starts_with("/aurafs/shards/"))
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No shard gossip topic subscribed"))?
        };

        let gossip_msg = ShardGossipMessage {
            shard_id: shard_id.clone(),
            metadata: metadata.clone(),
            peer_id: peer_id.clone(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        let msg_bytes = bincode::serialize(&gossip_msg)?;
        let mut swarm = self.swarm.write().await;
        swarm.behaviour_mut().publish(topic, msg_bytes)?;

        Ok(())
    }

    /// Start event loop for gossip message processing
    pub async fn run(&self) -> anyhow::Result<()> {
        loop {
            let mut swarm = self.swarm.write().await;
            match swarm.next_event().await {
                SwarmEvent::Behaviour(GossipsubEvent::Message { propagation_source, message_id, message }) => {
                    debug!("Received gossip message from {}: {:?}", propagation_source, message);
                    if let Ok(gossip_msg) = bincode::deserialize::<ShardGossipMessage>(&message.data) {
                        self.handle_gossip_message(gossip_msg).await?;
                    }
                }
                SwarmEvent::Behaviour(event) => {
                    debug!("Gossipsub behaviour event: {:?}", event);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    info!("Connected to peer {}", peer_id);
                }
                _ => {}
            }
        }
    }

    /// Handle incoming gossip shard message with ACL checks and geometry filtering
    async fn handle_gossip_message(&self, gossip_msg: ShardGossipMessage) -> anyhow::Result<()> {
        let mut known_shards = self.known_shards.write().await;

        // Check timestamp freshness
        let last_seen = known_shards.get(&gossip_msg.shard_id).cloned().unwrap_or_default();
        if gossip_msg.timestamp <= last_seen {
            return Ok(());
        }

        // ACL enforcement on shard metadata
        let acl = ShardACL::new(gossip_msg.peer_id.clone());
        self.acl_enforcer.enforce(OperationType::Read, &gossip_msg.shard_id, &acl, &BlissId::genesis()) 
            .await?;

        // Update cache
        known_shards.insert(gossip_msg.shard_id.clone(), gossip_msg.timestamp);

        // ✨ Phase II: Geometry-Aware Reaction
        // If this is a Triangular shard (Network optimized), we might want to propagate it faster 
        // or pre-cache it immediately.
        
        match gossip_msg.metadata.geometry {
            LatticeGeometry::Triangular => {
                // High priority network shard: aggressive prefetch
                 let _ = self.shard_manager.load_shard(&gossip_msg.shard_id).await;
            },
            LatticeGeometry::Kagome => {
                // Compute shard: only cache if we are a compute node (logic would be checked here)
            },
            _ => {
                // Standard behavior
            }
        }

        info!("Propagated shard {} ({:?}) from {}", 
            gossip_msg.shard_id, gossip_msg.metadata.geometry, gossip_msg.peer_id);

        Ok(())
    }
}