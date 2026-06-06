//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Fuse Daemon - `aurafs-fuse` PRODUCTION CLI + Full Stack Mount
//! 🛸 Single Binary Quantum Filesystem + Network + Storage + Auto-Discovery
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use afs::{
    fuse::*,
    network::*,
    storage::shard_store::ShardStore,
    gov::SoulACL,
};
use clap::{Parser, Subcommand};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};
use tokio::{
    signal,
    time::{interval, Duration},
};
use tracing::{info, warn, error};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "aurafs-fuse", about = "AuraFS Quantum Distributed Filesystem")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mount AuraFS filesystem (recommended)
    Mount {
        /// Mount point directory
        #[arg(short, long)]
        mountpoint: PathBuf,
        
        /// Storage directory for local shards
        #[arg(short, long, default_value = "/var/lib/aurafs")]
        storage: PathBuf,
        
        /// Network listen address
        #[arg(short, long, default_value = "0.0.0.0:6000")]
        network_addr: SocketAddr,
        
        /// Enable debug logging
        #[arg(short, long)]
        debug: bool,
        
        /// Allow other users to access
        #[arg(short, long)]
        allow_other: bool,
    },
    
    /// Unmount AuraFS
    Unmount {
        /// Mount point directory
        mountpoint: PathBuf,
    },
    
    /// Generate test shards for demo
    GenDemo {
        /// Number of demo files
        #[arg(short, long, default_value_t = 100)]
        count: usize,
        
        /// Storage directory
        #[arg(short, long, default_value = "/var/lib/aurafs")]
        storage: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Mount {
            mountpoint,
            storage,
            network_addr,
            debug,
            allow_other,
        } => {
            info!("🛸 Starting AuraFS Fuse Daemon");
            info!("📁 Mount: {}", mountpoint.display());
            info!("💾 Storage: {}", storage.display());
            info!("🌐 Network: {}", network_addr);
            
            // 1. Forge complete production stack
            let shard_store = Arc::new(ShardStore::forge_default(storage.clone())?);
            let network_config = NetworkConfig {
                listen_addr: network_addr,
                ..Default::default()
            };
            let (orchestrator, _tunnel, firewall) = production_network(network_config).await?;
            
            // 2. Fuse configuration
            let fuse_config = FuseConfig {
                mount_point: mountpoint.clone(),
                debug,
                ..Default::default()
            };
            
            let mount_options = MountOptions {
                allow_other,
                debug,
                fsname: "aurafs".to_string(),
                ..Default::default()
            };
            
            // 3. Start network background tasks
            orchestrator.start();
            
            // 4. Mount quantum filesystem
            MountManager::mount_static(
                mountpoint,
                shard_store,
                Arc::clone(&orchestrator),
                Arc::clone(&firewall),
                mount_options,
            ).await?;
            
            info!("✅ AuraFS mounted! Shards accessible as files 🛸💎");
            info!("📊 View: ls -la /mnt/aurafs");
            info!("🛑 Unmount: Ctrl+C or fusermount -u /mnt/aurafs");
        }
        
        Commands::Unmount { mountpoint } => {
            info!("🔄 Unmounting AuraFS at {}", mountpoint.display());
            // TODO: Graceful unmount via fuser
            std::process::Command::new("fusermount")
                .args(["-u", mountpoint.to_str().unwrap()])
                .status()?;
        }
        
        Commands::GenDemo { count, storage } => {
            info!("🔮 Generating {} demo quantum shards", count);
            let shard_store = ShardStore::forge_default(storage)?;
            
            for i in 0..count {
                let shard_id = ShardId::new();
                let demo_data = format!("Quantum shard #{} - AuraFS ✨💎", i).into_bytes();
                shard_store.store_shard_data(&shard_id, &demo_data).await?;
            }
            
            info!("✅ Generated {} demo shards in {:?}", count, storage);
        }
    }
    
    Ok(())
}

/// Quick production mount (one-liner)
pub async fn quick_mount(mountpoint: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    aurafs_mount!(mountpoint);
    Ok(())
}

/// Production health monitoring task
async fn health_monitor(orchestrator: Arc<Orchestrator>) {
    let mut interval = interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        match network_health(&orchestrator).await {
            Ok(health) => {
                info!(
                    "📊 AuraFS Health | Peers: {} | Healthy: {:.1}%",
                    health.peer_count,
                    health.healthy_ratio * 100.0
                );
            }
            Err(e) => warn!("⚠️ Network health check failed: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cli_parsing() {
        let cli = Cli::parse_from([
            "aurafs-fuse",
            "mount",
            "--mountpoint", "/tmp/aurafs",
            "--storage", "/tmp/storage",
            "--network-addr", "0.0.0.0:6001",
            "--debug"
        ]);
        
        if let Commands::Mount { mountpoint, storage, network_addr, debug, .. } = cli.command {
            assert_eq!(mountpoint, PathBuf::from("/tmp/aurafs"));
            assert_eq!(storage, PathBuf::from("/tmp/storage"));
            assert_eq!(network_addr.to_string(), "0.0.0.0:6001");
            assert!(debug);
        }
    }
}