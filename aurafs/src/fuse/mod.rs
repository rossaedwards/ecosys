//! AuraFS FUSE/Dokany VFS Layer - Aurphyx LLC
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx Division 💎
//!
//! This module manages the 'Portal'—the bridge between the Fractal Lattice
//! and the host operating system. Phase II enforces T2 Coherence (1600μs).

pub mod node;
pub mod session;
pub mod timed_lock;

use crate::prelude::*;
pub use node::FileNode;
pub use session::FileSystemSession;
pub use timed_lock::AuraTimedLock;

/// [Theorem 3.1: Topological Stability]
/// The AuraFsDriver trait abstracts the underlying kernel driver (Dokany/FUSE).
/// Every implementation MUST respect the T₂ coherence window (physics::INVARIANTS.coherence_window_us).
#[async_trait::async_trait]
pub trait AuraFsDriver: Send + Sync {
    /// Mounts the filesystem portal to the specified path.
    async fn mount(&self, mountpoint: &std::path::Path) -> Result<()>;

    /// Unmounts the portal and flushes all pending Aura-Shards.
    async fn unmount(&self) -> Result<()>;

    /// Performs a coherence check across the local lattice segment.
    fn check_coherence(&self) -> Result<f64> {
        // Default implementation checks the ds baseline via physics::INVARIANTS
        Ok(INVARIANTS.spectral_dimension)
    }
}

/// Initializes the VFS layer with Windows-specific Dokany configurations.
pub fn init_vfs(config: &crate::config::AuraConfig) -> Result<Arc<FileSystemSession>> {
    info!("[VFS] Initializing Portal on {} via {} driver.", 
        config.system.os_target, 
        config.system.fs_driver
    );

    // Verify system paths exist for Dokany logging and mounting
    if !config.system.paths.data_dir.exists() {
        warn!("[VFS] Data directory missing. Provisioning: {:?}", config.system.paths.data_dir);
        std::fs::create_dir_all(&config.system.paths.data_dir)?;
    }

    let session = FileSystemSession::new(config.clone());
    Ok(Arc::new(session))
}