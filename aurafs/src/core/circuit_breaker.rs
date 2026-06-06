//! ═══════════════════════════════════════════════════════════════════
//! ⚡ AuraFS Core Circuit Breaker - Enterprise Resilience Pattern
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Circuit breaker pattern for external dependencies with:
//! - Configurable failure thresholds
//! - Half-open state for recovery testing
//! - Metrics integration for monitoring
//! - Per-operation circuit breakers
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, internal};

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are rejected
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: u32,
    /// Number of successes in half-open to close circuit
    pub success_threshold: u32,
    /// Time to wait before moving from open to half-open
    pub timeout: Duration,
    /// Maximum concurrent requests in half-open state
    pub half_open_max_requests: u32,
    /// Enable circuit breaker
    pub enabled: bool,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            half_open_max_requests: 3,
            enabled: true,
        }
    }
}

/// Circuit breaker statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitStats {
    /// Total requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Rejected requests (circuit open)
    pub rejected_requests: u64,
    /// Times circuit was opened
    pub times_opened: u64,
    /// Times circuit was closed
    pub times_closed: u64,
    /// Current consecutive failures
    pub consecutive_failures: u32,
    /// Current consecutive successes (in half-open)
    pub consecutive_successes: u32,
}

/// Internal state for atomic operations
struct CircuitBreakerState {
    state: RwLock<CircuitState>,
    failures: AtomicU32,
    successes: AtomicU32,
    last_failure_time: RwLock<Option<Instant>>,
    half_open_requests: AtomicU32,
    stats: RwLock<CircuitStats>,
}

impl CircuitBreakerState {
    fn new() -> Self {
        Self {
            state: RwLock::new(CircuitState::Closed),
            failures: AtomicU32::new(0),
            successes: AtomicU32::new(0),
            last_failure_time: RwLock::new(None),
            half_open_requests: AtomicU32::new(0),
            stats: RwLock::new(CircuitStats::default()),
        }
    }
}

/// Circuit breaker for a single operation/service
pub struct CircuitBreaker {
    name: String,
    config: CircuitBreakerConfig,
    state: Arc<CircuitBreakerState>,
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new(name: impl Into<String>, config: CircuitBreakerConfig) -> Self {
        Self {
            name: name.into(),
            config,
            state: Arc::new(CircuitBreakerState::new()),
        }
    }
    
    /// Create with default config
    pub fn with_defaults(name: impl Into<String>) -> Self {
        Self::new(name, CircuitBreakerConfig::default())
    }
    
    /// Get current state
    pub async fn state(&self) -> CircuitState {
        *self.state.state.read().await
    }
    
    /// Get statistics
    pub async fn stats(&self) -> CircuitStats {
        self.state.stats.read().await.clone()
    }
    
    /// Check if circuit allows request
    pub async fn allow_request(&self) -> Result<bool> {
        if !self.config.enabled {
            return Ok(true);
        }
        
        let current_state = *self.state.state.read().await;
        
        match current_state {
            CircuitState::Closed => {
                let mut stats = self.state.stats.write().await;
                stats.total_requests += 1;
                Ok(true)
            }
            CircuitState::Open => {
                // Check if timeout has passed
                let last_failure = self.state.last_failure_time.read().await;
                if let Some(time) = *last_failure {
                    if time.elapsed() >= self.config.timeout {
                        // Transition to half-open
                        drop(last_failure);
                        self.transition_to_half_open().await;
                        
                        let mut stats = self.state.stats.write().await;
                        stats.total_requests += 1;
                        return Ok(true);
                    }
                }
                
                let mut stats = self.state.stats.write().await;
                stats.total_requests += 1;
                stats.rejected_requests += 1;
                Ok(false)
            }
            CircuitState::HalfOpen => {
                // Allow limited requests in half-open state
                let current = self.state.half_open_requests.fetch_add(1, Ordering::SeqCst);
                if current < self.config.half_open_max_requests {
                    let mut stats = self.state.stats.write().await;
                    stats.total_requests += 1;
                    Ok(true)
                } else {
                    self.state.half_open_requests.fetch_sub(1, Ordering::SeqCst);
                    let mut stats = self.state.stats.write().await;
                    stats.total_requests += 1;
                    stats.rejected_requests += 1;
                    Ok(false)
                }
            }
        }
    }
    
    /// Record successful operation
    pub async fn record_success(&self) {
        if !self.config.enabled {
            return;
        }
        
        let current_state = *self.state.state.read().await;
        
        {
            let mut stats = self.state.stats.write().await;
            stats.successful_requests += 1;
            stats.consecutive_failures = 0;
        }
        
        match current_state {
            CircuitState::Closed => {
                // Reset failures on success
                self.state.failures.store(0, Ordering::SeqCst);
            }
            CircuitState::HalfOpen => {
                self.state.half_open_requests.fetch_sub(1, Ordering::SeqCst);
                let successes = self.state.successes.fetch_add(1, Ordering::SeqCst) + 1;
                
                {
                    let mut stats = self.state.stats.write().await;
                    stats.consecutive_successes = successes;
                }
                
                if successes >= self.config.success_threshold {
                    self.transition_to_closed().await;
                }
            }
            CircuitState::Open => {
                // Shouldn't happen, but handle gracefully
            }
        }
    }
    
