//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS CLI entrypoint binary.
//! Wires clap-based CLI to AuraFS core + shard_server mesh components.

mod cli;

use aurafs::cli::banner::{
    print_aurphyx_signature, print_banner, print_error, print_info, print_operation, 
    print_quantum_init_animation, print_success, print_warning
};
use crate::cli::{
    AclCommands, CacheCommands, Cli, ClusterCommands, Commands, ExtendedCommands,
    SnapshotCommands, VersionCommands,
};

use std::path::PathBuf;
use std::sync::Arc;

use aurafs::{
    AclManager, CacheManager, CompressionAlgorithm, CompressionManager, DeduplicationEngine,
    NamespaceManager, SnapshotManager, VersionTracker, AuditLogger,
    network::{NodeManager, ReplicationManager, ReplicationPolicy},
};

use shard_server::{
    ShardServer,
    autoheal_daemon::AutohealDaemon,
    embed_splice_publish_engine::EmbedSplicePublishEngine,
    mesh_gossip::MeshGossip,
    metrics::MetricsCollector,
};

use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    init_tracing();

    // ✨ LEGENDARY AURAFS ENTRANCE ✨
    if std::env::args().any(|arg| arg == "--no-banner") {
        print_info("Quiet mode - skipping banner");
    } else {
        print_banner();
        print_quantum_init_animation();
    }

    let cli = Cli::parse();

    // Shared core managers – in real code these would likely be built
    // from config files/env vars.
    let namespace = Arc::new(NamespaceManager::new());
    let snapshots = Arc::new(SnapshotManager::new());
    let cache = Arc::new(CacheManager::new(1024 /* MB */));
    let versions = Arc::new(VersionTracker::new());
    let acl = Arc::new(AclManager::new());
    let dedup = Arc::new(DeduplicationEngine::new());
    let compression = Arc::new(CompressionManager::new(CompressionAlgorithm::Zstd));
    let audit_logger = Arc::new(AuditLogger::new());

    // Networking / shard mesh
    let node_manager = Arc::new(NodeManager::new());
    let replication_manager = Arc::new(ReplicationManager::new(
        ReplicationPolicy::Quorum(3),
    ));

    let (gossip_tx, gossip_rx) = mpsc::channel(1024);
    let mesh_gossip = Arc::new(MeshGossip::new(gossip_tx.clone()));
    let metrics = Arc::new(MetricsCollector::new());

    let shard_server = Arc::new(ShardServer::new(
        namespace.clone(),
        snapshots.clone(),
        cache.clone(),
        versions.clone(),
        acl.clone(),
        dedup.clone(),
        compression.clone(),
        node_manager.clone(),
        replication_manager.clone(),
        mesh_gossip.clone(),
        metrics.clone(),
        audit_logger.clone(),
    ));

    // Background workers (autoheal, gossip loop, metrics, etc.)
    spawn_background_daemons(
        shard_server.clone(),
        mesh_gossip.clone(),
        metrics.clone(),
        gossip_rx,
    );

    // Core command handling
    if let Err(e) = dispatch_core_command(
        cli.command,
        shard_server.clone(),
        namespace.clone(),
        snapshots.clone(),
        cache.clone(),
        versions.clone(),
        acl.clone(),
        dedup.clone(),
        compression.clone(),
    )
    .await
    {
        print_error(&format!("Command failed: {e}"));
        std::process::exit(1);
    }

    // Optional extended commands (audit replay, mesh dashboard, etc.)
    if let Some(ext) = cli.ext_command {
        if let Err(e) = dispatch_extended_command(ext, shard_server.clone()).await {
            print_error(&format!("Extended command failed: {e}"));
            std::process::exit(1);
        }
    }

    // ✨ AURPHYX EXIT WITH FLAIR ✨
    print_aurphyx_signature();
}

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Failed to init tracing subscriber: {e}");
    }
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("/var/aurafs")
}

