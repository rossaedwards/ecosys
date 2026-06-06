//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Auto-Heal Daemon - Self-Healing Quantum Shard Repair Engine
//! 🛸 Continuous Integrity Checks + Shard Replication + Swarm Health Monitoring
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    network::{
        peer::PeerState,
        mesh::Mesh,
        replication::{ReplicationEngine, ReplicationStatus},
    },
    shard::ShardId,
    gov::BlissId,
    storage::shard_store::ShardStore,
    network::defense::{dos_protector::DosProtector, intrusion_detector::IntrusionDetector},
    audit::holographic_logger::{AuditEvent, HolographicLogger},
    crypto::pqc::dilithium_sig,
};
use std::{
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::RwLock,
    time::{interval, Instant},
};
use tracing::{info, warn, error};
use futures::future::join_all;

/// Auto-Heal Daemon continuously scans for shard inconsistencies and triggers healing
pub struct AutoHealDaemon {
    /// Reference to shard store for direct operations
    shard_store: Arc<ShardStore>,
    
    /// Mesh for peer discovery and shard location
    mesh: Arc<Mesh>,
    
    /// Replication engine for shard repairs
    replication_engine: Arc<ReplicationEngine>,
    
    /// Heal interval duration
    heal_interval: Duration,
    
    /// Concurrency control for parallel healing
    concurrency_limit: usize,
    
    /// Active healing tasks
    active_heals: Arc<RwLock<usize>>,

    /// DoS protection
    dos_protector: Arc<RwLock<DosProtector>>,

    /// Intrusion detection
    intrusion_detector: Arc<RwLock<IntrusionDetector>>,

    /// Holographic audit logger for signed events
    audit_logger: Arc<HolographicLogger>,

    /// Dilithium keys for audit signing
    audit_public_key: dilithium_sig::PublicKey,
    audit_private_key: dilithium_sig::PrivateKey,
}

impl AutoHealDaemon {
    /// Forge new auto-heal daemon
    pub fn new(
        shard_store: Arc<ShardStore>,
        mesh: Arc<Mesh>,
        replication_engine: Arc<ReplicationEngine>,
        heal_interval: Duration,
        concurrency_limit: usize,
        audit_logger: Arc<HolographicLogger>,
        dos_max_per_sec: u64,
        intrusion_threshold: u64,
    ) -> Arc<Self> {
        let (audit_public_key, audit_private_key) = dilithium_sig::dilithium_keygen();
        Arc::new(Self {
            shard_store,
            mesh,
            replication_engine,
            heal_interval,
            concurrency_limit,
            active_heals: Arc::new(RwLock::new(0)),
            dos_protector: Arc::new(RwLock::new(DosProtector::new(dos_max_per_sec))),
            intrusion_detector: Arc::new(RwLock::new(IntrusionDetector::new(intrusion_threshold))),
            audit_logger,
            audit_public_key,
            audit_private_key,
        })
    }
    
    /// Start continuous healing loop
    pub async fn start(self: Arc<Self>) {
        let mut ticker = interval(self.heal_interval);
        
        loop {
            ticker.tick().await;
            self.scan_and_heal().await;
        }
    }
    
    /// Scan shards in mesh and heal missing or corrupted replicas
    async fn scan_and_heal(&self) {
        let replicas_map = self.replication_engine.replica_map.read().await;
        let peers_map = self.mesh.peers.read().await;
        let mut healing_futures = Vec::new();
        
        for (shard_id, replica_peers) in replicas_map.iter() {
            // Check for missing or unhealthy replicas
            let mut healthy_count = 0;
            let mut unhealthy_peers = Vec::new();
            
            for peer_id in replica_peers {
                if let Some(peer_state) = peers_map.get(peer_id) {
                    let peer = peer_state.snapshot().await;
                    if let Err(e) = self.apply_defense_checks(&peer).await {
                        warn!("Defense check failed for {}: {}", peer.id, e);
                    }
                    if peer.is_active && peer.health_score() >= 70 {
                        healthy_count += 1;
                    } else {
                        unhealthy_peers.push(peer);
                    }
                } else {
                    unhealthy_peers.push(PeerState::placeholder(peer_id.clone()));
                }
            }
            
            if healthy_count < self.replication_engine.config.replication_factor {
                info!("🩹 Healing shard {}: {} healthy replicas, {} unhealthy", shard_id, healthy_count, unhealthy_peers.len());
                
                // Spawn healing tasks with concurrency control
                for _ in 0..(self.replication_engine.config.replication_factor - healthy_count) {
                    if *self.active_heals.read().await >= self.concurrency_limit {
                        break; // Respect concurrency limit
                    }
                    
                    let engine = Arc::clone(&self.replication_engine);
                    let shard_id_clone = shard_id.clone();
                    let heals = Arc::clone(&self.active_heals);
                    
                    let heal_task = tokio::spawn(async move {
                        {
                            let mut active = heals.write().await;
                            *active += 1;
                        }
                        
                        // Trigger repair (simplified healing trigger)
                        let _ = engine.replicate_shard_placeholder(&shard_id_clone).await;
                        
                        {
                            let mut active = heals.write().await;
                            *active -= 1;
                        }
                    });
                    
                    healing_futures.push(heal_task);
                }
            }
        }
        
        // Wait for all healing tasks to complete
        let _ = join_all(healing_futures).await;
    }

