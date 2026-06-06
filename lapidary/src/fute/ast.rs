//! Fuxyez Universal Grammar & AST Bridge Matrix for Lapidary
//! 
//! This module provides the high-fidelity Abstract Syntax Tree required to map 
//! closed-ecosystem VS Code manifests into native Lapce configurations.
//! Compliant with Aurphyx Symbiotic Universal Xessability Standards (SUXS).

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// The root container for a transmuted extension tree.
/// Holds the master node and structural telemetry required by the pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAst {
    pub root: AstNode,
    pub metadata: AstMetadata,
}

/// Core metadata tracking the state of the transmutation across the FUTE pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AstMetadata {
    pub source_schema: String,
    pub target_runtime: String,
    pub symbiotic_mode: SymbioticMode,
    pub original_publisher: String,
    pub engine_warnings: Vec<String>,
}

/// Operational modes guiding the ceremonial transformation passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SymbioticMode {
    #[default]
    Standard,
    Sacred,
    Mystical,
}

/// The core Universal Node Enum.
/// Represents both abstract manifest structures and concrete execution targets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstNode {
    /// The root payload representing the entire parsed extension bundle.
    ExtensionBundle {
        name: String,
        version: String,
        publisher: String,
        description: String,
        children: Vec<AstNode>,
    },

    /// QUANTUM REGION: Represents a pure, standalone compiled binary language server.
    /// These pass through the transmutation engine nearly untouched.
    QuantumBinary {
        executable_path: String,
        execution_args: Vec<String>,
        profile: ExecutionProfile,
    },

    /// LATTICE REGION: Represents a node.js or heavily V8-dependent execution script.
    /// These trigger the ceremonial transformer to inject WASM shims.
    LatticeScript {
        entry_point: String,
        dependencies: Vec<String>,
    },

    /// SYNTHESIZED: The generated WebAssembly proxy that replaces a LatticeScript.
    WasmProxyShim {
        target_script: String,
        memory_pages: u32,
    },

    /// PRESENTATION BLOAT: VS Code specific UI elements (Webviews, Sidebars).
    /// These are explicitly targeted for extraction/stripping during the pipeline.
    WebviewBloat {
        view_id: String,
        bloat_weight: u32,
    },

    /// Configuration maps extracted from `contributes.configuration`.
    ConfigurationBlock {
        properties: HashMap<String, ConfigValue>,
    },

    /// Language triggers and file-extension hooks.
    ActivationEvent {
        event_type: ActivationType,
        target: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProfile {
    pub is_daemon: bool,
    pub requires_stdio: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigValue {
    String(String),
    Boolean(bool),
    Integer(i64),
    Array(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationType {
    OnLanguage,
    OnCommand,
    WorkspaceContains,
    OnStartup,
}

impl Default for ExecutionProfile {
    fn default() -> Self {
        Self {
            is_daemon: true,
            requires_stdio: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AST TRAVERSAL & MUTATION IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl UniversalAst {
    /// Bootstraps a new AST with a default empty root.
    pub fn new(root: AstNode) -> Self {
        Self {
            root,
            metadata: AstMetadata {
                source_schema: "vsix-manifest-v1".to_string(),
                target_runtime: "lapce-volt".to_string(),
                symbiotic_mode: SymbioticMode::Sacred,
                original_publisher: "unknown".to_string(),
                engine_warnings: Vec::new(),
            },
        }
    }

    /// Recursively calculates the total number of structural nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.root.count_descendants() + 1
    }

    /// Primary mutation entry point for the Ceremonial Transformer.
    /// Allows in-place rewriting of the AST matrix.
    pub fn traverse_mut<F>(&mut self, mutator_func: &mut F)
    where
        F: FnMut(&mut AstNode),
    {
        self.root.traverse_mut(mutator_func);
    }
}

impl AstNode {
    /// Recursively counts all child nodes.
    pub fn count_descendants(&self) -> usize {
        match self {
            AstNode::ExtensionBundle { children, .. } => {
                children.iter().map(|c| c.count_descendants() + 1).sum()
            }
            _ => 0, // Leaf nodes have no descendants
        }
    }

    /// Deep mutable traversal. Visits the current node, then recursively visits children.
    pub fn traverse_mut<F>(&mut self, mutator: &mut F)
    where
        F: FnMut(&mut AstNode),
    {
        // Mutate current node first (Pre-order traversal)
        mutator(self);

        // Then recursively traverse children if the node is a container
        if let AstNode::ExtensionBundle { children, .. } = self {
            for child in children.iter_mut() {
                child.traverse_mut(mutator);
            }
        }
    }

    /// Helper to identify if the current node is a pure binary execution target.
    pub fn is_quantum_region(&self) -> bool {
        matches!(self, AstNode::QuantumBinary { .. })
    }

    /// Helper to identify if the current node is bloated IDE UI.
    pub fn is_presentation_bloat(&self) -> bool {
        matches!(self, AstNode::WebviewBloat { .. })
    }

    /// Extracts all configuration properties bound within this tree.
    pub fn extract_configurations(&self) -> HashMap<String, ConfigValue> {
        let mut configs = HashMap::new();
        
        // Internal recursive closure
        fn walk(node: &AstNode, map: &mut HashMap<String, ConfigValue>) {
            match node {
                AstNode::ConfigurationBlock { properties } => {
                    for (k, v) in properties {
                        map.insert(k.clone(), v.clone());
                    }
                }
                AstNode::ExtensionBundle { children, .. } => {
                    for child in children {
                        walk(child, map);
                    }
                }
                _ => {}
            }
        }
        
        walk(self, &mut configs);
        configs
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIT TESTS (Validation & Sanity Checks)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_creation_and_counting() {
        let root = AstNode::ExtensionBundle {
            name: "test-ext".to_string(),
            version: "1.0.0".to_string(),
            publisher: "Aurphyx".to_string(),
            description: "Test bundle".to_string(),
            children: vec![
                AstNode::QuantumBinary {
                    executable_path: "bin/server".to_string(),
                    execution_args: vec!["--stdio".to_string()],
                    profile: ExecutionProfile::default(),
                },
                AstNode::WebviewBloat {
                    view_id: "heavy-sidebar".to_string(),
                    bloat_weight: 9000,
                },
            ],
        };

        let ast = UniversalAst::new(root);
        assert_eq!(ast.node_count(), 3); // 1 Root + 2 Children
    }

    #[test]
    fn test_mutable_traversal_stripping() {
        let mut root = AstNode::ExtensionBundle {
            name: "mut-test".to_string(),
            version: "0.1".to_string(),
            publisher: "sys".to_string(),
            description: "".to_string(),
            children: vec![
                AstNode::WebviewBloat { view_id: "ui".to_string(), bloat_weight: 100 },
                AstNode::LatticeScript { entry_point: "main.js".to_string(), dependencies: vec![] },
            ],
        };

        // Simulate the Ceremonial Transformer pass
        root.traverse_mut(&mut |node| {
            if let AstNode::LatticeScript { entry_point, .. } = node {
                // Transmute JS scripts into WASM Shims
                *node = AstNode::WasmProxyShim {
                    target_script: entry_point.clone(),
                    memory_pages: 1,
                };
            }
        });

        // Verify mutation
        if let AstNode::ExtensionBundle { children, .. } = root {
            assert!(matches!(children[1], AstNode::WasmProxyShim { .. }));
        } else {
            panic!("Root was unexpectedly mutated");
        }
    }
}