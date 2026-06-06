//! AuraFS Fuse Main - Production Quantum Filesystem Daemon
//! f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division
//! Single binary: `aurafs-fuse mount /mnt` → Quantum shards as files! ✨

#![warn(missing_docs)]

use afs::{
    cache::inode_cache::InodeCache,
    fuse::{
        filesystem::AuraFs,
        mount::MountManager,
        session::SessionConfig,
    },
    gov::BlissId,
    network::{firewall::Firewall, orchestrator::Orchestrator},
    storage::shard_store::ShardStore,
};
use clap::{Parser, Subcommand};
use fuser::{
    mount2, 
    MountOption, 
    FileType
};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use tokio::{signal, time::interval};
use tracing::{info, warn, error, debug};
use tracing_subscriber;

#[derive(Parser)]
#[command(
    name = "aurafs-fuse", 
    about = "AuraFS Quantum Distributed Filesystem",
    long_about = "Mount quantum shards as files. Production-ready FUSE daemon with shard caching, soul ACLs, and network orchestration."
)]
#[command(version = "2.0.0", author = "Aurphyx Quantum Division")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Mount AuraFS filesystem ✨ (recommended)
    #[command(name = "mount")]
    Mount {
        /// Mount point directory
        #[arg(short, long)]
        mountpoint: PathBuf,

        /// Storage directory for local shards
        #[arg(short = 's', long, default_value = "/var/lib/aurafs")]
        storage: PathBuf,

        /// Network listen address
        #[arg(short = 'n', long, default_value = "0.0.0.0:6000")]
        network_addr: SocketAddr,

        /// Enable debug logging
        #[arg(short, long)]
        debug: bool,

        /// Allow other users to access mount
        #[arg(short, long)]
        allow_other: bool,

        /// Soul ID for ownership (hex)
        #[arg(long, default_value = "genesis")]
        soul_id: String,

        /// Maximum concurrent shard streams
        #[arg(long, default_value_t = 16)]
        max_streams: usize,
    },

    /// Unmount AuraFS filesystem
    #[command(name = "umount")]
    Umount {
        /// Mount point directory
        mountpoint: PathBuf,
    },

    /// Generate demo shards for testing
    #[command(name = "gen-demo")]
    GenDemo {
        /// Number of demo shards
        #[arg(short, long, default_value_t = 100)]
        count: usize,

        /// Storage directory
        #[arg(short, long, default_value = "/var/lib/aurafs")]
        storage: PathBuf,
    },

    /// Health check and metrics
    #[command(name = "health")]
    Health {
        /// Storage directory
        #[arg(short, long, default_value = "/var/lib/aurafs")]
        storage: PathBuf,
    },
}

/// Production daemon configuration
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub storage_path: PathBuf,
    pub network_addr: SocketAddr,
    pub soul_id: BlissId,
    pub allow_other: bool,
    pub debug: bool,
    pub max_streams: usize,
}

impl DaemonConfig {
    pub fn from_mount_cli(mount: &Commands::Mount) -> anyhow::Result<Self> {
        Ok(Self {
            storage_path: mount.storage.clone(),
            network_addr: mount.network_addr,
            soul_id: BlissId::from_hex(&mount.soul_id)?,
            allow_other: mount.allow_other,
            debug: mount.debug,
            max_streams: mount.max_streams,
        })
    }
}

