//! ═══════════════════════════════════════════════════════════════════
//! 🌳 AuraFS Core Merkle Tree - Cryptographic Verification Structure
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Production-grade Merkle tree implementation with:
//! - SHA3-256 or Blake3 hash functions
//! - Proof generation and verification
//! - Partial tree updates
//! - Serialization support
//! ═══════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use sha3::{Sha3_256, Digest};

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, internal, client, ShardId};

/// Hash type used in Merkle tree
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MerkleHash(pub Vec<u8>);

impl MerkleHash {
    /// Create from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    
    /// Create from hex string
    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)
            .map_err(|e| client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: format!("Invalid hex: {}", e),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ))?;
        Ok(Self(bytes))
    }
    
    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
    
    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for MerkleHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Merkle proof for verifying inclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Leaf hash being proven
    pub leaf_hash: MerkleHash,
    /// Path of sibling hashes from leaf to root
    pub path: Vec<ProofNode>,
    /// Root hash
    pub root_hash: MerkleHash,
    /// Leaf index in tree
    pub leaf_index: usize,
    /// Total leaves in tree
    pub total_leaves: usize,
}

/// Node in Merkle proof path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofNode {
    /// Sibling hash
    pub hash: MerkleHash,
    /// Whether sibling is on the left
    pub is_left: bool,
}

impl MerkleProof {
    /// Verify the proof
    pub fn verify(&self) -> bool {
        if self.path.is_empty() && self.total_leaves == 1 {
            // Single leaf tree
            return self.leaf_hash == self.root_hash;
        }
        
        let mut current = self.leaf_hash.clone();
        
        for node in &self.path {
            let combined = if node.is_left {
                // Sibling is on the left
                [node.hash.as_bytes(), current.as_bytes()].concat()
            } else {
                // Sibling is on the right
                [current.as_bytes(), node.hash.as_bytes()].concat()
            };
            
            current = hash_data(&combined);
        }
        
        current == self.root_hash
    }
}

/// Merkle tree node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    /// Node hash
    pub hash: MerkleHash,
    /// Left child index (None for leaves)
    pub left: Option<usize>,
    /// Right child index (None for leaves)
    pub right: Option<usize>,
    /// Parent index (None for root)
    pub parent: Option<usize>,
    /// Whether this is a leaf
    pub is_leaf: bool,
    /// Data hash for leaves
    pub data_hash: Option<MerkleHash>,
}

/// Merkle tree implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    /// All nodes in the tree
    nodes: Vec<MerkleNode>,
    /// Root index
    root_index: Option<usize>,
    /// Leaf count
    leaf_count: usize,
    /// Map from data hash to leaf index
    leaf_indices: HashMap<MerkleHash, usize>,
    /// Async-pending leaves by data hash
    pending_leaves: HashSet<MerkleHash>,
}

