//! AuraFS ShardStore - Quantum Shard Persistence Engine
//! f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division
//! Production shard persistence: local storage, replication tracking, GC, IPFS hooks

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{RwLock, RwLockReadGuard};
use tracing::{info, warn, debug};
use blake3::Hasher;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    gov::{BlissId, SoulACL},
    network::{ShardId, PeerState},
    dedup::ContentFingerprint,
};

/// Shard metadata with replication and health tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShardMetadata {
    pub shard_id: ShardId,
    pub soul_owner: BlissId,
    pub size_bytes: u64,
    pub fingerprint: ContentFingerprint,
    pub replicas: HashSet<BlissId>, // Peer souls hosting replicas
    pub created: u64,               // UNIX timestamp ns
    pub last_access: u64,
    pub health_score: u8,           // 0-100
}

/// Core shard storage engine
#[derive(Clone, Default)]
pub struct ShardStore {
    pub shards: Arc<RwLock<HashMap<ShardId, ShardMetadata>>>,
    pub storage_path: Arc<RwLock<PathBuf>>,
    pub metrics: Arc<RwLock<ShardStoreMetrics>>,
}

#[derive(Clone, Default, Debug)]
pub struct ShardStoreMetrics {
    pub total_shards: usize,
    pub total_size_gb: f64,
    pub healthy_shards: usize,
    pub replication_factor_avg: f64,
}

/// Shard persistence errors
#[derive(Debug, thiserror::Error)]
pub enum ShardStoreError {
    #[error("Shard {0} not found")]
    NotFound(ShardId),
    #[error("Replication quorum failed: {0}/{1} replicas")]
    QuorumFailed(usize, usize),
    #[error("Storage path invalid: {0}")]
    InvalidPath(PathBuf),
}

/// Shard store operations
#[async_trait::async_trait]
pub trait ShardStoreTrait: Send + Sync {
    async fn load_shard(&self, shard_id: &ShardId) -> Result<ShardMetadata, ShardStoreError>;
    async fn store_shard(&self, metadata: ShardMetadata) -> Result<(), ShardStoreError>;
    async fn list_shards(&self) -> Result<Vec<ShardMetadata>, ShardStoreError>;
    async fn delete_shard(&self, shard_id: &ShardId) -> Result<(), ShardStoreError>;
}

#[async_trait::async_trait]
impl ShardStoreTrait for ShardStore {
    async fn load_shard(&self, shard_id: &ShardId) -> Result<ShardMetadata, ShardStoreError> {
        let shards = self.shards.read().await;
        shards.get(shard_id)
            .cloned()
            .ok_or_else(|| ShardStoreError::NotFound(shard_id.clone()))
    }

    async fn store_shard(&self, metadata: ShardMetadata) -> Result<(), ShardStoreError> {
        let mut shards = self.shards.write().await;
        shards.insert(metadata.shard_id.clone(), metadata.clone());
        self.update_metrics().await;
        info!("💾 Stored shard {} ({} bytes, {} replicas)", 
              metadata.shard_id, metadata.size_bytes, metadata.replicas.len());
        Ok(())
    }

    async fn list_shards(&self) -> Result<Vec<ShardMetadata>, ShardStoreError> {
        let shards = self.shards.read().await;
        Ok(shards.values().cloned().collect())
    }

    async fn delete_shard(&self, shard_id: &ShardId) -> Result<(), ShardStoreError> {
        let mut shards = self.shards.write().await;
        if shards.remove(shard_id).is_some() {
            self.update_metrics().await;
            info!("🗑️ Deleted shard {}", shard_id);
            Ok(())
        } else {
            Err(ShardStoreError::NotFound(shard_id.clone()))
        }
    }
}

