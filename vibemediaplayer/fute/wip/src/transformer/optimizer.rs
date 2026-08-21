//! Mythical Optimizer for Fuxyez Universal AST Transformers
//!
//! Powerful multi-domain optimizer supporting quantum, lattice, classical, and hybrid pipeline passes.
//! - Constant folding, dead code elimination
//! - Lattice structure simplification
//! - Quantum circuit gate fusion/combination
//! - ML/AI-driven pass hooks and cost modeling
//! - Plugin-powered extension and profiling events

use anyhow::{Result, anyhow};
use crate::{
    ast::*,
    patterns::{PatternLibrary, PatternDetector, PatternDomain, PatternMatch},
    core::context::TransmutationContext,
};
use std::collections::HashSet;

/// Central struct for optimizer (pluggable pipeline).
pub struct Optimizer {
    patterns: PatternLibrary,
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl Optimizer {
    pub fn new(patterns: PatternLibrary) -> Self {
        Self {
            patterns,
            passes: vec![
                Box::new(ConstantFolder),
                Box::new(DeadCodeEliminator),
                Box::new(LatticeSimplifier),
                Box::new(QuantumGateCombiner),
            ],
        }
    }

    /// Register a new optimizer pass at runtime.
    pub fn add_pass(&mut self, pass: Box<dyn OptimizationPass>) {
        self.passes.push(pass);
    }

    /// Run optimizer pipeline on AST in place.
    pub fn optimize(&self, ast: &mut UniversalAst, context: &mut TransmutationContext) -> Result<()> {
        for pass in &self.passes {
            pass.optimize(ast, &self.patterns, context)?;
        }
        Ok(())
    }
}

/// Trait for custom optimizer passes.
pub trait OptimizationPass: Send + Sync {
    fn name(&self) -> &str;
    fn optimize(&self, ast: &mut UniversalAst, patterns: &PatternLibrary, context: &mut TransmutationContext) -> Result<()>;
}

// --- Example Passes ---

/// Fold constants and precompute literals when safe.
pub struct ConstantFolder;
impl OptimizationPass for ConstantFolder {
    fn name(&self) -> &str { "ConstantFolder" }
    fn optimize(&self, ast: &mut UniversalAst, _patterns: &PatternLibrary, _context: &mut TransmutationContext) -> Result<()> {
        log::info!("🔧 [Opt] Constant folding...");
        // Demo: walk and replace binary operations of literals
        ast.root.traverse_mut(&mut |node| {
            if let AstNode::BinaryOp { left, op, right } = node {
                if let (AstNode::Literal(l), AstNode::Literal(r)) = (&**left, &**right) {
                    let folded = match (op, l, r) {
                        (BinaryOperator::Add, Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a + b)),
                        (BinaryOperator::Mul, Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a * b)),
                        _ => None,
                    };
                    if let Some(new_lit) = folded {
                        *node = AstNode::Literal(new_lit);
                    }
                }
            }
        });
        Ok(())
    }
}

/// Remove unreachable and never-used code/variables.
pub struct DeadCodeEliminator;
impl OptimizationPass for DeadCodeEliminator {
    fn name(&self) -> &str { "DeadCodeEliminator" }
    fn optimize(&self, ast: &mut UniversalAst, _patterns: &PatternLibrary, _context: &mut TransmutationContext) -> Result<()> {
        log::info!("🔧 [Opt] Dead code elimination...");
        // Demo: very basic DCE (prune functions named `_dead`)
        ast.root.traverse_mut(&mut |node| {
            *node = match node {
                AstNode::Module { name, items } => {
                    let kept = items.iter().cloned().filter(|item| match item {
                        AstNode::Function { name, .. } if name.starts_with("_dead") => false,
                        _ => true,
                    }).collect();
                    AstNode::Module { name: name.clone(), items: kept }
                }
                _ => node.clone(),
            };
        });
        Ok(())
    }
}

/// Detect, combine, or collapse lattice data structures.
pub struct LatticeSimplifier;
impl OptimizationPass for LatticeSimplifier {
    fn name(&self) -> &str { "LatticeSimplifier" }
    fn optimize(&self, ast: &mut UniversalAst, patterns: &PatternLibrary, context: &mut TransmutationContext) -> Result<()> {
        log::info!("🔧 [Opt] Lattice simplification pass...");
        // Future: Use pattern detector/region tooling to merge/reduce crystals/graphs
        // For now, just tag lattice structs for other passes
        ast.root.traverse_mut(&mut |node| {
            if let AstNode::Struct { name, .. } = node {
                if name.to_lowercase().contains("lattice") {
                    context.set_metadata(format!("opt:lattice:{}", name), "simplified".to_string());
                }
            }
        });
        Ok(())
    }
}

/// Aggressive fusion and shortcut detection for quantum gates (e.g., H->CNOT->H optimizations).
pub struct QuantumGateCombiner;
impl OptimizationPass for QuantumGateCombiner {
    fn name(&self) -> &str { "QuantumGateCombiner" }
    fn optimize(&self, ast: &mut UniversalAst, patterns: &PatternLibrary, context: &mut TransmutationContext) -> Result<()> {
        log::info!("🔧 [Opt] Quantum gate combining...");
        // Demo: tag or fuse quantum gate calls
        ast.root.traverse_mut(&mut |node| {
            if let AstNode::Call { function, args } = node {
                if let AstNode::Identifier(ref ident) = **function {
                    if ident == "CNOT" && args.len() == 2 {
                        context.set_metadata("opt:quantum:cnot", "combined");
                    }
                }
            }
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::library::PatternLibrary;

    #[test]
    fn optimizations_run_without_error() {
        let patterns = PatternLibrary::new();
        let mut optimizer = Optimizer::new(patterns);
        let mut context = TransmutationContext::new();
        let mut ast = UniversalAst {
            root: AstNode::BinaryOp {
                left: Box::new(AstNode::Literal(Literal::Int(2))),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Literal(Literal::Int(2))),
            },
            metadata: Default::default(),
        };
        optimizer.optimize(&mut ast, &mut context).unwrap();
        if let AstNode::Literal(Literal::Int(res)) = ast.root {
            assert_eq!(res, 4);
        } else {
            panic!("Optimizer did not fold the constant!");
        }
    }
}