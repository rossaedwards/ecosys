//! Universal AST representation for Fute transformations and analyses.
//! Enhanced with deep traversal, mutation utilities, error nodes, and extensibility hooks.
//! Supports serialization/deserialization and integrates with symbiotic transformation pipelines.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAst {
    pub root: AstNode,
    pub metadata: AstMetadata,
}

impl UniversalAst {
    pub fn new() -> Self {
        Self {
            root: AstNode::Module {
                name: "root".to_string(),
                items: Vec::new(),
            },
            metadata: AstMetadata::default(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.root.count_nodes()
    }

    pub fn functions(&self) -> Vec<&AstNode> {
        self.root.find_all(|node| matches!(node, AstNode::Function { .. }))
    }

    pub fn types(&self) -> Vec<&AstNode> {
        self.root.find_all(|node| matches!(node, AstNode::Struct { .. } | AstNode::Class { .. }))
    }

    /// Mutable traversal to mutate nodes in place.
    pub fn traverse_mut<F>(&mut self, func: &mut F)
    where
        F: FnMut(&mut AstNode),
    {
        self.root.traverse_mut(func);
    }

    /// Immutable traversal applying visitor pattern.
    pub fn traverse<F>(&self, func: &mut F)
    where
        F: FnMut(&AstNode),
    {
        self.root.traverse(func);
    }
}

impl Default for UniversalAst {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstMetadata {
    pub source_file: Option<String>,
    pub source_language: Option<String>,
    pub line_count: usize,
    pub custom: HashMap<String, String>,
}

/// Core AST node type. Includes ErrorNode and AnnotationNode for richer diagnostics and meta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstNode {
    Module {
        name: String,
        items: Vec<AstNode>,
    },
    Function {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<AstNode>,
        is_async: bool,
        visibility: Visibility,
        /// Optional direct symbol reference for faster resolution
        symbol_id: Option<usize>,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
        visibility: Visibility,
        symbol_id: Option<usize>,
    },
    Class {
        name: String,
        fields: Vec<Field>,
        methods: Vec<AstNode>,
        base_class: Option<String>,
        visibility: Visibility,
        symbol_id: Option<usize>,
    },
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
        visibility: Visibility,
    },
    VarDecl {
        name: String,
        ty: Option<Type>,
        value: Option<Box<AstNode>>,
        is_mutable: bool,
        symbol_id: Option<usize>,
    },
    Assignment {
        target: Box<AstNode>,
        value: Box<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        then_branch: Vec<AstNode>,
        else_branch: Option<Vec<AstNode>>,
    },
    Loop {
        kind: LoopKind,
        body: Vec<AstNode>,
    },
    Match {
        scrutinee: Box<AstNode>,
        arms: Vec<MatchArm>,
    },
    Call {
        function: Box<AstNode>,
        args: Vec<AstNode>,
    },
    BinaryOp {
        left: Box<AstNode>,
        op: BinaryOperator,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<AstNode>,
    },
    Literal(Literal),
    Identifier(String),
    Return(Option<Box<AstNode>>),
    Block(Vec<AstNode>),
    Import {
        path: String,
        items: Vec<String>,
    },
    Comment(String),

    /// Error node for attaching presence of errors, warnings etc.
    ErrorNode {
        message: String,
        location: Option<(usize, usize)>, // line and column numbers
    },

    /// Annotation node for attributes, pragmas, or meta comments
    AnnotationNode {
        key: String,
        value: String,
        inner: Box<AstNode>,
    },
}

impl AstNode {
    /// Recursive count nodes including self.
    pub fn count_nodes(&self) -> usize {
        1 + match self {
            AstNode::Module { items, .. } => items.iter().map(|n| n.count_nodes()).sum(),
            AstNode::Function { body, .. } => body.iter().map(|n| n.count_nodes()).sum(),
            AstNode::Block(stmts) => stmts.iter().map(|n| n.count_nodes()).sum(),
            AstNode::If { then_branch, else_branch, .. } => {
                then_branch.iter().map(|n| n.count_nodes()).sum::<usize>()
                    + else_branch.as_ref().map(|b| b.iter().map(|n| n.count_nodes()).sum()).unwrap_or(0)
            }
            AstNode::Class { methods, .. } => methods.iter().map(|n| n.count_nodes()).sum(),
            AstNode::Loop { body, .. } => body.iter().map(|n| n.count_nodes()).sum(),
            AstNode::Match { arms, .. } => arms.iter().map(|arm| arm.body.iter().map(|n| n.count_nodes()).sum::<usize>()).sum(),
            AstNode::Assignment { target, value } => target.count_nodes() + value.count_nodes(),
            AstNode::Call { function, args } => args.iter().map(|n| n.count_nodes()).sum::<usize>() + function.count_nodes(),
            AstNode::BinaryOp { left, right, .. } => left.count_nodes() + right.count_nodes(),
            AstNode::UnaryOp { operand, .. } => operand.count_nodes(),
            AstNode::VarDecl { value, .. } => value.as_ref().map(|v| v.count_nodes()).unwrap_or(0),
            AstNode::Return(expr) => expr.as_ref().map(|e| e.count_nodes()).unwrap_or(0),
            AstNode::AnnotationNode { inner, .. } => inner.count_nodes(),
            AstNode::ErrorNode { .. } => 0,
            _ => 0,
        }
    }

