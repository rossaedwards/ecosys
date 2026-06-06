//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS Main Entry Point
//!
//! Starts FUSE filesystem and API server

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber;

use aurafs::{
    AclManager, CacheManager, NamespaceManager, SnapshotManager,
    VersionTracker, DeduplicationEngine,
    network::NodeManager,
    storage::ShardStore,
    api::{ApiServer, AppState},
    monitoring::{MetricsCollector, HealthChecker},
    fuse::AuraFSFuse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🔥 AuraFS Starting - f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎");
    
    // Initialize components
    info!("Initializing AuraFS components...");
    
    let namespace = Arc::new(NamespaceManager::new("/var/aurafs/namespace")?);
    let acl = Arc::new(AclManager::new("/var/aurafs/acl.json")?);
    let cache = Arc::new(CacheManager::new(100)); // 100 MB cache
    let node_manager = Arc::new(NodeManager::new(30));
    let snapshot_manager = Arc::new(SnapshotManager::new("/var/aurafs/snapshots")?);
    let version_tracker = Arc::new(VersionTracker::new());
    let shard_store = Arc::new(ShardStore::new());
    
    info!("✅ Components initialized");
    
    // Initialize metrics
    let metrics = MetricsCollector::new()?;
    info!("✅ Metrics collector initialized");
    
    // Initialize health checker
    let health_checker = HealthChecker::new(
        Arc::clone(&namespace),
        Arc::clone(&acl),
        Arc::clone(&cache),
        Arc::clone(&node_manager),
    );
    info!("✅ Health checker initialized");
    
    // Create API state
    let api_state = AppState {
        namespace: Arc::clone(&namespace),
        acl: Arc::clone(&acl),
        cache: Arc::clone(&cache),
        node_manager: Arc::clone(&node_manager),
        snapshot_manager: Arc::clone(&snapshot_manager),
        version_tracker: Arc::clone(&version_tracker),
    };
    
    // Start API server
    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let api_server = ApiServer::new(api_state, addr);
    
    info!("🌐 API server starting on {}", addr);
    
    tokio::spawn(async move {
        if let Err(e) = api_server.serve().await {
            error!("API server error: {}", e);
        }
    });
    
    info!("✅ AuraFS is running!");
    info!("   API: http://0.0.0.0:8080");
    info!("   Health: http://0.0.0.0:8080/health");
    info!("   Metrics: http://0.0.0.0:8080/metrics");
    
    // Keep running
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down AuraFS...");
    
    Ok(())
}