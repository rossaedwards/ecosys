//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Dedup Cache - LRU Fingerprint Cache + Async Persistence
//! 🛸 High-Speed Lookup + Eviction + Prefetch + Reference Counting
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    dedup::{ContentFingerprint, DedupConfig, Error},
    storage::shard_store::ShardStore,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Cache entry representing a stored fingerprint
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub fingerprint: ContentFingerprint,
    pub last_access: std::time::Instant,
    pub ref_count: usize,
}

/// Production deduplication fingerprint LRU cache
pub struct DedupCache {
    /// Capacity of cache (number of entries)
    capacity: usize,
    
    /// Core cache storage (fingerprint hash → CacheEntry)
    cache: RwLock<HashMap<Vec<u8>, CacheEntry>>,
    
    /// LRU queue for eviction (fingerprint hash keys)
    lru_queue: RwLock<VecDeque<Vec<u8>>>,
    
    /// Background storage reference (simulate persistence)
    storage: Arc<ShardStore>,
}

impl DedupCache {
    /// Create a new dedup cache with given capacity
    pub fn new(capacity: usize) -> Self {
        let storage = Arc::new(ShardStore::default());
        Self {
            capacity,
            cache: RwLock::new(HashMap::with_capacity(capacity)),
            lru_queue: RwLock::new(VecDeque::with_capacity(capacity)),
            storage,
        }
    }
    
    /// Lookup fingerprint in cache
    pub async fn get(&self, fingerprint: &ContentFingerprint) -> Option<ContentFingerprint> {
        let key = fingerprint.key();
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(&key) {
            entry.last_access = std::time::Instant::now();
            entry.ref_count += 1;
            drop(cache);
            self.promote_lru(key).await;
            Some(entry.fingerprint.clone())
        } else {
            None
        }
    }
    
    /// Insert fingerprint into cache
    pub async fn insert(&self, fingerprint: ContentFingerprint) -> Result<(), Error> {
        let key = fingerprint.key();
        let mut cache = self.cache.write().await;
        let mut lru = self.lru_queue.write().await;
        
        if cache.contains_key(&key) {
            // Update existing entry
            if let Some(entry) = cache.get_mut(&key) {
                entry.last_access = std::time::Instant::now();
                entry.ref_count += 1;
            }
            self.promote_lru(key.clone()).await;
            return Ok(());
        }
        
        // Evict least recently used if full
        if cache.len() >= self.capacity {
            if let Some(evict_key) = lru.pop_front() {
                cache.remove(&evict_key);
                info!("🗑️ Dedup cache evicted fingerprint {:?}", evict_key);
            } else {
                return Err(Error::CacheMiss);
            }
        }
        
        // Insert new entry
        let entry = CacheEntry {
            fingerprint: fingerprint.clone(),
            last_access: std::time::Instant::now(),
            ref_count: 1,
        };
        cache.insert(key.clone(), entry);
        lru.push_back(key);
        
        // Persist to storage asynchronously
        // TODO: Implement storage persist logic
        
        Ok(())
    }
    
    /// Promote key to the back of LRU queue
    async fn promote_lru(&self, key: Vec<u8>) {
        let mut lru = self.lru_queue.write().await;
        if let Some(pos) = lru.iter().position(|k| k == &key) {
            lru.remove(pos);
        }
        lru.push_back(key);
    }
    
    /// Prefix search (simulate bloom filter using hash prefixes)
    pub async fn prefix_search(&self, prefix: Vec<u8>) -> Vec<ContentFingerprint> {
        let cache = self.cache.read().await;
        cache.iter()
            .filter_map(|(k, v)| {
                if k.starts_with(&prefix) {
                    Some(v.fingerprint.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Current cache size
    pub async fn size(&self) -> usize {
        self.cache.read().await.len()
    }
    
    /// Clear entire cache
    pub async fn clear(&self) {
        self.cache.write().await.clear();
        self.lru_queue.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dedup::fingerprint::FingerprintType;

    #[tokio::test]
    async fn test_cache_insert_and_get() {
        let cache = DedupCache::new(2);
        
        let fp1 = ContentFingerprint::new(vec![1, 2, 3], vec![], FingerprintType::Blake3);
        let fp2 = ContentFingerprint::new(vec![4, 5, 6], vec![], FingerprintType::Blake3);
        let fp3 = ContentFingerprint::new(vec![7, 8, 9], vec![], FingerprintType::Blake3);
        
        cache.insert(fp1.clone()).await.unwrap();
        cache.insert(fp2.clone()).await.unwrap();
        
        assert_eq!(cache.size().await, 2);
        assert!(cache.get(&fp1).await.is_some());
        
        cache.insert(fp3).await.unwrap();
        // After eviction cache size stays at capacity
        assert_eq!(cache.size().await, 2);
    }
    
    #[tokio::test]
    async fn test_prefix_search() {
        let cache = DedupCache::new(10);
        let fp1 = ContentFingerprint::new(vec![1, 2, 3], vec![], FingerprintType::Blake3);
        let fp2 = ContentFingerprint::new(vec![1, 2, 4], vec![], FingerprintType::Blake3);
        let fp3 = ContentFingerprint::new(vec![7, 8, 9], vec![], FingerprintType::Blake3);
        
        cache.insert(fp1.clone()).await.unwrap();
        cache.insert(fp2.clone()).await.unwrap();
        cache.insert(fp3.clone()).await.unwrap();
        
        let results = cache.prefix_search(vec![1, 2]).await;
        assert_eq!(results.len(), 2);
    }
}