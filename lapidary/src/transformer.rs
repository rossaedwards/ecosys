//! Ceremonial Transmutation Engine Passes (Lapidary Transformer)
//!
//! Provides an extensible, high-performance transformation engine over the Universal AST.
//! Supports dynamic, composable transformation passes with rich pattern-based rewrites,
//! contextual awareness using the LapidaryContext, async-ready design, and detailed logging.
//!
//! Designed to be the core workhorse for symbiotic transformations in the Lapidary engine.
//! Compliant with Symbiotic Xessability Standards.

use anyhow::{Context, Result};
use crate::context::{LapidaryContext, Symbol, SymbolType, XessMode};
use crate::fute::ast::{AstNode, UniversalAst};

/// Defines the primary architectural trajectory identified during pre-evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmutationTarget {
    /// Pure standalone LSP engine pass-through
    PreserveBinary, 
    /// Requires runtime abstraction translation (Node -> WASM)
    InjectWasmShim, 
    /// Heavy GUI element isolation required
    StripBloatUI,   
}

/// The core engine responsible for mutating the AST and registering structural changes.
pub struct ExtensionPass;

impl ExtensionPass {
    /// Initializes a new AST transformation pass.
    pub fn new() -> Self {
        Self {}
    }

    /// Executes the primary transmutation pass over the given Universal AST.
    /// Mutates the tree based on the active `XessMode` and context parameters,
    /// generating telemetry and populating the context's symbol table.
    pub fn transmutate_ast(&self, mut ast: UniversalAst, ctx: &mut LapidaryContext) -> Result<UniversalAst> {
        ctx.log_info("Transformer", format!("Initiating AST Transmutation pass in {:?} mode.", ctx.mode));

        // Extract the root node and run the recursive transformation matrix
        let transformed_root = self.process_node(ast.root, ctx)
            .context("Root node was completely stripped, resulting in an invalid AST state.")?;

        ast.root = transformed_root;

        ctx.log_info("Transformer", "AST Transmutation pass completed successfully.");
        Ok(ast)
    }

    /// Recursively processes nodes, applying stripping, transmutation, and symbol registration.
    fn process_node(&self, node: AstNode, ctx: &mut LapidaryContext) -> Option<AstNode> {
        match node {
            AstNode::ExtensionBundle { name, version, publisher, description, children } => {
                // Process all children recursively, filtering out any that return None (stripped nodes)
                let transformed_children: Vec<AstNode> = children
                    .into_iter()
                    .filter_map(|child| self.process_node(child, ctx))
                    .collect();

                Some(AstNode::ExtensionBundle {
                    name,
                    version,
                    publisher,
                    description,
                    children: transformed_children,
                })
            }

            AstNode::QuantumBinary { executable_path, execution_args, profile } => {
                ctx.log_info("Transformer", format!("Preserved QuantumBinary region: {}", executable_path));
                
                // Register the pure binary symbol into the telemetry matrix
                ctx.register_symbol(Symbol {
                    identifier: format!("quantum_bin_{}", executable_path.replace('/', "_")),
                    symbol_type: SymbolType::BinaryExecutable,
                    original_path: executable_path.clone(),
                });

                Some(AstNode::QuantumBinary { executable_path, execution_args, profile })
            }

            AstNode::LatticeScript { entry_point, dependencies } => {
                ctx.log_warning("Transformer", format!("Transmuting LatticeScript (Node.js) into WASM Proxy: {}", entry_point));

                // Register the original node script before it gets transmuted
                ctx.register_symbol(Symbol {
                    identifier: format!("lattice_script_{}", entry_point.replace('/', "_")),
                    symbol_type: SymbolType::NodeScript,
                    original_path: entry_point.clone(),
                });

                // Calculate WASI memory pages based on dependencies heuristics
                let memory_pages = if dependencies.len() > 10 { 4 } else { 2 };

                // Transmute into a highly optimized WasmProxyShim
                let shim_node = AstNode::WasmProxyShim {
                    target_script: entry_point.clone(),
                    memory_pages,
                };

                // Register the newly synthesized WASM proxy artifact
                ctx.register_symbol(Symbol {
                    identifier: format!("wasm_proxy_{}", entry_point.replace('/', "_")),
                    symbol_type: SymbolType::WasmProxy,
                    original_path: "bin/lapidary-shim.wasm".to_string(),
                });

                Some(shim_node)
            }

            AstNode::WebviewBloat { view_id, bloat_weight } => {
                // Stripping behavior depends strictly on the XessMode matrix
                if ctx.mode == XessMode::Standard && bloat_weight < 5000 {
                    ctx.log_warning("Transformer", format!("Preserving WebviewBloat '{}' due to Standard mode tolerance.", view_id));
                    Some(AstNode::WebviewBloat { view_id, bloat_weight })
                } else {
                    ctx.log_info("Transformer", format!("Stripped WebviewBloat '{}' (Weight: {}).", view_id, bloat_weight));
                    None // Drop the node entirely from the AST
                }
            }

            AstNode::ConfigurationBlock { properties } => {
                ctx.log_info("Transformer", format!("Preserved ConfigurationBlock with {} properties.", properties.len()));
                
                // Register the configuration map
                ctx.register_symbol(Symbol {
                    identifier: "extension_config_matrix".to_string(),
                    symbol_type: SymbolType::ConfigurationMap,
                    original_path: "package.json::contributes.configuration".to_string(),
                });

                Some(AstNode::ConfigurationBlock { properties })
            }

            AstNode::ActivationEvent { event_type, target } => {
                // Activation events are strictly passed through
                Some(AstNode::ActivationEvent { event_type, target })
            }

            AstNode::WasmProxyShim { .. } => {
                // Already transmuted, pass it through natively
                Some(node)
            }
        }
    }
    
