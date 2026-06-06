//! ═══════════════════════════════════════════════════════════════════
//! 🚦 AuraFS Core Rate Limiter - Enterprise Traffic Control
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Token bucket and sliding window rate limiting with:
//! - Per-soul rate limiting
//! - Per-operation rate limiting
//! - Distributed rate limiting support
//! - Metrics integration
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, client, BlissId};

/// Rate limiter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Window duration
    pub window: Duration,
    /// Maximum requests per window
    pub max_requests: u32,
    /// Burst capacity (for token bucket)
    pub burst_capacity: u32,
    /// Enable rate limiting
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            window: Duration::from_secs(60),
            max_requests: 1000,
            burst_capacity: 100,
            enabled: true,
        }
    }
}

/// Rate limit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResult {
    /// Whether request is allowed
    pub allowed: bool,
    /// Remaining requests in window
    pub remaining: u32,
    /// Time until window reset
    pub reset_after: Duration,
    /// Current request count in window
    pub current_count: u32,
    /// Limit
    pub limit: u32,
}

impl RateLimitResult {
    /// Check if request was allowed
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }
}

/// Sliding window entry
#[derive(Debug, Clone)]
struct SlidingWindowEntry {
    timestamps: Vec<Instant>,
    window_start: Instant,
}

impl SlidingWindowEntry {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            window_start: Instant::now(),
        }
    }
    
    /// Clean old timestamps outside window
    fn clean(&mut self, window: Duration) {
        let now = Instant::now();
        let cutoff = now - window;
        self.timestamps.retain(|t| *t > cutoff);
        
        if self.timestamps.is_empty() {
            self.window_start = now;
        }
    }
    
    /// Check and record request
    fn check_and_record(&mut self, window: Duration, max_requests: u32) -> RateLimitResult {
        self.clean(window);
        
        let now = Instant::now();
        let current_count = self.timestamps.len() as u32;
        let allowed = current_count < max_requests;
        
        if allowed {
            self.timestamps.push(now);
        }
        
        let oldest = self.timestamps.first().copied().unwrap_or(now);
        let reset_after = if oldest + window > now {
            oldest + window - now
        } else {
            Duration::ZERO
        };
        
        RateLimitResult {
            allowed,
            remaining: max_requests.saturating_sub(current_count + if allowed { 1 } else { 0 }),
            reset_after,
            current_count: current_count + if allowed { 1 } else { 0 },
            limit: max_requests,
        }
    }
}

/// Token bucket for burst handling
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64, // tokens per second
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            tokens: capacity as f64,
            last_refill: Instant::now(),
            capacity: capacity as f64,
            refill_rate,
        }
    }
    
    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;
        
        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill = now;
    }
    
    /// Try to consume a token
    fn try_consume(&mut self) -> bool {
        self.refill();
        
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
    
    /// Get available tokens
    fn available(&mut self) -> u32 {
        self.refill();
        self.tokens as u32
    }
}

/// Sliding window rate limiter
pub struct SlidingWindowRateLimiter {
    config: RateLimitConfig,
    entries: Arc<RwLock<HashMap<String, SlidingWindowEntry>>>,
}

impl SlidingWindowRateLimiter {
    /// Create new sliding window rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Check rate limit for key
    pub async fn check(&self, key: &str) -> RateLimitResult {
        if !self.config.enabled {
            return RateLimitResult {
                allowed: true,
                remaining: self.config.max_requests,
                reset_after: self.config.window,
                current_count: 0,
                limit: self.config.max_requests,
            };
        }
        
        let mut entries = self.entries.write().await;
        let entry = entries.entry(key.to_string())
            .or_insert_with(SlidingWindowEntry::new);
        
        entry.check_and_record(self.config.window, self.config.max_requests)
    }
    
    /// Check rate limit and return error if exceeded
    pub async fn check_or_error(&self, key: &str) -> Result<RateLimitResult> {
        let result = self.check(key).await;
        
        if !result.allowed {
            return Err(client(
                AuraFSError::Other {
                    message: format!(
                        "Rate limit exceeded for '{}': {} requests in {:?}",
                        key, result.limit, self.config.window
                    ),
                },
                ErrorPhase::Other,
                ErrorCode::ResourceExhausted,
            ));
        }
        
        Ok(result)
    }
    
    /// Clean up old entries
    pub async fn cleanup(&self) {
        let mut entries = self.entries.write().await;
        entries.retain(|_, entry| {
            let now = Instant::now();
            entry.timestamps.iter().any(|t| now.duration_since(*t) < self.config.window)
        });
    }
}

