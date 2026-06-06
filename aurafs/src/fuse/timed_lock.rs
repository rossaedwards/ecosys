//! AuraFS Timed Locking Utility - Aurphyx LLC
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx Division 💎
//!
//! Enforces the 100μs acquisition limit to maintain 1600μs total coherence.

use crate::prelude::*;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

/// [Theorem 3.1: Topological Stability]
/// A wrapper for RwLock that enforces the Aurphyx Coherence Heartbeat.
pub struct AuraTimedLock<T> {
    inner: RwLock<T>,
}

impl<T> AuraTimedLock<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: RwLock::new(val),
        }
    }

    /// Attempt to acquire a read lock within the lock_acquisition_timeout_us window.
    pub async fn read_coherent(&self) -> Result<RwLockReadGuard<'_, T>> {
        let start = Instant::now();
        let timeout = Duration::from_micros(INVARIANTS.lock_acquisition_timeout_us);

        match tokio::time::timeout(timeout, self.inner.read()).await {
            Ok(guard) => Ok(guard),
            Err(_) => {
                let elapsed = start.elapsed().as_micros() as u64;
                error!("[Physics] Read lock contention exceeded {}μs. Coherence at risk.", INVARIANTS.lock_acquisition_timeout_us);
                Err(RafsError::PhysicsViolation(
                    PhysicsViolationError::LockTimeout { 
                        limit: INVARIANTS.lock_acquisition_timeout_us 
                    }
                ))
            }
        }
    }

    /// Attempt to acquire a write lock within the lock_acquisition_timeout_us window.
    pub async fn write_coherent(&self) -> Result<RwLockWriteGuard<'_, T>> {
        let start = Instant::now();
        let timeout = Duration::from_micros(INVARIANTS.lock_acquisition_timeout_us);

        match tokio::time::timeout(timeout, self.inner.write()).await {
            Ok(guard) => Ok(guard),
            Err(_) => {
                let elapsed = start.elapsed().as_micros() as u64;
                error!("[Physics] Write lock contention exceeded {}μs. Triggering Sentinel alert.", INVARIANTS.lock_acquisition_timeout_us);
                Err(RafsError::PhysicsViolation(
                    PhysicsViolationError::LockTimeout { 
                        limit: INVARIANTS.lock_acquisition_timeout_us 
                    }
                ))
            }
        }
    }
}