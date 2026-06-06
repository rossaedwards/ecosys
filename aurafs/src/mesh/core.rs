//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Mesh Core - Fractal Routing + Chord DHT + Quantum Routing
//! 🕸️ Kademlia + XOR Distance + Fractal Finger Tables + Node Management
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::gov::BlissId;
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, RwLock as StdRwLock},
    net::SocketAddr,
};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Core mesh error enum
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Node not found")]
    NodeNotFound,
    #[error("Invalid finger index")]
    InvalidFingerIndex,
    #[error("General error: {0}")]
    General(String),
}

/// XOR distance metric for routing (u128 space for performance)
pub fn xor_distance(id1: &[u8; 16], id2: &[u8;16]) -> u128 {
    let xored: [u8; 16] = id1.iter()
        .zip(id2.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or([0u8;16]);
    
    u128::from_be_bytes(xored)
}

/// A finger table entry for Chord DHT routing
#[derive(Debug, Clone)]
pub struct FingerEntry {
    /// Start ID of this finger interval
    pub start: BlissId,
    
    /// Node responsible for interval
    pub node: Arc<FractalNode>,
}

/// FractalNode represents a peer in the mesh with ID and address
#[derive(Debug)]
pub struct FractalNode {
    /// Unique node ID (BlissId)
    pub id: BlissId,
    
    /// Network address (ip:port)
    pub addr: SocketAddr,
    
    /// Optional metadata (version, latency, etc)
    pub metadata: Option<String>,
}

impl FractalNode {
    /// Create new fractal node from id and address
    pub fn new(id: BlissId, addr: SocketAddr, metadata: Option<String>) -> Self {
        Self { id, addr, metadata }
    }
}

/// Chord-Compatible Distributed Hash Table with finger table
pub struct ChordDht {
    /// Local node identity
    pub local_node: Arc<FractalNode>,
    
    /// Finger table mapping indices to finger entries
    fingers: RwLock<Vec<FingerEntry>>,
    
    /// Successor nodes list for stabilization
    successors: RwLock<Vec<Arc<FractalNode>>>,
    
    /// Predecessor node (optional)
    predecessor: RwLock<Option<Arc<FractalNode>>>,
    
    /// Node ID space bits (typically 160)
    id_bits: usize,
    
    /// Routing table cache
    routing_cache: StdRwLock<BTreeMap<BlissId, Arc<FractalNode>>>,
}

impl ChordDht {
    /// Create new Chord DHT instance
    pub fn new(local_id: BlissId, id_bits: usize) -> Self {
        let local_node = Arc::new(FractalNode::new(local_id, "0.0.0.0:0".parse().unwrap(), None));
        let finger_count = id_bits;
        let mut fingers = Vec::with_capacity(finger_count);
        
        for i in 0..finger_count {
            // Start placeholders (will be updated by stabilization)
            fingers.push(FingerEntry {
                start: local_node.id.clone(), 
                node: Arc::clone(&local_node),
            });
        }
        
        Self {
            local_node,
            fingers: RwLock::new(fingers),
            successors: RwLock::new(vec![]),
            predecessor: RwLock::new(None),
            id_bits,
            routing_cache: StdRwLock::new(BTreeMap::new()),
        }
    }
    
    /// Lookup successor node for given key
    pub async fn find_successor(&self, key: &BlissId) -> Arc<FractalNode> {
        let local = Arc::clone(&self.local_node);
        if key == &local.id {
            return local;
        }
        
        let pred = self.find_predecessor(key).await;
        let succ = pred.get_successor().await;
        succ
    }
    
    /// Lookup predecessor for a key
    pub async fn find_predecessor(&self, key: &BlissId) -> Arc<FractalNode> {
        let mut node = Arc::clone(&self.local_node);
        
        while !self.in_interval(&key, &node.id, &node.get_successor().await.id, true, false).await {
            node = node.closest_preceding_finger(key).await;
        }
        
        node
    }
    
    /// Check if key ∈ (start, end) interval in ID ring
    async fn in_interval(&self, key: &BlissId, start: &BlissId, end: &BlissId, inclusive_start: bool, inclusive_end: bool) -> bool {
        let key_u = xor_distance(&key.0[0..16].try_into().unwrap(), &[0u8;16]);
        let start_u = xor_distance(&start.0[0..16].try_into().unwrap(), &[0u8;16]);
        let end_u = xor_distance(&end.0[0..16].try_into().unwrap(), &[0u8;16]);
        
        if start_u < end_u {
            (if inclusive_start { key_u >= start_u } else { key_u > start_u }) &&
            (if inclusive_end { key_u <= end_u } else { key_u < end_u })
        } else { // wrap-around
            key_u >= start_u || key_u <= end_u
        }
    }
    
    /// Get successor node from finger table
    pub async fn get_successor(&self) -> Arc<FractalNode> {
        let fingers = self.fingers.read().await;
        fingers[0].node.clone()
    }
    
    /// Find closest preceding finger for key
    pub async fn closest_preceding_finger(&self, key: &BlissId) -> Arc<FractalNode> {
        let fingers = self.fingers.read().await;
        let local_id = &self.local_node.id;
        
        for finger in fingers.iter().rev() {
            if self.in_interval(&finger.node.id, local_id, key, false, false).await {
                return finger.node.clone();
            }
        }
        
        self.local_node.clone()
    }
    
    /// Stabilize the DHT by updating fingers and successors periodically
    pub async fn stabilize(&self) {
        // Placeholder: Refresh fingers from network, update successors & predecessor
        debug!("Stabilizing ChordDht at node {}", self.local_node.id);
    }
    
    /// Update finger table entry at index
    pub async fn update_finger(&self, index: usize, node: Arc<FractalNode>) -> Result<(), CoreError> {
        let mut fingers = self.fingers.write().await;
        if index >= fingers.len() {
            return Err(CoreError::InvalidFingerIndex);
        }
        fingers[index].node = node;
        Ok(())
    }
    
    /// Insert a node into routing cache
    pub fn cache_node(&self, node: Arc<FractalNode>) {
        let mut cache = self.routing_cache.write().unwrap();
        cache.insert(node.id.clone(), node);
    }
}

impl FractalNode {
    /// Get successor of this node (stub for real network call)
    pub async fn get_successor(&self) -> Arc<FractalNode> {
        // Placeholder: In real mesh, query successor node
        Arc::new(self.clone())
    }
    
    /// Dummy closest preceding finger (stub for demo)
    pub async fn closest_preceding_finger(&self, _key: &BlissId) -> Arc<FractalNode> {
        Arc::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[tokio::test]
    async fn test_xor_distance_basic() {
        let id1 = [0u8; 16];
        let id2 = [255u8; 16];
        let dist = xor_distance(&id1, &id2);
        assert_eq!(dist, u128::MAX);
    }
    
    #[tokio::test]
    async fn test_chord_lookup() {
        let local_id = BlissId::new();
        let chord = ChordDht::new(local_id.clone(), 160);
        
        let key = BlissId::new();
        let successor = chord.find_successor(&key).await;
        
        assert_eq!(successor.id, local_id);
    }
}