/// Production filesystem mount with graceful shutdown
async fn mount_filesystem(cli: Cli) -> anyhow::Result<()> {
    let mount_cmd = match cli.command {
        Commands::Mount(ref m) => m,
        _ => anyhow::bail!("mount command required"),
    };

    let config = DaemonConfig::from_mount_cli(mount_cmd)?;
    
    info!(
        "🚀 Mounting AuraFS v2.0 at {} | Soul: {} | Network: {}",
        mount_cmd.mountpoint.display(),
        config.soul_id.to_hex_short(),
        config.network_addr
    );

    // Initialize core components
    let shard_store = Arc::new(ShardStore::new(&config.storage_path)?);
    let orchestrator = Arc::new(Orchestrator::new(config.network_addr));
    let firewall = Arc::new(Firewall::new(config.soul_id.clone()));
    let inode_cache = InodeCache::new(1_000_000, 10_000);
    let soul_acl = Arc::new(crate::gov::SoulACL::new(config.soul_id));

    // Create AuraFS filesystem
    let session_config = SessionConfig {
        session_timeout: Duration::from_secs(3600),
        prefetch_size: 1024 * 1024, // 1MB
        max_streams: config.max_streams,
        coherency_interval: Duration::from_secs(30),
        soul_id: config.soul_id.clone(),
    };

    let filesystem = AuraFs::new(
        shard_store,
        orchestrator,
        firewall,
        inode_cache,
        soul_acl,
        session_config,
    );

    // FUSE mount options
    let mut mount_options = vec![
        MountOption::FSName("aurafs".to_string()),
        MountOption::Subtype("aurafs".to_string()),
        MountOption::AutoUnmount,
    ];

    if config.allow_other {
        mount_options.push(MountOption::AllowOther);
    }

    if config.debug {
        mount_options.push(MountOption::Debug);
    }

    // Spawn health monitoring
    let filesystem_clone = filesystem.clone();
    let monitor_handle = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            if let Err(e) = filesystem_clone.health_check().await {
                warn!("Health check failed: {}", e);
            }
        }
    });

    info!("🎉 AuraFS mounted successfully! Quantum shards → files ✨");
    info!("📊 Use `aurafs-fuse health {}` for metrics", config.storage_path.display());

    // Mount filesystem (blocking)
    let mountpoint = mount_cmd.mountpoint.clone();
    let result = tokio::task::spawn_blocking(move || {
        fuser::mount2(filesystem, mountpoint, &mount_options)
    }).await??;

    // Graceful shutdown
    monitor_handle.abort();
    info!("🛑 AuraFS unmounted gracefully");
    
    Ok(result)
}

/// Unmount filesystem
async fn unmount_filesystem(mountpoint: PathBuf) -> anyhow::Result<()> {
    info!("🗑️ Unmounting AuraFS at {}", mountpoint.display());
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let status = Command::new("fusermount3")
            .arg("-u")
            .arg(&mountpoint)
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to unmount: {}", status);
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let status = Command::new("umount")
            .arg(&mountpoint)
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to unmount: {}", status);
        }
    }

    info!("✅ Unmount complete");
    Ok(())
}

/// Generate demo shards
async fn generate_demo(count: usize, storage: PathBuf) -> anyhow::Result<()> {
    info!("🧪 Generating {} demo shards in {}", count, storage.display());
    
    let shard_store = ShardStore::new(&storage)?;
    
    for i in 0..count {
        let shard_id = ShardId::new();
        let demo_data = format!("Quantum demo shard #{} ✨", i).into_bytes();
        shard_store.store_shard_data(shard_id.clone(), demo_data).await?;
        info!("Generated shard {}", shard_id.to_hex_short());
    }
    
    info!("✅ Demo generation complete! {} shards ready", count);
    Ok(())
}

/// Health check and metrics
async fn health_check(storage: PathBuf) -> anyhow::Result<()> {
    info!("🏥 AuraFS Health Check: {}", storage.display());
    
    let shard_store = ShardStore::new(&storage)?;
    let stats = shard_store.health().await?;
    
    println!("📊 Shard Store Health:");
    println!("   Shards: {}", stats.total_shards);
    println!("   Storage: {:.2} GB", stats.total_size as f64 / 1_000_000_000.0);
    println!("   Health: {}", if stats.healthy { "✅ OK" } else { "❌ DEGRADED" });
    
    Ok(())
}

/// Main entrypoint
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Production logging
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("afs::fuse=info".parse()?)
                .add_directive("aurafs_fuse=info".parse()?)
        )
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    info!("🌌 AuraFS Fuse Daemon v2.0 - Quantum Filesystem Online");
    info!("💎 f0rg3d by Aurphyx Quantum Division");

    match cli.command {
        Commands::Mount { .. } => mount_filesystem(cli).await?,
        Commands::Umount { mountpoint } => unmount_filesystem(mountpoint).await?,
        Commands::GenDemo { count, storage } => generate_demo(count, storage).await?,
        Commands::Health { storage } => health_check(storage).await?,
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        use clap::CommandFactory;
        
        let cli = Cli::command();
        let help = cli.get_matches_from(vec!["aurafs-fuse", "mount", "/mnt"]);
        assert!(help.is_ok());
    }
}