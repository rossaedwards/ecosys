//! Pattern Library for Quantum & Lattice Semantics - Fuxyez Fute
//!
//! Houses a curated knowledge base of reusable, parameterized semantic patterns
//! that power everything from ritual AST recognition to quantum/lattice-specific analysis.
//!
//! Designed for deep symbolic reasoning, shortcut detection, and cross-domain transformation triggers.

use crate::ast::{AstNode, Type, Pattern};
use std::collections::HashMap;

/// Semantic domain of a pattern for smart filtering and analytics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternDomain {
    Classical,       // General algorithms, OOP, functional, etc.
    Quantum,         // Qubits, gates, entanglement, measurement, superposition...
    Lattice,         // Graphs, tilings, complex networks, crystal-like domains...
    Ritual,          // Custom language, symbolic, annotation, and special-purpose.
    Hybrid,          // Classical + Quantum or Quantum + Lattice constructs.
}

/// Structure capturing the reusable, parameterized pattern definition.
#[derive(Debug, Clone)]
pub struct SemanticPattern {
    pub name: String,
    pub domain: PatternDomain,
    pub description: String,
    pub match_predicate: fn(&AstNode) -> bool,
    pub meta_tags: Vec<String>,
    // Option for parameter maps, thresholds, etc.
}

impl SemanticPattern {
    pub fn matches(&self, node: &AstNode) -> bool {
        (self.match_predicate)(node)
    }
}

/// A central knowledge base of registered semantic patterns.
///
/// You can use this to power bulk pattern scans, analytics, or drive transformation passes.
pub struct PatternLibrary {
    patterns: HashMap<String, SemanticPattern>,
}

impl PatternLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            patterns: HashMap::new(),
        };

        // ==== Add quantum domain patterns ====

        library.register(SemanticPattern {
            name: "QubitDeclaration".into(),
            domain: PatternDomain::Quantum,
            description: "Identifies declaration of a single qubit variable in the AST.".into(),
            match_predicate: |node| {
                matches!(node,
                    AstNode::VarDecl { ty: Some(Type::Named(t)), .. } if t == "Qubit"
                )
            },
            meta_tags: vec!["quantum".into(), "qubit".into()],
        });

        library.register(SemanticPattern {
            name: "HadamardGate".into(),
            domain: PatternDomain::Quantum,
            description: "Detects function or call representing a Hadamard gate application.".into(),
            match_predicate: |node| {
                matches!(node,
                    AstNode::Call { function, .. } if matches!(**function,
                        AstNode::Identifier(ref ident) if ident == "H" || ident == "hadamard"
                    )
                )
            },
            meta_tags: vec!["quantum".into(), "gate".into(), "hadamard".into()],
        });

        library.register(SemanticPattern {
            name: "EntanglementPattern".into(),
            domain: PatternDomain::Quantum,
            description: "Detects multi-qubit entanglement patterns via CNOT or similar gates.".into(),
            match_predicate: |node| {
                matches!(node,
                    AstNode::Call { function, args }
                        if matches!(**function, AstNode::Identifier(ref ident) if ident == "CNOT" || ident == "cnot")
                        && args.len() >= 2
                )
            },
            meta_tags: vec!["quantum".into(), "entanglement".into(), "cnot".into()],
        });

        // ==== Add lattice domain patterns ====

        library.register(SemanticPattern {
            name: "LatticeGraph".into(),
            domain: PatternDomain::Lattice,
            description: "Detects declaration of a lattice or graph structure.".into(),
            match_predicate: |node| {
                matches!(node,
                    AstNode::Struct{ name, .. } if name.to_lowercase().contains("lattice")
                )
            },
            meta_tags: vec!["lattice".into(), "graph".into()],
        });

        library.register(SemanticPattern {
            name: "HamiltonianCycleCandidate".into(),
            domain: PatternDomain::Lattice,
            description: "Detects recursive functions named like 'hamiltonian_cycle', hinting algorithmic intent.".into(),
            match_predicate: |node| {
                matches!(node,
                    AstNode::Function { name, .. } if name.to_lowercase().contains("hamiltonian")
                )
            },
            meta_tags: vec!["lattice".into(), "hamiltonian".into()],
        });

        library.register(SemanticPattern {
            name: "HybridQuantumClassicalLoop".into(),
            domain: PatternDomain::Hybrid,
            description: "Detects loops manipulating both quantum and classical data structures.".into(),
            match_predicate: |_node| {
                // Example placeholder: in practice, you'd match deeper loop/AST contexts
                false
            },
            meta_tags: vec!["hybrid".into(), "quantum".into(), "classical".into()],
        });

        // ==== Add ritual/symbolic custom patterns ====

        library.register(SemanticPattern {
            name: "RitualAnnotation".into(),
            domain: PatternDomain::Ritual,
            description: "Detects nodes tagged or annotated as 'ritual' for custom transformations.".into(),
            match_predicate: |node| {
                matches!(node, AstNode::AnnotationNode { key, .. } if key == "ritual")
            },
            meta_tags: vec!["ritual".into(), "annotation".into()],
        });

        library
    }

    /// Register a new semantic pattern to the library.
    pub fn register(&mut self, pattern: SemanticPattern) {
        self.patterns.insert(pattern.name.clone(), pattern);
    }

    /// Retrieve a pattern by name, if exists.
    pub fn get(&self, name: &str) -> Option<&SemanticPattern> {
        self.patterns.get(name)
    }

    /// Iterate all registered patterns.
    pub fn all(&self) -> impl Iterator<Item = &SemanticPattern> {
        self.patterns.values()
    }

    /// Run all patterns that match the node, returning the result set.
    pub fn match_node(&self, node: &AstNode) -> Vec<&SemanticPattern> {
        self.patterns
            .values()
            .filter(|p| (p.match_predicate)(node))
            .collect()
    }

    /// Filter patterns by domain.
    pub fn domain(&self, domain: PatternDomain) -> impl Iterator<Item = &SemanticPattern> {
        self.patterns.values().filter(move |p| p.domain == domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Type};

    #[test]
    fn qubit_declaration_pattern_detects_qubit_var() {
        let library = PatternLibrary::new();
        let qubit_decl = AstNode::VarDecl {
            name: "q0".into(),
            ty: Some(Type::Named("Qubit".into())),
            value: None,
            is_mutable: false,
            symbol_id: None,
        };
        let matches = library.match_node(&qubit_decl);
        assert!(matches.iter().any(|p| p.name == "QubitDeclaration"));
    }

    #[test]
    fn lattice_graph_pattern_detects_struct() {
        let library = PatternLibrary::new();
        let struct_node = AstNode::Struct {
            name: "HexagonalLattice".into(),
            fields: vec![],
            visibility: crate::ast::Visibility::Public,
            symbol_id: None,
        };
        let matches = library.match_node(&struct_node);
        assert!(matches.iter().any(|p| p.domain == PatternDomain::Lattice));
    }
}