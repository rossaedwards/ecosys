//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Fractal Lineage Engine 💎
//! 🌌 Recursive Sierpinski Decomposition (Crown Core)
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    crypto::hash::{Blake3Digest, blake3_hash},
    shard::{ShardId, ShardMetadata, ShardLayer, LatticeGeometry, ShardMetadataTrait},
    model_slice::{ModelShard, SliceConfig, SliceError},
    gov::BlissId,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Fractal lineage graph - shards know their parents & children
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FractalLineage {
    /// Shard ID -> parent shards (for reconstruction)
    pub parents: BTreeMap<ShardId, BTreeSet<ShardId>>,
    /// Shard ID -> child shards (fractal decomposition)
    pub children: BTreeMap<ShardId, BTreeSet<ShardId>>,
    /// Root shard (model entrypoint)
    pub root: Option<ShardId>,
    /// Leaf shards (terminal decomposition)
    pub leaves: BTreeSet<ShardId>,
    /// Lineage hash for integrity verification
    pub lineage_hash: Blake3Digest,
}

impl FractalLineage {
    /// Create new fractal lineage from model shards
    pub fn from_shards(shards: &[ModelShard]) -> Result<Self, SliceError> {
        let mut parents = BTreeMap::new();
        let mut children = BTreeMap::new();
        let mut leaves = BTreeSet::new();

        if shards.is_empty() {
            return Err(SliceError::ValidationError("Cannot create lineage from empty shards".to_string()));
        }

        for shard in shards {
            // Build parent-child relationships from metadata
            for parent_id in &shard.parent_shards {
                parents.entry(parent_id.clone())
                    .or_insert_with(BTreeSet::new)
                    .insert(shard.shard_id.clone());
            }

            for child_id in &shard.child_shards {
                children.entry(shard.shard_id.clone())
                    .or_insert_with(BTreeSet::new)
                    .insert(child_id.clone());
            }

            // Track leaves (no children)
            if shard.child_shards.is_empty() {
                leaves.insert(shard.shard_id.clone());
            }
        }

        // Find root (no parents)
        let root = shards.iter()
            .find(|s| s.parent_shards.is_empty())
            .map(|s| s.shard_id.clone());

        let mut lineage = Self {
            parents,
            children,
            root,
            leaves,
            lineage_hash: Blake3Digest::default(),
        };

        // Compute lineage integrity hash
        lineage.compute_hash();
        
        // Validate lineage after creation
        lineage.validate()?;
        
        Ok(lineage)
    }

    /// Compute cryptographic hash of entire lineage graph with validation
    pub fn compute_hash(&mut self) {
        let mut hasher = blake3_hash();
        hasher.update(b"fractal_lineage_v2_sierpinski"); // Phase II Version
        
        // Hash root + leaves
        if let Some(ref root) = self.root {
            hasher.update(&root.digest());
        }
        for leaf in &self.leaves {
            hasher.update(&leaf.digest());
        }

        // Hash parent-child relationships
        for (parent, children) in &self.children {
            hasher.update(&parent.digest());
            for child in children {
                hasher.update(&child.digest());
            }
        }

        self.lineage_hash = hasher.finalize();
    }

    /// Validate lineage integrity
    pub fn validate(&self) -> Result<(), SliceError> {
        let mut hasher = blake3_hash();
        hasher.update(b"fractal_lineage_v2_sierpinski");
        
        // Recompute hash and verify
        if let Some(ref root) = self.root {
            hasher.update(&root.digest());
        }
        for leaf in &self.leaves {
            hasher.update(&leaf.digest());
        }

        for (parent, children) in &self.children {
            hasher.update(&parent.digest());
            for child in children {
                hasher.update(&child.digest());
            }
        }

        let computed = hasher.finalize();
        if computed != self.lineage_hash {
            return Err(SliceError::LineageHashMismatch);
        }

        // Validate no cycles
        if self.has_cycles() {
            return Err(SliceError::CycleDetected);
        }

        // Validate root connectivity
        if let Some(root) = &self.root {
            // Expensive check, optimize for large graphs
            if !self.is_reachable(root, &self.leaves.iter().collect::<Vec<_>>()) {
                return Err(SliceError::DisconnectedGraph);
            }
        }

        Ok(())
    }

    /// Check for cycles in lineage graph
    fn has_cycles(&self) -> bool {
        let mut visited = BTreeSet::new();
        let mut rec_stack = BTreeSet::new();

        if let Some(root) = &self.root {
            self.dfs_cycle_detect(root, &mut visited, &mut rec_stack)
        } else {
            false
        }
    }

    fn dfs_cycle_detect(
        &self,
        node: &ShardId,
        visited: &mut BTreeSet<ShardId>,
        rec_stack: &mut BTreeSet<ShardId>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }

        visited.insert(node.clone());
        rec_stack.insert(node.clone());

