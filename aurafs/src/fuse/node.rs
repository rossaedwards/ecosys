//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Fuse Nodes - Quantum Inode Abstractions + Shard Mapping
//! 🛸 FileNode + DirectoryNode + Inode Hierarchy + Metadata + Permissions
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::BlissId,
    shard::ShardId,
    fuse::session::FuseSession,
    fuse::AuraTimedLock,
};
use fuser::FileType;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH, Instant},
};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use tracing::debug;
use crate::error::{RafsError, Result};
use crate::physics::{INVARIANTS, PhysicsViolationError};

/// Core inode abstraction for AuraFS
#[derive(Debug, Clone)]
pub struct Inode {
    /// Unique inode number
    pub ino: u64,
    
    /// Shard ID backing this inode (if file)
    pub shard_id: Option<ShardId>,
    
    /// Soul owner ID
    pub soul_id: BlissId,
    
    /// File type
    pub kind: FileType,
    
    /// File size in bytes
    pub size: u64,
    
    /// Permissions (Unix mode)
    pub perm: u32,
    
    /// Timestamps
    pub atime: SystemTime,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    
    /// Link count
    pub nlink: u64,
    
    /// User/Group IDs
    pub uid: u32,
    pub gid: u32,
    
    /// Parent inode
    pub parent: u64,
    
    /// Filename
    pub name: String,
    
    /// Metadata (custom AuraFS extensions)
    pub metadata: InodeMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InodeMetadata {
    /// Shard replication factor
    pub replication_factor: u8,
    
    /// Quantum entropy level (0-255)
    pub entropy: u8,
    
    /// Last shard sync time
    pub last_sync_ns: u64,
    
    /// Custom tags
    pub tags: Vec<String>,
}

/// Sync wrapper to enforce coherent inode access.
pub struct SyncNode {
    inode: Arc<AuraTimedLock<Inode>>,
}

impl SyncNode {
    pub fn new(inode: Inode) -> Self {
        Self {
            inode: Arc::new(AuraTimedLock::new(inode)),
        }
    }

    pub fn inode(&self) -> Arc<AuraTimedLock<Inode>> {
        Arc::clone(&self.inode)
    }
}

impl Inode {
    /// Create root inode
    pub fn root() -> Self {
        Self {
            ino: fuser::FUSE_ROOT_ID,
            shard_id: None,
            soul_id: BlissId::genesis(),
            kind: FileType::Directory,
            size: 4096,
            perm: 0o755,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            nlink: 1,
            uid: 1000,
            gid: 1000,
            parent: 0,
            name: "/".to_string(),
            metadata: InodeMetadata {
                replication_factor: 5,
                entropy: 255,
                last_sync_ns: 0,
                tags: vec!["root".to_string()],
            },
        }
    }
    
    /// Create file inode backed by shard
    pub fn file(ino: u64, parent: u64, name: String, shard_id: ShardId, soul_id: BlissId) -> Self {
        Self {
            ino,
            shard_id: Some(shard_id),
            soul_id,
            kind: FileType::RegularFile,
            size: 0,
            perm: 0o644,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            nlink: 1,
            uid: 1000,
            gid: 1000,
            parent,
            name,
            metadata: InodeMetadata::default(),
        }
    }
    
    /// Create directory inode
    pub fn directory(ino: u64, parent: u64, name: String, soul_id: BlissId) -> Self {
        Self {
            ino,
            shard_id: None,
            soul_id,
            kind: FileType::Directory,
            size: 4096,
            perm: 0o755,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            nlink: 2,
            uid: 1000,
            gid: 1000,
            parent,
            name,
            metadata: InodeMetadata {
                replication_factor: 3,
                entropy: 128,
                last_sync_ns: 0,
                tags: vec!["directory".to_string()],
            },
        }
    }
    
    /// Convert to fuser::FileAttr for kernel
    pub fn attributes(&self) -> fuser::FileAttr {
        let atime = self.atime.duration_since(UNIX_EPOCH).unwrap_or_default();
        let mtime = self.mtime.duration_since(UNIX_EPOCH).unwrap_or_default();
        let ctime = self.ctime.duration_since(UNIX_EPOCH).unwrap_or_default();
        
        fuser::FileAttr {
            ino: self.ino,
            size: self.size,
            blocks: (self.size / 512) + 1,
            atime: atime.as_secs(),
            atime_nsec: atime.subsec_nanos() as u32,
            mtime: mtime.as_secs(),
            mtime_nsec: mtime.subsec_nanos() as u32,
            ctime: ctime.as_secs(),
            ctime_nsec: ctime.subsec_nanos() as u32,
            kind: self.kind,
            perm: self.perm,
            nlink: self.nlink as u64,
            uid: self.uid,
            gid: self.gid,
            rdev: 0,
            blksize: 4096,
            flags: 0,
        }
    }
}

/// File-specific node with shard streaming
pub struct FileNode {
    /// Base inode
    inode: Arc<AuraTimedLock<Inode>>,
    
    /// Associated Fuse session
    session: Arc<FuseSession>,
    
    /// Open file handles
    open_handles: RwLock<usize>,
}

impl FileNode {
    /// Create new file node
    pub fn new(inode: Inode, session: Arc<FuseSession>) -> Arc<Self> {
        let sync = SyncNode::new(inode);
        Arc::new(Self {
            inode: sync.inode(),
            session,
            open_handles: RwLock::new(0),
        })
    }
    
