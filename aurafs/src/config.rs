//! AuraFS Configuration Manager - Aurphyx LLC
//! Handles Phase II Physics Invariants and Tiered Hardware Mapping.

use crate::physics::INVARIANTS;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraConfig {
    pub package: PackageConfig,
    pub physics: PhysicsConfig,
    pub sharding: ShardingConfig,
    pub network: NetworkConfig,
    pub crypto: CryptoConfig,
    pub system: SystemConfig,
    pub governance: GovernanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
}

/// [Theorem 2.1 & 3.1 Compliance]
/// Invariant physics constants governing the Fractal Lattice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    #[serde(default = "default_scaling_bias")]
    pub fractal_scaling_bias: f64,
    #[serde(default = "default_spectral_dimension")]
    pub spectral_dimension: f64,
    #[serde(default = "default_coherence_window")]
    pub coherence_window_us: u64,
    #[serde(default = "default_pbg")]
    pub photonic_band_gap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardingConfig {
    pub default_chunk_size: String,
    pub min_replicas_base: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub p2p_port: u16,
    pub tiers: NetworkTiers,
}

/// Hardware Tier Definitions for GhostLink, DataSlayer, and Titan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTiers {
    pub tier_1: TierConfig,
    pub tier_2: TierConfig,
    pub tier_3: TierConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    pub cost_usd: f64,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub sentinel_count: usize,
    pub threat_detection_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    pub kem_algorithm: String,
    pub sig_algorithm: String,
    pub identity_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub os_target: String,
    pub fs_driver: String,
    pub paths: SystemPaths,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPaths {
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub logs_dir: PathBuf,
}

// Default Physics Values (via physics::INVARIANTS singleton — aurafs.toml source of truth)
fn default_scaling_bias() -> f64 { INVARIANTS.hilbert_scaling_bias }
fn default_spectral_dimension() -> f64 { INVARIANTS.spectral_dimension }
fn default_coherence_window() -> u64 { INVARIANTS.coherence_window_us }
fn default_pbg() -> f64 { INVARIANTS.photonic_band_gap }

impl AuraConfig {
    /// Loads the master configuration from aurafs.toml
    pub fn load() -> Result<Self> {
        let config_path = "aurafs.toml";
        let content = std::fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config at {}", config_path))?;
        
        let config: AuraConfig = toml::from_str(&content)
            .with_context(|| "Failed to parse aurafs.toml physics/tier sections")?;
        
        Ok(config)
    }

    /// Verifies if the current node has the computational density for S.A.G.E.S.
    pub fn is_titan_node(&self) -> bool {
        self.network.tiers.tier_3.mode == "titan-isp"
    }
}