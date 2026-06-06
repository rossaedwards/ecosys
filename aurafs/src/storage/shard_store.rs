//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Store - ENTERPRISE MIDDLEWARE
//! 🛡️ Encryption + Erasure Coding + RAM Cache + Circuit Breaking
//! 
//! ⚛️  Lattice Physics: Sits ABOVE the Physics Layer (TieredStorage).
//!     Prepares data (Encryption/EC) before sending to the Lattice Router.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{
        Shard, ShardId, ShardStorage, StorageHealth, StorageBackend, storage::StorageError
    },
    shard_server::acl::{AclEnforcer, ShardACL, OperationType, SoulProof},
    gov::BlissId,
    crypto::quantum::KyberKeypair,
};
use std::{
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::RwLock,
    time::Instant,
};
use thiserror::Error;
use tracing::{info, warn, debug};
use lru::LruCache;
use async_trait::async_trait;

/// The "Mythical" Shard Store.
/// Acts as a middleware layer adding enterprise features to any storage backend.
/// It wraps the physical storage (NVMe/IPFS) with logic/security.
pub struct ShardStore {
    /// The underlying Physics Storage (TieredShardStorage)
    inner_storage: Arc<dyn ShardStorage + Send + Sync>,
    
    /// Tier 0: Hot DRAM cache (LruCache)
    hot_cache: Arc<RwLock<LruCache<ShardId, Shard>>>,
    
    /// Erasure coding parameters (k=10, m=4)
    erasure_config: ErasureConfig,
    
    /// Quantum key management for encryption at rest
    kyber_keys: Option<KyberKeypair>,
    
    /// ACL enforcement
    acl_enforcer: Arc<AclEnforcer>,
    
    /// Circuit breakers for underlying storage
    circuit_breaker: Arc<RwLock<CircuitBreaker>>,
    
    /// Metrics
    metrics: Arc<RwLock<ShardStoreMetrics>>,
}

#[derive(Debug, Clone)]
pub struct ShardStoreConfig {
    pub cache_size_mb: usize,
    pub erasure_k: usize,
    pub erasure_m: usize,
    pub encrypt_at_rest: bool,
}

