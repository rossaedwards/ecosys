//! ═══════════════════════════════════════════════════════════════════
//! 📦 AuraFS LRU Cache - Quantum Production Cache Engine
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx LLC 💎
//!
//! High-performance, async-safe LRU with soul-scoped partitioning.
//!
//! ## Features
//! - Async-safe with tokio::sync::RwLock
//! - TTL-based expiration
//! - Metrics tracking (hits, misses, evictions)
//! - LRU promotion on access
//! - Retention predicate support
//! ═══════════════════════════════════════════════════════════════════

use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Production LRU cache metrics
#[derive(Debug, Clone, Default)]
pub struct LruMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
    pub capacity: usize,
    pub ttl_seconds: u64,
}

/// LRU cache entry with metadata
#[derive(Debug, Clone)]
pub struct LruEntry<K, V> {
    pub key: K,
    pub value: Arc<V>,
    pub access_time: Instant,
    pub created_time: Instant,
    pub ttl: Option<Duration>,
}

impl<K: Clone, V: Clone> LruEntry<K, V> {
    pub fn new(key: K, value: V, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            key,
            value: Arc::new(value),
            access_time: now,
            created_time: now,
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.ttl.map_or(false, |ttl| {
            Instant::now().saturating_duration_since(self.access_time) > ttl
        })
    }
}

/// Soul-scoped LRU cache partition
#[derive(Debug)]
pub struct SoulLru<K, V> {
    inner: RwLock<HashMap<K, LruEntry<K, V>>>,
    order: RwLock<Vec<K>>,
    capacity: usize,
    metrics: std::sync::Mutex<LruMetrics>,
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static, V: Clone + Send + Sync + 'static>
    SoulLru<K, V>
{
    /// Create new soul-scoped LRU partition
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
            order: RwLock::new(Vec::new()),
            capacity,
            metrics: std::sync::Mutex::new(LruMetrics::default()),
        }
    }

    /// Insert key-value pair, evict LRU if full
    pub async fn put(&mut self, key: K, value: V) {
        let now = Instant::now();
        let entry = LruEntry::new(key.clone(), value, None);

        let mut inner = self.inner.write().await;
        let mut order = self.order.write().await;

        // Evict LRU if at capacity
        if inner.len() >= self.capacity {
            if let Some(evicted_key) = order.first().cloned() {
                inner.remove(&evicted_key);
                let mut metrics = self.metrics.lock().unwrap();
                metrics.evictions += 1;
                debug!("LRU eviction: {:?}", evicted_key);
            }
        }

        // Insert new entry
        inner.insert(key.clone(), entry);
        
        // Maintain LRU order (remove if exists, push back)
        if let Some(pos) = order.iter().position(|k| k == &key) {
            order.remove(pos);
        }
        order.push(key);

        let mut metrics = self.metrics.lock().unwrap();
        metrics.size = inner.len();
    }

    /// Get value by key, promote to MRU
    pub async fn get(&self, key: &K) -> Option<Arc<V>> {
        let now = Instant::now();
        let mut inner = self.inner.write().await;
        let mut order = self.order.write().await;

        if let Some(entry) = inner.get_mut(key) {
            if entry.is_expired() {
                // Expired entry cleanup
                inner.remove(key);
                if let Some(pos) = order.iter().position(|k| k == key) {
                    order.remove(pos);
                }
                return None;
            }

            // Update access time and promote to MRU
            entry.access_time = now;
            if let Some(pos) = order.iter().position(|k| k == key) {
                order.remove(pos);
            }
            order.push(key.clone());

            let mut metrics = self.metrics.lock().unwrap();
            metrics.hits += 1;
            Some(entry.value.clone())
        } else {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.misses += 1;
            None
        }
    }

    /// Remove LRU entry (returns evicted key-value)
    pub async fn pop_lru(&mut self) -> Option<(K, Arc<V>)> {
        let mut order = self.order.write().await;
        let mut inner = self.inner.write().await;

        if let Some(evicted_key) = order.first().cloned() {
            order.remove(0);
            if let Some(entry) = inner.remove(&evicted_key) {
                let mut metrics = self.metrics.lock().unwrap();
                metrics.evictions += 1;
                metrics.size = inner.len();
                return Some((evicted_key, entry.value));
            }
        }
        None
    }

    /// Get current metrics
    pub fn metrics(&self) -> LruMetrics {
        self.metrics.lock().unwrap().clone()
    }

    /// Clear all entries
    pub async fn clear(&mut self) {
        let mut inner = self.inner.write().await;
        let mut order = self.order.write().await;
        inner.clear();
        order.clear();
        let mut metrics = self.metrics.lock().unwrap();
        metrics.size = 0;
    }

    /// Current size
    pub fn len(&self) -> usize {
        self.inner.blocking_read().len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retain only valid entries (cleanup expired)
    pub async fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut LruEntry<K, V>) -> bool,
    {
        let mut inner = self.inner.write().await;
        let mut order = self.order.write().await;
        let mut to_remove = Vec::new();

        for (key, entry) in inner.iter_mut() {
            if !f(key, entry) {
                to_remove.push(key.clone());
            }
        }

        for key in to_remove {
            inner.remove(&key);
            if let Some(pos) = order.iter().position(|k| k == &key) {
                order.remove(pos);
            }
        }
    }
}

