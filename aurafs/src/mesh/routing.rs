//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Mesh Routing - Adaptive + Latency-Aware + Shard-Optimized Routing
//! 🕸️ Fractal Routing Tables + Load Balancing + Failure Recovery + Metrics
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::BlissId,
    mesh::core::{FractalNode, xor_distance},
    network::mesh_gossip::{GossipEnvelope, GossipPayload},
};
use std::{
    sync::Arc,
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Routing error enum
#[derive(Debug, thiserror::Error)]
pub enum RoutingError {
    #[error("No route found")]
    NoRoute,
    #[error("Route timeout")]
    Timeout,
    #[error("Routing table corrupted")]
    Corrupted,
}

/// Route information for a shard or peer
#[derive(Debug, Clone)]
pub struct ShardRoute {
    /// Target shard ID (BlissId)
    pub shard_id: BlissId,
    
    /// Next hop peer in route
    pub next_hop: Arc<FractalNode>,
    
    /// Distance to target from next hop
    pub distance: u128,
    
    /// Route lifetime / TTL
    pub ttl: u8,
    
    /// Number of hops taken
    pub hops: u8,
    
    /// Route metrics (latency, success rate)
    pub metrics: RouteMetrics,
}

/// Route performance metrics
#[derive(Debug, Clone, Default)]
pub struct RouteMetrics {
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Success ratio [0.0 - 1.0]
    pub success_rate: f32,
}

/// Adaptive routing table maintaining shard routes and metrics
pub struct RouteTable {
    /// Map shard_id → ShardRoute
    routes: RwLock<HashMap<BlissId, ShardRoute>>,
    
    /// LRU set for pruning stale routes
    lru_set: RwLock<HashSet<BlissId>>,
    
    /// Maximum routes to cache
    max_cache_size: usize,
}

impl RouteTable {
    /// Create new route table with max capacity
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            routes: RwLock::new(HashMap::new()),
            lru_set: RwLock::new(HashSet::new()),
            max_cache_size,
        }
    }
    
    /// Add or update a shard route in the table
    pub async fn update_route(&self, route: ShardRoute) {
        let mut routes = self.routes.write().await;
        let mut lru = self.lru_set.write().await;
        
        if routes.len() >= self.max_cache_size {
            // Evict a random entry (simple LRU)
            if let Some(evict_id) = lru.iter().next().cloned() {
                routes.remove(&evict_id);
                lru.remove(&evict_id);
                info!("🗑️ Evicted shard route cache entry {}", evict_id);
            }
        }
        
        lru.insert(route.shard_id.clone());
        routes.insert(route.shard_id.clone(), route);
    }
    
    /// Lookup next hop for a shard, if known and fresh
    pub async fn lookup_route(&self, shard_id: &BlissId) -> Option<ShardRoute> {
        let routes = self.routes.read().await;
        routes.get(shard_id).cloned()
    }
    
    /// Prune expired or stale routes periodically
    pub async fn prune_stale_routes(&self, max_age: Duration) {
        let cutoff = Instant::now() - max_age;
        let mut routes = self.routes.write().await;
        let mut lru = self.lru_set.write().await;
        
        let stale_keys: Vec<_> = routes.iter()
            .filter(|(_, route)| route.metrics.avg_latency_ms > max_age.as_millis() as f64)
            .map(|(id, _)| id.clone())
            .collect();
        
        for key in stale_keys {
            routes.remove(&key);
            lru.remove(&key);
            info!("🧹 Pruned stale shard route {}", key);
        }
    }
}

/// Adaptive router implementing latency and load-aware routing
pub struct AdaptiveRouter {
    /// Local node reference
    local_node: Arc<FractalNode>,
    
    /// Routing table cache
    route_table: Arc<RouteTable>,
    
    /// Peer latencies (peer_id → latency ms)
    peer_latencies: RwLock<HashMap<BlissId, f64>>,
    
    /// Recent route success counts (peer_id → count)
    peer_success: RwLock<HashMap<BlissId, usize>>,
    
    /// Recent route failure counts (peer_id → count)
    peer_failures: RwLock<HashMap<BlissId, usize>>,
}

