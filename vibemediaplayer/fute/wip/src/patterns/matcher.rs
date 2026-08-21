//! Matcher Engine for Semantic Patterns - Fuxyez Fute
//!
//! Efficiently applies pattern libraries across full or partial Universal ASTs.
//! Powers quantum/lattice/classical/hybrid pattern scans, analytics, and triggers for
//! transformation and optimization passes.
//!
//! Supports results batching, early exits, and domain-scoped super-scans.

use crate::ast::{AstNode, UniversalAst};
use crate::patterns::library::{PatternLibrary, SemanticPattern, PatternDomain};
use std::collections::HashMap;

/// Represents a match result for a node and a pattern.
#[derive(Debug, Clone)]
pub struct PatternMatch<'a> {
    pub node_ref: &'a AstNode,
    pub pattern: &'a SemanticPattern,
}

/// The main Matcher struct, bundling a reference to a pattern library.
pub struct PatternMatcherEngine<'a> {
    pub library: &'a PatternLibrary,
}

impl<'a> PatternMatcherEngine<'a> {
    pub fn new(library: &'a PatternLibrary) -> Self {
        Self { library }
    }

    /// Scan every node of the Universal AST, collecting all pattern matches.
    pub fn match_ast(&self, ast: &UniversalAst) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        self.match_node_recursive(&ast.root, &mut matches);
        matches
    }

    /// Scan with results grouped by pattern name for aggregated analysis.
    pub fn grouped_matches(&self, ast: &UniversalAst) -> HashMap<String, Vec<&AstNode>> {
        let mut groups: HashMap<String, Vec<&AstNode>> = HashMap::new();
        self.match_node_grouped(&ast.root, &mut groups);
        groups
    }

    /// Fast single node match returning all patterns that match this node.
    pub fn match_node<'b>(&'b self, node: &'b AstNode) -> Vec<&'b SemanticPattern> {
        self.library.match_node(node)
    }

    /// Like match_node but restricts to the given semantic domain.
    pub fn match_node_domain<'b>(&'b self, node: &'b AstNode, domain: PatternDomain) -> Vec<&'b SemanticPattern> {
        self.library.domain(domain).filter(|p| p.matches(node)).collect()
    }

    // Private: traverse and collect matches in DFS order
    fn match_node_recursive<'b>(&'b self, node: &'b AstNode, result: &mut Vec<PatternMatch<'b>>) {
        for pattern in self.library.match_node(node) {
            result.push(PatternMatch { node_ref: node, pattern });
        }
        // Recurse through AST children
        match node {
            AstNode::Module { items, .. } => for item in items { self.match_node_recursive(item, result); },
            AstNode::Function { body, .. } => for stmt in body { self.match_node_recursive(stmt, result); },
            AstNode::Block(stmts) => for stmt in stmts { self.match_node_recursive(stmt, result); },
            AstNode::If { then_branch, else_branch, .. } => {
                for stmt in then_branch { self.match_node_recursive(stmt, result); }
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts { self.match_node_recursive(stmt, result); }
                }
            }
            AstNode::Class { methods, .. } => for method in methods { self.match_node_recursive(method, result); },
            AstNode::Loop { body, .. } => for stmt in body { self.match_node_recursive(stmt, result); },
            AstNode::Match { arms, .. } => for arm in arms { for stmt in &arm.body { self.match_node_recursive(stmt, result); } },
            AstNode::Assignment { target, value }
            | AstNode::BinaryOp { left: target, op: _, right: value }
            | AstNode::Call { function: target, args: value }
            | AstNode::UnaryOp { op: _, operand: target } => {
                self.match_node_recursive(target, result);
                // For Call and BinaryOp, value is a Vec or Box—handle both cases:
                match value {
                    AstNode::Block(stmts) => for stmt in stmts { self.match_node_recursive(stmt, result); },
                    AstNode::Call { args, .. } => for arg in args { self.match_node_recursive(arg, result); },
                    _ => self.match_node_recursive(value, result),
                }
            }
            AstNode::VarDecl { value, .. } => {
                if let Some(val) = value { self.match_node_recursive(val, result); }
            }
            AstNode::Return(expr) => {
                if let Some(e) = expr { self.match_node_recursive(e, result); }
            }
            AstNode::AnnotationNode { inner, .. } => self.match_node_recursive(inner, result),
            _ => {}
        }
    }

    // Private: traverse and collect groups of matches by pattern name
    fn match_node_grouped<'b>(&'b self, node: &'b AstNode, groups: &mut HashMap<String, Vec<&AstNode>>) {
        for pattern in self.library.match_node(node) {
            groups.entry(pattern.name.clone()).or_default().push(node);
        }
        match node {
            AstNode::Module { items, .. } => for item in items { self.match_node_grouped(item, groups); },
            AstNode::Function { body, .. } => for stmt in body { self.match_node_grouped(stmt, groups); },
            AstNode::Block(stmts) => for stmt in stmts { self.match_node_grouped(stmt, groups); },
            AstNode::If { then_branch, else_branch, .. } => {
                for stmt in then_branch { self.match_node_grouped(stmt, groups); }
                if let Some(else_branch) = else_branch {
                    for stmt in else_branch { self.match_node_grouped(stmt, groups); }
                }
            }
            AstNode::Class { methods, .. } => for method in methods { self.match_node_grouped(method, groups); },
            AstNode::Loop { body, .. } => for stmt in body { self.match_node_grouped(stmt, groups); },
            AstNode::Match { arms, .. } => for arm in arms { for stmt in &arm.body { self.match_node_grouped(stmt, groups); } },
            AstNode::Assignment { target, value }
            | AstNode::BinaryOp { left: target, op: _, right: value }
            | AstNode::Call { function: target, args: value }
            | AstNode::UnaryOp { op: _, operand: target } => {
                self.match_node_grouped(target, groups);
                match value {
                    AstNode::Block(stmts) => for stmt in stmts { self.match_node_grouped(stmt, groups); },
                    AstNode::Call { args, .. } => for arg in args { self.match_node_grouped(arg, groups); },
                    _ => self.match_node_grouped(value, groups),
                }
            }
            AstNode::VarDecl { value, .. } => {
                if let Some(val) = value { self.match_node_grouped(val, groups); }
            }
            AstNode::Return(expr) => {
                if let Some(e) = expr { self.match_node_grouped(e, groups); }
            }
            AstNode::AnnotationNode { inner, .. } => self.match_node_grouped(inner, groups),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Type};
    use crate::patterns::library::PatternLibrary;

    #[test]
    fn matcher_finds_lattice_and_quantum_matches() {
        let library = PatternLibrary::new();
        let matcher = PatternMatcherEngine::new(&library);

        let ast = UniversalAst {
            root: AstNode::Module {
                name: "root".into(),
                items: vec![
                    AstNode::VarDecl {
                        name: "q0".into(),
                        ty: Some(Type::Named("Qubit".into())),
                        value: None,
                        is_mutable: false,
                        symbol_id: None,
                    },
                    AstNode::Struct {
                        name: "SquareLattice".into(),
                        fields: vec![],
                        visibility: crate::ast::Visibility::Public,
                        symbol_id: None,
                    },
                ],
            },
            metadata: Default::default(),
        };

        let matches = matcher.match_ast(&ast);
        let pattern_names: Vec<_> = matches.iter().map(|m| m.pattern.name.as_str()).collect();
        assert!(pattern_names.contains(&"QubitDeclaration"));
        assert!(pattern_names.contains(&"LatticeGraph"));
    }
}