    /// Record failed operation
    pub async fn record_failure(&self) {
        if !self.config.enabled {
            return;
        }
        
        let current_state = *self.state.state.read().await;
        
        {
            let mut stats = self.state.stats.write().await;
            stats.failed_requests += 1;
            stats.consecutive_failures += 1;
            stats.consecutive_successes = 0;
        }
        
        match current_state {
            CircuitState::Closed => {
                let failures = self.state.failures.fetch_add(1, Ordering::SeqCst) + 1;
                
                if failures >= self.config.failure_threshold {
                    self.transition_to_open().await;
                }
            }
            CircuitState::HalfOpen => {
                self.state.half_open_requests.fetch_sub(1, Ordering::SeqCst);
                // Any failure in half-open reopens the circuit
                self.transition_to_open().await;
            }
            CircuitState::Open => {
                // Already open, just update last failure time
                let mut last = self.state.last_failure_time.write().await;
                *last = Some(Instant::now());
            }
        }
    }
    
    /// Execute operation with circuit breaker protection
    pub async fn execute<F, T, E>(&self, operation: F) -> Result<T>
    where
        F: std::future::Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display,
    {
        if !self.allow_request().await? {
            return Err(internal(
                AuraFSError::External {
                    dependency: self.name.clone(),
                    message: "Circuit breaker is open".to_string(),
                },
                ErrorPhase::Network,
            ));
        }
        
        match operation.await {
            Ok(result) => {
                self.record_success().await;
                Ok(result)
            }
            Err(e) => {
                self.record_failure().await;
                Err(internal(
                    AuraFSError::External {
                        dependency: self.name.clone(),
                        message: format!("Operation failed: {}", e),
                    },
                    ErrorPhase::Network,
                ))
            }
        }
    }
    
    /// Execute with timeout and circuit breaker protection
    pub async fn execute_with_timeout<F, T, E>(
        &self,
        operation: F,
        timeout: Duration,
    ) -> Result<T>
    where
        F: std::future::Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display,
    {
        if !self.allow_request().await? {
            return Err(internal(
                AuraFSError::External {
                    dependency: self.name.clone(),
                    message: "Circuit breaker is open".to_string(),
                },
                ErrorPhase::Network,
            ));
        }
        
        match tokio::time::timeout(timeout, operation).await {
            Ok(Ok(result)) => {
                self.record_success().await;
                Ok(result)
            }
            Ok(Err(e)) => {
                self.record_failure().await;
                Err(internal(
                    AuraFSError::External {
                        dependency: self.name.clone(),
                        message: format!("Operation failed: {}", e),
                    },
                    ErrorPhase::Network,
                ))
            }
            Err(_) => {
                self.record_failure().await;
                Err(internal(
                    AuraFSError::External {
                        dependency: self.name.clone(),
                        message: "Operation timed out".to_string(),
                    },
                    ErrorPhase::Network,
                ))
            }
        }
    }
    
    /// Transition to open state
    async fn transition_to_open(&self) {
        let mut state = self.state.state.write().await;
        if *state != CircuitState::Open {
            *state = CircuitState::Open;
            drop(state);
            
            let mut last = self.state.last_failure_time.write().await;
            *last = Some(Instant::now());
            drop(last);
            
            self.state.successes.store(0, Ordering::SeqCst);
            
            let mut stats = self.state.stats.write().await;
            stats.times_opened += 1;
            
            tracing::warn!(
                circuit = %self.name,
                "Circuit breaker opened after {} failures",
                self.config.failure_threshold
            );
        }
    }
    
    /// Transition to half-open state
    async fn transition_to_half_open(&self) {
        let mut state = self.state.state.write().await;
        if *state == CircuitState::Open {
            *state = CircuitState::HalfOpen;
            drop(state);
            
            self.state.failures.store(0, Ordering::SeqCst);
            self.state.successes.store(0, Ordering::SeqCst);
            self.state.half_open_requests.store(0, Ordering::SeqCst);
            
            tracing::info!(
                circuit = %self.name,
                "Circuit breaker transitioned to half-open"
            );
        }
    }
    
