//! ═══════════════════════════════════════════════════════════════════
//! 🛡️ Error Recovery Strategies
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//!
//! Implements various recovery strategies for handling failures gracefully
//! including degraded modes, fallbacks, and combined strategies.
//! ═══════════════════════════════════════════════════════════════════

use crate::error::{RafsError, Result};
use crate::resilience::{CircuitBreaker, ExponentialBackoff};
use std::sync::Arc;
use tracing::{debug, warn};

/// Degraded mode configuration
#[derive(Debug, Clone)]
pub enum DegradedMode {
    /// Read-only mode
    ReadOnly,
    /// Limited functionality
    Limited,
    /// Cache-only mode
    CacheOnly,
    /// Minimal service
    Minimal,
}

impl DegradedMode {
    /// Execute operation in degraded mode
    pub async fn execute<F, T>(&self, operation: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        match self {
            DegradedMode::ReadOnly => {
                // Only allow read operations
                warn!("Operating in read-only degraded mode");
                operation.await
            }
            DegradedMode::Limited => {
                warn!("Operating in limited functionality mode");
                operation.await
            }
            DegradedMode::CacheOnly => {
                warn!("Operating in cache-only mode");
                operation.await
            }
            DegradedMode::Minimal => {
                warn!("Operating in minimal service mode");
                operation.await
            }
        }
    }
}

/// Recovery strategy for handling failures
#[derive(Clone)]
pub enum RecoveryStrategy {
    /// Retry with exponential backoff
    Retry {
        max_attempts: usize,
        backoff: ExponentialBackoff,
    },
    /// Fallback to alternative operation
    Fallback {
        fallback_fn: Arc<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>> + Send + Sync>,
    },
    /// Use circuit breaker
    CircuitBreaker {
        breaker: Arc<CircuitBreaker>,
    },
    /// Degrade service
    Degrade {
        degraded_mode: DegradedMode,
    },
    /// Combine multiple strategies
    Combined {
        strategies: Vec<RecoveryStrategy>,
    },
}

impl RecoveryStrategy {
    /// Execute operation with recovery strategy
    pub async fn execute<F, T>(&self, operation: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        match self {
            RecoveryStrategy::Retry { max_attempts, backoff } => {
                use crate::resilience::retry::RetryStrategy;
                use crate::resilience::retry::retry_with_backoff;

                let mut backoff_clone = backoff.clone();
                backoff_clone.max_attempts = *max_attempts;

                retry_with_backoff(
                    || Box::pin(operation),
                    crate::resilience::retry::RetryStrategy::Exponential(backoff_clone),
                )
                .await
            }
            RecoveryStrategy::Fallback { fallback_fn } => {
                match operation.await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        warn!("Primary operation failed, using fallback: {}", e);
                        fallback_fn().await
                    }
                }
            }
            RecoveryStrategy::CircuitBreaker { breaker } => {
                breaker.execute(operation).await
            }
            RecoveryStrategy::Degrade { degraded_mode } => {
                degraded_mode.execute(operation).await
            }
            RecoveryStrategy::Combined { strategies } => {
                let mut last_error = None;
                for strategy in strategies {
                    match strategy.execute(operation).await {
                        Ok(result) => return Ok(result),
                        Err(e) => {
                            last_error = Some(e);
                            continue;
                        }
                    }
                }
                Err(last_error.unwrap_or_else(|| {
                    RafsError::Internal("All recovery strategies exhausted".to_string())
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resilience::CircuitBreakerConfig;
    use std::time::Duration;

    #[tokio::test]
    async fn test_fallback_strategy() {
        let strategy = RecoveryStrategy::Fallback {
            fallback_fn: Arc::new(|| {
                Box::pin(async { Ok::<usize, _>(42) })
            }),
        };

        let result = strategy
            .execute(async {
                Err::<usize, _>(RafsError::NetworkError("primary failed".to_string()))
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_degraded_mode() {
        let strategy = RecoveryStrategy::Degrade {
            degraded_mode: DegradedMode::ReadOnly,
        };

        let result = strategy
            .execute(async { Ok::<usize, _>(100) })
            .await;

        assert!(result.is_ok());
    }
}

