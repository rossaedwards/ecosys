//! Meshwerk routing with PBG-aware safeguards.

use crate::error::{RafsError, Result};
use crate::gov::BlissId;
use crate::mesh::routing::{AdaptiveRouter, RoutingError, ShardRoute};
use crate::physics::{INVARIANTS, PhysicsViolationError};

/// [Theorem 3.1: Universality]
/// PBG-aware routing wrapper around the adaptive router.
pub struct MeshRouter {
    adaptive: AdaptiveRouter,
    pbg_floor: f64,
}

impl MeshRouter {
    /// [Theorem 3.1: Universality]
    pub fn new(adaptive: AdaptiveRouter) -> Self {
        Self {
            adaptive,
            pbg_floor: INVARIANTS.photonic_band_gap,
        }
    }

    /// [Theorem 3.1: Universality]
    /// Route to a shard and enforce the photonic band gap overhead.
    pub async fn route_to_shard(&self, shard_id: &BlissId, overhead_ratio: f64) -> Result<ShardRoute> {
        if overhead_ratio < self.pbg_floor {
            return Err(RafsError::PhysicsViolation(
                PhysicsViolationError::CrosstalkBreach { pbg: overhead_ratio },
            ));
        }
        self.adaptive
            .route_to_shard(shard_id)
            .await
            .map_err(|e| RafsError::NetworkError(format!("Routing failure: {:?}", e)))
    }

    /// [Theorem 3.1: Universality]
    pub async fn update_latency(&self, peer_id: BlissId, latency_ms: f64) {
        self.adaptive.update_latency(peer_id, latency_ms).await;
    }

    /// [Theorem 3.1: Universality]
    pub async fn record_success(&self, peer_id: BlissId) {
        self.adaptive.record_success(peer_id).await;
    }

    /// [Theorem 3.1: Universality]
    pub async fn record_failure(&self, peer_id: BlissId) {
        self.adaptive.record_failure(peer_id).await;
    }
}