async fn dispatch_core_command(
    cmd: Commands,
    shard_server: Arc<ShardServer>,
    namespace: Arc<NamespaceManager>,
    snapshots: Arc<SnapshotManager>,
    cache: Arc<CacheManager>,
    versions: Arc<VersionTracker>,
    acl: Arc<AclManager>,
    dedup: Arc<DeduplicationEngine>,
    compression: Arc<CompressionManager>,
) -> Result<(), String> {
    match cmd {
        Commands::Init { data_dir } => {
            let dir = data_dir_or_default(data_dir);
            print_operation("Initializing", &dir.display().to_string());
            shard_server
                .init_data_dir(&dir)
                .await
                .map_err(|e| format!("init failed: {e}"))?;
            print_success(&format!("AuraFS initialized at {}", dir.display()));
        }

        Commands::Mount { mountpoint, data_dir } => {
            let dir = data_dir_or_default(data_dir);
            print_operation("Mounting", &mountpoint.display().to_string());
            shard_server
                .mount(&mountpoint, &dir)
                .await
                .map_err(|e| format!("mount failed: {e}"))?;
            print_success(&format!("Mounted AuraFS at {}", mountpoint.display()));
        }

        Commands::Umount { mountpoint } => {
            print_operation("Unmounting", &mountpoint.display().to_string());
            shard_server
                .umount(&mountpoint)
                .await
                .map_err(|e| format!("umount failed: {e}"))?;
            print_success(&format!("Unmounted {}", mountpoint.display()));
        }

        Commands::Snapshot { action } => {
            handle_snapshot_command(action, snapshots, namespace).await?;
        }

        Commands::Cluster { action } => {
            handle_cluster_command(action, shard_server).await?;
        }

        Commands::Cache { action } => {
            handle_cache_command(action, cache).await?;
        }

        Commands::Version { action } => {
            handle_version_command(action, versions).await?;
        }

        Commands::Acl { action } => {
            handle_acl_command(action, acl).await?;
        }

        Commands::Dedup { data_dir } => {
            let dir = data_dir_or_default(data_dir);
            print_operation("Analyzing", "deduplication");
            dedup
                .print_stats(&dir)
                .await
                .map_err(|e| format!("dedup stats failed: {e}"))?;
            print_success("Deduplication analysis complete");
        }
    }

    Ok(())
}

fn data_dir_or_default(dir: PathBuf) -> PathBuf {
    if dir.as_os_str().is_empty() {
        default_data_dir()
    } else {
        dir
    }
}

// SNAPSHOT COMMANDS
async fn handle_snapshot_command(
    action: SnapshotCommands,
    snapshots: Arc<SnapshotManager>,
    namespace: Arc<NamespaceManager>,
) -> Result<(), String> {
    match action {
        SnapshotCommands::Create { path, description, snapshot_dir } => {
            print_operation("Creating snapshot", &path.display().to_string());
            let id = snapshots
                .create_snapshot(&path, &snapshot_dir, &description, namespace.clone())
                .await
                .map_err(|e| format!("snapshot create failed: {e}"))?;
            print_success(&format!("Snapshot {} created for {}", id, path.display()));
        }

        SnapshotCommands::List { snapshot_dir } => {
            print_operation("Listing", "snapshots");
            let list = snapshots
                .list_snapshots(&snapshot_dir)
                .await
                .map_err(|e| format!("snapshot list failed: {e}"))?;
            print_success(&format!("Found {} snapshots", list.len()));
            for s in list {
                println!("  {s}");
            }
        }

        SnapshotCommands::Rollback { id, snapshot_dir } => {
            print_operation("Rolling back", &id);
            snapshots
                .rollback(&id, &snapshot_dir, namespace.clone())
                .await
                .map_err(|e| format!("snapshot rollback failed: {e}"))?;
            print_success(&format!("Rolled back to snapshot {}", id));
        }
    }
    Ok(())
}

// CLUSTER COMMANDS
async fn handle_cluster_command(
    action: ClusterCommands,
    shard_server: Arc<ShardServer>,
) -> Result<(), String> {
    match action {
        ClusterCommands::Status { data_dir } => {
            let dir = data_dir_or_default(data_dir);
            print_operation("Checking", "cluster status");
            let status = shard_server
                .cluster_status(&dir)
                .await
                .map_err(|e| format!("cluster status failed: {e}"))?;
            print_success("Cluster status retrieved");
            println!("{status}");
        }

        ClusterCommands::Join { node, address } => {
            print_operation("Joining cluster", &node);
            shard_server
                .join_cluster(&node, &address)
                .await
                .map_err(|e| format!("cluster join failed: {e}"))?;
            print_success(&format!("Node {} joined cluster at {}", node, address));
        }

        ClusterCommands::Leave { node } => {
            print_operation("Leaving cluster", &node);
            shard_server
                .leave_cluster(&node)
                .await
                .map_err(|e| format!("cluster leave failed: {e}"))?;
            print_success(&format!("Node {} left cluster", node));
        }
    }
    Ok(())
}

