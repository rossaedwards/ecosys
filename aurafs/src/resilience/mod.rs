//! ═══════════════════════════════════════════════════════════════════
//! ⚡ AuraFS Resilience Patterns - Enterprise Fault Tolerance
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//! 
//! Implements circuit breaker, retry strategies, and error recovery patterns
//! for quantum enterprise-grade distributed filesystem operations.
//!
//! This module re-exports and extends patterns from `core::circuit_breaker`
//! and `core::rate_limiter` while providing additional recovery strategies.
//! ═══════════════════════════════════════════════════════════════════

pub mod circuit_breaker;
pub mod retry;
pub mod recovery;

// Re-exports from local modules
pub use circuit_breaker::{CircuitBreaker, CircuitState, CircuitBreakerConfig};
pub use retry::{ExponentialBackoff, RetryStrategy, retry_with_backoff};
pub use recovery::{RecoveryStrategy, DegradedMode};

// Re-exports from core for unified access
pub use crate::core::circuit_breaker::{
    CircuitBreaker as CoreCircuitBreaker,
    CircuitBreakerConfig as CoreCircuitBreakerConfig,
    CircuitState as CoreCircuitState,
    CircuitBreakerRegistry,
    CircuitStats,
};

pub use crate::core::rate_limiter::{
    RateLimitConfig,
    RateLimitResult,
    SlidingWindowRateLimiter,
    TokenBucketRateLimiter,
    SoulRateLimiter,
    CompositeRateLimiter,
};

/// Resilience configuration combining all patterns
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Retry configuration
    pub retry: ExponentialBackoff,
    /// Enable circuit breaker
    pub circuit_breaker_enabled: bool,
    /// Enable retries
    pub retry_enabled: bool,
    /// Enable rate limiting
    pub rate_limit_enabled: bool,
    /// Rate limit configuration
    pub rate_limit: RateLimitConfig,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            circuit_breaker: CircuitBreakerConfig::default(),
            retry: ExponentialBackoff::default(),
            circuit_breaker_enabled: true,
            retry_enabled: true,
            rate_limit_enabled: true,
            rate_limit: RateLimitConfig::default(),
        }
    }
}

/// Builder for resilience patterns
pub struct ResilienceBuilder {
    config: ResilienceConfig,
}

impl ResilienceBuilder {
    /// Create new builder with defaults
    pub fn new() -> Self {
        Self {
            config: ResilienceConfig::default(),
        }
    }
    
    /// Configure circuit breaker
    pub fn with_circuit_breaker(mut self, config: CircuitBreakerConfig) -> Self {
        self.config.circuit_breaker = config;
        self
    }
    
    /// Configure retry strategy
    pub fn with_retry(mut self, config: ExponentialBackoff) -> Self {
        self.config.retry = config;
        self
    }
    
    /// Configure rate limiting
    pub fn with_rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.config.rate_limit = config;
        self
    }
    
    /// Disable circuit breaker
    pub fn without_circuit_breaker(mut self) -> Self {
        self.config.circuit_breaker_enabled = false;
        self
    }
    
    /// Disable retries
    pub fn without_retry(mut self) -> Self {
        self.config.retry_enabled = false;
        self
    }
    
    /// Disable rate limiting
    pub fn without_rate_limit(mut self) -> Self {
        self.config.rate_limit_enabled = false;
        self
    }
    
    /// Build configuration
    pub fn build(self) -> ResilienceConfig {
        self.config
    }
}

impl Default for ResilienceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resilience_builder() {
        let config = ResilienceBuilder::new()
            .without_circuit_breaker()
            .without_rate_limit()
            .build();
        
        assert!(!config.circuit_breaker_enabled);
        assert!(!config.rate_limit_enabled);
        assert!(config.retry_enabled);
    }
    
    #[test]
    fn test_default_config() {
        let config = ResilienceConfig::default();
        assert!(config.circuit_breaker_enabled);
        assert!(config.retry_enabled);
        assert!(config.rate_limit_enabled);
    }
}