/// Token bucket rate limiter for burst handling
pub struct TokenBucketRateLimiter {
    config: RateLimitConfig,
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
}

impl TokenBucketRateLimiter {
    /// Create new token bucket rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Check rate limit for key
    pub async fn check(&self, key: &str) -> RateLimitResult {
        if !self.config.enabled {
            return RateLimitResult {
                allowed: true,
                remaining: self.config.burst_capacity,
                reset_after: Duration::ZERO,
                current_count: 0,
                limit: self.config.burst_capacity,
            };
        }
        
        let refill_rate = self.config.max_requests as f64 / self.config.window.as_secs_f64();
        
        let mut buckets = self.buckets.write().await;
        let bucket = buckets.entry(key.to_string())
            .or_insert_with(|| TokenBucket::new(self.config.burst_capacity, refill_rate));
        
        let allowed = bucket.try_consume();
        let remaining = bucket.available();
        
        // Calculate reset time (time until one token is available)
        let reset_after = if remaining == 0 {
            Duration::from_secs_f64(1.0 / refill_rate)
        } else {
            Duration::ZERO
        };
        
        RateLimitResult {
            allowed,
            remaining,
            reset_after,
            current_count: self.config.burst_capacity - remaining,
            limit: self.config.burst_capacity,
        }
    }
    
    /// Check rate limit and return error if exceeded
    pub async fn check_or_error(&self, key: &str) -> Result<RateLimitResult> {
        let result = self.check(key).await;
        
        if !result.allowed {
            return Err(client(
                AuraFSError::Other {
                    message: format!(
                        "Rate limit exceeded for '{}': burst capacity {} exhausted",
                        key, self.config.burst_capacity
                    ),
                },
                ErrorPhase::Other,
                ErrorCode::ResourceExhausted,
            ));
        }
        
        Ok(result)
    }
}

/// Per-soul rate limiter
pub struct SoulRateLimiter {
    /// Default limits for all operations
    default_limiter: SlidingWindowRateLimiter,
    /// Per-operation limits
    operation_limiters: Arc<RwLock<HashMap<String, SlidingWindowRateLimiter>>>,
    /// Operation-specific configs
    operation_configs: HashMap<String, RateLimitConfig>,
}

impl SoulRateLimiter {
    /// Create new soul rate limiter
    pub fn new(default_config: RateLimitConfig) -> Self {
        Self {
            default_limiter: SlidingWindowRateLimiter::new(default_config),
            operation_limiters: Arc::new(RwLock::new(HashMap::new())),
            operation_configs: HashMap::new(),
        }
    }
    
    /// Configure rate limit for specific operation
    pub fn with_operation_limit(mut self, operation: impl Into<String>, config: RateLimitConfig) -> Self {
        self.operation_configs.insert(operation.into(), config);
        self
    }
    
    /// Check rate limit for soul and operation
    pub async fn check(&self, soul_id: &BlissId, operation: Option<&str>) -> RateLimitResult {
        let key = match operation {
            Some(op) => format!("{}:{}", soul_id, op),
            None => soul_id.to_string(),
        };
        
        // Check operation-specific limit if configured
        if let Some(op) = operation {
            if let Some(config) = self.operation_configs.get(op) {
                let mut limiters = self.operation_limiters.write().await;
                let limiter = limiters.entry(op.to_string())
                    .or_insert_with(|| SlidingWindowRateLimiter::new(config.clone()));
                
                let op_result = limiter.check(&key).await;
                if !op_result.allowed {
                    return op_result;
                }
            }
        }
        
        // Check default limit
        self.default_limiter.check(&key).await
    }
    
    /// Check rate limit and return error if exceeded
    pub async fn check_or_error(&self, soul_id: &BlissId, operation: Option<&str>) -> Result<RateLimitResult> {
        let result = self.check(soul_id, operation).await;
        
        if !result.allowed {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::ResourceExhausted,
                    soul_id: Some(soul_id.clone()),
                    message: format!(
                        "Rate limit exceeded: {} requests allowed per {:?}",
                        result.limit,
                        self.default_limiter.config.window
                    ),
                },
                ErrorPhase::Other,
                ErrorCode::ResourceExhausted,
            ));
        }
        
        Ok(result)
    }
}

/// Rate limiter statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimiterStats {
    /// Total requests checked
    pub total_checks: u64,
    /// Requests allowed
    pub allowed: u64,
    /// Requests denied
    pub denied: u64,
    /// Current active keys
    pub active_keys: usize,
}

