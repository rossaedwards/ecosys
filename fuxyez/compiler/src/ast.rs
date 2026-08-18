//! # Fuxyez Abstract Syntax Tree (AST)
//!
//! ```
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  FUXYEZ AST - Where Code Becomes Intention                    ║
//! ║  "Every node is a ritual. Every tree is a temple."            ║
//! ║                                                               ║
//! ║  Blessed by: Thoth (Architect of Language)                    ║
//! ║              Hecate (Keeper of Sacred Structure)              ║
//! ║              Anubis (Guardian of Transformation)              ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## Architecture
//!
//! The Fuxyez AST is a **recursive, type-safe, quantum-aware** representation
//! of ritual code. It supports:
//!
//! - **Sigils**: Executable glyphs (functions/operations)
//! - **Echoes**: Event-driven reactive responses
//! - **Oracles**: Divinatory data queries
//! - **Spinons**: Quantum state carriers (variables)
//! - **Glyphs**: Sacred configuration blueprints
//! - **Conditionals**: Ritual branching (if/match)
//! - **Logical Nodes**: AND/OR composition
//! - **Pattern Matching**: Polymorphic ritual responses
//! - **Modifiers**: Transformation layers (mutate, amplify, shield)
//! - **Async/Concurrency**: Parallel intention execution
//! - **Temporal Schedulers**: Time-bound rituals
//! - **Annotations**: Metadata for SAGES/g0dm0d3 validation
//!
//! ## Example
//!
//! ```
//! :: ritual_greeting
//!     sigil greet(name: String) {
//!         echo "Hello, $name!"
//!     }
//! :::
//! ```
//!
//! Parses to:
//!
//! ```
//! RitualNode::RitualBlock {
//!     open: RitualOpen::DoubleColon,
//!     nodes: vec![
//!         RitualNode::Sigil(Sigil {
//!             name: "greet".into(),
//!             params: vec![("name".into(), "String".into())],
//!             body: vec![...],
//!         })
//!     ],
//!     close: RitualClose::TripleColon,
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════
// LOCATION & SPAN TRACKING - For precise error reporting
// ═══════════════════════════════════════════════════════════════════════════

/// Source location (byte offsets + line/column for human-readable errors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Span {
    /// Byte offset start (inclusive)
    pub start: usize,
    /// Byte offset end (exclusive)
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn dummy() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Merge two spans (for multi-token constructs)
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: self.column.min(other.column),
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::dummy()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ANNOTATION SYSTEM - Metadata for SAGES, g0dm0d3, and tooling
// ═══════════════════════════════════════════════════════════════════════════

/// Annotations for nodes (e.g., `#[love_score = 0.95]`, `#[chakra = "heart"]`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Annot {
    /// Attribute tags (e.g., `"no_bugs"`, `"async"`)
    pub annotations: Vec<String>,
    /// Documentation string
    pub doc: Option<String>,
    /// Diagnostic message (for errors/warnings)
    pub diagnostic: Option<String>,
    /// Love score (0.0 = harmful, 1.0 = pure love)
    pub love_score: Option<f64>,
    /// Chakra alignment (if applicable)
    pub chakra: Option<String>,
}

