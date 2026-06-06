//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS IPFS Cluster Federation - Multi-Cluster Shard Orchestration
//! 🌐 Leader Election + Geo-Replication + Pin Consensus + Failover
//! 
//! ⚛️  Lattice Physics: Manages the "Inter-Lattice Bridge" between
//!     multiple Bethe (Storage) Lattices for global durability.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::shard_server::ipfs::{IpfsConfig, IpfsClusterStorage, IpfsError};
use crate::{
    shard::{Shard, ShardId, ShardStorage, StorageHealth, StorageBackend, storage::StorageError},
    shard_server::acl::AclEnforcer,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{info, warn, error, debug};

/// Multi-cluster federation with leader election & consensus pinning.
/// Acts as a unified StorageBackend that multiplexes over multiple IPFS clusters.
pub struct IpfsClusterFederation {
    /// Active cluster connections mapped by name
    clusters: RwLock<BTreeMap<String, Arc<IpfsClusterStorage>>>,
    
    /// Leader cluster (primary pinning target / read source)
    leader_cluster: RwLock<Option<String>>,
    
    /// Consensus quorum size (how many clusters must pin for success)
    quorum_size: usize,
    
    /// HTTP client for inter-cluster coordination checks
    client: Client,
    
    /// Federation health metrics
    metrics: Arc<RwLock<FederationMetrics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Unique name for the cluster (e.g., "us-east-1")
    pub name: String,
    /// Connection details
    pub ipfs_config: IpfsConfig,
    /// Leader election priority (higher wins)
    pub priority: u32,
    /// Geographic region tag
    pub region: String,
}

#[derive(Debug, Clone, Default)]
pub struct FederationMetrics {
    pub cluster_count: usize,
    pub leader: Option<String>,
    pub healthy_clusters: usize,
    pub pin_quorum_success: f64,
    pub last_election: u64,
}

impl IpfsClusterFederation {
    /// Create federated multi-cluster storage
    pub async fn new(
        cluster_configs: Vec<ClusterConfig>,
        quorum_size: usize,
        acl_enforcer: Arc<AclEnforcer>,
    ) -> Result<Self, FederationError> {
        let mut clusters = BTreeMap::new();
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| FederationError::InitError(e.to_string()))?;

        // Initialize cluster connections
        for config in cluster_configs {
            info!("🌐 Connecting to federation cluster: {} ({})", config.name, config.ipfs_config.cluster_url);
            match IpfsClusterStorage::new(config.ipfs_config.clone(), acl_enforcer.clone()).await {
                Ok(storage) => {
                    clusters.insert(config.name.clone(), Arc::new(storage));
                }
                Err(e) => {
                    warn!("⚠️ Failed to connect to cluster '{}': {}", config.name, e);
                    // We continue even if some fail, as long as we have enough for quorum later
                }
            }
        }

        if clusters.is_empty() {
             return Err(FederationError::NoHealthyClusters);
        }

        let federation = Self {
            clusters: RwLock::new(clusters),
            leader_cluster: RwLock::new(None),
            quorum_size,
            client,
            metrics: Arc::new(RwLock::new(FederationMetrics::default())),
        };

        // Elect initial leader
        if let Err(e) = federation.elect_leader().await {
            warn!("⚠️ Initial election failed: {}", e);
        }

        Ok(federation)
    }

    /// Elect leader based on priority + availability
    /// In a real system, this would use Raft/Paxos. Here we use a simplified priority check.
    pub async fn elect_leader(&self) -> Result<String, FederationError> {
        let clusters = self.clusters.read().await;
        
        // Filter for healthy clusters
        // Note: checking health() involves a network call, so we do it serially or parallel.
        // For simplicity, we assume if it's in the map, it was healthy at init.
        // A robust system would check active health here.
        
        // Placeholder priority logic: just pick first lexicographically for now, 
        // or assume configs were sorted by priority. 
        // Real implementation requires storing priority alongside storage instance.
        
        let candidate_name = clusters.keys().next()
            .ok_or(FederationError::NoHealthyClusters)?
            .clone();

        let mut leader_lock = self.leader_cluster.write().await;
        *leader_lock = Some(candidate_name.clone());

        let mut metrics = self.metrics.write().await;
        metrics.leader = Some(candidate_name.clone());
        metrics.last_election = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        info!("👑 Elected federation leader: {}", candidate_name);
        Ok(candidate_name)
    }

    /// Pin shard across federation with quorum consensus
    pub async fn pin_shard_quorum(&self, shard: &Shard) -> Result<(), FederationError> {
        let clusters = self.clusters.read().await;
        let cluster_list: Vec<(String, Arc<IpfsClusterStorage>)> = clusters.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
            
        let total_clusters = cluster_list.len();
        
        if total_clusters < self.quorum_size {
            warn!("⚠️ Federation size ({}) smaller than quorum ({})", total_clusters, self.quorum_size);
        }

        let mut handles = Vec::new();

        // Parallel pinning
        for (name, cluster) in cluster_list {
            let shard_clone = shard.clone();
            let handle = tokio::spawn(async move {
                match cluster.store(&shard_clone).await {
                    Ok(_) => (name, true),
                    Err(e) => {
                        warn!("Cluster pin failed: {}", e);
                        (name, false)
                    }
                }
            });
            handles.push(handle);
        }

        let mut success_count = 0;
        for handle in handles {
            if let Ok((_name, success)) = handle.await {
                if success { success_count += 1; }
            }
        }

        if success_count < self.quorum_size {
            return Err(FederationError::QuorumFailed(success_count, self.quorum_size));
        }

        debug!("✅ Federation Quorum reached: {}/{}", success_count, total_clusters);
        Ok(())
    }

    /// Leader failover detection
    pub async fn monitor_leader_health(&self) -> Result<(), FederationError> {
        let leader = {
            self.leader_cluster.read().await.clone()
        };

        if let Some(leader_name) = leader {
            let clusters = self.clusters.read().await;
            if let Some(cluster) = clusters.get(&leader_name) {
                // Check if leader is actually healthy
                if cluster.health().await.is_err() {
                    warn!("🚨 Leader {} is unhealthy! Triggering election...", leader_name);
                    drop(clusters); // Release lock before election
                    self.elect_leader().await?;
                }
            }
        } else {
            // No leader, try to elect one
            self.elect_leader().await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl ShardStorage for IpfsClusterFederation {
    async fn store(&self, shard: &Shard) -> Result<(), StorageError> {
        self.pin_shard_quorum(shard).await
            .map_err(|e| StorageError::BackendError(e.to_string()))
    }

    async fn load(&self, shard_id: &ShardId) -> Result<Shard, StorageError> {
        // 1. Try Leader
        let leader_opt = self.leader_cluster.read().await.clone();
        
        if let Some(leader) = leader_opt {
            let clusters = self.clusters.read().await;
            if let Some(storage) = clusters.get(&leader) {
                match storage.load(shard_id).await {
                    Ok(shard) => return Ok(shard),
                    Err(e) => debug!("Leader failed to load shard: {}", e),
                }
            }
        }

        // 2. Broadcast / Fallback to any healthy cluster
        let clusters = self.clusters.read().await;
        for (name, storage) in clusters.iter() {
            if Some(name) == self.leader_cluster.read().await.as_ref() { continue; } // Skip leader as we tried it

            match storage.load(shard_id).await {
                Ok(shard) => {
                    debug!("Recovered shard {} from secondary cluster {}", shard_id, name);
                    return Ok(shard);
                }
                Err(_) => continue,
            }
        }

        Err(StorageError::NotFound)
    }

    async fn delete(&self, shard_id: &ShardId) -> Result<(), StorageError> {
        // Delete from ALL clusters to ensure it's gone
        let clusters = self.clusters.read().await;
        let mut futures = Vec::new();

        for storage in clusters.values() {
             let s = storage.clone();
             let id = shard_id.clone();
             futures.push(tokio::spawn(async move {
                 s.delete(&id).await
             }));
        }
        
        // Wait for all (best effort)
        for f in futures { let _ = f.await; }
        
        Ok(())
    }

    async fn list(&self, prefix: Option<&str>) -> Result<Vec<ShardId>, StorageError> {
        // List from leader only for consistency
        let leader_opt = self.leader_cluster.read().await.clone();
         if let Some(leader) = leader_opt {
            let clusters = self.clusters.read().await;
            if let Some(storage) = clusters.get(&leader) {
                return storage.list(prefix).await;
            }
        }
        Err(StorageError::BackendError("No healthy leader for listing".into()))
    }

    async fn health(&self) -> Result<StorageHealth, StorageError> {
        let metrics = self.metrics.read().await;
        let clusters = self.clusters.read().await;
        
        // Aggregate health
        Ok(StorageHealth {
            backend: StorageBackend::IPFS {
                node_id: format!("federation-leader-{:?}", metrics.leader),
            },
            available_bytes: u64::MAX,
            used_bytes: 0, 
            shard_count: 0, // Should aggregate
            latency_ms: 300.0, // Federation has overhead
            healthy: metrics.leader.is_some() && !clusters.is_empty(),
        })
    }
}

#[derive(Debug, Error)]
pub enum FederationError {
    #[error("Initialization error: {0}")]
    InitError(String),
    #[error("No healthy clusters available")]
    NoHealthyClusters,
    #[error("Quorum failed: {0}/{1} succeeded")]
    QuorumFailed(usize, usize),
}

/// Production federation macro
#[macro_export]
macro_rules! quick_federation {
    ($configs:expr, $acl:expr) => {{
        // Simple quorum: Majority + 1
        let quorum = $configs.len() / 2 + 1;
        IpfsClusterFederation::new($configs, quorum, $acl).await.unwrap()
    }};
}