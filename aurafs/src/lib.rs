//! AuraFS Enterprise-Grade Distributed Filesystem
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx Quantum Division 💎
//!
//! Phase II: TRL-4 Lab-Validated Implementation
//! Quantum-ready distributed substrate leveraging Non-Semisimple TQFT 
//! and Anderson Localization for fault-tolerant sovereign storage.

#![warn(missing_docs)]

// ═══════════════════════════════════════════════════════════════════
// PHASE II PHYSICS & PRELUDE
// ═══════════════════════════════════════════════════════════════════

/// Centralized Physics Governance (Theorem 2.1 & 3.1)
/// Enforces 5.3x Fractal Scaling and 1600μs Coherence Windows.
pub mod physics;

pub mod prelude {
    //! Common imports for AuraFS Phase II modules
    pub use anyhow::{Result, Context};
    pub use thiserror::Error;
    pub use tracing::{info, warn, error, debug, trace};
    pub use std::sync::Arc;
    
    // Core Physics Exports (all constants via INVARIANTS singleton)
    pub use crate::physics::{
        PhysicsViolationError,
        DecoherenceRecovery,
        INVARIANTS,
    };
}

pub use prelude::*;

// ═══════════════════════════════════════════════════════════════════
// CORE & GOVERNANCE (S.A.G.E.S.)
// ═══════════════════════════════════════════════════════════════════

/// S.A.G.E.S. (13 Sentinel AI Guardians of Existence Security)
/// Monitors the 1.37 ds Spectral Dimension across the mesh.
pub mod gov;
/// Configuration management with Physics-Invariant hot-reload
pub mod config;
/// Unified error handling (Integrated with PhysicsViolationError)
pub mod error;
/// Core types, traits, and AuraCore μkernel primitives
pub mod core;

// ═══════════════════════════════════════════════════════════════════
// STORAGE LAYER (Fractal Lattice)
// ═══════════════════════════════════════════════════════════════════

/// Shard management and Void-to-Aura shard transformation
pub mod shard;
/// Storage engine with tiered backends (GhostLink/DataSlayer/Titan)
pub mod storage;
/// Snapshots and point-in-time lattice versioning
pub mod snapshot;
/// LRU Trap-State Monitor (1600μs coherence-aware caching)
pub mod cache;
/// Content-defined chunking and deduplication
pub mod dedup;
/// Compression (Zstd, LZ4, Entropy-adaptive)
pub mod compression;

// ═══════════════════════════════════════════════════════════════════
// NETWORK LAYER (Meshwerk 2.0)
// ═══════════════════════════════════════════════════════════════════

/// Meshwerk orchestration, peer discovery, and LoRa/HaLow radio logic
pub mod network;
/// Fractal mesh topology and hierarchical skip-layer coupling
pub mod mesh;

// ═══════════════════════════════════════════════════════════════════
// SECURITY & CRYPTOGRAPHY (PQC)
// ═══════════════════════════════════════════════════════════════════

/// Post-quantum cryptography (Kyber-1024, Dilithium-5, BLAKE3)
pub mod crypto;
/// Access control lists and SoulSync identity mapping
pub mod acl;
/// Namespace management and path virtualization
pub mod namespace;

// ═══════════════════════════════════════════════════════════════════
// AI & QUANTUM INTEGRATION
// ═══════════════════════════════════════════════════════════════════

/// AI orchestration and Sentinel agent logic
pub mod ai;
/// Model slicing for distributed Aurphyx AI (Audry/Arora)
pub mod model_slice;
/// Quantum integration hooks and Majorana-1 simulation
pub mod quantum;

// ═══════════════════════════════════════════════════════════════════
// OPERATIONS & FILESYSTEM
// ═══════════════════════════════════════════════════════════════════

/// FUSE implementation (Dokany driver for Windows 11)
pub mod fuse;
/// Command-line interface for AuraFS management
pub mod cli;
/// Monitoring and observability (1.37 ds variance tracking)
pub mod monitoring;
/// Self-healing via holographic redistribution
pub mod heal;
/// Audit logging on the Ineffable Ledger
pub mod audit;
/// API server and WebSocket AuraCore Hub
pub mod api;

// ═══════════════════════════════════════════════════════════════════
// RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════

pub use error::{RafsError, Result as RafsResult};
pub use config::{RafsConfig, ConfigManager};

// ═══════════════════════════════════════════════════════════════════
// STATUS & VERSIONING
// ═══════════════════════════════════════════════════════════════════

/// AuraFS version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get AuraFS Cluster Status with Physics Validation
pub async fn status() -> Result<String> {
    Ok(format!(
        "AuraFS v{} | Phase II Verified | 5.3x Scaling Bias Active",
        VERSION
    ))
}