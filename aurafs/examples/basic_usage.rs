//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Basic AuraFS usage example demonstrating all major features

use std::sync::Arc;
use aurafs::{
    acl::AclManager,
    cache::CacheManager,
    compression::{CompressionAlgorithm, CompressionManager},
    dedup::DeduplicationEngine,
    namespace::NamespaceManager,
    network::{node_manager::NodeManager, replication::{ReplicationManager, ReplicationPolicy}},
    snapshot::SnapshotManager,
    versioning::VersionTracker,
    audit::AuditLogger,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("🔥 AuraFS Integration Demo - f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎\n");
    
    // 1. Initialize ACL Manager
    println!("1️⃣ Initializing ACL Manager...");
    let acl = Arc::new(AclManager::new("config/acl.json")?);
    acl.add_user("alice", "user")?;
    acl.add_user("bob", "admin")?;
    println!("   ✅ ACL initialized with users: alice, bob\n");
    
    // 2. Initialize Namespace Manager
    println!("2️⃣ Initializing Namespace Manager...");
    let namespace = Arc::new(NamespaceManager::new("/tmp/aurafs/namespace")?);
    namespace.create_directory("/projects", "alice")?;
    namespace.create_directory("/projects/demo", "alice")?;
    namespace.create_file("/projects/demo/file1.txt", "alice", None)?;
    println!("   ✅ Created directory structure: /projects/demo/file1.txt\n");
    
    // 3. Initialize Cache
    println!("3️⃣ Initializing LRU Cache (100 MB)...");
    let cache = Arc::new(CacheManager::new(100));
    println!("   ✅ Cache initialized\n");
    
    // 4. Initialize Deduplication Engine
    println!("4️⃣ Initializing Deduplication Engine...");
    let dedup = DeduplicationEngine::new("/tmp/aurafs/dedup")?;
    let chunk_hashes = dedup.add_file("Cargo.toml")?;
    println!("   ✅ File chunked into {} deduplicated chunks", chunk_hashes.len());
    let (total_chunks, total_refs, savings) = dedup.get_stats();
    println!("   📊 Stats: {} unique chunks, {} references, {}% space saved\n", 
             total_chunks, total_refs, savings);
    
    // 5. Initialize Compression Manager
    println!("5️⃣ Initializing Compression Manager...");
    let compression = CompressionManager::new(CompressionAlgorithm::Zstd, 6);
    let test_data = b"Hello AuraFS! This is a test of compression.".repeat(10);
    let compressed = compression.compress(&test_data)?;
    let ratio = compression.compression_ratio(test_data.len(), compressed.len());
    println!("   ✅ Compression ratio: {:.1}%\n", ratio);
    
    // 6. Initialize Node Manager
    println!("6️⃣ Initializing Node Manager...");
    let node_manager = Arc::new(NodeManager::new(30));
    node_manager.register_node("node1", "192.168.1.100:5000")?;
    node_manager.register_node("node2", "192.168.1.101:5000")?;
    node_manager.register_node("node3", "192.168.1.102:5000")?;
    let (total, live, _, _) = node_manager.get_cluster_stats();
    println!("   ✅ Cluster: {}/{} nodes live\n", live, total);
    
    // 7. Initialize Replication Manager
    println!("7️⃣ Initializing Replication Manager...");
    let replication = ReplicationManager::new(
        Arc::clone(&node_manager),
        ReplicationPolicy::Fixed(2),
    );
    println!("   ✅ Replication policy: 2 replicas per shard\n");
    
    // 8. Initialize Snapshot Manager
    println!("8️⃣ Creating Snapshot...");
    let snapshot_manager = SnapshotManager::new("/tmp/aurafs/snapshots")?;
    let snap_id = snapshot_manager.create_snapshot(
        "/tmp/aurafs",
        "Demo snapshot",
        None,
    )?;
    println!("   ✅ Snapshot created: {}\n", snap_id);
    
    // 9. Initialize Version Tracker
    println!("9️⃣ Tracking File Versions...");
    let version_tracker = VersionTracker::new();
    let shard_id = aurafs::core::shard::ShardId::new();
    version_tracker.track_shard(shard_id.clone());
    let v1 = version_tracker.create_version(&shard_id, "alice", "Initial version", "hash1")?;
    let v2 = version_tracker.create_version(&shard_id, "alice", "Updated content", "hash2")?;
    println!("   ✅ Created versions: v{}, v{}\n", v1, v2);
    
    // 10. Initialize Audit Logger
    println!("🔟 Initializing Audit Logger...");
    let audit = AuditLogger::new("/tmp/aurafs/audit.log")?;
    audit.log_fs_operation("write", "alice", "/projects/demo/file1.txt", "success")?;
    audit.log_acl_change("bob", "/projects", "granted write to alice")?;
    audit.log_snapshot(&snap_id, "alice", "Demo snapshot")?;
    println!("   ✅ Audit events logged\n");
    
    println!("🎉 All AuraFS components initialized successfully!");
    println!("💎 AuraFS is ready for production use!");
    
    Ok(())
}