//! Tiered Storage with Automatic Promotion/Demotion
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Implements hot/warm/cold storage tiers with automatic data movement.

use std::sync::Arc;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;
use crate::error::{RafsError, Result};
// Note: ShardId type may vary - adjust import based on your codebase
// This is a placeholder - update to match your actual ShardId type
pub type ShardId = String; // Update this to match your actual type
use tracing::{debug, info, warn};

/// Storage tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageTier {
    /// Hot tier: NVMe SSD (frequently accessed)
    Hot,
    /// Warm tier: SATA SSD (moderately accessed)
    Warm,
    /// Cold tier: HDD or Object Storage (rarely accessed)
    Cold,
}

impl std::fmt::Display for StorageTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageTier::Hot => write!(f, "HOT"),
            StorageTier::Warm => write!(f, "WARM"),
            StorageTier::Cold => write!(f, "COLD"),
        }
    }
}

/// Storage backend trait
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    async fn read(&self, shard_id: &ShardId) -> Result<Vec<u8>>;
    async fn write(&self, shard_id: &ShardId, data: &[u8]) -> Result<()>;
    async fn exists(&self, shard_id: &ShardId) -> Result<bool>;
    async fn delete(&self, shard_id: &ShardId) -> Result<()>;
}

/// Access statistics for shards
#[derive(Debug, Clone)]
struct AccessStats {
    access_count: usize,
    last_access: Instant,
    total_bytes_read: u64,
}

impl AccessStats {
    fn new() -> Self {
        Self {
            access_count: 0,
            last_access: Instant::now(),
            total_bytes_read: 0,
        }
    }

    fn record_access(&mut self, bytes: u64) {
        self.access_count += 1;
        self.last_access = Instant::now();
        self.total_bytes_read += bytes;
    }
}

/// Tiered storage implementation
pub struct TieredStorage {
    hot_storage: Arc<dyn StorageBackend>,
    warm_storage: Arc<dyn StorageBackend>,
    cold_storage: Arc<dyn StorageBackend>,
    access_tracker: Arc<RwLock<HashMap<ShardId, AccessStats>>>,
    tier_mapping: Arc<RwLock<HashMap<ShardId, StorageTier>>>,
    promotion_threshold: usize,
    demotion_threshold: usize,
}

impl TieredStorage {
    /// Create new tiered storage
    pub fn new(
        hot_storage: Arc<dyn StorageBackend>,
        warm_storage: Arc<dyn StorageBackend>,
        cold_storage: Arc<dyn StorageBackend>,
        promotion_threshold: usize,
        demotion_threshold: usize,
    ) -> Self {
        Self {
            hot_storage,
            warm_storage,
            cold_storage,
            access_tracker: Arc::new(RwLock::new(HashMap::new())),
            tier_mapping: Arc::new(RwLock::new(HashMap::new())),
            promotion_threshold,
            demotion_threshold,
        }
    }

    /// Read shard from appropriate tier
    pub async fn read(&self, shard_id: &ShardId) -> Result<Vec<u8>> {
        // Determine current tier
        let tier = self.get_shard_tier(shard_id).await;
        
        // Read from appropriate tier
        let data = match tier {
            StorageTier::Hot => {
                self.hot_storage.read(shard_id).await?
            }
            StorageTier::Warm => {
                let data = self.warm_storage.read(shard_id).await?;
                // Promote to hot if frequently accessed
                self.maybe_promote(shard_id, StorageTier::Hot).await;
                data
            }
            StorageTier::Cold => {
                let data = self.cold_storage.read(shard_id).await?;
                // Promote to warm
                self.maybe_promote(shard_id, StorageTier::Warm).await;
                data
            }
        };
        
        // Update access stats
        self.update_access_stats(shard_id, data.len() as u64).await;
        
        Ok(data)
    }

    /// Write shard to hot tier initially
    pub async fn write(&self, shard_id: &ShardId, data: &[u8]) -> Result<()> {
        // Write to hot tier
        self.hot_storage.write(shard_id, data).await?;
        
        // Update tier mapping
        {
            let mut mapping = self.tier_mapping.write().await;
            mapping.insert(shard_id.clone(), StorageTier::Hot);
        }
        
        // Initialize access stats
        {
            let mut tracker = self.access_tracker.write().await;
            tracker.insert(shard_id.clone(), AccessStats::new());
        }
        
        Ok(())
    }

    /// Get current tier for shard
    async fn get_shard_tier(&self, shard_id: &ShardId) -> StorageTier {
        let mapping = self.tier_mapping.read().await;
        mapping.get(shard_id).copied().unwrap_or(StorageTier::Cold)
    }

    /// Check if shard should be promoted
    async fn maybe_promote(&self, shard_id: &ShardId, target_tier: StorageTier) {
        let stats = self.access_tracker.read().await;
        if let Some(access_stats) = stats.get(shard_id) {
            if access_stats.access_count >= self.promotion_threshold {
                drop(stats);
                if let Err(e) = self.promote_shard(shard_id, target_tier).await {
                    warn!("Failed to promote shard {}: {}", shard_id, e);
                }
            }
        }
    }

