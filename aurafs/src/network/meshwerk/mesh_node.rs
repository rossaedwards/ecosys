//! Meshwerk node profiles and tier mapping.

use crate::config::AuraConfig;
use crate::error::{RafsError, Result};
use crate::physics::{INVARIANTS, PhysicsViolationError};

/// [Theorem 3.1: Universality]
/// Hardware tier definitions for Meshwerk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshTier {
    GhostLink,
    DataSlayer,
    Titan,
}

impl MeshTier {
    /// [Theorem 3.1: Universality]
    pub fn from_mode(mode: &str) -> Self {
        match mode {
            "ghost-sensor" => MeshTier::GhostLink,
            "data-slayer" => MeshTier::DataSlayer,
            "titan-isp" => MeshTier::Titan,
            _ => MeshTier::GhostLink,
        }
    }
}

/// [Theorem 3.1: Universality]
/// Tier profile derived from `aurafs.toml`.
#[derive(Debug, Clone)]
pub struct MeshNodeProfile {
    pub tier: MeshTier,
    pub cost_usd: f64,
    pub max_hops: u8,
    pub max_latency_ms: u64,
    pub pbg_floor: f64,
}

impl MeshNodeProfile {
    /// [Theorem 3.1: Universality]
    pub fn from_config(config: &AuraConfig, tier: MeshTier) -> Self {
        let tier_cfg = match tier {
            MeshTier::GhostLink => &config.network.tiers.tier_1,
            MeshTier::DataSlayer => &config.network.tiers.tier_2,
            MeshTier::Titan => &config.network.tiers.tier_3,
        };
        let (max_hops, max_latency_ms) = match tier {
            MeshTier::GhostLink => (8, 2500),
            MeshTier::DataSlayer => (16, 1000),
            MeshTier::Titan => (32, 250),
        };

        Self {
            tier,
            cost_usd: tier_cfg.cost_usd,
            max_hops,
            max_latency_ms,
            pbg_floor: INVARIANTS.photonic_band_gap,
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn ensure_pbg(&self, overhead_ratio: f64) -> Result<()> {
        if overhead_ratio < self.pbg_floor {
            return Err(RafsError::PhysicsViolation(
                PhysicsViolationError::CrosstalkBreach { pbg: overhead_ratio },
            ));
        }
        Ok(())
    }
}

/// [Theorem 3.1: Universality]
/// Mesh node with tier profile applied.
#[derive(Debug, Clone)]
pub struct MeshNode {
    pub node_id: String,
    pub profile: MeshNodeProfile,
}

impl MeshNode {
    /// [Theorem 3.1: Universality]
    pub fn new(node_id: String, profile: MeshNodeProfile) -> Self {
        Self { node_id, profile }
    }
}