    /// Evaluates a raw manifest string to determine the primary architectural target.
    /// This bridges compatibility with standard scaffolding pipelines.
    pub fn evaluate_regions(&self, raw_json: &str, ctx: &mut LapidaryContext) -> Result<TransmutationTarget> {
        ctx.log_info("Transformer", "Evaluating architectural regions for dynamic targeting.");
        
        if raw_json.contains("rust-analyzer") || raw_json.contains("gopls") {
            Ok(TransmutationTarget::PreserveBinary)
        } else if raw_json.contains("views") || raw_json.contains("webview") {
            Ok(TransmutationTarget::StripBloatUI)
        } else {
            Ok(TransmutationTarget::InjectWasmShim)
        }
    }
}

impl Default for ExtensionPass {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIT TESTS (Validation & Integrity)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fute::ast::{UniversalAst, ExecutionProfile};

    fn create_test_ast() -> UniversalAst {
        let root = AstNode::ExtensionBundle {
            name: "test-ext".to_string(),
            version: "1.0".to_string(),
            publisher: "lapidary".to_string(),
            description: "".to_string(),
            children: vec![
                AstNode::LatticeScript {
                    entry_point: "dist/server.js".to_string(),
                    dependencies: vec!["typescript".to_string()],
                },
                AstNode::WebviewBloat {
                    view_id: "heavy-panel".to_string(),
                    bloat_weight: 9000,
                },
            ],
        };
        UniversalAst::new(root)
    }

    #[test]
    fn test_sacred_mode_strips_bloat_and_transmutes_lattice() {
        let transformer = ExtensionPass::new();
        let mut ctx = LapidaryContext::new().with_mode(XessMode::Sacred);
        let ast = create_test_ast();

        let transmuted_ast = transformer.transmutate_ast(ast, &mut ctx).unwrap();

        if let AstNode::ExtensionBundle { children, .. } = transmuted_ast.root {
            assert_eq!(children.len(), 1, "WebviewBloat should have been stripped.");
            
            // Check that the LatticeScript was transmuted into a WasmProxyShim
            match &children[0] {
                AstNode::WasmProxyShim { target_script, .. } => {
                    assert_eq!(target_script, "dist/server.js");
                }
                _ => panic!("Expected WasmProxyShim, found something else."),
            }
        } else {
            panic!("Root node mutated unexpectedly.");
        }

        // Verify symbol tracking was correctly populated
        assert!(ctx.get_symbol("lattice_script_dist_server.js").is_some());
        assert!(ctx.get_symbol("wasm_proxy_dist_server.js").is_some());
    }

    #[test]
    fn test_standard_mode_preserves_lightweight_bloat() {
        let transformer = ExtensionPass::new();
        let mut ctx = LapidaryContext::new().with_mode(XessMode::Standard);
        
        let root = AstNode::ExtensionBundle {
            name: "test".to_string(),
            version: "1.0".to_string(),
            publisher: "lap".to_string(),
            description: "".to_string(),
            children: vec![
                AstNode::WebviewBloat {
                    view_id: "light-panel".to_string(),
                    bloat_weight: 1000, // Below 5000 threshold
                },
            ],
        };
        let ast = UniversalAst::new(root);

        let transmuted_ast = transformer.transmutate_ast(ast, &mut ctx).unwrap();

        if let AstNode::ExtensionBundle { children, .. } = transmuted_ast.root {
            assert_eq!(children.len(), 1, "Lightweight bloat should be preserved in Standard mode.");
        }
    }
}