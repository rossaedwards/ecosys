//! Semantic Analysis & Type Inference - Fuxyez Transformer
//!
//! High-level semantic passes for Universal AST:
//! - Full type inference (primitives, quantum types, lattices)
//! - Contextual error/warning annotation
//! - Dataflow/usage checks for quantum/classical/lattice logic
//! - Symbol resolution, shadowing detection, scope verification
//!
//! Integrates with core context and patterns for rich diagnostics and transformation feedback.

use anyhow::{Result, anyhow};
use crate::{
    ast::*,
    core::context::{TransmutationContext, Symbol, SymbolKind},
    patterns::{PatternLibrary, PatternDomain, PatternDetector},
};

/// Core semantic analyzer struct supporting extensible analysis passes.
pub struct SemanticAnalyzer {
    patterns: PatternLibrary,
}

impl SemanticAnalyzer {
    pub fn new(patterns: PatternLibrary) -> Self {
        Self { patterns }
    }

    /// Top-level semantic analysis entry point.
    pub fn analyze(
        &self,
        ast: &mut UniversalAst,
        context: &mut TransmutationContext,
    ) -> Result<()> {
        log::info!("🧠 [Semantic] Global semantic analysis starting...");

        // Run main passes
        self.build_symbol_table(&ast.root, context)?;
        self.type_inference(&ast.root, context)?;
        self.region_semantics(&ast.root, context)?;
        self.check_scopes_and_shadowing(&ast.root, context)?;
        self.detect_potential_issues(&ast.root, context)?;

        log::info!("🧠 [Semantic] All analysis phases complete.");
        Ok(())
    }

    /// Build symbol tables for all defined symbols, recursively.
    fn build_symbol_table(&self, node: &AstNode, context: &mut TransmutationContext) -> Result<()> {
        match node {
            AstNode::Module { name, items } => {
                for item in items {
                    self.build_symbol_table(item, context)?;
                }
                context.add_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Module,
                    scope: "global".to_string(),
                    ty: None,
                });
            }
            AstNode::Function { name, params, return_type, .. } => {
                context.add_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Function,
                    scope: "function".to_string(),
                    ty: return_type.as_ref().map(|t| format!("{:?}", t)),
                });
                for p in params {
                    context.add_symbol(Symbol {
                        name: p.name.clone(),
                        kind: SymbolKind::Variable,
                        scope: name.clone(),
                        ty: Some(format!("{:?}", p.ty)),
                    });
                }
            }
            AstNode::Struct { name, fields, .. } => {
                context.add_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Type,
                    scope: "global".to_string(),
                    ty: None,
                });
                for field in fields {
                    context.add_symbol(Symbol {
                        name: field.name.clone(),
                        kind: SymbolKind::Variable,
                        scope: name.clone(),
                        ty: Some(format!("{:?}", field.ty)),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Infer types where possible (including quantum/lattice types via pattern matches).
    fn type_inference(&self, node: &AstNode, context: &mut TransmutationContext) -> Result<()> {
        match node {
            AstNode::VarDecl { name, ty, value, .. } => {
                // Infer type from value or pattern
                if ty.is_none() {
                    if let Some(val) = value {
                        let inferred_ty = Self::infer_type_from_node(val, &self.patterns)?;
                        // Attach type info (for real engine, prefer context mutation/AST mutation)
                        context.set_metadata(format!("type:{}", name), inferred_ty.clone());
                    }
                }
            }
            AstNode::Function { body, .. } => {
                for stmt in body {
                    self.type_inference(stmt, context)?;
                }
            }
            AstNode::Block(stmts) => {
                for stmt in stmts {
                    self.type_inference(stmt, context)?;
                }
            }
            AstNode::Module { items, .. } => {
                for item in items {
                    self.type_inference(item, context)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Example: infer type from literal or patterns.
    fn infer_type_from_node(node: &AstNode, patterns: &PatternLibrary) -> Result<String> {
        match node {
            AstNode::Literal(Literal::Int(_)) => Ok("i64".to_string()),
            AstNode::Literal(Literal::Float(_)) => Ok("f64".to_string()),
            AstNode::Literal(Literal::String(_)) => Ok("String".to_string()),
            AstNode::Literal(Literal::Bool(_)) => Ok("bool".to_string()),
            AstNode::VarDecl { ty: Some(Type::Named(t)), .. } => Ok(t.clone()),
            AstNode::VarDecl { ty: None, .. } => Ok("unknown".to_string()),
            _ => {
                // If not primitive, try matching a quantum/lattice/classical pattern
                let matches = patterns.match_node(node);
                if !matches.is_empty() {
                    let pat = &matches[0]; // Use most confident match for demo
                    Ok(format!("{:?}", pat.domain))
                } else {
                    Ok("unknown".to_string())
                }
            }
        }
    }

    /// Region/zone marking for quantum and lattice regions.
    fn region_semantics(&self, node: &AstNode, context: &mut TransmutationContext) -> Result<()> {
        let detector = PatternDetector::new(&self.patterns);

        // You could store these in the context, or attach annotations/lints globally.
        let quantum_regions = detector.detect_quantum_regions(&UniversalAst { root: node.clone(), metadata: Default::default() }, context);
        let lattice_regions = detector.detect_lattice_regions(&UniversalAst { root: node.clone(), metadata: Default::default() }, context);

        context.set_metadata("quantum_regions", format!("{:?}", quantum_regions.len()));
        context.set_metadata("lattice_regions", format!("{:?}", lattice_regions.len()));
        Ok(())
    }

    /// Symbol shadowing, leakage, scope boundary checks.
    fn check_scopes_and_shadowing(&self, node: &AstNode, context: &mut TransmutationContext) -> Result<()> {
        // Example pass: ensure no duplicate variable in same scope
        match node {
            AstNode::Function { params, body, name, .. } => {
                let mut seen = std::collections::HashSet::new();
                for param in params {
                    if !seen.insert(&param.name) {
                        context.warn(format!("Variable '{}' shadowed in function '{}'", param.name, name));
                    }
                }
                for stmt in body {
                    self.check_scopes_and_shadowing(stmt, context)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Quantum/classical/lattice unsafe usage detection & error annotation.
    fn detect_potential_issues(&self, node: &AstNode, context: &mut TransmutationContext) -> Result<()> {
        // Example: warn if CNOT is used with less than 2 qubits (toy logic)
        match node {
            AstNode::Call { function, args } => {
                if let AstNode::Identifier(ident) = &**function {
                    if ident == "CNOT" && args.len() < 2 {
                        context.warn("CNOT called with less than 2 arguments (not physical)".to_string());
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::library::PatternLibrary;

    #[test]
    fn symbol_and_type_inference_runs() {
        let patterns = PatternLibrary::new();
        let mut analyzer = SemanticAnalyzer::new(patterns);
        let mut context = TransmutationContext::new();

        let ast = UniversalAst {
            root: AstNode::VarDecl {
                name: "foo".into(),
                ty: None,
                value: Some(Box::new(AstNode::Literal(Literal::Int(1)))),
                is_mutable: false,
                symbol_id: None,
            },
            metadata: Default::default(),
        };
        let mut ast_clone = ast.clone();
        analyzer.analyze(&mut ast_clone, &mut context).unwrap();
        assert!(context.metadata.contains_key("type:foo"));
    }
}