// CACHE COMMANDS
async fn handle_cache_command(
    action: CacheCommands,
    cache: Arc<CacheManager>,
) -> Result<(), String> {
    match action {
        CacheCommands::Stats { max_size_mb } => {
            print_operation("Analyzing", "cache stats");
            let stats = cache
                .stats(max_size_mb)
                .await
                .map_err(|e| format!("cache stats failed: {e}"))?;
            print_success("Cache stats retrieved");
            println!("{stats}");
        }

        CacheCommands::Clear => {
            print_operation("Clearing", "cache");
            cache
                .clear()
                .await
                .map_err(|e| format!("cache clear failed: {e}"))?;
            print_success("Cache cleared");
        }
    }
    Ok(())
}

// VERSION COMMANDS
async fn handle_version_command(
    action: VersionCommands,
    versions: Arc<VersionTracker>,
) -> Result<(), String> {
    match action {
        VersionCommands::List { shard } => {
            print_operation("Listing", "versions");
            let list = versions
                .list_versions(&shard)
                .await
                .map_err(|e| format!("version list failed: {e}"))?;
            print_success(&format!("Found {} versions for shard {}", list.len(), shard));
            for v in list {
                println!("  {v}");
            }
        }

        VersionCommands::Checkout { shard, version } => {
            print_operation("Checking out", &format!("{}/{}", shard, version));
            versions
                .checkout_version(&shard, version)
                .await
                .map_err(|e| format!("version checkout failed: {e}"))?;
            print_success(&format!("Checked out shard {}/{}", shard, version));
        }
    }
    Ok(())
}

// ACL COMMANDS
async fn handle_acl_command(
    action: AclCommands,
    acl_mgr: Arc<AclManager>,
) -> Result<(), String> {
    match action {
        AclCommands::AddUser { user, role, acl_file } => {
            print_operation("Adding user", &user);
            acl_mgr
                .add_user(&user, &role, &acl_file)
                .await
                .map_err(|e| format!("add user failed: {e}"))?;
            print_success(&format!("Added user {} with role {}", user, role));
        }

        AclCommands::ListUsers { acl_file } => {
            print_operation("Listing", "ACL users");
            let users = acl_mgr
                .list_users(&acl_file)
                .await
                .map_err(|e| format!("list users failed: {e}"))?;
            print_success(&format!("Found {} users", users.len()));
            for u in users {
                println!("  {u}");
            }
        }

        AclCommands::Grant { user, resource, permission, acl_file } => {
            print_operation("Granting", &format!("{} on {}", permission, resource));
            acl_mgr
                .grant(&user, &resource, &permission, &acl_file)
                .await
                .map_err(|e| format!("grant failed: {e}"))?;
            print_success(&format!("Granted {} {} {}", permission, resource, user));
        }

        AclCommands::Revoke { user, resource, acl_file } => {
            print_operation("Revoking", &format!("{} access", resource));
            acl_mgr
                .revoke(&user, &resource, &acl_file)
                .await
                .map_err(|e| format!("revoke failed: {e}"))?;
            print_success(&format!("Revoked {} access from {}", resource, user));
        }
    }
    Ok(())
}

// EXTENDED COMMANDS
async fn dispatch_extended_command(
    ext: ExtendedCommands,
    shard_server: Arc<ShardServer>,
) -> Result<(), String> {
    match ext {
        ExtendedCommands::AuditPlayback => {
            print_operation("Starting", "audit playback");
            shard_server
                .playback_audit_log()
                .await
                .map_err(|e| format!("audit playback failed: {e}"))?;
            print_success("Audit playback complete");
        }

        ExtendedCommands::MeshDashboard => {
            print_operation("Launching", "mesh dashboard");
            shard_server
                .run_mesh_dashboard()
                .await
                .map_err(|e| format!("mesh dashboard failed: {e}"))?;
            print_success("Mesh dashboard launched");
        }
    }
    Ok(())
}

// BACKGROUND DAEMONS
fn spawn_background_daemons(
    shard_server: Arc<ShardServer>,
    mesh_gossip: Arc<MeshGossip>,
    metrics: Arc<MetricsCollector>,
    gossip_rx: mpsc::Receiver<mesh_gossip::GossipMessage>,
) {
    print_info("Spawning background daemons");

    // Autoheal daemon
    tokio::spawn(async move {
        if let Err(e) = AutohealDaemon::new(shard_server.clone())
            .run()
            .await
        {
            error!("Autoheal daemon exited with error: {e}");
        }
    });

    // Gossip loop
    tokio::spawn(async move {
        if let Err(e) = mesh_gossip.run(gossip_rx).await {
            error!("Mesh gossip loop exited with error: {e}");
        }
    });

    // Metrics loop
    tokio::spawn(async move {
        if let Err(e) = metrics.run().await {
            error!("Metrics collector exited with error: {e}");
        }
    });

    print_success("Background daemons spawned");
}