impl MerkleTree {
    /// Create empty Merkle tree
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_index: None,
            leaf_count: 0,
            leaf_indices: HashMap::new(),
            pending_leaves: HashSet::new(),
        }
    }
    
    /// Build Merkle tree from leaf data
    pub fn from_leaves(leaves: &[&[u8]]) -> Result<Self> {
        if leaves.is_empty() {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: "Cannot build Merkle tree from empty leaves".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ));
        }
        
        let mut tree = Self::new();
        
        // Create leaf nodes
        let mut current_level: Vec<usize> = leaves.iter()
            .enumerate()
            .map(|(i, data)| {
                let data_hash = hash_data(data);
                let leaf_hash = hash_leaf(&data_hash);
                
                tree.leaf_indices.insert(data_hash.clone(), tree.nodes.len());
                
                tree.nodes.push(MerkleNode {
                    hash: leaf_hash,
                    left: None,
                    right: None,
                    parent: None,
                    is_leaf: true,
                    data_hash: Some(data_hash),
                });
                
                tree.nodes.len() - 1
            })
            .collect();
        
        tree.leaf_count = current_level.len();
        
        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let left_idx = chunk[0];
                let right_idx = if chunk.len() > 1 {
                    chunk[1]
                } else {
                    // Odd number of nodes, duplicate the last one
                    chunk[0]
                };
                
                let combined_hash = hash_internal(
                    &tree.nodes[left_idx].hash,
                    &tree.nodes[right_idx].hash,
                );
                
                let parent_idx = tree.nodes.len();
                tree.nodes.push(MerkleNode {
                    hash: combined_hash,
                    left: Some(left_idx),
                    right: Some(right_idx),
                    parent: None,
                    is_leaf: false,
                    data_hash: None,
                });
                
                // Update children's parent reference
                tree.nodes[left_idx].parent = Some(parent_idx);
                if right_idx != left_idx {
                    tree.nodes[right_idx].parent = Some(parent_idx);
                }
                
                next_level.push(parent_idx);
            }
            
            current_level = next_level;
        }
        
        tree.root_index = current_level.first().copied();
        
        Ok(tree)
    }
    
    /// Build from shard IDs
    pub fn from_shard_ids(shard_ids: &[ShardId]) -> Result<Self> {
        if shard_ids.is_empty() {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: "Cannot build Merkle tree from empty shard list".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ));
        }
        
        let leaves: Vec<&[u8]> = shard_ids.iter()
            .map(|s| s.as_bytes())
            .collect();
        
        Self::from_leaves(&leaves)
    }
    
    /// Get root hash
    pub fn root(&self) -> Option<&MerkleHash> {
        self.root_index.map(|idx| &self.nodes[idx].hash)
    }
    
    /// Get root hash as string
    pub fn root_hex(&self) -> Option<String> {
        self.root().map(|h| h.to_hex())
    }
    
    /// Get leaf count
    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }
    
    /// Get total node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Mark a leaf as async-pending using its data hash.
    pub fn mark_async_pending(&mut self, data_hash: &MerkleHash) -> Result<()> {
        let leaf_index = self.leaf_indices.get(data_hash).ok_or_else(|| client(
            AuraFSError::Crypto {
                code: ErrorCode::NotFound,
                message: "Leaf not found for async-pending mark".to_string(),
            },
            ErrorPhase::Crypto,
            ErrorCode::NotFound,
        ))?;

        if self.pending_leaves.insert(data_hash.clone()) {
            let pending_hash = hash_pending(data_hash);
            self.nodes[*leaf_index].hash = pending_hash;
            self.recalculate_upwards(*leaf_index);
        }

        Ok(())
    }

    /// Mark a leaf as async-pending using raw data.
    pub fn mark_async_pending_by_data(&mut self, data: &[u8]) -> Result<()> {
        let data_hash = hash_data(data);
        self.mark_async_pending(&data_hash)
    }

    /// Check if a leaf is marked async-pending.
    pub fn is_async_pending(&self, data_hash: &MerkleHash) -> bool {
        self.pending_leaves.contains(data_hash)
    }
    
    /// Generate proof for a leaf by index
    pub fn proof_by_index(&self, leaf_index: usize) -> Result<MerkleProof> {
        if leaf_index >= self.leaf_count {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: format!("Leaf index {} out of range (max: {})", leaf_index, self.leaf_count - 1),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ));
        }
        
        let root = self.root().ok_or_else(|| internal(
            AuraFSError::Crypto {
                code: ErrorCode::InternalInconsistency,
                message: "Merkle tree has no root".to_string(),
            },
            ErrorPhase::Crypto,
        ))?;
        
        let leaf_hash = self.nodes[leaf_index].hash.clone();
        let mut path = Vec::new();
        let mut current_idx = leaf_index;
        
        while let Some(parent_idx) = self.nodes[current_idx].parent {
            let parent = &self.nodes[parent_idx];
            
            // Determine if current node is left or right child
            let (sibling_idx, is_left) = if parent.left == Some(current_idx) {
                (parent.right.unwrap_or(current_idx), false)
            } else {
                (parent.left.unwrap_or(current_idx), true)
            };
            
            // Don't add proof node if sibling is the same (odd tree case)
            if sibling_idx != current_idx {
                path.push(ProofNode {
                    hash: self.nodes[sibling_idx].hash.clone(),
                    is_left,
                });
            }
            
            current_idx = parent_idx;
        }
        
        Ok(MerkleProof {
            leaf_hash,
            path,
            root_hash: root.clone(),
            leaf_index,
            total_leaves: self.leaf_count,
        })
    }
    
    /// Generate proof for a leaf by data
    pub fn proof_by_data(&self, data: &[u8]) -> Result<MerkleProof> {
        let data_hash = hash_data(data);
        let leaf_index = self.leaf_indices.get(&data_hash)
            .ok_or_else(|| client(
                AuraFSError::Crypto {
                    code: ErrorCode::NotFound,
                    message: "Data not found in Merkle tree".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::NotFound,
            ))?;
        
        self.proof_by_index(*leaf_index)
    }
    
    /// Verify that data is included in tree
    pub fn verify_inclusion(&self, data: &[u8]) -> Result<bool> {
        match self.proof_by_data(data) {
            Ok(proof) => Ok(proof.verify()),
            Err(_) => Ok(false),
        }
    }
    
    /// Verify proof against this tree's root
    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        if let Some(root) = self.root() {
            proof.root_hash == *root && proof.verify()
        } else {
            false
        }
    }

    /// Recalculate hashes from a leaf up to root.
    fn recalculate_upwards(&mut self, start_index: usize) {
        let mut current = start_index;
        while let Some(parent_idx) = self.nodes[current].parent {
            let left_idx = self.nodes[parent_idx].left.unwrap_or(current);
            let right_idx = self.nodes[parent_idx].right.unwrap_or(current);
            let combined_hash = hash_internal(
                &self.nodes[left_idx].hash,
                &self.nodes[right_idx].hash,
            );
            self.nodes[parent_idx].hash = combined_hash;
            current = parent_idx;
        }
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash data using SHA3-256
fn hash_data(data: &[u8]) -> MerkleHash {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    MerkleHash(hasher.finalize().to_vec())
}

/// Hash a leaf node (prefix with 0x00)
fn hash_leaf(data_hash: &MerkleHash) -> MerkleHash {
    let mut hasher = Sha3_256::new();
    hasher.update(&[0x00]); // Leaf prefix
    hasher.update(data_hash.as_bytes());
    MerkleHash(hasher.finalize().to_vec())
}

/// Hash an internal node (prefix with 0x01)
fn hash_internal(left: &MerkleHash, right: &MerkleHash) -> MerkleHash {
    let mut hasher = Sha3_256::new();
    hasher.update(&[0x01]); // Internal node prefix
    hasher.update(left.as_bytes());
    hasher.update(right.as_bytes());
    MerkleHash(hasher.finalize().to_vec())
}

/// Hash a pending leaf node (prefix with 0x02)
fn hash_pending(data_hash: &MerkleHash) -> MerkleHash {
    let mut hasher = Sha3_256::new();
    hasher.update(&[0x02]); // Pending leaf prefix
    hasher.update(data_hash.as_bytes());
    MerkleHash(hasher.finalize().to_vec())
}

/// Utility functions for Merkle operations
pub mod utils {
    use super::*;
    
    /// Calculate Merkle root from a list of hashes without building full tree
    pub fn calculate_root(leaves: &[MerkleHash]) -> Result<MerkleHash> {
        if leaves.is_empty() {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: "Cannot calculate root from empty leaves".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ));
        }
        
        if leaves.len() == 1 {
            return Ok(hash_leaf(&leaves[0]));
        }
        
        // Hash all leaves
        let mut current_level: Vec<MerkleHash> = leaves.iter()
            .map(|h| hash_leaf(h))
            .collect();
        
        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let left = &chunk[0];
                let right = if chunk.len() > 1 { &chunk[1] } else { &chunk[0] };
                next_level.push(hash_internal(left, right));
            }
            
            current_level = next_level;
        }
        
        Ok(current_level.into_iter().next().unwrap())
    }
    
    /// Verify a standalone proof without full tree
    pub fn verify_proof(proof: &MerkleProof) -> bool {
        proof.verify()
    }
    
    /// Calculate tree depth for given leaf count
    pub fn tree_depth(leaf_count: usize) -> usize {
        if leaf_count <= 1 {
            0
        } else {
            (leaf_count as f64).log2().ceil() as usize
        }
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merkle_tree_single_leaf() {
        let leaves = vec![b"hello".as_slice()];
        let tree = MerkleTree::from_leaves(&leaves).unwrap();
        
        assert_eq!(tree.leaf_count(), 1);
        assert!(tree.root().is_some());
    }
    
    #[test]
    fn test_merkle_tree_multiple_leaves() {
        let leaves = vec![
            b"hello".as_slice(),
            b"world".as_slice(),
            b"foo".as_slice(),
            b"bar".as_slice(),
        ];
        
        let tree = MerkleTree::from_leaves(&leaves).unwrap();
        
        assert_eq!(tree.leaf_count(), 4);
        assert!(tree.root().is_some());
        
        // Verify all leaves
        for leaf in &leaves {
            assert!(tree.verify_inclusion(leaf).unwrap());
        }
        
        // Verify non-existent data
        assert!(!tree.verify_inclusion(b"nonexistent").unwrap());
    }
    
    #[test]
    fn test_merkle_proof_verification() {
        let leaves = vec![
            b"a".as_slice(),
            b"b".as_slice(),
            b"c".as_slice(),
            b"d".as_slice(),
            b"e".as_slice(),
        ];
        
        let tree = MerkleTree::from_leaves(&leaves).unwrap();
        
        // Generate and verify proof for each leaf
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.proof_by_index(i).unwrap();
            assert!(proof.verify(), "Proof for leaf {} should verify", i);
            assert!(tree.verify_proof(&proof), "Tree should verify proof for leaf {}", i);
            
            let proof_by_data = tree.proof_by_data(leaf).unwrap();
            assert!(proof_by_data.verify(), "Proof by data for leaf {} should verify", i);
        }
    }
    
    #[test]
    fn test_merkle_tree_odd_leaves() {
        let leaves = vec![
            b"one".as_slice(),
            b"two".as_slice(),
            b"three".as_slice(),
        ];
        
        let tree = MerkleTree::from_leaves(&leaves).unwrap();
        
        assert_eq!(tree.leaf_count(), 3);
        
        // All proofs should verify
        for i in 0..3 {
            let proof = tree.proof_by_index(i).unwrap();
            assert!(proof.verify());
        }
    }
    
    #[test]
    fn test_merkle_hash_consistency() {
        let data = b"test data";
        let hash1 = hash_data(data);
        let hash2 = hash_data(data);
        
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.to_hex(), hash2.to_hex());
    }
    
    #[test]
    fn test_calculate_root_utility() {
        let hashes: Vec<MerkleHash> = vec![
            hash_data(b"a"),
            hash_data(b"b"),
            hash_data(b"c"),
            hash_data(b"d"),
        ];
        
        let root = utils::calculate_root(&hashes).unwrap();
        
        // Compare with full tree
        let leaves = vec![
            b"a".as_slice(),
            b"b".as_slice(),
            b"c".as_slice(),
            b"d".as_slice(),
        ];
        let tree = MerkleTree::from_leaves(&leaves).unwrap();
        
        assert_eq!(root, *tree.root().unwrap());
    }
    
    #[test]
    fn test_tree_depth() {
        assert_eq!(utils::tree_depth(1), 0);
        assert_eq!(utils::tree_depth(2), 1);
        assert_eq!(utils::tree_depth(3), 2);
        assert_eq!(utils::tree_depth(4), 2);
        assert_eq!(utils::tree_depth(5), 3);
        assert_eq!(utils::tree_depth(8), 3);
        assert_eq!(utils::tree_depth(9), 4);
    }

    #[test]
    fn test_async_pending_leaf_mark() {
        let leaves = vec![
            b"alpha".as_slice(),
            b"beta".as_slice(),
        ];
        let mut tree = MerkleTree::from_leaves(&leaves).unwrap();
        let original_root = tree.root_hex().unwrap();

        tree.mark_async_pending_by_data(b"alpha").unwrap();
        assert!(tree.is_async_pending(&hash_data(b"alpha")));

        let new_root = tree.root_hex().unwrap();
        assert_ne!(original_root, new_root);
    }
}