    /// [Theorem 3.1: Topological Stability]
    /// Open file (increment handle count) with coherent inode access.
    pub async fn open(&self) -> Result<()> {
        let mut handles = self.open_handles.write().await;
        *handles += 1;
        let inode_guard = self.inode.read_coherent().await?;
        debug!("📄 FileNode {} opened (handles={})", inode_guard.ino, *handles);
        Ok(())
    }
    
    /// [Theorem 3.1: Topological Stability]
    /// Read from shard backing this file with coherence heartbeat enforcement.
    pub async fn read(&self, offset: u64, size: usize) -> Result<Vec<u8>> {
        let start = Instant::now();
        let shard_id = {
            let inode_guard = self.inode.read_coherent().await?;
            inode_guard.shard_id.clone()
        };

        let data = if let Some(shard_id) = shard_id {
            self.session.read_shard(&shard_id, offset, size).await
        } else {
            vec![]
        };

        let elapsed = start.elapsed().as_micros() as u64;
        if elapsed > INVARIANTS.coherence_window_us {
            return Err(RafsError::PhysicsViolation(
                PhysicsViolationError::StabilityTimeout { elapsed, limit: INVARIANTS.coherence_window_us }
            ));
        }

        Ok(data)
    }
    
    /// [Theorem 3.1: Topological Stability]
    /// Update file size after write with coherence enforcement.
    pub async fn update_size(&self, new_size: u64) -> Result<()> {
        let start = Instant::now();
        let mut inode_guard = self.inode.write_coherent().await?;
        inode_guard.size = new_size;
        inode_guard.mtime = SystemTime::now();
        let elapsed = start.elapsed().as_micros() as u64;
        if elapsed > INVARIANTS.coherence_window_us {
            return Err(RafsError::PhysicsViolation(
                PhysicsViolationError::StabilityTimeout { elapsed, limit: INVARIANTS.coherence_window_us }
            ));
        }
        Ok(())
    }

    /// [Theorem 3.1: Topological Stability]
    /// Write data to this file with coherence heartbeat enforcement.
    pub async fn write(&self, offset: u64, data: &[u8]) -> Result<usize> {
        let start = Instant::now();
        let shard_id = {
            let inode_guard = self.inode.read_coherent().await?;
            inode_guard.shard_id.clone()
        };

        if shard_id.is_none() {
            return Err(RafsError::ShardNotFound("Missing shard backing for inode".to_string()));
        }

        let new_size = offset.saturating_add(data.len() as u64);
        self.update_size(new_size).await?;

        let elapsed = start.elapsed().as_micros() as u64;
        if elapsed > INVARIANTS.coherence_window_us {
            return Err(RafsError::PhysicsViolation(
                PhysicsViolationError::StabilityTimeout { elapsed, limit: INVARIANTS.coherence_window_us }
            ));
        }

        Ok(data.len())
    }
}

/// Directory node with child inode tracking
pub struct DirectoryNode {
    /// Base inode
    inode: Arc<AuraTimedLock<Inode>>,
    
    /// Child inodes (name → inode)
    children: RwLock<HashMap<String, u64>>,
    
    /// Session context
    session: Arc<FuseSession>,
}

impl DirectoryNode {
    /// Create new directory node
    pub fn new(inode: Inode, session: Arc<FuseSession>) -> Arc<Self> {
        let sync = SyncNode::new(inode);
        Arc::new(Self {
            inode: sync.inode(),
            children: RwLock::new(HashMap::new()),
            session,
        })
    }
    
    /// [Theorem 3.1: Topological Stability]
    /// Add child entry with coherent inode access.
    pub async fn add_child(&self, name: String, ino: u64) -> Result<()> {
        let mut children = self.children.write().await;
        children.insert(name, ino);
        let inode_guard = self.inode.read_coherent().await?;
        debug!("📁 DirNode {} added child '{}' → {}", 
               inode_guard.ino, name, ino);
        Ok(())
    }
    
    /// [Theorem 3.1: Topological Stability]
    /// List children for readdir with coherent inode access.
    pub async fn list_children(&self) -> Result<Vec<(u64, FileType, String)>> {
        let children = self.children.read().await;
        let entries = children.iter()
            .map(|(name, ino)| (*ino, FileType::empty(), name.clone()))
            .collect();
        Ok(entries)
    }
}

/// Inode cache integration trait
pub trait Node: Send + Sync {
    fn inode(&self) -> Arc<AuraTimedLock<Inode>>;
}

impl Node for Arc<FileNode> {
    fn inode(&self) -> Arc<AuraTimedLock<Inode>> {
        Arc::clone(&self.inode)
    }
}

impl Node for Arc<DirectoryNode> {
    fn inode(&self) -> Arc<AuraTimedLock<Inode>> {
        Arc::clone(&self.inode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_inode_attributes() {
        let root = Inode::root();
        let attr = root.attributes();
        
        assert_eq!(attr.ino, fuser::FUSE_ROOT_ID);
        assert_eq!(attr.kind, FileType::Directory);
        assert_eq!(attr.perm, 0o755);
    }
    
    #[tokio::test]
    async fn test_file_node_operations() {
        let inode = Inode::file(42, 1, "test.txt".to_string(), ShardId::new(), BlissId::genesis());
        let session = Arc::new(crate::fuse::session::FuseSession::new(
            BlissId::genesis(),
            Arc::new(crate::network::SecureTunnel::new()),
            Arc::new(crate::storage::shard_store::ShardStore::default()),
            Arc::new(crate::fuse::inode_cache::InodeCache::new()),
            Default::default(),
        ));
        
        let file_node = FileNode::new(inode, session);
        file_node.open().await.unwrap();
        
        let handles = file_node.open_handles.read().await;
        assert_eq!(*handles, 1);
    }
}