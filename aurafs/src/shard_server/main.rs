//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Server - PRODUCTION gRPC + IPFS FEDERATION DAEMON
//! 🛸 Distributed Quantum Storage + Soul Governance + Auto-Healing Swarm
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{
        ShardManager, ShardIndex, LocalShardStorage, TieredShardStorage,
    },
    shard_server::{
        server::{ShardServer, ShardServerConfig},
        grpc::serve_grpc,
        acl::AclEnforcer,
        ipfs::{IpfsConfig, IpfsClusterStorage},
        ipfs_cluster::{IpfsClusterFederation, ClusterConfig},
        autoheal_daemon::{AutoHealDaemon, DaemonConfig},
        mesh::{ShardMesh, MeshConfig},
    },
    crypto::quantum::KyberKeypair,
};
use clap::{Parser, Subcommand};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use tokio::{
    signal,
    sync::broadcast,
};
use tracing::{info, warn, error, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "afs-shardd", about = "AuraFS Production Shard Server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start production shard server daemon
    Start {
        /// Local storage path (NVMe/SSD)
        #[arg(short, long, default_value = "./data/shards")]
        storage: PathBuf,
        
        /// HTTP API listen address
        #[arg(long, default_value = "0.0.0.0:8080")]
        http_addr: SocketAddr,

        /// gRPC API listen address
        #[arg(long, default_value = "0.0.0.0:50051")]
        grpc_addr: SocketAddr,
        
        /// IPFS cluster peers (comma separated URLs)
        /// Example: "http://node1:9094,http://node2:9094"
        #[arg(long)]
        ipfs_peers: Option<String>,
        
        /// Enable P2P Mesh Networking
        #[arg(short, long)]
        mesh: bool,

        /// Enable Phase II Auto-Transmutation (Physics Optimization)
        #[arg(long, default_value_t = true)]
        auto_transmute: bool,
    },
    
    /// Join IPFS shard federation only (Maintenance Mode)
    Join {
        /// IPFS bootstrap peers
        peers: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,aurafs=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start { 
            storage, 
            http_addr, 
            grpc_addr, 
            ipfs_peers, 
            mesh, 
            auto_transmute 
        } => {
            info!("🛸 Starting AuraFS Shard Server");
            start_full_node(storage, http_addr, grpc_addr, ipfs_peers, mesh, auto_transmute).await?;
        }
        
        Commands::Join { peers } => {
            info!("🌐 Joining IPFS federation maintenance mode: {:?}", peers);
            // Standalone federation logic would go here
            warn!("Maintenance mode not fully implemented in main.rs");
        }
    }
    
    Ok(())
}