    /// Find all AST nodes matching predicate recursively.
    pub fn find_all<F>(&self, predicate: F) -> Vec<&AstNode>
    where
        F: Fn(&AstNode) -> bool + Copy,
    {
        let mut results = Vec::new();
        if predicate(self) {
            results.push(self);
        }
        match self {
            AstNode::Module { items, .. } => for item in items { results.extend(item.find_all(predicate)); },
            AstNode::Function { body, .. } => for stmt in body { results.extend(stmt.find_all(predicate)); },
            AstNode::Block(stmts) => for stmt in stmts { results.extend(stmt.find_all(predicate)); },
            AstNode::If { then_branch, else_branch, .. } => {
                for stmt in then_branch { results.extend(stmt.find_all(predicate)); }
                if let Some(else_branch) = else_branch {
                    for stmt in else_branch { results.extend(stmt.find_all(predicate)); }
                }
            }
            AstNode::Class { methods, .. } => for method in methods { results.extend(method.find_all(predicate)); },
            AstNode::Loop { body, .. } => for stmt in body { results.extend(stmt.find_all(predicate)); },
            AstNode::Match { arms, .. } => for arm in arms { for stmt in &arm.body { results.extend(stmt.find_all(predicate)); } },
            AstNode::Assignment { target, value } => {
                results.extend(target.find_all(predicate));
                results.extend(value.find_all(predicate));
            }
            AstNode::Call { function, args } => {
                results.extend(function.find_all(predicate));
                for arg in args { results.extend(arg.find_all(predicate)); }
            }
            AstNode::BinaryOp { left, right, .. } => {
                results.extend(left.find_all(predicate));
                results.extend(right.find_all(predicate));
            }
            AstNode::UnaryOp { operand, .. } => results.extend(operand.find_all(predicate)),
            AstNode::VarDecl { value, .. } => {
                if let Some(val) = value { results.extend(val.find_all(predicate)); }
            }
            AstNode::Return(expr) => {
                if let Some(e) = expr { results.extend(e.find_all(predicate)); }
            }
            AstNode::AnnotationNode { inner, .. } => results.extend(inner.find_all(predicate)),
            _ => {}
        }
        results
    }

    /// Mutable traversal applying closure to nodes recursively.
    pub fn traverse_mut<F>(&mut self, func: &mut F)
    where
        F: FnMut(&mut AstNode),
    {
        func(self);
        match self {
            AstNode::Module { items, .. } => for item in items { item.traverse_mut(func); },
            AstNode::Function { body, .. } => for stmt in body { stmt.traverse_mut(func); },
            AstNode::Block(stmts) => for stmt in stmts { stmt.traverse_mut(func); },
            AstNode::If { then_branch, else_branch, .. } => {
                for stmt in then_branch { stmt.traverse_mut(func); }
                if let Some(else_branch) = else_branch {
                    for stmt in else_branch { stmt.traverse_mut(func); }
                }
            }
            AstNode::Class { methods, .. } => for method in methods { method.traverse_mut(func); },
            AstNode::Loop { body, .. } => for stmt in body { stmt.traverse_mut(func); },
            AstNode::Match { arms, .. } => for arm in arms { for stmt in &mut arm.body { stmt.traverse_mut(func); } },
            AstNode::Assignment { target, value } => {
                target.traverse_mut(func);
                value.traverse_mut(func);
            }
            AstNode::Call { function, args } => {
                function.traverse_mut(func);
                for arg in args { arg.traverse_mut(func); }
            }
            AstNode::BinaryOp { left, right, .. } => {
                left.traverse_mut(func);
                right.traverse_mut(func);
            }
            AstNode::UnaryOp { operand, .. } => operand.traverse_mut(func),
            AstNode::VarDecl { value, .. } => {
                if let Some(val) = value { val.traverse_mut(func); }
            }
            AstNode::Return(expr) => {
                if let Some(e) = expr { e.traverse_mut(func); }
            }
            AstNode::AnnotationNode { inner, .. } => inner.traverse_mut(func),
            _ => {}
        }
    }
}

// Supporting types (Parameter, Field, EnumVariant etc.) remain mostly the same,
// with potential for adding links to symbols and optional metadata annotations.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
    pub default: Option<Literal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub visibility: Visibility,
    pub symbol_id: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<Type>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<AstNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Identifier(String),
    Tuple(Vec<Pattern>),
    Struct { name: String, fields: Vec<(String, Pattern)> },
}

// ... Types, Visibility, LoopKind, Operators, Literal as in previous baseline snippets ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Simple(String),
    Generic { base: String, args: Vec<Type> },
    Function { params: Vec<Type>, return_type: Box<Type> },
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
}
/// ... rest of the supporting types remain unchanged ...
/// Visibility of AST nodes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}
/// Kinds of loops
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoopKind {
    While,
    For,
    Infinite,
}
/// Binary operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
/// Unary operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOperator {
    Negate,
    Not,
}
/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

//! Copyright (c) 2025 Aurphyx Inc.
