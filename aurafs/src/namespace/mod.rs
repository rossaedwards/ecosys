//! ═══════════════════════════════════════════════════════════════════
//! 📁 AuraFS Namespace Module - Fractal Pathspace Orchestrator
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx LLC 💎
//!
//! Provides hierarchical namespace management on top of the fractal shard
//! filesystem, re-exporting the NamespaceManager and related types for use
//! across CLI, FUSE, and core services.
//!
//! ## Features
//! - Virtual directory management
//! - Path mapping to shards
//! - Metadata management
//! - Persistence to disk
//! ═══════════════════════════════════════════════════════════════════

pub mod manager;

use std::sync::Arc;
use tokio::sync::RwLock;
use manager::NamespaceManager;

/// Shared async-safe namespace handle used by services.
pub type SharedNamespace = Arc<RwLock<NamespaceManager>>;

/// Create a new shared namespace manager rooted at the given data directory.
pub fn new_shared_namespace(data_dir: &str) -> Result<SharedNamespace, manager::NamespaceError> {
    let mgr = NamespaceManager::new(data_dir)?;
    Ok(Arc::new(RwLock::new(mgr)))
}

// Convenient re-exports so other modules can just `use crate::namespace::*;`
pub use manager::{
    NamespaceError,
    Result as NamespaceResult,
    NamespaceEntry,
    EntryType,
};