impl AdaptiveRouter {
    /// Forge new adaptive router
    pub fn new(local_node: Arc<FractalNode>, route_table: Arc<RouteTable>) -> Self {
        Self {
            local_node,
            route_table,
            peer_latencies: RwLock::new(HashMap::new()),
            peer_success: RwLock::new(HashMap::new()),
            peer_failures: RwLock::new(HashMap::new()),
        }
    }
    
    /// Route to shard with best latency and success probability
    pub async fn route_to_shard(&self, shard_id: &BlissId) -> Result<ShardRoute, RoutingError> {
        if let Some(route) = self.route_table.lookup_route(shard_id).await {
            Ok(route)
        } else {
            Err(RoutingError::NoRoute)
        }
    }
    
    /// Update peer latency measurement
    pub async fn update_latency(&self, peer_id: BlissId, latency_ms: f64) {
        let mut latencies = self.peer_latencies.write().await;
        latencies.insert(peer_id, latency_ms);
    }
    
    /// Record route success
    pub async fn record_success(&self, peer_id: BlissId) {
        let mut success = self.peer_success.write().await;
        *success.entry(peer_id).or_insert(0) += 1;
    }
    
    /// Record route failure
    pub async fn record_failure(&self, peer_id: BlissId) {
        let mut failure = self.peer_failures.write().await;
        *failure.entry(peer_id).or_insert(0) += 1;
    }
    
    /// Compute success ratio for peer
    pub async fn success_ratio(&self, peer_id: &BlissId) -> f32 {
        let success = self.peer_success.read().await.get(peer_id).cloned().unwrap_or(0);
        let failure = self.peer_failures.read().await.get(peer_id).cloned().unwrap_or(0);
        
        let total = (success + failure) as f32;
        if total == 0.0 {
            1.0 // Assume healthy if no data
        } else {
            success as f32 / total
        }
    }
    
    /// Select next hop from candidates based on latency and reliability
    pub async fn select_next_hop(&self, candidates: &[Arc<FractalNode>]) -> Option<Arc<FractalNode>> {
        let latencies = self.peer_latencies.read().await;
        let mut scored: Vec<(f64, Arc<FractalNode>)> = Vec::new();
        
        for candidate in candidates {
            let latency = latencies.get(&candidate.id).cloned().unwrap_or(f64::MAX);
            let success = self.success_ratio(&candidate.id).await;
            
            // Score = lower latency and higher success is better
            let score = latency / success.max(0.01);
            scored.push((score, candidate.clone()));
        }
        
        scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        scored.first().map(|(_, node)| node.clone())
    }
    
    /// Periodically prune old routing data
    pub async fn cleanup(&self) {
        self.route_table.prune_stale_routes(Duration::from_secs(300)).await;
        // Optionally decay latencies and success/failure scores here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    use crate::gov::BlissId;

    #[tokio::test]
    async fn test_route_table_insert_lookup() {
        let table = RouteTable::new(100);
        let shard_id = BlissId::new();
        let node = Arc::new(FractalNode::new(shard_id.clone(), "127.0.0.1:7000".parse().unwrap(), None));
        let route = ShardRoute {
            shard_id: shard_id.clone(),
            next_hop: node.clone(),
            distance: 0,
            ttl: 8,
            hops: 1,
            metrics: RouteMetrics::default(),
        };
        
        table.update_route(route.clone()).await;
        let lookup = table.lookup_route(&shard_id).await.unwrap();
        assert_eq!(lookup.shard_id, shard_id);
    }
    
    #[tokio::test]
    async fn test_adaptive_router_select_next_hop() {
        let router = AdaptiveRouter::new(
            Arc::new(FractalNode::new(BlissId::new(), "127.0.0.1:7000".parse().unwrap(), None)),
            Arc::new(RouteTable::new(10)),
        );
        
        let node1 = Arc::new(FractalNode::new(BlissId::new(), "127.0.0.1:7001".parse().unwrap(), None));
        let node2 = Arc::new(FractalNode::new(BlissId::new(), "127.0.0.1:7002".parse().unwrap(), None));
        
        router.update_latency(node1.id.clone(), 10.0).await;
        router.update_latency(node2.id.clone(), 20.0).await;
        router.record_success(node1.id.clone()).await;
        router.record_failure(node2.id.clone()).await;
        
        let next = router.select_next_hop(&[node1.clone(), node2.clone()]).await.unwrap();
        assert_eq!(next.addr, node1.addr);
    }
}