use crate::physics::INVARIANTS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeRole {
    /// GhostLink: Low-power edge node. Minimal state, lock_acquisition_timeout_us max jitter.
    GhostLink,
    /// Titan: Mesh-Lord. High compute, manages S.A.G.E.S. voting and LLL reduction.
    Titan,
}

pub struct NodeCapabilities {
    pub role: NodeRole,
    pub throughput_gbps: u32,
    pub supports_lattice_reduction: bool,
}

impl NodeCapabilities {
    pub fn calculate_max_shards(&self) -> usize {
        match self.role {
            NodeRole::GhostLink => (INVARIANTS.hilbert_scaling_bias.log2() as usize).max(1),
            NodeRole::Titan => (INVARIANTS.hilbert_scaling_bias.powi(2) as usize).max(16),
        }
    }

    pub fn can_handle_heartbeat(&self, latency_us: u64) -> bool {
        // GhostLinks must respond within 10% of T2 to avoid decoherence
        latency_us < (INVARIANTS.coherence_window_us / 10)
    }
}