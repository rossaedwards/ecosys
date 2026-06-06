//! ═══════════════════════════════════════════════════════════════════
//! 🔄 Retry Strategies with Exponential Backoff
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//!
//! Implements exponential backoff with jitter for retrying failed operations.
//! Includes fixed, exponential, and custom retry strategies with proper
//! error classification for retryable vs non-retryable errors.
//! ═══════════════════════════════════════════════════════════════════

use std::time::Duration;
use rand::Rng;
use crate::error::{RafsError, Result};
use tracing::{debug, warn};

/// Exponential backoff configuration
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    /// Initial delay before first retry
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential growth
    pub multiplier: f64,
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Enable jitter to prevent thundering herd
    pub jitter: bool,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            max_attempts: 5,
            jitter: true,
        }
    }
}

impl ExponentialBackoff {
    /// Create new exponential backoff strategy
    pub fn new(
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
        max_attempts: usize,
    ) -> Self {
        Self {
            initial_delay,
            max_delay,
            multiplier,
            max_attempts,
            jitter: true,
        }
    }

    /// Calculate delay for a given attempt number
    pub fn calculate_delay(&self, attempt: usize) -> Duration {
        let base_delay = self.initial_delay.as_secs_f64()
            * self.multiplier.powi(attempt as i32);
        let delay_secs = base_delay.min(self.max_delay.as_secs_f64());

        let delay = if self.jitter {
            // Add jitter: random value between 0 and 20% of delay
            let jitter_amount = delay_secs * 0.2 * rand::thread_rng().gen::<f64>();
            delay_secs + jitter_amount
        } else {
            delay_secs
        };

        Duration::from_secs_f64(delay.min(self.max_delay.as_secs_f64()))
    }
}

/// Retry strategy configuration
#[derive(Debug, Clone)]
pub enum RetryStrategy {
    /// No retry
    None,
    /// Fixed number of attempts with fixed delay
    Fixed {
        attempts: usize,
        delay: Duration,
    },
    /// Exponential backoff
    Exponential(ExponentialBackoff),
    /// Custom retry logic
    Custom {
        max_attempts: usize,
        should_retry: fn(&RafsError) -> bool,
        calculate_delay: fn(usize) -> Duration,
    },
}

impl Default for RetryStrategy {
    fn default() -> Self {
        RetryStrategy::Exponential(ExponentialBackoff::default())
    }
}

/// Retry an operation with the given strategy
pub async fn retry_with_backoff<F, T>(
    operation: F,
    strategy: RetryStrategy,
) -> Result<T>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
{
    match strategy {
        RetryStrategy::None => operation().await,
        RetryStrategy::Fixed { attempts, delay } => {
            let mut last_error = None;

            for attempt in 0..attempts {
                match operation().await {
                    Ok(result) => {
                        if attempt > 0 {
                            debug!("Operation succeeded after {} retries", attempt);
                        }
                        return Ok(result);
                    }
                    Err(e) => {
                        last_error = Some(e);
                        if attempt < attempts - 1 {
                            tokio::time::sleep(delay).await;
                        }
                    }
                }
            }

            Err(last_error.unwrap_or_else(|| {
                RafsError::Internal("All retry attempts exhausted".to_string())
            }))
        }
        RetryStrategy::Exponential(backoff) => {
            let mut last_error = None;

            for attempt in 0..backoff.max_attempts {
                match operation().await {
                    Ok(result) => {
                        if attempt > 0 {
                            debug!("Operation succeeded after {} retries", attempt);
                        }
                        return Ok(result);
                    }
                    Err(e) => {
                        last_error = Some(e);

                        // Check if error is retryable
                        if !e.is_retryable() {
                            warn!("Non-retryable error encountered: {}", e);
                            return Err(e);
                        }

                        if attempt < backoff.max_attempts - 1 {
                            let delay = backoff.calculate_delay(attempt);
                            debug!(
                                "Retry attempt {} after {:?} (error: {})",
                                attempt + 1, delay, e
                            );
                            tokio::time::sleep(delay).await;
                        }
                    }
                }
            }

            Err(last_error.unwrap_or_else(|| {
                RafsError::Internal(format!(
                    "All {} retry attempts exhausted",
                    backoff.max_attempts
                ))
            }))
        }
        RetryStrategy::Custom {
            max_attempts,
            should_retry,
            calculate_delay,
        } => {
            let mut last_error = None;

            for attempt in 0..max_attempts {
                match operation().await {
                    Ok(result) => {
                        if attempt > 0 {
                            debug!("Operation succeeded after {} retries", attempt);
                        }
                        return Ok(result);
                    }
                    Err(e) => {
                        last_error = Some(e);

                        if !should_retry(&e) {
                            warn!("Error not retryable: {}", e);
                            return Err(e);
                        }

                        if attempt < max_attempts - 1 {
                            let delay = calculate_delay(attempt);
                            debug!(
                                "Retry attempt {} after {:?} (error: {})",
                                attempt + 1, delay, e
                            );
                            tokio::time::sleep(delay).await;
                        }
                    }
                }
            }

            Err(last_error.unwrap_or_else(|| {
                RafsError::Internal(format!(
                    "All {} retry attempts exhausted",
                    max_attempts
                ))
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[tokio::test]
    async fn test_exponential_backoff() {
        let backoff = ExponentialBackoff::new(
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
            5,
        );

        let delay1 = backoff.calculate_delay(0);
        let delay2 = backoff.calculate_delay(1);
        let delay3 = backoff.calculate_delay(2);

        assert!(delay2 > delay1);
        assert!(delay3 > delay2);
        assert!(delay3 <= Duration::from_secs(10));
    }

    #[tokio::test]
    async fn test_retry_success() {
        let counter = AtomicUsize::new(0);
        let strategy = RetryStrategy::Fixed {
            attempts: 3,
            delay: Duration::from_millis(10),
        };

        let result = retry_with_backoff(
            || {
                Box::pin(async {
                    let count = counter.fetch_add(1, Ordering::SeqCst);
                    if count < 2 {
                        Err(RafsError::NetworkError("retry".to_string()))
                    } else {
                        Ok(count)
                    }
                })
            },
            strategy,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let strategy = RetryStrategy::Fixed {
            attempts: 3,
            delay: Duration::from_millis(10),
        };

        let result = retry_with_backoff(
            || {
                Box::pin(async {
                    Err::<usize, _>(RafsError::NetworkError("always fails".to_string()))
                })
            },
            strategy,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let strategy = RetryStrategy::Exponential(ExponentialBackoff::default());

        let result = retry_with_backoff(
            || {
                Box::pin(async {
                    Err::<usize, _>(RafsError::InvalidShard("fatal".to_string()))
                })
            },
            strategy,
        )
        .await;

        assert!(result.is_err());
        // Should not retry for non-retryable errors
    }
}