/// Primary LRU cache facade for AuraFS
#[derive(Debug, Clone)]
pub struct LruCache<K, V> {
    inner: Arc<SoulLru<K, V>>,
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static, V: Clone + Send + Sync + 'static>
    LruCache<K, V>
{
    /// Create new LRU cache with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(SoulLru::new(capacity)),
        }
    }

    /// Insert key-value (async)
    pub async fn put(&self, key: K, value: V) {
        self.inner.put(key, value).await;
    }

    /// Get value by key (async)
    pub async fn get(&self, key: &K) -> Option<Arc<V>> {
        self.inner.get(key).await
    }

    /// Pop LRU entry
    pub async fn pop_lru(&self) -> Option<(K, Arc<V>)> {
        Arc::get_mut(&mut self.inner.clone()).unwrap().pop_lru().await
    }

    /// Get metrics
    pub fn metrics(&self) -> LruMetrics {
        self.inner.metrics()
    }

    /// Clear cache
    pub async fn clear(&self) {
        self.inner.clear().await;
    }

    /// Retain predicate
    pub async fn retain<F>(&self, f: F)
    where
        F: FnMut(&K, &mut LruEntry<K, V>) -> bool + Send + 'static,
    {
        self.inner.retain(f).await;
    }
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static, V: Clone + Send + Sync + 'static> LruCache<K, V> {
    /// Blocking get for sync contexts
    pub fn get_sync(&self, key: &K) -> Option<Arc<V>> {
        tokio::runtime::Handle::current().block_on(self.get(key))
    }

    /// Blocking put for sync contexts
    pub fn put_sync(&self, key: K, value: V) {
        tokio::runtime::Handle::current().block_on(self.put(key, value));
    }
}

/// Convenience macros for cache operations
#[macro_export]
macro_rules! lru_cache {
    ($capacity:expr) => {
        $crate::cache::LruCache::new($capacity)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_lru_basic() {
        let mut cache = SoulLru::new(3);
        
        // Insert 4 items (should evict 1)
        cache.put("a".to_string(), "apple".to_string()).await;
        cache.put("b".to_string(), "banana".to_string()).await;
        cache.put("c".to_string(), "cherry".to_string()).await;
        cache.put("d".to_string(), "date".to_string()).await;

        assert_eq!(cache.len(), 3);
        assert!(cache.get(&"a".to_string()).await.is_none()); // LRU evicted
        assert!(cache.get(&"d".to_string()).await.is_some()); // MRU present
    }

    #[tokio::test]
    async fn test_promotion() {
        let mut cache = SoulLru::new(2);
        
        cache.put("a".to_string(), "1".to_string()).await;
        cache.put("b".to_string(), "2".to_string()).await;
        
        // Access 'a' - should promote to MRU
        cache.get(&"a".to_string()).await.unwrap();
        
        cache.put("c".to_string(), "3".to_string()).await;
        assert!(cache.get(&"b".to_string()).await.is_none()); // b evicted, not a
    }

    #[tokio::test]
    async fn test_metrics() {
        let cache = LruCache::new(100);
        cache.put_sync("test".to_string(), "value".to_string());
        let _ = cache.get_sync(&"test".to_string());
        let _ = cache.get_sync(&"missing".to_string());
        
        let metrics = cache.metrics();
        assert_eq!(metrics.hits, 1);
        assert_eq!(metrics.misses, 1);
    }
}