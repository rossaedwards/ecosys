//! Neighbor table for Meshwerk.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// [Theorem 3.1: Universality]
/// Neighbor tracking entry.
#[derive(Debug, Clone)]
pub struct NeighborEntry {
    pub node_id: String,
    pub last_seen: Instant,
    pub latency_ms: f64,
    pub hops: u8,
}

/// [Theorem 3.1: Universality]
/// In-memory neighbor table with TTL pruning.
pub struct NeighborTable {
    neighbors: HashMap<String, NeighborEntry>,
    ttl: Duration,
}

impl NeighborTable {
    /// [Theorem 3.1: Universality]
    pub fn new(ttl: Duration) -> Self {
        Self {
            neighbors: HashMap::new(),
            ttl,
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn upsert(&mut self, entry: NeighborEntry) {
        self.neighbors.insert(entry.node_id.clone(), entry);
    }

    /// [Theorem 3.1: Universality]
    pub fn prune(&mut self) {
        let cutoff = Instant::now() - self.ttl;
        self.neighbors.retain(|_, entry| entry.last_seen >= cutoff);
    }

    /// [Theorem 3.1: Universality]
    pub fn snapshot(&self) -> Vec<NeighborEntry> {
        self.neighbors.values().cloned().collect()
    }
}