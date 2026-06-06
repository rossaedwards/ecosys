//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: Reforged with Quantum Precision by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Fuse Filesystem - Updated Quantum Distributed Filesystem Implementation
//! 🛸 Fuse + Async Shard Routing + Network Awareness + Coherency + Secure Access
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    fuse::{node::Inode, inode_cache::InodeCache},
    gov::{BlissId, SoulACL},
    network::{Orchestrator, Firewall},
    shard::ShardId,
    storage::shard_store::ShardStore,
};
use fuser::{
    FileAttr, FileType, Filesystem, FUSE_ROOT_ID, ReplyAttr, ReplyData, ReplyDirectory,
    ReplyEntry, ReplyOpen, ReplyWrite, Request, TimeOrNow,
};
use std::{
    sync::Arc,
    time::{Duration, UNIX_EPOCH, SystemTime},
};
use tokio::sync::RwLock;
use tracing::{info, debug, warn};

/// Session context for authenticated requests
#[derive(Debug, Clone)]
pub struct SessionContext {
    /// Authenticated BlissID
    pub bliss_id: BlissId,
    /// Session token
    pub session_token: Option<String>,
    /// Request UID
    pub uid: u32,
    /// Request GID
    pub gid: u32,
}

impl SessionContext {
    /// Create context from FUSE request
    pub fn from_request(req: &Request<'_>, session_map: &std::collections::HashMap<u32, BlissId>) -> Self {
        let bliss_id = session_map
            .get(&req.uid())
            .cloned()
            .unwrap_or_else(BlissId::genesis);
        
        Self {
            bliss_id,
            session_token: None,
            uid: req.uid(),
            gid: req.gid(),
        }
    }
}

/// Quantum AuraFS Fuse filesystem implementation
pub struct AuraFs {
    /// Inode cache manager
    inode_cache: Arc<InodeCache>,

    /// Local shard store
    shard_store: Arc<ShardStore>,

    /// Network orchestrator
    orchestrator: Arc<Orchestrator>,

    /// Firewall for access control and authorization
    firewall: Arc<Firewall>,

    /// ACL root for Soul-based permissions
    soul_acl: Arc<SoulACL>,

    /// File attribute TTL
    attr_timeout: Duration,

    /// Directory entry TTL
    entry_timeout: Duration,
    
    /// Session map (UID → BlissID)
    session_map: Arc<RwLock<std::collections::HashMap<u32, BlissId>>>,
}

