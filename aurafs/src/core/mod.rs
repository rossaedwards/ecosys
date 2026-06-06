//! ═══════════════════════════════════════════════════════════════════
//! 💎 AuraFS Core - Unified Module Root
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Centralized types, error handling, and core abstractions.
//! ═══════════════════════════════════════════════════════════════════
//!
//! # Enterprise-Grade Features (v2.0)
//!
//! This module provides the foundation for AuraFS with:
//!
//! - **Error Handling**: Comprehensive error types with context
//! - **Configuration**: Hot-reloadable configuration management
//! - **Health Checks**: Kubernetes-compatible health probes
//! - **Resilience**: Circuit breakers, rate limiters, retry logic
//! - **Observability**: Distributed tracing, metrics
//! - **Cryptography**: Quantum-safe operations
//! - **Identity**: BlissID management with persistence
//! - **Network**: Orchestration and peer management
//! - **Data Structures**: Merkle trees, shards

#![warn(missing_docs)]

// ═══════════════════════════════════════════════════════════════════
// CORE MODULES - Foundation
// ═══════════════════════════════════════════════════════════════════

/// Error handling system with comprehensive types
pub mod error;
/// Shard primitives and storage operations
pub mod shard;
/// Zero-knowledge soul proofs
pub mod soulproof;
/// Prometheus metrics and observability
pub mod metrics;
/// BlissID identity management
pub mod bliss;
/// Quantum-safe cryptography
pub mod crypto;

// ═══════════════════════════════════════════════════════════════════
// ENTERPRISE MODULES - Added in v2.0
// ═══════════════════════════════════════════════════════════════════

/// Configuration management with hot-reload
pub mod config;
/// Health check system with K8s probes
pub mod health;
/// Circuit breaker resilience pattern
pub mod circuit_breaker;
/// Rate limiting (sliding window, token bucket)
pub mod rate_limiter;
/// Production Merkle tree implementation
pub mod merkle;
/// Distributed tracing (OpenTelemetry-compatible)
pub mod tracing;
/// Network orchestration and peer management
pub mod network;
/// Database persistence for core data
pub mod persistence;

// ═══════════════════════════════════════════════════════════════════
// RE-EXPORTS - Convenient access to key types
// ═══════════════════════════════════════════════════════════════════

// Error types
pub use error::{AuraFSError, CoreError, ErrorClass, ErrorPhase, ErrorCode, internal, client, transient_network};

// Core data types
pub use shard::{ShardId, ShardHandle, ShardMetadata, ShardStoreOps, FractalShard};
pub use soulproof::{SoulProof, ProofStatus, SoulProofType};
pub use bliss::{BlissId, BlissIdManager, BlissIdRecord, InMemoryBlissIdManager, PersistentBlissIdManager};
pub use metrics::AuraFSMetrics;

// Crypto
pub use crypto::{DilithiumKeypair, sha3_256_digest, shake256_xof, gen_random_bytes};

// Configuration
pub use config::{CoreConfig, ConfigManager, CryptoConfig, ShardConfig, IdentityConfig};

// Health
pub use health::{HealthStatus, HealthReport, ComponentHealth, HealthManager, HealthChecker};

// Resilience
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState, CircuitBreakerRegistry};
pub use rate_limiter::{RateLimitConfig, RateLimitResult, SlidingWindowRateLimiter, TokenBucketRateLimiter, SoulRateLimiter};

// Merkle
pub use merkle::{MerkleTree, MerkleHash, MerkleProof, ProofNode};

// Tracing
pub use self::tracing::{TraceId, SpanId, Span, SpanKind, SpanStatus, TraceContext, Tracer};

// Network
pub use network::{NodeId, NodeRole, NodeHealth, PeerNode, NetworkOrchestrator, DefaultNetworkOrchestrator, OrchestratorConfig, ReplicationStrategy, NetworkEvent};

// Persistence
pub use persistence::{DatabaseConfig, SqliteBlissIdManager};

/// Core result type used throughout AuraFS.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Soul-aware result for operations requiring identity verification.
pub type SoulResult<T> = std::result::Result<T, CoreError>;

/// Marker trait for AuraFS-aware types.
pub trait AuraFSComponent {
    /// Component name for metrics / logging.
    fn component_name() -> &'static str;
}

/// Soul-verified operation trait.
pub trait SoulVerified {
    /// Verify soul identity for this operation.
    fn verify_soul(&self, soul_id: &BlissId) -> Result<()>;
}

// Re-exports for convenience
pub use ::tracing::{info, warn, error, debug, trace};

/// Core system builder for initialization
pub struct CoreSystemBuilder {
    config: CoreConfig,
    health_manager: Option<HealthManager>,
    tracer: Option<Tracer>,
}

impl CoreSystemBuilder {
    /// Create new builder with default config
    pub fn new() -> Self {
        Self {
            config: CoreConfig::default(),
            health_manager: None,
            tracer: None,
        }
    }
    
    /// Set configuration
    pub fn with_config(mut self, config: CoreConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Enable health checks
    pub fn with_health_checks(mut self) -> Self {
        let mut hm = HealthManager::new(
            env!("CARGO_PKG_VERSION"),
            &self.config.environment,
        );
        hm.register_defaults();
        self.health_manager = Some(hm);
        self
    }
    
    /// Enable tracing
    pub fn with_tracing(mut self, service_name: impl Into<String>) -> Self {
        let tracer = Tracer::new(service_name)
            .with_sample_rate(self.config.observability.tracing_sample_rate)
            .with_enabled(self.config.observability.tracing_enabled);
        self.tracer = Some(tracer);
        self
    }
    
    /// Build and initialize the core system
    pub async fn build(self) -> Result<CoreSystem> {
        // Validate config
        self.config.validate()?;
        
        // Initialize metrics
        if self.config.observability.metrics_enabled {
            AuraFSMetrics::init().await
                .map_err(|e| internal(
                    AuraFSError::Config {
                        message: format!("Failed to initialize metrics: {}", e),
                        key: None,
                    },
                    ErrorPhase::Init,
                ))?;
        }
        
        ::tracing::info!(
            environment = %self.config.environment,
            debug_mode = %self.config.debug_mode,
            "AuraFS core system initialized"
        );
        
        Ok(CoreSystem {
            config: self.config,
            health_manager: self.health_manager,
            tracer: self.tracer,
        })
    }
}

impl Default for CoreSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialized core system
pub struct CoreSystem {
    /// Configuration
    pub config: CoreConfig,
    /// Health manager
    pub health_manager: Option<HealthManager>,
    /// Tracer
    pub tracer: Option<Tracer>,
}

impl CoreSystem {
    /// Get health report
    pub async fn health(&self) -> Option<HealthReport> {
        if let Some(ref hm) = self.health_manager {
            hm.check_health().await.ok()
        } else {
            None
        }
    }
    
    /// Shutdown the core system
    pub async fn shutdown(&self) {
        if let Some(ref tracer) = self.tracer {
            tracer.shutdown().await;
        }
        ::tracing::info!("AuraFS core system shutdown complete");
    }
}

/// Initialize core AuraFS metrics and tracing (legacy compatibility)
pub async fn init_core() -> Result<()> {
    let _ = CoreSystemBuilder::new()
        .with_health_checks()
        .build()
        .await?;
    
    ::tracing::info!("AuraFS core initialized successfully");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_core_system_builder() {
        let system = CoreSystemBuilder::new()
            .with_config(CoreConfig::default())
            .build()
            .await;
        
        assert!(system.is_ok());
    }
    
    #[test]
    fn test_result_type() {
        let ok: Result<i32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);
    }
}