    /// Promote shard to higher tier
    async fn promote_shard(&self, shard_id: &ShardId, target_tier: StorageTier) -> Result<()> {
        let current_tier = self.get_shard_tier(shard_id).await;
        
        // Only promote if target is higher than current
        if target_tier == StorageTier::Hot && current_tier != StorageTier::Hot {
            // Read from current tier
            let data = match current_tier {
                StorageTier::Warm => self.warm_storage.read(shard_id).await?,
                StorageTier::Cold => self.cold_storage.read(shard_id).await?,
                _ => return Ok(()),
            };
            
            // Write to hot tier
            self.hot_storage.write(shard_id, &data).await?;
            
            // Update tier mapping
            {
                let mut mapping = self.tier_mapping.write().await;
                mapping.insert(shard_id.clone(), StorageTier::Hot);
            }
            
            info!("Promoted shard {} to HOT tier", shard_id);
        } else if target_tier == StorageTier::Warm && current_tier == StorageTier::Cold {
            // Read from cold
            let data = self.cold_storage.read(shard_id).await?;
            
            // Write to warm
            self.warm_storage.write(shard_id, &data).await?;
            
            // Update tier mapping
            {
                let mut mapping = self.tier_mapping.write().await;
                mapping.insert(shard_id.clone(), StorageTier::Warm);
            }
            
            info!("Promoted shard {} to WARM tier", shard_id);
        }
        
        Ok(())
    }

    /// Update access statistics
    async fn update_access_stats(&self, shard_id: &ShardId, bytes: u64) {
        let mut tracker = self.access_tracker.write().await;
        let stats = tracker.entry(shard_id.clone()).or_insert_with(AccessStats::new);
        stats.record_access(bytes);
    }

    /// Periodic demotion of rarely accessed shards
    pub async fn demote_inactive_shards(&self, inactive_threshold: std::time::Duration) {
        let now = Instant::now();
        let mut to_demote = Vec::new();
        
        {
            let tracker = self.access_tracker.read().await;
            let mapping = self.tier_mapping.read().await;
            
            for (shard_id, stats) in tracker.iter() {
                if now.duration_since(stats.last_access) > inactive_threshold {
                    if let Some(&tier) = mapping.get(shard_id) {
                        if tier != StorageTier::Cold {
                            to_demote.push((shard_id.clone(), tier));
                        }
                    }
                }
            }
        }
        
        for (shard_id, current_tier) in to_demote {
            if let Err(e) = self.demote_shard(&shard_id, current_tier).await {
                warn!("Failed to demote shard {}: {}", shard_id, e);
            }
        }
    }

    /// Demote shard to lower tier
    async fn demote_shard(&self, shard_id: &ShardId, current_tier: StorageTier) -> Result<()> {
        match current_tier {
            StorageTier::Hot => {
                // Move from hot to warm
                let data = self.hot_storage.read(shard_id).await?;
                self.warm_storage.write(shard_id, &data).await?;
                
                {
                    let mut mapping = self.tier_mapping.write().await;
                    mapping.insert(shard_id.clone(), StorageTier::Warm);
                }
                
                debug!("Demoted shard {} from HOT to WARM", shard_id);
            }
            StorageTier::Warm => {
                // Move from warm to cold
                let data = self.warm_storage.read(shard_id).await?;
                self.cold_storage.write(shard_id, &data).await?;
                
                {
                    let mut mapping = self.tier_mapping.write().await;
                    mapping.insert(shard_id.clone(), StorageTier::Cold);
                }
                
                debug!("Demoted shard {} from WARM to COLD", shard_id);
            }
            StorageTier::Cold => {
                // Already at lowest tier
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockStorage {
        data: Arc<RwLock<HashMap<ShardId, Vec<u8>>>>,
        read_count: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl StorageBackend for MockStorage {
        async fn read(&self, shard_id: &ShardId) -> Result<Vec<u8>> {
            self.read_count.fetch_add(1, Ordering::SeqCst);
            let data = self.data.read().await;
            data.get(shard_id)
                .cloned()
                .ok_or_else(|| RafsError::ShardNotFound(shard_id.to_string()))
        }

        async fn write(&self, shard_id: &ShardId, data: &[u8]) -> Result<()> {
            let mut storage = self.data.write().await;
            storage.insert(shard_id.clone(), data.to_vec());
            Ok(())
        }

        async fn exists(&self, shard_id: &ShardId) -> Result<bool> {
            let data = self.data.read().await;
            Ok(data.contains_key(shard_id))
        }

        async fn delete(&self, shard_id: &ShardId) -> Result<()> {
            let mut data = self.data.write().await;
            data.remove(shard_id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_tiered_storage_promotion() {
        let hot = Arc::new(MockStorage {
            data: Arc::new(RwLock::new(HashMap::new())),
            read_count: Arc::new(AtomicUsize::new(0)),
        });
        let warm = Arc::new(MockStorage {
            data: Arc::new(RwLock::new(HashMap::new())),
            read_count: Arc::new(AtomicUsize::new(0)),
        });
        let cold = Arc::new(MockStorage {
            data: Arc::new(RwLock::new(HashMap::new())),
            read_count: Arc::new(AtomicUsize::new(0)),
        });

        let storage = TieredStorage::new(hot.clone(), warm.clone(), cold.clone(), 3, 10);

        let shard_id = ShardId::new();
        let data = b"test data".to_vec();

        // Write to hot initially
        storage.write(&shard_id, &data).await.unwrap();

        // Read multiple times to trigger promotion
        for _ in 0..3 {
            let _ = storage.read(&shard_id).await;
        }

        // Should be promoted to hot
        assert!(hot.data.read().await.contains_key(&shard_id));
    }
}

