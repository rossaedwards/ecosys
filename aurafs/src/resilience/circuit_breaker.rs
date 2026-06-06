//! ═══════════════════════════════════════════════════════════════════
//! ⚡ Circuit Breaker Pattern Implementation
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//!
//! State machine circuit breaker for protecting against cascading failures
//! in distributed systems. This module provides a specialized circuit breaker
//! that integrates with the RafsError type system.
//!
//! For core circuit breaker implementation, see `crate::core::circuit_breaker`.
//! ═══════════════════════════════════════════════════════════════════

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;
use crate::error::{RafsError, Result};
use tracing::{debug, info, warn};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation - requests pass through
    Closed,
    /// Failing - requests are rejected immediately
    Open,
    /// Testing if service has recovered
    HalfOpen,
}

impl std::fmt::Display for CircuitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "CLOSED"),
            CircuitState::Open => write!(f, "OPEN"),
            CircuitState::HalfOpen => write!(f, "HALF_OPEN"),
        }
    }
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Number of successes needed to close from half-open
    pub success_threshold: usize,
    /// Operation timeout
    pub timeout: Duration,
    /// Time to wait before attempting half-open
    pub half_open_timeout: Duration,
    /// Name for logging
    pub name: String,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(30),
            half_open_timeout: Duration::from_secs(60),
            name: "default".to_string(),
        }
    }
}

/// Circuit breaker implementation
pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    config: CircuitBreakerConfig,
    failure_count: Arc<RwLock<usize>>,
    success_count: Arc<RwLock<usize>>,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    last_success_time: Arc<RwLock<Option<Instant>>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            config,
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
            last_success_time: Arc::new(RwLock::new(None)),
        }
    }

    /// Execute an operation through the circuit breaker
    pub async fn execute<F, T>(&self, operation: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let state = *self.state.read().await;

        match state {
            CircuitState::Open => {
                // Check if we should transition to half-open
                if self.should_attempt_reset().await {
                    debug!("Circuit breaker {} transitioning to HALF_OPEN", self.config.name);
                    self.transition_to_half_open().await;
                } else {
                    let last_failure = *self.last_failure_time.read().await;
                    let wait_time = last_failure
                        .map(|t| self.config.half_open_timeout.saturating_sub(t.elapsed()))
                        .unwrap_or(Duration::ZERO);
                    
                    warn!(
                        "Circuit breaker {} is OPEN, rejecting request. Next attempt in {:?}",
                        self.config.name, wait_time
                    );
                    return Err(RafsError::NetworkError(format!(
                        "Circuit breaker {} is open. Next attempt in {:?}",
                        self.config.name, wait_time
                    )));
                }
            }
            CircuitState::HalfOpen => {
                debug!("Circuit breaker {} is HALF_OPEN, allowing test request", self.config.name);
            }
            CircuitState::Closed => {
                // Normal operation
            }
        }

        // Execute operation with timeout
        let result = timeout(self.config.timeout, operation).await;

        match result {
            Ok(Ok(value)) => {
                self.on_success().await;
                Ok(value)
            }
            Ok(Err(e)) => {
                self.on_failure().await;
                Err(e)
            }
            Err(_) => {
                self.on_timeout().await;
                Err(RafsError::Timeout(format!(
                    "Operation timed out after {:?}",
                    self.config.timeout
                )))
            }
        }
    }

    /// Handle successful operation
    async fn on_success(&self) {
        let mut state = self.state.write().await;
        let mut success_count = self.success_count.write().await;
        let mut failure_count = self.failure_count.write().await;

        match *state {
            CircuitState::HalfOpen => {
                *success_count += 1;
                if *success_count >= self.config.success_threshold {
                    info!(
                        "Circuit breaker {} recovered, transitioning to CLOSED",
                        self.config.name
                    );
                    *state = CircuitState::Closed;
                    *success_count = 0;
                    *failure_count = 0;
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                *failure_count = 0;
            }
            CircuitState::Open => {
                // Should not happen, but handle gracefully
            }
        }

        *self.last_success_time.write().await = Some(Instant::now());
    }

    /// Handle failed operation
    async fn on_failure(&self) {
        let mut state = self.state.write().await;
        let mut failure_count = self.failure_count.write().await;

        match *state {
            CircuitState::HalfOpen => {
                // Failed during half-open, go back to open
                warn!(
                    "Circuit breaker {} failed during HALF_OPEN, transitioning to OPEN",
                    self.config.name
                );
                *state = CircuitState::Open;
                *self.last_failure_time.write().await = Some(Instant::now());
                *failure_count = 0;
                *self.success_count.write().await = 0;
            }
            CircuitState::Closed => {
                *failure_count += 1;
                if *failure_count >= self.config.failure_threshold {
                    warn!(
                        "Circuit breaker {} opened after {} failures",
                        self.config.name, *failure_count
                    );
                    *state = CircuitState::Open;
                    *self.last_failure_time.write().await = Some(Instant::now());
                }
            }
            CircuitState::Open => {
                // Already open, update failure time
                *self.last_failure_time.write().await = Some(Instant::now());
            }
        }
    }

    /// Handle timeout
    async fn on_timeout(&self) {
        self.on_failure().await;
    }

    /// Check if we should attempt to reset (transition to half-open)
    async fn should_attempt_reset(&self) -> bool {
        let last_failure = *self.last_failure_time.read().await;
        if let Some(time) = last_failure {
            time.elapsed() >= self.config.half_open_timeout
        } else {
            true
        }
    }

    /// Transition to half-open state
    async fn transition_to_half_open(&self) {
        let mut state = self.state.write().await;
        *state = CircuitState::HalfOpen;
        *self.success_count.write().await = 0;
        *self.failure_count.write().await = 0;
    }

    /// Get current state
    pub async fn state(&self) -> CircuitState {
        *self.state.read().await
    }

    /// Get failure count
    pub async fn failure_count(&self) -> usize {
        *self.failure_count.read().await
    }

    /// Get success count
    pub async fn success_count(&self) -> usize {
        *self.success_count.read().await
    }

    /// Reset circuit breaker to closed state
    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        *state = CircuitState::Closed;
        *self.failure_count.write().await = 0;
        *self.success_count.write().await = 0;
        *self.last_failure_time.write().await = None;
        *self.last_success_time.write().await = None;
        info!("Circuit breaker {} manually reset", self.config.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_circuit_breaker_closed_to_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(1),
            half_open_timeout: Duration::from_secs(1),
            name: "test".to_string(),
        };
        let breaker = CircuitBreaker::new(config);

        // Should start closed
        assert_eq!(breaker.state().await, CircuitState::Closed);

        // Fail 3 times to open circuit
        for _ in 0..3 {
            let _ = breaker
                .execute(async {
                    Err::<(), _>(RafsError::NetworkError("test".to_string()))
                })
                .await;
        }

        assert_eq!(breaker.state().await, CircuitState::Open);
    }

    #[tokio::test]
    async fn test_circuit_breaker_success_reset() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_secs(1),
            half_open_timeout: Duration::from_millis(100),
            name: "test".to_string(),
        };
        let breaker = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = breaker
                .execute(async {
                    Err::<(), _>(RafsError::NetworkError("test".to_string()))
                })
                .await;
        }
        assert_eq!(breaker.state().await, CircuitState::Open);

        // Wait for half-open timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Succeed 2 times to close circuit
        for _ in 0..2 {
            let _ = breaker.execute(async { Ok::<(), _>(()) }).await;
        }

        assert_eq!(breaker.state().await, CircuitState::Closed);
    }
}