    async fn apply_defense_checks(&self, peer: &crate::network::peer::Peer) -> Result<(), String> {
        {
            let mut dos = self.dos_protector.write().await;
            if !dos.allow(&peer.id.to_string()) {
                return Err("DoS limiter triggered".to_string());
            }
        }

        let intrusion_flagged = {
            let mut detector = self.intrusion_detector.write().await;
            detector.record_event(&peer.id.to_string())
        };

        if intrusion_flagged {
            self.trigger_decoherence_recovery(peer).await?;
        }

        Ok(())
    }

    async fn trigger_decoherence_recovery(&self, peer: &crate::network::peer::Peer) -> Result<(), String> {
        self.audit_logger
            .trigger_holographic_redistribution()
            .map_err(|e| format!("Decoherence recovery failed: {}", e))?;

        let event = AuditEvent::new(
            "autoheal".to_string(),
            "BAN_NODE".to_string(),
            Some(peer.id.to_string()),
            format!("Intrusion detected; banning peer {}", peer.id),
            &self.audit_private_key,
        ).map_err(|e| e.to_string())?;

        self.audit_logger
            .log_and_verify(event, &self.audit_public_key)
            .await
            .map_err(|e| format!("Audit log failure: {}", e))?;

        Ok(())
    }
}

impl ReplicationEngine {
    /// Placeholder to initiate replicate shard by shard ID only for healing
    pub async fn replicate_shard_placeholder(&self, shard_id: &ShardId) -> Result<(), crate::network::replication::ReplicationError> {
        // Load shard from local store to replicate anew
        let shard_opt = self.shard_store.load_shard(shard_id).await.ok();
        if let Some(shard) = shard_opt {
            self.replicate_shard(&shard, &BlissId::genesis()).await?;
        } else {
            warn!("Shard {} missing locally; cannot start replication", shard_id);
        }
        Ok(())
    }
}

impl PeerState {
    /// Create placeholder peer state for missing/unreachable peer IDs
    pub fn placeholder(peer_id: BlissId) -> Self {
        let peer = crate::network::peer::Peer {
            id: peer_id,
            signature: Default::default(),
            address: "0.0.0.0:0".parse().unwrap(),
            last_heartbeat_ns: 0,
            is_active: false,
            shards: Vec::new(),
            acl: SoulACL::default(),
            metadata: None,
        };
        Self::new(peer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::shard_store::ShardStore;
    use crate::network::{mesh::Mesh, replication::ReplicationConfig};
    use std::time::Duration;
    use futures::executor::block_on;
    use crate::tts::tts_engine::TtsVoice;

    #[tokio::test]
    async fn test_autoheal_trigger() {
        let shard_store = Arc::new(ShardStore::default());
        let mesh = Arc::new(Mesh::new(
            Arc::new(PeerState::placeholder(BlissId::new())),
            Arc::new(NodeManager::new(Arc::clone(&shard_store))),
            crate::network::mesh::MeshConfig::default(),
        ));
        
        let replication_engine = ReplicationEngine::new(shard_store.clone(), mesh.clone(), ReplicationConfig::default());
        let audit_logger = Arc::new(HolographicLogger::new("autoheal_test_log.json", TtsVoice::Default).unwrap());
        let autoheal = AutoHealDaemon::new(
            shard_store,
            mesh,
            replication_engine,
            Duration::from_secs(1),
            2,
            audit_logger,
            100,
            5,
        );
        
        let autoheal_clone = autoheal.clone();
        
        // Run healing loop for one tick
        tokio::spawn(async move {
            autoheal_clone.scan_and_heal().await;
        });
        
        // Wait a moment for async tasks
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}