//! TTS Audio Caching
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::types::SynthesisResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cache key for TTS results
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    /// Text hash
    pub text_hash: String,
    /// Voice ID
    pub voice_id: String,
    /// Rate
    pub rate: u32, // Stored as fixed-point
    /// Pitch
    pub pitch: i32, // Stored as fixed-point
}

/// Cached audio entry
#[derive(Debug, Clone)]
pub struct CachedAudio {
    /// Audio data
    pub response: SynthesisResponse,
    /// Cache timestamp
    pub cached_at: u64,
    /// Access count
    pub access_count: u64,
}

/// TTS cache with LRU eviction
pub struct TtsCache {
    /// Cache storage
    cache: Arc<RwLock<HashMap<CacheKey, CachedAudio>>>,
    /// Maximum cache size in bytes
    max_size: usize,
    /// Current size
    current_size: Arc<RwLock<usize>>,
}

impl TtsCache {
    /// Create new cache with max size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            current_size: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Get cached audio
    pub async fn get(&self, key: &CacheKey) -> Option<SynthesisResponse> {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get_mut(key) {
            entry.access_count += 1;
            return Some(entry.response.clone());
        }
        None
    }
    
    /// Store audio in cache
    pub async fn put(&self, key: CacheKey, response: SynthesisResponse) {
        let size = response.audio.len();
        
        // Check if we need to evict
        let current = *self.current_size.read().await;
        if current + size > self.max_size {
            self.evict_lru().await;
        }
        
        let entry = CachedAudio {
            response,
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            access_count: 1,
        };
        
        let mut cache = self.cache.write().await;
        cache.insert(key, entry);
        
        let mut current_size = self.current_size.write().await;
        *current_size += size;
    }
    
    /// Evict least recently used entries
    async fn evict_lru(&self) {
        let mut cache = self.cache.write().await;
        
        // Find entry with lowest access count
        if let Some(key) = cache.iter()
            .min_by_key(|(_, v)| v.access_count)
            .map(|(k, _)| k.clone())
        {
            if let Some(entry) = cache.remove(&key) {
                let mut current_size = self.current_size.write().await;
                *current_size = current_size.saturating_sub(entry.response.audio.len());
            }
        }
    }
    
    /// Clear the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        
        let mut current_size = self.current_size.write().await;
        *current_size = 0;
    }
}
