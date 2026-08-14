//! Advanced Pattern Detector - Fuxyez Fute Patterns
//!
//! Traverses the AST to find complex graph, quantum, and ritual regions.
//! Powerful enough for entanglement graphs, resonance detection, Hamiltonian extraction, and more.
//! Emits match events and contextual region tags for deep engine analysis.

use crate::ast::{AstNode, UniversalAst};
use crate::patterns::library::{PatternLibrary, PatternDomain, SemanticPattern};
use crate::patterns::matcher::PatternMatcherEngine;
use crate::core::context::TransmutationContext;

/// Region match marking a higher-order semantic block in quantum/lattice analysis.
#[derive(Debug, Clone)]
pub struct SemanticRegion<'a> {
    pub domain: PatternDomain,
    pub start_node: &'a AstNode,
    pub region_nodes: Vec<&'a AstNode>,
    pub pattern: &'a SemanticPattern,
    pub description: String,
}

/// Core detector runs full AST traversal and emits semantic regions.
pub struct PatternDetector<'a> {
    matcher: PatternMatcherEngine<'a>,
}

impl<'a> PatternDetector<'a> {
    pub fn new(library: &'a PatternLibrary) -> Self {
        Self {
            matcher: PatternMatcherEngine::new(library),
        }
    }

    /// Detects semantic regions of the requested domain (quantum/lattice/hybrid/etc).
    pub fn detect_regions(
        &self,
        ast: &'a UniversalAst,
        domain: PatternDomain,
        _context: &mut TransmutationContext,
    ) -> Vec<SemanticRegion<'a>> {
        let mut regions = Vec::new();
        let groups = self.matcher.grouped_matches(ast);

        for (pattern_name, nodes) in groups {
            let pattern = self.matcher.library.get(&pattern_name).unwrap();
            if pattern.domain == domain {
                for node in nodes {
                    let mut region_subtree = Vec::new();
                    // Example: collect immediate children for the region
                    match node {
                        AstNode::Module { items, .. } => region_subtree.extend(items.iter()),
                        AstNode::Function { body, .. } => region_subtree.extend(body.iter()),
                        AstNode::Class { methods, .. } => region_subtree.extend(methods.iter()),
                        AstNode::Block(stmts) => region_subtree.extend(stmts.iter()),
                        _ => {}
                    }
                    regions.push(SemanticRegion {
                        domain: pattern.domain.clone(),
                        start_node: node,
                        region_nodes: region_subtree,
                        pattern,
                        description: pattern.description.clone(),
                    });
                }
            }
        }

        regions
    }

    /// Convenience: Detect quantum regions specifically.
    pub fn detect_quantum_regions(
        &self,
        ast: &'a UniversalAst,
        context: &mut TransmutationContext,
    ) -> Vec<SemanticRegion<'a>> {
        self.detect_regions(ast, PatternDomain::Quantum, context)
    }

    /// Detect lattice regions (graph, tiling, crystalline etc).
    pub fn detect_lattice_regions(
        &self,
        ast: &'a UniversalAst,
        context: &mut TransmutationContext,
    ) -> Vec<SemanticRegion<'a>> {
        self.detect_regions(ast, PatternDomain::Lattice, context)
    }

    /// Detect hybrid regions (classical + quantum/lattice interaction).
    pub fn detect_hybrid_regions(
        &self,
        ast: &'a UniversalAst,
        context: &mut TransmutationContext,
    ) -> Vec<SemanticRegion<'a>> {
        self.detect_regions(ast, PatternDomain::Hybrid, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Type};
    use crate::patterns::library::PatternLibrary;
    use crate::core::context::TransmutationContext;

    #[test]
    fn test_detect_quantum_regions() {
        let library = PatternLibrary::new();
        let detector = PatternDetector::new(&library);
        let mut context = TransmutationContext::new();

        let ast = UniversalAst {
            root: AstNode::Module {
                name: "demo".into(),
                items: vec![
                    AstNode::VarDecl {
                        name: "q0".into(),
                        ty: Some(Type::Named("Qubit".into())),
                        value: None,
                        is_mutable: false,
                        symbol_id: None,
                    },
                ],
            },
            metadata: Default::default(),
        };

        let regions = detector.detect_quantum_regions(&ast, &mut context);
        assert!(!regions.is_empty());
        assert_eq!(regions[0].domain, PatternDomain::Quantum);
        assert!(regions[0].description.contains("qubit"));
    }

    #[test]
    fn test_detect_lattice_regions() {
        let library = PatternLibrary::new();
        let detector = PatternDetector::new(&library);
        let mut context = TransmutationContext::new();

        let ast = UniversalAst {
            root: AstNode::Module {
                name: "root".into(),
                items: vec![
                    AstNode::Struct {
                        name: "CrystalLattice".into(),
                        fields: vec![],
                        visibility: crate::ast::Visibility::Public,
                        symbol_id: None,
                    },
                ],
            },
            metadata: Default::default(),
        };

        let regions = detector.detect_lattice_regions(&ast, &mut context);
        assert!(!regions.is_empty());
        assert_eq!(regions[0].domain, PatternDomain::Lattice);
        assert!(regions[0].description.contains("lattice"));
    }
}