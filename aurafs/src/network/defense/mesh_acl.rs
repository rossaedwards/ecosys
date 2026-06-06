//! Mesh ACL enforcement.

use std::collections::HashSet;

/// [Theorem 3.1: Universality]
/// Simple allowlist-based ACL for mesh peers.
pub struct MeshAcl {
    allowed: HashSet<String>,
}

impl MeshAcl {
    /// [Theorem 3.1: Universality]
    pub fn new() -> Self {
        Self {
            allowed: HashSet::new(),
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn allow_peer(&mut self, peer_id: String) {
        self.allowed.insert(peer_id);
    }

    /// [Theorem 3.1: Universality]
    pub fn deny_peer(&mut self, peer_id: &str) {
        self.allowed.remove(peer_id);
    }

    /// [Theorem 3.1: Universality]
    pub fn is_allowed(&self, peer_id: &str) -> bool {
        self.allowed.contains(peer_id)
    }
}