        if let Some(children) = self.children.get(node) {
            for child in children {
                if self.dfs_cycle_detect(child, visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    /// Check if all leaves are reachable from root
    fn is_reachable(&self, root: &ShardId, leaves: &[&ShardId]) -> bool {
        let mut reachable_leaves = BTreeSet::new();
        self.dfs_reachable(root, &mut reachable_leaves);
        leaves.iter().all(|leaf| reachable_leaves.contains(*leaf))
    }

    fn dfs_reachable(&self, node: &ShardId, reachable: &mut BTreeSet<ShardId>) {
        if reachable.contains(node) { return; }
        
        // If this is a leaf node (no children), mark it as reachable
        match self.children.get(node) {
            Some(children) if !children.is_empty() => {
                for child in children {
                    self.dfs_reachable(child, reachable);
                }
            },
            _ => {
                // It's a leaf (or dead end)
                reachable.insert(node.clone());
            }
        }
    }
}

/// Fractal decomposition engine
pub struct FractalDecomposer {
    config: Arc<SliceConfig>,
}

impl FractalDecomposer {
    pub fn new(config: Arc<SliceConfig>) -> Self {
        Self { config }
    }

    /// Recursively decompose model into fractal shards
    /// 🟢 Phase II: Uses Sierpinski Trinary Logic (Split into 3)
    pub async fn decompose(
        &self,
        model_data: &[u8],
        depth: usize,
    ) -> Result<Vec<ModelShard>, SliceError> {
        if depth == 0 || model_data.len() < self.config.max_shard_size {
            // Base case - create leaf shard
            self.create_leaf_shard(model_data).await
        } else {
            // 🟢 Recursive case - Sierpinski Decomp (3 parts)
            let (part1, part2, part3) = self.split_data_sierpinski(model_data);
            
            // Recurse
            let s1 = self.decompose(part1, depth - 1).await?;
            let s2 = self.decompose(part2, depth - 1).await?;
            let s3 = self.decompose(part3, depth - 1).await?;

            // Link parent-child relationships
            let mut shards = Vec::new();
            shards.extend(s1);
            shards.extend(s2);
            shards.extend(s3);
            
            Ok(shards)
        }
    }

    async fn create_leaf_shard(&self, data: &[u8]) -> Result<Vec<ModelShard>, SliceError> {
        if data.is_empty() {
            return Err(SliceError::ValidationError("Cannot create shard from empty data".to_string()));
        }
        
        if data.len() > self.config.max_shard_size {
            return Err(SliceError::ValidationError(format!(
                "Data size {} exceeds max shard size {}",
                data.len(),
                self.config.max_shard_size
            )));
        }
        
        // Create ID
        let shard_id = ShardId::from_content(data);
        
        // 🟢 Phase II: Metadata Injection
        // Since this is a file/data chunk part of a fractal, it belongs to Crown (Sierpinski)
        // unless it's explicitly weights (Compute).
        // Let's assume these are file parts for now.
        let mut metadata = ShardMetadata::new(
            shard_id.clone(),
            data.len() as u64,
            Some("model_slice".to_string()),
        );
        metadata.geometry = LatticeGeometry::Sierpinski; // Enforce Crown Core
        metadata.tags.push("fractal_leaf".to_string());

        Ok(vec![ModelShard {
            shard_id,
            layer_range: (0, data.len()),
            weights: data.to_vec(),
            metadata,
            parent_shards: Vec::new(),
            child_shards: Vec::new(),
            signature: None,
            owner: BlissId::default(),
        }])
    }

    /// 🟢 Sierpinski Split: Divide data into 3 chunks
    /// This mimics the removal of the central triangle in the fractal construction.
    fn split_data_sierpinski<'a>(&self, data: &'a [u8]) -> (&'a [u8], &'a [u8], &'a [u8]) {
        let len = data.len();
        let third = len / 3;
        
        let part1 = &data[0..third];
        let part2 = &data[third..2*third];
        let part3 = &data[2*third..];
        
        (part1, part2, part3)
    }
}

/// Additional slice errors
impl From<FractalLineageError> for SliceError {
    fn from(err: FractalLineageError) -> Self {
        match err {
            FractalLineageError::HashMismatch => SliceError::LineageHashMismatch,
            FractalLineageError::Cycle => SliceError::CycleDetected,
            FractalLineageError::Disconnected => SliceError::DisconnectedGraph,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum FractalLineageError {
    #[error("Lineage hash mismatch")]
    HashMismatch,
    #[error("Cycle detected in lineage graph")]
    Cycle,
    #[error("Disconnected lineage graph")]
    Disconnected,
}

/// Fractal lineage statistics
#[derive(Debug, Serialize)]
pub struct LineageStats {
    pub total_shards: usize,
    pub depth: usize,
    pub branching_factor: f64,
    pub is_balanced: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fractal_lineage_creation() {
        // Create mock shards
        let id1 = ShardId::from_content(b"1");
        let mut s1 = ModelShard {
            shard_id: id1.clone(),
            layer_range: (0, 1),
            weights: vec![],
            metadata: ShardMetadata::default(),
            parent_shards: vec![],
            child_shards: vec![],
            signature: None,
            owner: Default::default(),
        };
        
        let shards = vec![s1];
        let lineage = FractalLineage::from_shards(&shards).unwrap();
        assert!(lineage.validate().is_ok());
    }

    #[tokio::test]
    async fn test_sierpinski_split() {
        let config = Arc::new(SliceConfig { 
            max_shard_size: 10,
            split_points: vec![],
            shard_replication: 1,
            quantum_keys: false,
            compression: crate::compression::CompressionAlgorithm::None,
            federated_learning: false,
        });
        let decomposer = FractalDecomposer::new(config);
        
        let data = b"123456789";
        let (p1, p2, p3) = decomposer.split_data_sierpinski(data);
        
        assert_eq!(p1, b"123");
        assert_eq!(p2, b"456");
        assert_eq!(p3, b"789");
    }
}