impl ShardStore {
    /// Forge production shard store
    pub fn new(storage_path: impl Into<PathBuf>) -> Arc<Self> {
        let store = Arc::new(Self {
            shards: Arc::new(RwLock::new(HashMap::new())),
            storage_path: Arc::new(RwLock::new(storage_path.into())),
            metrics: Arc::new(RwLock::new(ShardStoreMetrics::default())),
        });

        // Background metrics updater
        let store_clone = store.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = store_clone.update_metrics().await {
                    warn!("Metrics update failed: {}", e);
                }
            }
        });

        store
    }

    /// Create new shard with replication tracking
    pub async fn create_shard(
        &self,
        data: &[u8],
        soul_owner: BlissId,
        replicas: Vec<BlissId>,
    ) -> Result<ShardId, ShardStoreError> {
        let shard_id = self.generate_shard_id(data);
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        let fingerprint = ContentFingerprint::from(data);
        let metadata = ShardMetadata {
            shard_id: shard_id.clone(),
            soul_owner,
            size_bytes: data.len() as u64,
            fingerprint,
            replicas: replicas.into_iter().collect(),
            created: now_ns,
            last_access: now_ns,
            health_score: 100,
        };

        self.store_shard(metadata).await?;
        Ok(shard_id)
    }

    /// Update shard health and replication status
    pub async fn update_shard_health(
        &self,
        shard_id: &ShardId,
        replicas: Vec<BlissId>,
        health_score: u8,
    ) -> Result<(), ShardStoreError> {
        let mut shards = self.shards.write().await;
        if let Some(metadata) = shards.get_mut(shard_id) {
            metadata.replicas = replicas.into_iter().collect();
            metadata.health_score = health_score;
            metadata.last_access = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
            Ok(())
        } else {
            Err(ShardStoreError::NotFound(shard_id.clone()))
        }
    }

    /// Garbage collect unreferenced shards
    pub async fn garbage_collect(&self, soul_acl: &SoulACL) -> Result<usize, ShardStoreError> {
        let shards = self.shards.read().await;
        let mut deleted = 0;

        for (shard_id, metadata) in shards.iter() {
            // Check if soul still has access
            if !soul_acl.has_access(&metadata.soul_owner, shard_id).await {
                drop(shards); // Release read lock before write
                self.delete_shard(shard_id).await.ok();
                deleted += 1;
            }
        }

        self.update_metrics().await;
        info!("🧹 GC deleted {} unreferenced shards", deleted);
        Ok(deleted)
    }

    fn generate_shard_id(&self, data: &[u8]) -> ShardId {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    async fn update_metrics(&self) {
        let shards = self.shards.read().await;
        let mut metrics = self.metrics.write().await;

        metrics.total_shards = shards.len();
        let total_size: u64 = shards.values().map(|m| m.size_bytes).sum();
        metrics.total_size_gb = total_size as f64 / 1_073_741_824.0;
        metrics.healthy_shards = shards.values()
            .filter(|m| m.health_score >= 70)
            .count();

        let avg_replicas: f64 = shards.values()
            .map(|m| m.replicas.len() as f64)
            .sum::<f64>() / shards.len().max(1) as f64;
        metrics.replication_factor_avg = avg_replicas;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gov::SoulACL;

    #[tokio::test]
    async fn test_shard_create_load() {
        let store = ShardStore::new("/tmp/test");
        let soul = BlissId::new();
        let data = b"quantum shard test data";

        let shard_id = store.create_shard(data, soul, vec![]).await.unwrap();
        let metadata = store.load_shard(&shard_id).await.unwrap();

        assert_eq!(metadata.size_bytes, data.len() as u64);
        assert_eq!(metadata.soul_owner, soul);
    }

    #[tokio::test]
    async fn test_shard_health_update() {
        let store = ShardStore::new("/tmp/test");
        let soul = BlissId::new();
        let data = b"test";
        let shard_id = store.create_shard(data, soul.clone(), vec![soul.clone()]).await.unwrap();

        store.update_shard_health(&shard_id, vec![soul.clone()], 85).await.unwrap();
        let metadata = store.load_shard(&shard_id).await.unwrap();

        assert_eq!(metadata.health_score, 85);
        assert_eq!(metadata.replicas.len(), 1);
    }
}