/// Composite rate limiter with multiple strategies
pub struct CompositeRateLimiter {
    /// Sliding window for sustained rate limiting
    sliding_window: SlidingWindowRateLimiter,
    /// Token bucket for burst handling
    token_bucket: TokenBucketRateLimiter,
    /// Statistics
    stats: Arc<RwLock<RateLimiterStats>>,
}

impl CompositeRateLimiter {
    /// Create new composite rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            sliding_window: SlidingWindowRateLimiter::new(config.clone()),
            token_bucket: TokenBucketRateLimiter::new(config),
            stats: Arc::new(RwLock::new(RateLimiterStats::default())),
        }
    }
    
    /// Check rate limit (both strategies must allow)
    pub async fn check(&self, key: &str) -> RateLimitResult {
        let mut stats = self.stats.write().await;
        stats.total_checks += 1;
        drop(stats);
        
        // Check token bucket first (burst protection)
        let bucket_result = self.token_bucket.check(key).await;
        if !bucket_result.allowed {
            let mut stats = self.stats.write().await;
            stats.denied += 1;
            return bucket_result;
        }
        
        // Check sliding window (sustained rate)
        let window_result = self.sliding_window.check(key).await;
        
        let mut stats = self.stats.write().await;
        if window_result.allowed {
            stats.allowed += 1;
        } else {
            stats.denied += 1;
        }
        
        window_result
    }
    
    /// Check rate limit and return error if exceeded
    pub async fn check_or_error(&self, key: &str) -> Result<RateLimitResult> {
        let result = self.check(key).await;
        
        if !result.allowed {
            return Err(client(
                AuraFSError::Other {
                    message: format!("Rate limit exceeded for '{}'", key),
                },
                ErrorPhase::Other,
                ErrorCode::ResourceExhausted,
            ));
        }
        
        Ok(result)
    }
    
    /// Get statistics
    pub async fn stats(&self) -> RateLimiterStats {
        self.stats.read().await.clone()
    }
    
    /// Cleanup old entries
    pub async fn cleanup(&self) {
        self.sliding_window.cleanup().await;
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sliding_window_rate_limiter() {
        let config = RateLimitConfig {
            window: Duration::from_millis(100),
            max_requests: 5,
            burst_capacity: 10,
            enabled: true,
        };
        
        let limiter = SlidingWindowRateLimiter::new(config);
        
        // Should allow up to max_requests
        for i in 0..5 {
            let result = limiter.check("test").await;
            assert!(result.allowed, "Request {} should be allowed", i);
        }
        
        // Should deny after limit
        let result = limiter.check("test").await;
        assert!(!result.allowed, "Request should be denied after limit");
        
        // Wait for window to reset
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should allow again
        let result = limiter.check("test").await;
        assert!(result.allowed, "Request should be allowed after window reset");
    }
    
    #[tokio::test]
    async fn test_token_bucket_rate_limiter() {
        let config = RateLimitConfig {
            window: Duration::from_secs(1),
            max_requests: 10,
            burst_capacity: 3,
            enabled: true,
        };
        
        let limiter = TokenBucketRateLimiter::new(config);
        
        // Should allow burst
        for i in 0..3 {
            let result = limiter.check("test").await;
            assert!(result.allowed, "Burst request {} should be allowed", i);
        }
        
        // Should deny after burst
        let result = limiter.check("test").await;
        assert!(!result.allowed, "Request should be denied after burst");
    }
    
    #[tokio::test]
    async fn test_soul_rate_limiter() {
        let config = RateLimitConfig {
            window: Duration::from_millis(100),
            max_requests: 3,
            burst_capacity: 5,
            enabled: true,
        };
        
        let limiter = SoulRateLimiter::new(config);
        let soul = BlissId::new(b"test_soul");
        
        // Should allow up to limit
        for _ in 0..3 {
            let result = limiter.check(&soul, None).await;
            assert!(result.allowed);
        }
        
        // Should deny after limit
        let result = limiter.check(&soul, None).await;
        assert!(!result.allowed);
    }
    
    #[tokio::test]
    async fn test_rate_limiter_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            ..Default::default()
        };
        
        let limiter = SlidingWindowRateLimiter::new(config);
        
        // Should always allow when disabled
        for _ in 0..100 {
            let result = limiter.check("test").await;
            assert!(result.allowed);
        }
    }
}