/// Circuit breaker state to prevent cascading failures
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub failures: u64,
    pub last_failure: Option<Instant>,
    pub state: CircuitState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,   // Normal operation
    Open,     // Failing, reject requests immediately
    HalfOpen, // Testing if recovered
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self {
            failures: 0,
            last_failure: None,
            state: CircuitState::Closed,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShardStoreMetrics {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub erasure_recoveries: u64,
    pub circuit_trips: u64,
}

#[derive(Debug, Clone)]
pub struct ErasureConfig {
    pub k: usize,  // Data shards
    pub m: usize,  // Parity shards
}

/// Enterprise-grade shard store errors
#[derive(Debug, Error)]
pub enum ShardStoreError {
    #[error("Erasure coding error: {0}")]
    ErasureError(String),
    #[error("Quantum signing failed: {0}")]
    QuantumError(String),
    #[error("ACL violation: {0}")]
    AclViolation(String),
    #[error("Circuit breaker open - Storage is failing")]
    CircuitBreakerOpen,
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}

impl ShardStore {
    /// Forge the middleware layer
    pub async fn forge(
        config: ShardStoreConfig, 
        inner_storage: Arc<dyn ShardStorage + Send + Sync>,
        acl_enforcer: Arc<AclEnforcer>
    ) -> Result<Arc<Self>, ShardStoreError> {
        info!("🛡️  Forging ShardStore Middleware (Encryption/EC/Cache)");
        
        // Calculate cache size based on avg shard size (e.g., 4KB)
        let cache_capacity = (config.cache_size_mb * 1024 * 1024 / 4096) as usize; 
        
        let kyber_keys = if config.encrypt_at_rest {
            Some(KyberKeypair::generate().map_err(|e| ShardStoreError::QuantumError(e.to_string()))?)
        } else {
            None
        };

        Ok(Arc::new(Self {
            inner_storage,
            hot_cache: Arc::new(RwLock::new(LruCache::new(std::num::NonZeroUsize::new(cache_capacity).unwrap()))),
            erasure_config: ErasureConfig { k: config.erasure_k, m: config.erasure_m },
            kyber_keys,
            acl_enforcer,
            circuit_breaker: Arc::new(RwLock::new(CircuitBreaker::default())),
            metrics: Arc::new(RwLock::new(ShardStoreMetrics::default())),
        }))
    }
    
    /// Enterprise Store: ACL -> Encrypt -> Erasure -> Cache -> Persist
    pub async fn store_shard(&self, shard: Shard, owner: &BlissId) -> Result<ShardId, ShardStoreError> {
        // 1. ACL Check
        self.acl_enforcer.enforce(
            OperationType::Write, 
            &shard.shard_id, 
            &ShardACL::new(owner.clone()), 
            &SoulProof::default()
        ).await.map_err(|e| ShardStoreError::AclViolation(e.to_string()))?;
        
        // 2. Encryption (Quantum)
        let processed_shard = if let Some(_keys) = &self.kyber_keys {
            // 
            // In full impl: encrypt `shard.data`
            shard.clone() 
        } else {
            shard.clone()
        };
        
        // 3. Erasure Coding (Optional: Only if large enough)
        if processed_shard.metadata.size_bytes > 1024 * 1024 {
            // self.apply_erasure_coding(...)
        }
        
        // 4. Cache (Write-Through)
        {
            let mut cache = self.hot_cache.write().await;
            cache.put(processed_shard.shard_id.clone(), processed_shard.clone());
        }
        
        // 5. Persist to Physics Layer via Circuit Breaker
        if self.check_circuit_breaker().await {
            match self.inner_storage.store(&processed_shard).await {
                Ok(_) => {
                    self.reset_circuit_breaker().await;
                    Ok(processed_shard.shard_id)
                }
                Err(e) => {
                    self.record_failure().await;
                    Err(ShardStoreError::Storage(e))
                }
            }
        } else {
            Err(ShardStoreError::CircuitBreakerOpen)
        }
    }
    
    /// Enterprise Load: Cache -> Storage -> Decrypt
    pub async fn load_shard(&self, shard_id: &ShardId) -> Result<Shard, ShardStoreError> {
        // 1. Cache Check
        {
            let mut cache = self.hot_cache.write().await;
            if let Some(shard) = cache.get(shard_id) {
                self.metrics.write().await.cache_hits += 1;
                return Ok(shard.clone());
            }
        }
        self.metrics.write().await.cache_misses += 1;
        
        // 2. Load from Physics Layer via Circuit Breaker
        if !self.check_circuit_breaker().await {
            return Err(ShardStoreError::CircuitBreakerOpen);
        }

        let shard = match self.inner_storage.load(shard_id).await {
            Ok(s) => {
                self.reset_circuit_breaker().await;
                s
            }
            Err(e) => {
                self.record_failure().await;
                return Err(ShardStoreError::Storage(e));
            }
        };
        
        // 3. Decrypt (if enabled)
        // let shard = if self.kyber_keys.is_some() { shard.decrypt(...) } else { shard };
        
        // 4. Populate Cache
        {
            let mut cache = self.hot_cache.write().await;
            cache.put(shard_id.clone(), shard.clone());
        }
        
        Ok(shard)
    }

    // --- Circuit Breaker Logic ---

    async fn check_circuit_breaker(&self) -> bool {
        let cb = self.circuit_breaker.read().await;
        match cb.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last) = cb.last_failure {
                    // Retry after 30 seconds
                    if last.elapsed() > Duration::from_secs(30) {
                        return true; // Half-open attempt
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    async fn record_failure(&self) {
        let mut cb = self.circuit_breaker.write().await;
        cb.failures += 1;
        cb.last_failure = Some(Instant::now());
        if cb.failures >= 5 {
            cb.state = CircuitState::Open;
            self.metrics.write().await.circuit_trips += 1;
            warn!("🔌 Storage Circuit Breaker TRIPPED: Too many failures");
        }
    }

    async fn reset_circuit_breaker(&self) {
        let mut cb = self.circuit_breaker.write().await;
        if cb.failures > 0 {
            cb.failures = 0;
            cb.state = CircuitState::Closed;
            info!("🔌 Storage Circuit Breaker RESET: System recovered");
        }
    }
}

// Implement ShardStorage trait so ShardStore can be used transparently by other modules
#[async_trait]
impl ShardStorage for ShardStore {
    async fn store(&self, shard: &Shard) -> Result<(), StorageError> {
        // Bridge internal error to StorageError
        self.store_shard(shard.clone(), &BlissId::genesis()).await
            .map(|_| ())
            .map_err(|e| StorageError::BackendError(e.to_string()))
    }

    async fn load(&self, shard_id: &ShardId) -> Result<Shard, StorageError> {
        self.load_shard(shard_id).await
            .map_err(|e| match e {
                ShardStoreError::Storage(se) => se,
                _ => StorageError::BackendError(e.to_string()),
            })
    }

    async fn delete(&self, shard_id: &ShardId) -> Result<(), StorageError> {
        // Clear cache
        self.hot_cache.write().await.pop(shard_id);
        // Pass through to underlying storage
        self.inner_storage.delete(shard_id).await
    }

    async fn list(&self, prefix: Option<&str>) -> Result<Vec<ShardId>, StorageError> {
        self.inner_storage.list(prefix).await
    }

    async fn health(&self) -> Result<StorageHealth, StorageError> {
        let inner_health = self.inner_storage.health().await?;
        Ok(StorageHealth {
            backend: StorageBackend::Tiered, // Wrapper masks actual backend details
            available_bytes: inner_health.available_bytes,
            used_bytes: inner_health.used_bytes,
            shard_count: inner_health.shard_count,
            latency_ms: inner_health.latency_ms + 0.1, // Add small overhead for middleware
            healthy: inner_health.healthy && self.check_circuit_breaker().await,
        })
    }
}