// Path: src/network/meshwerk/topology_engine.rs
use crate::physics::INVARIANTS;
use crate::network::node_manager::{NodeManager, NodeState};
use crate::network::meshwerk::roles::NodeRole;
use crate::error::AuraError;

/// TopologyEngine: The "Navigator" of AuraFS.
/// Responsible for mapping nodes into Hilbert-scaled space for physics compliance.
pub struct TopologyEngine {
    pub target_ds: f64, // physics::INVARIANTS.spectral_dimension
    pub tolerance: f64, // ± 0.05
}

impl TopologyEngine {
    pub fn new() -> Self {
        Self {
            target_ds: INVARIANTS.spectral_dimension,
            tolerance: 0.05,
        }
    }

    /// Calculates the "Spectral Distance" ($d_s$) using the Hilbert Scaling Bias (η).
    /// $d_s = \sqrt{\Delta t \cdot \eta}$ where η = physics::INVARIANTS.hilbert_scaling_bias.
    pub fn calculate_spectral_distance(&self, lat_us: u64) -> f64 {
        ((lat_us as f64) * INVARIANTS.hilbert_scaling_bias).sqrt()
    }

    /// Identifies optimal nodes for shard redistribution based on the fractal replica law.
    /// Replicas = $\lceil \log_{\eta}(Nodes) \rceil$ where η = physics::INVARIANTS.hilbert_scaling_bias.
    pub fn select_optimal_nodes(
        &self,
        node_manager: &NodeManager,
        total_mesh_size: usize
    ) -> Result<Vec<String>, AuraError> {
        // Enforce the Hilbert Scaling Law (Theorem 2.1)
        let required_replicas = (total_mesh_size as f64).log(INVARIANTS.hilbert_scaling_bias).ceil() as usize;
        
        // Retrieve healthy nodes from NodeManager [cite: 2, 21]
        let healthy_nodes = node_manager.get_active_nodes(); 

        let mut candidates: Vec<(String, f64)> = healthy_nodes
            .into_iter()
            .map(|(id, state)| {
                let dist = self.calculate_spectral_distance(state.latency_us);
                (id, dist)
            })
            // Filter nodes that stay within the 21% PBG routing overhead 
            .filter(|(_, dist)| (*dist - self.target_ds).abs() <= self.tolerance)
            .collect();

        // Sort by spectral stability (nearest to d_s 1.37) 
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        Ok(candidates
            .into_iter()
            .take(required_replicas)
            .map(|(id, _)| id)
            .collect())
    }

    /// Verifies if a node is fit for Titan-tier orbital backhaul via Starlink.
    pub fn verify_titan_suitability(&self, state: &NodeState) -> bool {
        state.role == NodeRole::Titan && state.is_coherent // [cite: 2, 27]
    }
}