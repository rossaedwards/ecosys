//! Lock-Free Sharded Cache
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! High-performance sharded cache using DashMap for lock-free operations.

use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;
use tracing::debug;

/// Sharded cache implementation
pub struct ShardedCache<K, V> {
    shards: Vec<Arc<DashMap<K, V>>>,
    shard_count: usize,
}

impl<K, V> ShardedCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create new sharded cache
    pub fn new(shard_count: usize) -> Self {
        let shards = (0..shard_count)
            .map(|_| Arc::new(DashMap::new()))
            .collect();
        
        Self { shards, shard_count }
    }

    /// Get shard for a key
    fn get_shard(&self, key: &K) -> &Arc<DashMap<K, V>> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let shard_index = (hasher.finish() as usize) % self.shard_count;
        &self.shards[shard_index]
    }

    /// Get value from cache
    pub fn get(&self, key: &K) -> Option<V> {
        self.get_shard(key).get(key).map(|v| v.clone())
    }

    /// Insert value into cache
    pub fn insert(&self, key: K, value: V) {
        self.get_shard(&key).insert(key, value);
    }

    /// Remove value from cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.get_shard(key).remove(key).map(|(_, v)| v)
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.get_shard(key).contains_key(key)
    }

    /// Get number of entries across all shards
    pub fn len(&self) -> usize {
        self.shards.iter().map(|shard| shard.len()).sum()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.shards.iter().all(|shard| shard.is_empty())
    }

    /// Clear all shards
    pub fn clear(&self) {
        for shard in &self.shards {
            shard.clear();
        }
    }

    /// Get statistics about shard distribution
    pub fn shard_stats(&self) -> Vec<usize> {
        self.shards.iter().map(|shard| shard.len()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sharded_cache() {
        let cache = ShardedCache::<String, usize>::new(4);

        cache.insert("key1".to_string(), 1);
        cache.insert("key2".to_string(), 2);
        cache.insert("key3".to_string(), 3);

        assert_eq!(cache.get(&"key1".to_string()), Some(1));
        assert_eq!(cache.get(&"key2".to_string()), Some(2));
        assert_eq!(cache.get(&"key3".to_string()), Some(3));
        assert_eq!(cache.get(&"key4".to_string()), None);

        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_sharded_cache_remove() {
        let cache = ShardedCache::<String, usize>::new(4);

        cache.insert("key1".to_string(), 1);
        assert_eq!(cache.remove(&"key1".to_string()), Some(1));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_shard_distribution() {
        let cache = ShardedCache::<String, usize>::new(4);

        // Insert many keys
        for i in 0..100 {
            cache.insert(format!("key{}", i), i);
        }

        let stats = cache.shard_stats();
        assert_eq!(stats.len(), 4);
        
        // Keys should be distributed across shards
        let total: usize = stats.iter().sum();
        assert_eq!(total, 100);
    }
}

