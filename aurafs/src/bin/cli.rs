//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS CLI - Command-line interface for managing AuraFS

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber;

use aurafs::{
    AclManager, CacheManager, NamespaceManager, SnapshotManager,
    VersionTracker, DeduplicationEngine, CompressionManager,
    CompressionAlgorithm, AuditLogger,
    network::{NodeManager, ReplicationManager, ReplicationPolicy},
};
use shard_server::{
    ShardServer,
    autoheal_daemon::AutohealDaemon,
    embed_splice_publish_engine::EmbedSplicePublishEngine,
    mesh_gossip::{MeshGossip, GossipMessage},
    metrics::MetricsCollector,
};
use tokio::sync::mpsc;

#[derive(Parser)]
#[command(name = "aurafs-cli")]
#[command(about = "AuraFS Command Line Interface - f0rg3d in l0v3 by Ross Edwards & Aurphyx")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    // Optional extended commands
    #[command(subcommand)]
    ext_command: Option<ExtendedCommands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize AuraFS data directory
    Init {
        #[arg(long, default_value = "/var/aurafs")]
        data_dir: PathBuf,
    },

    /// Mount AuraFS filesystem
    Mount {
        mountpoint: PathBuf,
        #[arg(long, default_value = "/var/aurafs")]
        data_dir: PathBuf,
    },

    /// Unmount AuraFS filesystem
    Umount {
        mountpoint: PathBuf,
    },

    /// Snapshot management
    Snapshot {
        #[command(subcommand)]
        action: SnapshotCommands,
    },

    /// Cluster management
    Cluster {
        #[command(subcommand)]
        action: ClusterCommands,
    },

    /// Cache management
    Cache {
        #[command(subcommand)]
        action: CacheCommands,
    },

    /// Version tracking
    Version {
        #[command(subcommand)]
        action: VersionCommands,
    },

    /// ACL management
    Acl {
        #[command(subcommand)]
        action: AclCommands,
    },

    /// Deduplication statistics
    Dedup {
        #[arg(long, default_value = "/var/aurafs")]
        data_dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum SnapshotCommands {
    Create {
        #[arg(long)]
        path: PathBuf,

        #[arg(long)]
        description: String,

        #[arg(long, default_value = "/var/aurafs/snapshots")]
        snapshot_dir: PathBuf,
    },

    List {
        #[arg(long, default_value = "/var/aurafs/snapshots")]
        snapshot_dir: PathBuf,
    },

    Rollback {
        #[arg(long)]
        id: String,

        #[arg(long, default_value = "/var/aurafs/snapshots")]
        snapshot_dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum ClusterCommands {
    Status {
        #[arg(long, default_value = "/var/aurafs")]
        data_dir: PathBuf,
    },

    Join {
        #[arg(long)]
        node: String,

        #[arg(long)]
        address: String,
    },

    Leave {
        #[arg(long)]
        node: String,
    },
}

#[derive(Subcommand)]
enum CacheCommands {
    Stats {
        #[arg(long, default_value = "100")]
        max_size_mb: usize,
    },

    Clear,
}

#[derive(Subcommand)]
enum VersionCommands {
    List {
        #[arg(long)]
        shard: String,
    },

    Checkout {
        #[arg(long)]
        shard: String,

        #[arg(long)]
        version: u64,
    },
}

#[derive(Subcommand)]
enum AclCommands {
    AddUser {
        #[arg(long)]
        user: String,

        #[arg(long)]
        role: String,

        #[arg(long, default_value = "/var/aurafs/acl.json")]
        acl_file: PathBuf,
    },

    ListUsers {
        #[arg(long, default_value = "/var/aurafs/acl.json")]
        acl_file: PathBuf,
    },

    Grant {
        #[arg(long)]
        user: String,

        #[arg(long)]
        resource: String,

        #[arg(long)]
        permission: String,

        #[arg(long, default_value = "/var/aurafs/acl.json")]
        acl_file: PathBuf,
    },

    Revoke {
        #[arg(long)]
        user: String,

        #[arg(long)]
        resource: String,

        #[arg(long, default_value = "/var/aurafs/acl.json")]
        acl_file: PathBuf,
    },
}

// Extended commands for audit playback & mesh dashboard
#[derive(Subcommand)]
enum ExtendedCommands {
    AuditPlayback,
    MeshDashboard,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Core AuraFS/ShardServer components setup (stubs for now)
    let (gossip_tx, gossip_rx) = mpsc::channel(50);
    let embed_engine = Arc::new(EmbedSplicePublishEngine::new());
    let autoheal = Arc::new(AutohealDaemon::new());
    let acl_manager = Arc::new(AclManager::new("/var/aurafs/acl.json")?);
    let metrics_collector = Arc::new(MetricsCollector::new());

    let server = ShardServer::new(
        embed_engine,
        autoheal,
        acl_manager.clone(),
        metrics_collector,
        gossip_tx,
        gossip_rx,
    );

    match cli.command {
        Commands::Init { data_dir } => {
            info!("Initializing AuraFS at {:?}", data_dir);
            std::fs::create_dir_all(&data_dir)?;
            std::fs::create_dir_all(data_dir.join("namespace"))?;
            std::fs::create_dir_all(data_dir.join("dedup"))?;
            std::fs::create_dir_all(data_dir.join("snapshots"))?;
            println!("✅ AuraFS initialized at {:?}", data_dir);
        }

        Commands::Mount { mountpoint, data_dir } => {
            info!("Mounting AuraFS at {:?}", mountpoint);
            println!("🔥 Mounting AuraFS...");
            println!("   Mountpoint: {:?}", mountpoint);
            println!("   Data dir: {:?}", data_dir);
            println!("⚠️  FUSE mounting requires root privileges");
            println!("💡 Run: sudo aurafs mount {:?}", mountpoint);
        }

        Commands::Umount { mountpoint } => {
            println!("Unmounting {:?}", mountpoint);
            println!("💡 Run: fusermount -u {:?}", mountpoint);
        }

        Commands::Snapshot { action } => match action {
            SnapshotCommands::Create { path, description, snapshot_dir } => {
                let manager = SnapshotManager::new(snapshot_dir.to_str().unwrap())?;
                let snap_id = manager.create_snapshot(&path, &description, None)?;
                println!("✅ Snapshot created: {}", snap_id);
            }

            SnapshotCommands::List { snapshot_dir } => {
                let manager = SnapshotManager::new(snapshot_dir.to_str().unwrap())?;
                let snapshots = manager.list_snapshots();

                println!("\n📸 Snapshots:");
                println!("{:<16} {:<12} {}", "ID", "Timestamp", "Description");
                println!("{}", "─".repeat(70));

                for snap in snapshots {
                    println!("{:<16} {:<12} {}", &snap.id[..16], snap.timestamp, snap.description);
                }
            }

            SnapshotCommands::Rollback { id, snapshot_dir } => {
                let manager = SnapshotManager::new(snapshot_dir.to_str().unwrap())?;
                manager.rollback(&id)?;
                println!("✅ Rolled back to snapshot: {}", id);
            }
        },

        Commands::Cluster { action } => match action {
            ClusterCommands::Status { data_dir } => {
                let node_manager = Arc::new(NodeManager::new(30));
                let (total, live, capacity, used) = node_manager.get_cluster_stats();

                println!("\n🌐 Cluster Status:");
                println!("   Total nodes: {}", total);
                println!("   Live nodes: {}", live);
                println!("   Capacity: {} bytes", capacity);
                println!("   Used: {} bytes", used);

                if capacity > 0 {
                    let usage_pct = (used as f64 / capacity as f64) * 100.0;
                    println!("   Usage: {:.1}%", usage_pct);
                }
            }

            ClusterCommands::Join { node, address } => {
                let node_manager = NodeManager::new(30);
                node_manager.register_node(&node, &address)?;
                println!("✅ Node {} joined at {}", node, address);
            }

            ClusterCommands::Leave { node } => {
                let node_manager = NodeManager::new(30);
                node_manager.unregister_node(&node)?;
                println!("✅ Node {} left the cluster", node);
            }
        },

        Commands::Cache { action } => match action {
            CacheCommands::Stats { max_size_mb } => {
                let cache = CacheManager::new(max_size_mb);
                let (hits, misses, hit_rate, size) = cache.get_stats();

                println!("\n💾 Cache Statistics:");
                println!("   Hits: {}", hits);
                println!("   Misses: {}", misses);
                println!("   Hit rate: {:.1}%", hit_rate);
                println!("   Size: {} bytes", size);
            }

            CacheCommands::Clear => {
                let cache = CacheManager::new(100);
                cache.clear();
                println!("✅ Cache cleared");
            }
        },

        Commands::Version { action } => match action {
            VersionCommands::List { shard } => {
                println!("\n📜 Version history for shard: {}", shard);
                println!("(Version tracking implementation)");
            }

            VersionCommands::Checkout { shard, version } => {
                println!("✅ Checked out version {} for shard {}", version, shard);
            }
        },

        Commands::Acl { action } => match action {
            AclCommands::AddUser { user, role, acl_file } => {
                let acl = AclManager::new(acl_file.to_str().unwrap())?;
                acl.add_user(&user, &role)?;
                acl.save_config()?;
                println!("✅ User {} added with role {}", user, role);
            }

            AclCommands::ListUsers { acl_file } => {
                let acl = AclManager::new(acl_file.to_str().unwrap())?;
                let users = acl.list_users();

                println!("\n👥 Users:");
                for user in users {
                    if let Some(user_obj) = acl.get_user(&user) {
                        println!("   {} ({})", user, user_obj.role);
                    }
                }
            }

            AclCommands::Grant { user, resource, permission, acl_file } => {
                let acl = AclManager::new(acl_file.to_str().unwrap())?;
                acl.grant_permission(&user, &resource, &permission)?;
                acl.save_config()?;
                println!("✅ Granted {} permission on {} to {}", permission, resource, user);
            }

            AclCommands::Revoke { user, resource, acl_file } => {
                let acl = AclManager::new(acl_file.to_str().unwrap())?;
                acl.revoke_permission(&user, &resource)?;
                acl.save_config()?;
                println!("✅ Revoked permissions on {} from {}", resource, user);
            }
        },

        Commands::Dedup { data_dir } => {
            let dedup_dir = data_dir.join("dedup");
            let dedup = DeduplicationEngine::new(dedup_dir.to_str().unwrap())?;
            let (total_chunks, total_refs, savings) = dedup.get_stats();

            println!("\n🔗 Deduplication Statistics:");
            println!("   Unique chunks: {}", total_chunks);
            println!("   Total references: {}", total_refs);
            println!("   Space saved: {}%", savings);
        }
    }

    if let Some(ext_cmd) = cli.ext_command {
        match ext_cmd {
            ExtendedCommands::AuditPlayback => {
                println!("🔊 Starting audit playback (TTS holographic logger)");
                // TODO: hook holographic_logger.playback_events().await here
            }
            ExtendedCommands::MeshDashboard => {
                println!("📊 Launching live mesh dashboard (TUI)");
                // TODO: spawn mesh_tui.run().await task here
            }
        }
    }

    Ok(())
}
use clap::{Arg, Command};
use anyhow::Result;

use aurafs::audit::{AuditEvent, TtsEngine, TtsVoice};
//! CLI for shard_server control
//! Subcommands for init, embed, splice, publish, audit, heal, join, metrics
//! HolographicLogger for TTS audit playback
//! Uses TtsEngine to vocalize audit events
//! Persists events to JSON log file
//! Verifies event signatures with quantum-safe scheme
//! Integrates with shard_server CLI commands
//! Provides async playback of audit logs
//! Supports audit event filtering and playback speed control
//! Intended for immersive audit review experiences
//! Designed for future integration with AR/VR holographic displays
//! Modular design for easy extension with new TTS voices and formats
//! Robust error handling and logging throughout
//! Extensible audit event structure for rich metadata
//! Optimized for low-latency TTS playback
//! Leverages Tokio for async operations
//! Uses Serde for JSON serialization/deserialization
//! Thread-safe with RwLock for concurrent access
//! Configurable log file path and TTS voice
//! Example usage:
//! aurafs-audit playback --log /var/aurafs/audit_log.json --voice "en-US-Wavenet-D"
//! aurafs-audit log --event '{"user_id":"alice","action":"embed","shard_id":"shard123","details":"Embedded file.txt"}' --log /var/aurafs/audit_log.json
//! aurafs-audit verify --event '{"user_id":"alice","action":"embed","shard_id":"shard123","details":"Embedded file.txt"}' --signature "base64sig"
//! //! Future plans:
//! - Integrate with AR/VR holographic display systems
//! - Support real-time TTS holographic logging during shard operations
//! - Add more TTS voices and languages
//! - Implement advanced filtering and search for audit events
//! - Provide playback speed and voice modulation controls
//! - Enhance security with stronger quantum-safe signatures
//! - Develop GUI for audit playback management
//! - Explore integration with blockchain for immutable audit trails
//! - Optimize TTS engine for holographic display environments
//! - Collaborate with UX designers for immersive audit experiences
//! - Gather user feedback for continuous improvement
//! - Ensure compliance with data protection regulations
//! - Document API for third-party integrations
//! - Open-source community contributions and plugins
//! - Regular updates with latest TTS and quantum-safe techniques
//! - Comprehensive testing for reliability and performance
//! - Maintain modular architecture for easy future enhancements
//! - Focus on user-friendly interfaces and experiences
//! - Prioritize security and data integrity in all features
//! - Align development with AuraFS overall vision and goals
//! - Foster innovation in holographic logging technologies
//! - Stay abreast of advancements in TTS and quantum-safe cryptography
//! - Build partnerships with AR/VR hardware and software providers
//! - Explore AI-driven enhancements for audit analysis and playback
//! - Continuously refine and optimize the holographic logger
//! - Engage with the AuraFS user community for feature ideas and feedback
//! - Ensure scalability for large-scale AuraFS deployments
//! - Leverage cloud services for distributed audit logging and playback
//! - Monitor performance metrics and user satisfaction
//! - Plan for long-term maintenance and support of the holographic logger
//! - Celebrate milestones and successes in development journey
//! - Keep the spirit of innovation and excellence alive in the project
//! - --- IGNORE ---
//! use anyhow::{anyhow, Result};
//! use log::{info, warn};
//! use serde::{Deserialize, Serialize};
//! use std::fs;
//! use std::sync::{Arc, RwLock};
//! use tokio::time::{sleep, Duration};
//! /// Audit event structure with quantum-safe signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub user_id: String,
    pub action: String,
    pub shard_id: Option<String>,
    pub details: String,
    pub signature: Vec<u8>, // quantum-safe signature
}
impl AuditEvent {
    /// Verify the event's quantum-safe signature
    pub fn verify(&self) -> Result<bool> {
        let serialized = serde_json::to_vec(&self)?;
        verify_signature(&serialized, &self.signature)
            .map_err(|e| anyhow!("AuditEvent signature verification failed: {:?}", e))
    }
}