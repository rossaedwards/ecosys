//! Mythical Pattern Cache & Index for Fuxyez Fute Semantic Engine
//!
//! Ultra-fast, concurrent pattern cache and index gears: 
//! AST node-pattern hit mapping, cross-referenced reverse lookups,
//! smart concurrent eviction, hot-reload, metadata overlays, and cache diagnostics.
//!
//! Ready for large codebases, multi-user transformation, and quantum/lattice analytics.

use crate::ast::AstNode;
use crate::patterns::library::{SemanticPattern, PatternDomain};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

// --- Unique Node Keying ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeKey(pub Uuid);

impl NodeKey {
    pub fn new() -> Self { NodeKey(Uuid::new_v4()) }
}

// --- Cache Index Core ---

pub struct PatternCache {
    /// Forward index: node_uuid -> { pattern_name }
    node_hits: Arc<RwLock<HashMap<NodeKey, HashSet<String>>>>,
    /// Reverse index: pattern_name -> { node_uuid }
    pattern_hits: Arc<RwLock<HashMap<String, HashSet<NodeKey>>>>,
    /// Node metadata mapping
    node_meta: Arc<RwLock<HashMap<NodeKey, HashMap<String, String>>>>,
    /// Pattern metadata mapping
    pattern_meta: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    /// Capacity management/eviction support
    max_nodes: usize,
    eviction_policy: EvictionPolicy,
}

#[derive(Debug, Clone, Copy)]
pub enum EvictionPolicy {
    /// Least-recently used, time-decay based
    LRU,
    /// FIFO (first-in, first-out)
    FIFO,
    /// Never evict, manual-only
    Manual,
}

impl PatternCache {
    pub fn new(max_nodes: usize, eviction_policy: EvictionPolicy) -> Self {
        Self {
            node_hits: Arc::new(RwLock::new(HashMap::new())),
            pattern_hits: Arc::new(RwLock::new(HashMap::new())),
            node_meta: Arc::new(RwLock::new(HashMap::new())),
            pattern_meta: Arc::new(RwLock::new(HashMap::new())),
            max_nodes,
            eviction_policy,
        }
    }

    /// Insert a node-pattern hit, updating forward/reverse index and metadata.
    pub fn insert_hit(&self, node: &AstNode, key: NodeKey, pattern: &SemanticPattern) {
        // Forward insert
        let mut node_hits = self.node_hits.write().unwrap();
        node_hits.entry(key).or_default().insert(pattern.name.clone());

        // Reverse insert
        let mut pattern_hits = self.pattern_hits.write().unwrap();
        pattern_hits.entry(pattern.name.clone()).or_default().insert(key);

        // Overlay pattern metadata if domain changed
        let mut pattern_meta = self.pattern_meta.write().unwrap();
        pattern_meta.entry(pattern.name.clone()).or_default().insert("domain".into(), format!("{:?}", pattern.domain));
    }

    /// Get all patterns that match a node key, with metadata.
    pub fn get_patterns_for_node(&self, key: &NodeKey) -> Vec<String> {
        self.node_hits.read().unwrap()
            .get(key)
            .cloned()
            .map(|set| set.into_iter().collect())
            .unwrap_or_default()
    }

    /// Get all nodes that hit a pattern name.
    pub fn get_nodes_for_pattern(&self, pattern_name: &str) -> Vec<NodeKey> {
        self.pattern_hits.read().unwrap()
            .get(pattern_name)
            .cloned()
            .map(|set| set.into_iter().collect())
            .unwrap_or_default()
    }

    /// Overlay arbitrary metadata on node.
    pub fn set_node_metadata(&self, key: &NodeKey, meta: HashMap<String, String>) {
        self.node_meta.write().unwrap().insert(*key, meta);
    }

    /// Get node metadata.
    pub fn get_node_metadata(&self, key: &NodeKey) -> Option<HashMap<String, String>> {
        self.node_meta.read().unwrap().get(key).cloned()
    }

    /// Overlay arbitrary metadata on pattern.
    pub fn set_pattern_metadata(&self, pattern_name: &str, meta: HashMap<String, String>) {
        self.pattern_meta.write().unwrap().insert(pattern_name.to_owned(), meta);
    }

    /// Get pattern metadata.
    pub fn get_pattern_metadata(&self, pattern_name: &str) -> Option<HashMap<String, String>> {
        self.pattern_meta.read().unwrap().get(pattern_name).cloned()
    }

    /// Diagnostics and cache stats.
    pub fn stats(&self) -> CacheStats {
        let node_count = self.node_hits.read().unwrap().len();
        let pattern_count = self.pattern_hits.read().unwrap().len();
        CacheStats {
            total_node_keys: node_count,
            total_pattern_keys: pattern_count,
            max_nodes: self.max_nodes,
            eviction_policy: self.eviction_policy,
        }
    }

    /// Evict nodes if over capacity (strategy dependent).
    pub fn evict_nodes(&self) {
        let mut node_hits = self.node_hits.write().unwrap();
        if node_hits.len() > self.max_nodes {
            // Mythical: implement LRU, FIFO...
            match self.eviction_policy {
                EvictionPolicy::LRU => {
                    // Placeholder: real LRU tracking would need timestamps
                    while node_hits.len() > self.max_nodes {
                        if let Some((&key, _)) = node_hits.iter().next() {
                            node_hits.remove(&key);
                        }
                    }
                }
                EvictionPolicy::FIFO => {
                    // Placeholder: FIFO by insert order not tracked in HashMap; real impl would track ordering
                    while node_hits.len() > self.max_nodes {
                        if let Some((&key, _)) = node_hits.iter().next() {
                            node_hits.remove(&key);
                        }
                    }
                }
                EvictionPolicy::Manual => {} // Do nothing
            }
        }
    }

    /// Manual clear-all.
    pub fn clear(&self) {
        self.node_hits.write().unwrap().clear();
        self.pattern_hits.write().unwrap().clear();
        self.node_meta.write().unwrap().clear();
        self.pattern_meta.write().unwrap().clear();
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_node_keys: usize,
    pub total_pattern_keys: usize,
    pub max_nodes: usize,
    pub eviction_policy: EvictionPolicy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::library::{SemanticPattern, PatternDomain};

    #[test]
    fn insert_and_query_cache() {
        let cache = PatternCache::new(100, EvictionPolicy::Manual);
        let key = NodeKey::new();
        let pattern = SemanticPattern {
            name: "QubitDeclaration".into(),
            domain: PatternDomain::Quantum,
            description: "".into(),
            match_predicate: |_| true,
            meta_tags: vec!["test".into()],
        };
        // Dummy node
        cache.insert_hit(&crate::ast::AstNode::Identifier("q0".into()), key, &pattern);
        let hits = cache.get_patterns_for_node(&key);
        assert!(hits.contains(&"QubitDeclaration".into()));
        let nodes = cache.get_nodes_for_pattern("QubitDeclaration");
        assert!(nodes.contains(&key));
    }
}