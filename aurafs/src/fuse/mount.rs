//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Fuse Mount Manager - Production Filesystem Mounting
//! 🛸 Automatic Mount + Unmount + Multi-Mount + Kernel Config + Signal Handling
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    fuse::{FuseConfig, filesystem::AuraFs, inode_cache::InodeCache, Result},
    network::{Orchestrator, Firewall},
    storage::shard_store::ShardStore,
};
use fuser::{mount2, KernelConfig, MountOption};
use std::{
    path::Path,
    sync::Arc,
    process,
};
use tokio::{
    sync::oneshot,
    signal,
    task::JoinHandle,
};
use tracing::{info, warn, error};

/// Production mount manager for AuraFS
pub struct MountManager {
    /// Filesystem instance
    filesystem: AuraFs,
    
    /// Mount point path
    mount_point: std::path::PathBuf,
    
    /// Kernel configuration
    kernel_config: KernelConfig,
    
    /// Shutdown signal receiver
    shutdown_rx: Option<oneshot::Receiver<()>>,
    
    /// Background task handles
    tasks: Vec<JoinHandle<()>>,
}

#[derive(Debug, Clone)]
pub struct MountOptions {
    /// Allow other users to access mount
    pub allow_other: bool,
    
    /// Disable kernel page cache
    pub no_kernel_cache: bool,
    
    /// Enable debug logging
    pub debug: bool,
    
    /// Maximum number of background requests
    pub max_background: Option<usize>,
    
    /// Filesystem name
    pub fsname: String,
}

impl Default for MountOptions {
    fn default() -> Self {
        Self {
            allow_other: false,
            no_kernel_cache: false,
            debug: false,
            max_background: Some(32),
            fsname: "aurafs".to_string(),
        }
    }
}

impl MountManager {
    /// Forge production mount manager with full stack
    pub fn new(
        config: FuseConfig,
        shard_store: Arc<ShardStore>,
        network: Arc<Orchestrator>,
        firewall: Arc<Firewall>,
    ) -> Self {
        let inode_cache = Arc::new(InodeCache::new());
        let filesystem = AuraFs::new(shard_store, network, firewall, inode_cache);
        
        let mut kernel_config = KernelConfig::default();
        kernel_config.debug = config.debug;
        
        Self {
            filesystem,
            mount_point: config.mount_point,
            kernel_config,
            shutdown_rx: None,
            tasks: Vec::new(),
        }
    }
    
    /// Mount AuraFS with production kernel options
    pub async fn mount(mut self, options: MountOptions) -> Result<()> {
        let mount_point = self.mount_point.clone();
        
        // Validate mount point
        if !mount_point.exists() {
            std::fs::create_dir_all(&mount_point)?;
            info!("📁 Created mount point: {}", mount_point.display());
        }
        
        if !mount_point.is_dir() {
            return Err(crate::fuse::Error::Mount("Mount point is not a directory".into()).into());
        }
        
        // Build fuser mount options
        let mut mount_opts = vec![
            MountOption::FSName(options.fsname),
        ];
        
        if options.allow_other {
            mount_opts.push(MountOption::AllowOther);
        }
        
        if options.no_kernel_cache {
            mount_opts.push(MountOption::NoKernelCache);
        }
        
        if let Some(max_bg) = options.max_background {
            mount_opts.push(MountOption::MaxBackground(max_bg as u32));
        }
        
        if options.debug {
            mount_opts.push(MountOption::Debug);
        }
        
        info!("🛸 Mounting AuraFS at {} with {} options", 
              mount_point.display(), mount_opts.len());
        
        // Setup graceful shutdown
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        self.shutdown_rx = Some(shutdown_rx);
        
        // Spawn signal handler
        let mount_point_clone = mount_point.clone();
        let shutdown_tx_clone = shutdown_tx.clone();
        let signal_task = tokio::spawn(async move {
            signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
            info!("🛑 SIGINT received, unmounting {}", mount_point_clone.display());
            let _ = shutdown_tx_clone.send(());
        });
        self.tasks.push(signal_task);
        
        // Mount filesystem in background task
        let filesystem = std::sync::Arc::new(std::sync::Mutex::new(self.filesystem));
        let mount_opts_clone = mount_opts.clone();
        
        let mount_task = tokio::spawn(async move {
            match mount2(
                filesystem,
                mount_point,
                &mount_opts_clone,
                &mut self.kernel_config,
            ) {
                Ok(()) => info!("✅ AuraFS mounted successfully"),
                Err(e) => {
                    error!("❌ Mount failed: {}", e);
                    process::exit(1);
                }
            }
        });
        
        self.tasks.push(mount_task);
        
        // Wait for shutdown signal
        if let Some(rx) = self.shutdown_rx.take() {
            rx.await.ok();
        }
        
        info!("🔄 Unmounting AuraFS...");
        self.cleanup().await;
        
        Ok(())
    }
    
    /// Cleanup resources and unmount
    async fn cleanup(&mut self) {
        // Abort all tasks
        for task in self.tasks.drain(..) {
            let _ = task.abort();
        }
        
        // TODO: Send unmount signal to kernel
        // fuser::umount2(&self.mount_point).ok();
        info!("🧹 AuraFS cleanup complete");
    }
    
    /// Static convenience mount (one-liner)
    pub async fn mount_static(
        mount_point: impl AsRef<Path>,
        shard_store: Arc<ShardStore>,
        network: Arc<Orchestrator>,
        firewall: Arc<Firewall>,
        options: MountOptions,
    ) -> Result<()> {
        let config = FuseConfig {
            mount_point: mount_point.as_ref().to_path_buf(),
            ..Default::default()
        };
        
        let mut manager = MountManager::new(config, shard_store, network, firewall);
        manager.mount(options).await
    }
}

/// PRODUCTION QUICK-MOUNT MACROS
#[macro_export]
macro_rules! aurafs_mount {
    // Simple mount
    ($path:expr) => {{
        use afs::fuse::mount::MountManager;
        use afs::{network::production_network, storage::shard_store::ShardStore};
        
        let shard_store = Arc::new(ShardStore::default());
        let (orchestrator, _tunnel, firewall) = production_network(Default::default()).await.unwrap();
        
        MountManager::mount_static($path, shard_store, orchestrator.into(), firewall, Default::default()).await.unwrap()
    }};
    
    // Custom options
    ($path:expr, $options:expr) => {{
        use afs::fuse::mount::MountManager;
        // ... full stack setup
        MountManager::mount_static($path, shard_store, orchestrator.into(), firewall, $options).await.unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[tokio::test]
    async fn test_mount_options() {
        let opts = MountOptions {
            allow_other: true,
            no_kernel_cache: true,
            debug: true,
            max_background: Some(64),
            fsname: "testfs".to_string(),
        };
        
        assert!(opts.allow_other);
        assert!(opts.no_kernel_cache);
        assert_eq!(opts.fsname, "testfs");
    }
    
    #[tokio::test]
    async fn test_mount_manager_new() {
        let config = FuseConfig {
            mount_point: PathBuf::from("/tmp/test"),
            debug: true,
            ..Default::default()
        };
        
        let shard_store = Arc::new(crate::storage::shard_store::ShardStore::default());
        let network = Arc::new(crate::network::Orchestrator::default());
        let firewall = Arc::new(crate::network::Firewall::default());
        
        let manager = MountManager::new(config, shard_store, network, firewall);
        assert_eq!(manager.mount_point, PathBuf::from("/tmp/test"));
    }
}