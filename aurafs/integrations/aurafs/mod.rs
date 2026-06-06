//! AuraFS Integration Module
//! 
//! High-level API for Fuxyez ↔ AuraFS integration

mod backend;
mod shard;

pub use backend::{AuraFsBackend, AuraFsConfig, AuraFsError, Shard};
pub use shard::{ShardManager, ShardQuery};

use crate::core::lattice::{Lattice, LatticeError};

/// High-level API for lattice persistence
pub struct AuraFsPersistence {
    backend: AuraFsBackend,
}

impl AuraFsPersistence {
    /// Create new persistence layer
    pub async fn new() -> Result<Self, AuraFsError> {
        let backend = AuraFsBackend::connect().await?;
        Ok(Self { backend })
    }
    
    /// Persist lattice to AuraFS
    pub async fn save_lattice<T>(&self, lattice: &Lattice<T>) -> Result<String, LatticeError>
    where
        T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        // Delegate to lattice's built-in AuraFS persistence
        lattice.persist_to_aurafs().await
    }
    
    /// Load lattice from AuraFS
    pub async fn load_lattice<T>(&self, lattice_id: &str) -> Result<Lattice<T>, LatticeError>
    where
        T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        Lattice::load_from_aurafs(lattice_id).await
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Quick persist without managing backend
pub async fn persist<T>(lattice: &Lattice<T>) -> Result<String, LatticeError>
where
    T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    lattice.persist_to_aurafs().await
}

/// Quick load without managing backend
pub async fn load<T>(lattice_id: &str) -> Result<Lattice<T>, LatticeError>
where
    T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    Lattice::load_from_aurafs(lattice_id).await
}
    Collapse {
        sigil: String,
    },
    // Additional statements can be added here
}