    /// Transition to closed state
    async fn transition_to_closed(&self) {
        let mut state = self.state.state.write().await;
        if *state != CircuitState::Closed {
            *state = CircuitState::Closed;
            drop(state);
            
            self.state.failures.store(0, Ordering::SeqCst);
            self.state.successes.store(0, Ordering::SeqCst);
            self.state.half_open_requests.store(0, Ordering::SeqCst);
            
            let mut stats = self.state.stats.write().await;
            stats.times_closed += 1;
            stats.consecutive_failures = 0;
            
            tracing::info!(
                circuit = %self.name,
                "Circuit breaker closed after successful recovery"
            );
        }
    }
    
    /// Force reset the circuit breaker
    pub async fn reset(&self) {
        let mut state = self.state.state.write().await;
        *state = CircuitState::Closed;
        drop(state);
        
        self.state.failures.store(0, Ordering::SeqCst);
        self.state.successes.store(0, Ordering::SeqCst);
        self.state.half_open_requests.store(0, Ordering::SeqCst);
        
        let mut last = self.state.last_failure_time.write().await;
        *last = None;
        
        tracing::info!(circuit = %self.name, "Circuit breaker manually reset");
    }
}

/// Circuit breaker registry for managing multiple circuits
pub struct CircuitBreakerRegistry {
    circuits: RwLock<HashMap<String, Arc<CircuitBreaker>>>,
    default_config: CircuitBreakerConfig,
}

impl CircuitBreakerRegistry {
    /// Create new registry
    pub fn new(default_config: CircuitBreakerConfig) -> Self {
        Self {
            circuits: RwLock::new(HashMap::new()),
            default_config,
        }
    }
    
    /// Create with default config
    pub fn with_defaults() -> Self {
        Self::new(CircuitBreakerConfig::default())
    }
    
    /// Get or create circuit breaker
    pub async fn get_or_create(&self, name: &str) -> Arc<CircuitBreaker> {
        // Check if exists
        {
            let circuits = self.circuits.read().await;
            if let Some(cb) = circuits.get(name) {
                return Arc::clone(cb);
            }
        }
        
        // Create new
        let mut circuits = self.circuits.write().await;
        // Double-check after acquiring write lock
        if let Some(cb) = circuits.get(name) {
            return Arc::clone(cb);
        }
        
        let cb = Arc::new(CircuitBreaker::new(name, self.default_config.clone()));
        circuits.insert(name.to_string(), Arc::clone(&cb));
        cb
    }
    
    /// Get circuit breaker by name
    pub async fn get(&self, name: &str) -> Option<Arc<CircuitBreaker>> {
        let circuits = self.circuits.read().await;
        circuits.get(name).cloned()
    }
    
    /// Get all circuit breakers
    pub async fn all(&self) -> Vec<Arc<CircuitBreaker>> {
        let circuits = self.circuits.read().await;
        circuits.values().cloned().collect()
    }
    
    /// Get all stats
    pub async fn all_stats(&self) -> HashMap<String, CircuitStats> {
        let circuits = self.circuits.read().await;
        let mut stats = HashMap::new();
        
        for (name, cb) in circuits.iter() {
            stats.insert(name.clone(), cb.stats().await);
        }
        
        stats
    }
    
    /// Reset all circuit breakers
    pub async fn reset_all(&self) {
        let circuits = self.circuits.read().await;
        for cb in circuits.values() {
            cb.reset().await;
        }
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_circuit_breaker_transitions() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            half_open_max_requests: 2,
            enabled: true,
        };
        
        let cb = CircuitBreaker::new("test", config);
        
        // Should start closed
        assert_eq!(cb.state().await, CircuitState::Closed);
        
        // Record failures to trigger open
        cb.record_failure().await;
        cb.record_failure().await;
        assert_eq!(cb.state().await, CircuitState::Closed);
        
        cb.record_failure().await;
        assert_eq!(cb.state().await, CircuitState::Open);
        
        // Should reject requests when open
        assert!(!cb.allow_request().await.unwrap());
        
        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should transition to half-open
        assert!(cb.allow_request().await.unwrap());
        assert_eq!(cb.state().await, CircuitState::HalfOpen);
        
        // Record successes to close
        cb.record_success().await;
        cb.record_success().await;
        assert_eq!(cb.state().await, CircuitState::Closed);
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_disabled() {
        let config = CircuitBreakerConfig {
            enabled: false,
            ..Default::default()
        };
        
        let cb = CircuitBreaker::new("test", config);
        
        // Should always allow when disabled
        for _ in 0..10 {
            cb.record_failure().await;
        }
        
        assert!(cb.allow_request().await.unwrap());
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_registry() {
        let registry = CircuitBreakerRegistry::with_defaults();
        
        let cb1 = registry.get_or_create("service1").await;
        let cb2 = registry.get_or_create("service1").await;
        
        // Should return same instance
        assert!(Arc::ptr_eq(&cb1, &cb2));
        
        let cb3 = registry.get_or_create("service2").await;
        assert!(!Arc::ptr_eq(&cb1, &cb3));
        
        assert_eq!(registry.all().await.len(), 2);
    }
}
