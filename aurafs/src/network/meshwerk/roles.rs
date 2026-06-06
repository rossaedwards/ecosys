//! Meshwerk role assignment based on tier profiles.

use crate::network::meshwerk::mesh_node::MeshTier;

/// [Theorem 3.1: Universality]
/// Role classifications used by Meshwerk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshRole {
    Repeater,
    Storage,
    Backbone,
}

impl MeshRole {
    /// [Theorem 3.1: Universality]
    pub fn from_tier(tier: MeshTier) -> Self {
        match tier {
            MeshTier::GhostLink => MeshRole::Repeater,
            MeshTier::DataSlayer => MeshRole::Storage,
            MeshTier::Titan => MeshRole::Backbone,
        }
    }
}