impl AuraFs {
    /// Create new instance of AuraFs filesystem
    pub fn new(
        shard_store: Arc<ShardStore>,
        orchestrator: Arc<Orchestrator>,
        firewall: Arc<Firewall>,
        inode_cache: Arc<InodeCache>,
        soul_acl: Arc<SoulACL>,
    ) -> Self {
        Self {
            inode_cache,
            shard_store,
            orchestrator,
            firewall,
            soul_acl,
            attr_timeout: Duration::from_secs(1),
            entry_timeout: Duration::from_secs(60),
            session_map: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Register a session for a UID
    pub async fn register_session(&self, uid: u32, bliss_id: BlissId) {
        let mut sessions = self.session_map.write().await;
        sessions.insert(uid, bliss_id);
        info!("Registered session for UID {} with BlissID {}", uid, bliss_id);
    }
    
    /// Get BlissID for a request
    async fn get_bliss_id(&self, req: &Request<'_>) -> BlissId {
        let sessions = self.session_map.read().await;
        sessions.get(&req.uid()).cloned().unwrap_or_else(BlissId::genesis)
    }
    
    /// Route shard request to network if local not found
    async fn network_shard_fallback(&self, shard_id: &ShardId) -> Option<Vec<u8>> {
        // Try to fetch from network peers
        match self.orchestrator.request_shard(shard_id).await {
            Ok(data) => {
                info!("Fetched shard {} from network", shard_id);
                // Cache locally for future access
                if let Err(e) = self.shard_store.store_shard_data(shard_id, &data).await {
                    warn!("Failed to cache network shard {}: {}", shard_id, e);
                }
                Some(data)
            }
            Err(e) => {
                debug!("Network shard fetch failed for {}: {}", shard_id, e);
                None
            }
        }
    }

    /// Helper for generating common FileAttr for an inode
    fn file_attr_from_inode(&self, inode: &Inode) -> FileAttr {
        let atime = inode.atime.duration_since(UNIX_EPOCH).unwrap_or_default();
        let mtime = inode.mtime.duration_since(UNIX_EPOCH).unwrap_or_default();
        let ctime = inode.ctime.duration_since(UNIX_EPOCH).unwrap_or_default();

        FileAttr {
            ino: inode.ino,
            size: inode.size,
            blocks: (inode.size / 512) + 1,
            atime: atime.as_secs(),
            atime_nsec: atime.subsec_nanos(),
            mtime: mtime.as_secs(),
            mtime_nsec: mtime.subsec_nanos(),
            ctime: ctime.as_secs(),
            ctime_nsec: ctime.subsec_nanos(),
            kind: inode.kind,
            perm: inode.perm,
            nlink: inode.nlink as u64,
            uid: inode.uid,
            gid: inode.gid,
            rdev: 0,
            blksize: 4096,
            flags: 0,
        }
    }
}

impl Filesystem for AuraFs {
    fn init(&mut self, _req: &Request<'_>, _config: &mut fuser::KernelConfig) {
        info!("🚀 AuraFS quantum Fuse filesystem mounted");
    }

    fn lookup(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: ReplyEntry,
    ) {
        let filename = name.to_string_lossy();
        debug!("🔍 lookup: parent={} name={}", parent, filename);

        // Attempt async lookup to network + cache
        let reply = Arc::new(tokio::sync::Mutex::new(reply));
        let inode_cache = Arc::clone(&self.inode_cache);
        let soul_acl = Arc::clone(&self.soul_acl);
        let firewall = Arc::clone(&self.firewall);
        let orchestrator = Arc::clone(&self.orchestrator);
        let name_owned = filename.to_string();
        let req_uid = req.uid();

        tokio::spawn(async move {
            // Access control: check Soul ACL + Firewall for read permission
            let peer_id = BlissId::genesis(); // TODO: extract from request context/session
            
            if firewall
                .authorize_request(&peer_id, None, crate::network::firewall::Operation::Read)
                .await
                .is_err()
            {
                reply.lock().await.error(libc::EACCES);
                return;
            }

            // Check inode cache if entry exists
            if let Some(parent_node) = inode_cache.get(parent).await {
                let entries = parent_node.as_ref().directory_entries().await;
                if let Some(child_ino) = entries.iter().find_map(|(name, ino)| {
                    if name == &name_owned {
                        Some(*ino)
                    } else {
                        None
                    }
                }) {
                    if let Some(child_node) = inode_cache.get(child_ino).await {
                        let inode_guard = match child_node.inode().read_coherent().await {
                            Ok(guard) => guard,
                            Err(_) => {
                                reply.lock().await.error(libc::EIO);
                                return;
                            }
                        };
                        let attr = inode_guard.attributes();
                        reply.lock().await.entry(&Duration::from_secs(60), &attr, 0);
                        return;
                    }
                }
            }

            // Fallback: create new stub inode (simulate async network read)
            let new_inode = inode_cache.get_or_create_file(parent, name_owned.clone()).await;
            let inode_guard = match new_inode.inode().read_coherent().await {
                Ok(guard) => guard,
                Err(_) => {
                    reply.lock().await.error(libc::EIO);
                    return;
                }
            };
            let attr = inode_guard.attributes();

            reply.lock().await.entry(&Duration::from_secs(60), &attr, 0);
        });
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        debug!("📋 getattr ino={}", ino);

        let reply = Arc::new(tokio::sync::Mutex::new(reply));
        let inode_cache = Arc::clone(&self.inode_cache);

        tokio::spawn(async move {
            if let Some(node) = inode_cache.get(ino).await {
                let inode_guard = match node.inode().read_coherent().await {
                    Ok(guard) => guard,
                    Err(_) => {
                        reply.lock().await.error(libc::EIO);
                        return;
                    }
                };
                let attr = inode_guard.attributes();
                reply.lock().await.attr(&Duration::from_secs(1), &attr);
            } else {
                reply.lock().await.error(libc::ENOENT);
            }
        });
    }

    fn read(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        debug!("📖 read ino={} offset={} size={}", ino, offset, size);

        let reply = Arc::new(tokio::sync::Mutex::new(reply));
        let inode_cache = Arc::clone(&self.inode_cache);
        let shard_store = Arc::clone(&self.shard_store);
        let orchestrator = Arc::clone(&self.orchestrator);
        let firewall = Arc::clone(&self.firewall);

        tokio::spawn(async move {
            if let Some(node) = inode_cache.get(ino).await {
                let inode_guard = match node.inode().read_coherent().await {
                    Ok(guard) => guard,
                    Err(_) => {
                        reply.lock().await.error(libc::EIO);
                        return;
                    }
                };
                let shard_id = match &inode_guard.shard_id {
                    Some(shard) => shard.clone(),
                    None => {
                        reply.lock().await.error(libc::EINVAL);
                        return;
                    }
                };

                let peer_id = BlissId::genesis(); // TODO: extract from context

                if firewall
                    .authorize_request(&peer_id, Some(&shard_id), crate::network::firewall::Operation::Read)
                    .await
                    .is_err()
                {
                    reply.lock().await.error(libc::EACCES);
                    return;
                }

                // Query shard data from local or network (async)
                // For demo: load synchronously from local shard store
                match shard_store.load_shard_data(&shard_id).await {
                    Some(data) => {
                        let start = offset as usize;
                        let end = ((offset as usize) + (size as usize)).min(data.len());
                        reply.lock().await.data(&data[start..end]);
                    }
                    None => {
                        // TODO: Network shard routing fallback
                        reply.lock().await.error(libc::ENOENT);
                    }
                }
            } else {
                reply.lock().await.error(libc::ENOENT);
            }
        });
    }

    fn readdir(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        debug!("📂 readdir ino={} offset={}", ino, offset);

        let inode_cache = Arc::clone(&self.inode_cache);
        let reply = Arc::new(tokio::sync::Mutex::new(reply));

        tokio::spawn(async move {
            if ino == FUSE_ROOT_ID {
                let entries = vec![
                    (1, FileType::Directory, "shards"),
                    (2, FileType::Directory, "souls"),
                    (3, FileType::Directory, "quantum"),
                    (4, FileType::RegularFile, "README.md"),
                ];

                for (i, (ino, kind, name)) in entries.into_iter().enumerate().skip(offset as usize) {
                    if reply.lock().await.add(ino, (i + 1) as i64, kind, name) {
                        break;
                    }
                }
                reply.lock().await.ok();
                return;
            }

            if let Some(node) = inode_cache.get(ino).await {
                // If directory, list children
                let entries = node.as_ref().directory_entries().await;
                for (i, (name, child_ino)) in entries.into_iter().enumerate().skip(offset as usize) {
                    // Use FileType::RegularFile placeholder (TODO: file/dir check)
                    if reply.lock().await.add(child_ino, (i + 1) as i64, FileType::RegularFile, &name) {
                        break;
                    }
                }
                reply.lock().await.ok();
            } else {
                reply.lock().await.error(libc::ENOENT);
            }
        });
    }

    fn write(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        debug!("✍️ write ino={} offset={} size={}", ino, offset, data.len());

        let reply = Arc::new(tokio::sync::Mutex::new(reply));
        let inode_cache = Arc::clone(&self.inode_cache);
        let orchestrator = Arc::clone(&self.orchestrator);
        let firewall = Arc::clone(&self.firewall);
        let shard_store = Arc::clone(&self.shard_store);

        // Clone data for static async usage
        let write_data = data.to_vec();

        tokio::spawn(async move {
            if let Some(node) = inode_cache.get(ino).await {
                let inode_guard = match node.inode().read_coherent().await {
                    Ok(guard) => guard,
                    Err(_) => {
                        reply.lock().await.error(libc::EIO);
                        return;
                    }
                };
                if let Some(shard_id) = &inode_guard.shard_id {
                    let peer_id = BlissId::genesis(); // TODO: get from session

                    if firewall
                        .authorize_request(&peer_id, Some(&shard_id), crate::network::firewall::Operation::Write)
                        .await
                        .is_err()
                    {
                        reply.lock().await.error(libc::EACCES);
                        return;
                    }

                    // For demo, write locally to shard store (async)
                    // TODO: Implement distributed replication writes
                    match shard_store.store_shard_data(&shard_id, &write_data).await {
                        Ok(_) => {
                            // Update inode size & timestamps
                            if let Ok(mut inode_write) = node.inode().write_coherent().await {
                                inode_write.size = offset as u64 + write_data.len() as u64;
                            }
                            reply.lock().await.written(write_data.len() as u32);
                        }
                        Err(_) => {
                            reply.lock().await.error(libc::EIO);
                        }
                    }
                    return;
                }
            }
            reply.lock().await.error(libc::ENOENT);
        });
    }

    fn mkdir(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        _umask: u32,
        reply: ReplyEntry,
    ) {
        let dirname = name.to_string_lossy();
        debug!("📁 mkdir parent={} name={} mode={:o}", parent, dirname, mode);

        let inode_cache = Arc::clone(&self.inode_cache);
        let reply = Arc::new(tokio::sync::Mutex::new(reply));
        let soul_acl = Arc::clone(&self.soul_acl);

        tokio::spawn(async move {
            // Permissions check (simplified)
            // TODO: Check SoulACL and firewall authorization here

            // Create directory inode
            let soul_id = BlissId::genesis(); // TODO: from session context
            let dir_node = inode_cache.create_directory(parent, dirname.to_string(), soul_id).await;
            let inode_guard = match dir_node.inode().read_coherent().await {
                Ok(guard) => guard,
                Err(_) => {
                    reply.lock().await.error(libc::EIO);
                    return;
                }
            };
            let attr = inode_guard.attributes();
            reply.lock().await.entry(&Duration::from_secs(60), &attr, 0);
        });
    }

    fn unlink(&mut self, _req: &Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        let filename = name.to_string_lossy();
        debug!("🗑️ unlink parent={} name={}", parent, filename);
        
        let inode_cache = Arc::clone(&self.inode_cache);
        let shard_store = Arc::clone(&self.shard_store);
        let name_owned = filename.to_string();
        
        tokio::spawn(async move {
            // Find the child inode
            if let Some(parent_node) = inode_cache.get(parent).await {
                let entries = parent_node.as_ref().directory_entries().await;
                if let Some(child_ino) = entries.iter().find_map(|(n, ino)| {
                    if n == &name_owned { Some(*ino) } else { None }
                }) {
                    // Remove from cache
                    inode_cache.remove(child_ino).await;
                    
                    // Remove from parent's directory entries
                    parent_node.as_ref().remove_entry(&name_owned).await;
                    
                    reply.ok();
                    return;
                }
            }
            reply.error(libc::ENOENT);
        });
    }
    
    fn rename(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        newparent: u64,
        newname: &std::ffi::OsStr,
        _flags: u32,
        reply: fuser::ReplyEmpty,
    ) {
        let old_name = name.to_string_lossy().to_string();
        let new_name = newname.to_string_lossy().to_string();
        debug!("📝 rename parent={} name={} -> newparent={} newname={}", 
            parent, old_name, newparent, new_name);
        
        let inode_cache = Arc::clone(&self.inode_cache);
        
        tokio::spawn(async move {
            // Find source entry
            if let Some(parent_node) = inode_cache.get(parent).await {
                let entries = parent_node.as_ref().directory_entries().await;
                if let Some(child_ino) = entries.iter().find_map(|(n, ino)| {
                    if n == &old_name { Some(*ino) } else { None }
                }) {
                    // Remove from old parent
                    parent_node.as_ref().remove_entry(&old_name).await;
                    
                    // Add to new parent
                    if let Some(new_parent_node) = inode_cache.get(newparent).await {
                        new_parent_node.as_ref().add_entry(new_name, child_ino).await;
                        reply.ok();
                        return;
                    }
                }
            }
            reply.error(libc::ENOENT);
        });
    }
    
    fn open(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: ReplyOpen) {
        debug!("📂 open ino={} flags={}", ino, flags);
        
        let inode_cache = Arc::clone(&self.inode_cache);
        
        tokio::spawn(async move {
            if inode_cache.get(ino).await.is_some() {
                // Return a file handle (simplified - just use inode number)
                reply.opened(ino, 0);
            } else {
                reply.error(libc::ENOENT);
            }
        });
    }
    
    fn release(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        debug!("🔓 release ino={}", ino);
        // For now, just acknowledge - could implement deferred write flush
        reply.ok();
    }
    
    fn create(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        _umask: u32,
        flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        let filename = name.to_string_lossy().to_string();
        debug!("✨ create parent={} name={} mode={:o}", parent, filename, mode);
        
        let inode_cache = Arc::clone(&self.inode_cache);
        let session_map = Arc::clone(&self.session_map);
        let req_uid = req.uid();
        
        tokio::spawn(async move {
            // Get BlissID from session
            let sessions = session_map.read().await;
            let bliss_id = sessions.get(&req_uid).cloned().unwrap_or_else(BlissId::genesis);
            drop(sessions);
            
            // Create new file inode
            let file_node = inode_cache.create_file(parent, filename, bliss_id).await;
            let inode_guard = match file_node.inode().read_coherent().await {
                Ok(guard) => guard,
                Err(_) => {
                    reply.error(libc::EIO);
                    return;
                }
            };
            let attr = inode_guard.attributes();
            
            reply.created(&Duration::from_secs(60), &attr, 0, inode_guard.ino, 0);
        });
    }
    
    fn setattr(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<TimeOrNow>,
        mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        debug!("⚙️ setattr ino={}", ino);
        
        let inode_cache = Arc::clone(&self.inode_cache);
        
        tokio::spawn(async move {
            if let Some(node) = inode_cache.get(ino).await {
                let mut inode = match node.inode().write_coherent().await {
                    Ok(guard) => guard,
                    Err(_) => {
                        reply.error(libc::EIO);
                        return;
                    }
                };
                
                // Apply updates
                if let Some(m) = mode {
                    inode.perm = m as u16;
                }
                if let Some(u) = uid {
                    inode.uid = u;
                }
                if let Some(g) = gid {
                    inode.gid = g;
                }
                if let Some(s) = size {
                    inode.size = s;
                }
                if let Some(TimeOrNow::SpecificTime(t)) = atime {
                    inode.atime = t;
                }
                if let Some(TimeOrNow::SpecificTime(t)) = mtime {
                    inode.mtime = t;
                }
                
                let attr = inode.attributes();
                drop(inode);
                
                reply.attr(&Duration::from_secs(1), &attr);
            } else {
                reply.error(libc::ENOENT);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuse::inode_cache::InodeCache;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_aura_fs_creation() {
        let cache = Arc::new(InodeCache::new());
        let shard_store = Arc::new(ShardStore::default());
        let orchestrator = Arc::new(Orchestrator::default());
        let firewall = Arc::new(Firewall::default());
        let acl = Arc::new(SoulACL::root());

        let fs = AuraFs::new(cache, shard_store, orchestrator, firewall, acl);
        assert_eq!(fs.attr_timeout.as_secs(), 1);
    }
}