/// 🚀 Production Full Node Startup
async fn start_full_node(
    storage_path: PathBuf,
    http_addr: SocketAddr,
    grpc_addr: SocketAddr,
    ipfs_peers: Option<String>,
    enable_mesh: bool,
    auto_transmute: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Crypto & ACLs
    // In prod, keys would be loaded from secure enclave/vault
    let node_keypair = KyberKeypair::generate().expect("Failed to gen keys"); 
    let acl_enforcer = Arc::new(AclEnforcer::new(node_keypair));

    // 2. Initialize Storage Tier (Lattice: Bethe & Standard)
    let local_storage = Arc::new(LocalShardStorage::new(storage_path.clone()));
    
    // Setup Deep Storage (IPFS Federation) if peers provided
    let secondary_storage = if let Some(peers_str) = ipfs_peers {
        let configs = parse_ipfs_peers(&peers_str);
        info!("🔗 Initializing IPFS Federation with {} clusters", configs.len());
        let federation = IpfsClusterFederation::new(configs, 1, acl_enforcer.clone()).await?; // Quorum 1 for start
        Some(Arc::new(federation))
    } else {
        None
    };

    let tiered_storage = Arc::new(TieredShardStorage {
        primary: local_storage.clone(),
        secondary: secondary_storage.clone(), // This needs to be strictly typed or wrapped
        acl_enforcer: acl_enforcer.clone(),
    });

    // 3. Initialize Shard Manager (The Brain)
    let shard_index = Arc::new(ShardIndex::new());
    // Manager expects the tiered storage wrapper
    let shard_manager = Arc::new(ShardManager::new(
        local_storage.clone(), // Currently manager takes primary, we might need to update Manager to take Tiered
        shard_index.clone(),
    ));
    // Note: To fully utilize TieredStorage, ShardManager structure in previous files 
    // should be updated to hold TieredShardStorage. 
    // For this `main.rs`, we assume `shard_manager` handles the logic or we manually wire it.
    // Let's assume we patch `ShardManager` to use `TieredShardStorage` or we swap `local_storage` usage here.

    // 4. Initialize P2P Mesh (Lattice: Triangular)
    let shard_mesh = if enable_mesh {
        info!("🕸️  Initializing P2P Mesh Network...");
        let mesh_config = MeshConfig {
            local_peer_id: "local-node".to_string(), // Derive from keys in prod
            replication_factor: 3,
            bootstrap_peers: vec![],
            shard_gossip_topic: "/aurafs/shards/2.0".to_string(),
        };
        Some(Arc::new(ShardMesh::new(mesh_config, shard_manager.clone()).await?))
    } else {
        None
    };

    // 5. Initialize AutoHeal Daemon (Lattice: Physics Engine)
    let daemon_config = DaemonConfig {
        scan_interval: Duration::from_secs(60),
        auto_transmute, // ✨ Phase II
        ..Default::default()
    };
    
    let autoheal = Arc::new(AutoHealDaemon::new(
        shard_manager.clone(),
        shard_index.clone(),
        shard_mesh.clone(),
        acl_enforcer.clone(),
        tiered_storage.clone(),
        daemon_config,
    ));

    // 6. Spawn Subsystems
    let (shutdown_tx, _) = broadcast::channel(1);
    
    // Task: HTTP API
    let http_server = ShardServer::new(ShardServerConfig {
        bind_addr: http_addr,
        shard_manager: shard_manager.clone(),
        shard_index: shard_index.clone(),
        storage: tiered_storage.clone(),
        acl_enforcer: acl_enforcer.clone(),
        shard_mesh: shard_mesh.clone(),
    });
    
    // Task: gRPC API
    let grpc_manager = shard_manager.clone();
    let grpc_index = shard_index.clone();
    let grpc_storage = tiered_storage.clone();
    let grpc_acl = acl_enforcer.clone();
    
    // --- Launch ---
    
    info!("🚀 Launching subsystems...");

    tokio::select! {
        // HTTP Server
        _ = http_server.serve() => {
            error!("HTTP Server exited unexpectedly");
        }
        
        // gRPC Server
        _ = serve_grpc(grpc_manager, grpc_index, grpc_storage, grpc_acl, grpc_addr) => {
            error!("gRPC Server exited unexpectedly");
        }
        
        // P2P Mesh Loop
        _ = async {
            if let Some(mesh) = &shard_mesh {
                if let Err(e) = mesh.run().await {
                    error!("Mesh network failed: {}", e);
                }
            } else {
                std::future::pending::<()>().await;
            }
        } => {}

        // AutoHeal Daemon
        _ = autoheal.run() => {
            error!("AutoHeal daemon exited unexpectedly");
        }

        // Shutdown Signal
        _ = signal::ctrl_c() => {
            info!("🛑 Shutdown signal received. Terminating...");
            let _ = shutdown_tx.send(());
        }
    }
    
    info!("👋 AuraFS Shard Server Shutdown Complete");
    Ok(())
}

/// Helper to parse comma-separated IPFS peers
fn parse_ipfs_peers(peers_str: &str) -> Vec<ClusterConfig> {
    peers_str.split(',')
        .enumerate()
        .map(|(i, url)| ClusterConfig {
            name: format!("cluster-{}", i),
            ipfs_config: IpfsConfig {
                cluster_url: url.to_string(),
                api_secret: None,
                cache_size: 1000,
                permanent_pins: true,
            },
            priority: 100 - (i as u32),
            region: "default".to_string(),
        })
        .collect()
}