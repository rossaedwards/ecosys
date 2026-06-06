// Path: src/network/node_manager.rs
use crate::network::meshwerk::roles::NodeRole;
use crate::network::transport::starlink_client::MeshMedium;
use std::collections::HashMap;

pub struct NodeState {
    pub role: NodeRole,
    pub active_medium: MeshMedium,
    pub is_coherent: bool,
    pub last_seen: u64,
}

pub struct NodeManager {
    nodes: HashMap<String, NodeState>,
}

impl NodeManager {
    /// Determines if a node is capable of acting as a Titan gateway for Starlink.
    pub fn get_reliable_titans(&self) -> Vec<String> {
        self.nodes.iter()
            .filter(|(_, state)| state.role == NodeRole::Titan && state.is_coherent)
            .map(|(id, _)| id.clone())
            .collect()
    }
}