impl Annot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_love_score(mut self, score: f64) -> Self {
        self.love_score = Some(score);
        self
    }

    pub fn with_chakra(mut self, chakra: impl Into<String>) -> Self {
        self.chakra = Some(chakra.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RITUAL DELIMITERS - Sacred bracket notation (:: and :::)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RitualOpen {
    /// `::`
    DoubleColon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RitualClose {
    /// `:::`
    TripleColon,
}

// ═══════════════════════════════════════════════════════════════════════════
// CORE AST NODE - The Universal Ritual Element
// ═══════════════════════════════════════════════════════════════════════════

/// The primary AST node type—every Fuxyez construct is a `RitualNode`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RitualNode {
    /// Executable glyph (function/operation)
    Sigil(Sigil),

    /// Event-driven response
    Echo(Echo),

    /// Divinatory data query
    Oracle(Oracle),

    /// Quantum state carrier (variable)
    Spinon(Spinon),

    /// Sacred configuration blueprint
    Glyph(Glyph),

    /// Group of nodes (for parsing convenience)
    RitualGroup(Vec<RitualNode>),

    /// Ritual block with explicit delimiters
    RitualBlock {
        open: RitualOpen,
        nodes: Vec<RitualNode>,
        close: RitualClose,
    },

    /// Conditional (if/else, match)
    Condition(ConditionNode),

    /// Logical composition (AND/OR)
    LogicalNode(LogicalNode),

    /// Pattern matching
    PatternMatch(PatternMatchNode),

    /// Transformation modifier (mutate, amplify, invert, shield)
    Modifier(ModifierNode),

    /// Async block
    AsyncBlock(Vec<RitualNode>),

    /// Parallel block (concurrent execution)
    ParallelBlock(Vec<RitualNode>),

    /// Temporal scheduler (time-bound ritual)
    Temporal(TemporalNode),

    /// Annotated node (with metadata)
    Annotated(Box<RitualNode>, Annot),

    /// Node with span (for error reporting)
    WithSpan(Box<RitualNode>, Span),

    /// Quantum node (superposition/entanglement)
    Quantum(QuantumNode),

    /// Literal value (string, number, bool, etc.)
    Literal(Literal),

    /// Identifier (variable name, function name, etc.)
    Identifier(String),

    /// Empty placeholder
    Empty,
}

impl RitualNode {
    /// Get the span of this node (if available)
    pub fn span(&self) -> Option<Span> {
        match self {
            RitualNode::WithSpan(_, span) => Some(*span),
            _ => None,
        }
    }

    /// Count total nodes in tree (for metrics)
    pub fn node_count(&self) -> usize {
        match self {
            RitualNode::RitualGroup(nodes) => {
                1 + nodes.iter().map(|n| n.node_count()).sum::<usize>()
            }
            RitualNode::RitualBlock { nodes, .. } => {
                1 + nodes.iter().map(|n| n.node_count()).sum::<usize>()
            }
            RitualNode::AsyncBlock(nodes) | RitualNode::ParallelBlock(nodes) => {
                1 + nodes.iter().map(|n| n.node_count()).sum::<usize>()
            }
            RitualNode::Sigil(sigil) => {
                1 + sigil.body.iter().map(|n| n.node_count()).sum::<usize>()
            }
            RitualNode::Annotated(node, _) | RitualNode::WithSpan(node, _) => 1 + node.node_count(),
            _ => 1,
        }
    }

    /// Max depth of tree (for metrics)
    pub fn max_depth(&self) -> usize {
        match self {
            RitualNode::RitualGroup(nodes) | RitualNode::RitualBlock { nodes, .. } => {
                1 + nodes.iter().map(|n| n.max_depth()).max().unwrap_or(0)
            }
            RitualNode::Annotated(node, _) | RitualNode::WithSpan(node, _) => 1 + node.max_depth(),
            _ => 1,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SIGIL - Executable Glyph (Function/Operation)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sigil {
    pub name: String,
    pub params: Vec<(String, String)>, // (name, type)
    pub body: Vec<RitualNode>,
    pub return_type: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ECHO - Event-Driven Response
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Echo {
    pub event: String,
    pub response: Box<RitualNode>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ORACLE - Divinatory Data Query
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Oracle {
    pub query: String,
    pub output: Option<Box<RitualNode>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// SPINON - Quantum State Carrier (Variable)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spinon {
    pub binding: String,
    pub value: Box<RitualNode>,
    pub mutable: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// GLYPH - Sacred Configuration Blueprint
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Glyph {
    pub key: String,
    pub value: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// CONDITIONAL CONSTRUCTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionNode {
    If {
        cond: Box<RitualNode>,
        then_branch: Vec<RitualNode>,
        else_branch: Option<Vec<RitualNode>>,
    },
    Match {
        target: Box<RitualNode>,
        arms: Vec<(Pattern, Vec<RitualNode>)>,
    },
}

// ═══════════════════════════════════════════════════════════════════════════
// LOGICAL NODES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalNode {
    pub op: LogicalOp,
    pub nodes: Vec<RitualNode>,
}

// ═══════════════════════════════════════════════════════════════════════════
// PATTERN MATCHING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternMatchNode {
    pub target: Box<RitualNode>,
    pub arms: Vec<(Pattern, Vec<RitualNode>)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pattern {
    pub kind: PatternKind,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternKind {
    Wildcard, // `_`
    Literal,  // `42`, `"hello"`
    Name,     // `x`
    Type,     // `:String`
    Variant,  // `Some(x)`
}

// ═══════════════════════════════════════════════════════════════════════════
// MODIFIERS/TRANSFORMERS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifierKind {
    Mutate,
    Amplify,
    Invert,
    Shield,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifierNode {
    pub kind: ModifierKind,
    pub target: Box<RitualNode>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TEMPORAL SCHEDULERS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalKind {
    /// Execute at specific time (e.g., "2025-12-25T00:00:00Z")
    At(String),
    /// Execute repeatedly (e.g., "0 0 * * *" cron format)
    Every(String),
    /// Execute after event/delay (e.g., "after sunrise")
    After(String),
    /// Execute before event (e.g., "before midnight")
    Before(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalNode {
    pub kind: TemporalKind,
    pub nodes: Vec<RitualNode>,
}

// ═══════════════════════════════════════════════════════════════════════════
// QUANTUM NODES (Experimental)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantumNode {
    /// Superposition (multiple states simultaneously)
    Superposition(Vec<RitualNode>),

    /// Entanglement (correlated state pairs)
    Entanglement {
        left: Box<RitualNode>,
        right: Box<RitualNode>,
    },

    /// Quantum gate operation
    Gate {
        gate_type: String,
        qubits: Vec<usize>,
    },
}

// ═══════════════════════════════════════════════════════════════════════════
// LITERAL VALUES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIVERSAL AST (Top-Level Container)
// ═══════════════════════════════════════════════════════════════════════════

/// Top-level AST container (represents a whole file/module)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniversalAst {
    pub nodes: Vec<RitualNode>,
    pub metadata: AstMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AstMetadata {
    pub file_path: Option<String>,
    pub language: Option<String>,
    pub love_score: Option<f64>,
    pub chakra_alignment: Option<String>,
}

impl UniversalAst {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            metadata: AstMetadata::default(),
        }
    }

    pub fn with_nodes(nodes: Vec<RitualNode>) -> Self {
        Self {
            nodes,
            metadata: AstMetadata::default(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.iter().map(|n| n.node_count()).sum()
    }

    pub fn max_depth(&self) -> usize {
        self.nodes.iter().map(|n| n.max_depth()).max().unwrap_or(0)
    }
}

impl Default for UniversalAst {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_merge_works() {
        let span1 = Span::new(0, 10, 1, 1);
        let span2 = Span::new(5, 15, 1, 6);
        let merged = span1.merge(&span2);
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 15);
    }

    #[test]
    fn ast_node_count_works() {
        let ast = UniversalAst::with_nodes(vec![RitualNode::Sigil(Sigil {
            name: "test".into(),
            params: vec![],
            body: vec![RitualNode::Empty, RitualNode::Empty],
            return_type: None,
        })]);
        assert_eq!(ast.node_count(), 3); // Sigil + 2 Empty
    }
}
