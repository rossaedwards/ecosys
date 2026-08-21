//! AST Module Facade for Fuxyez Fute
//!
//! Aggregates core AST submodules and exports unified API for AST manipulation,
//! traversal, parsing, and transformation pipeline integration.

pub mod universal;
pub mod traversal;
pub mod parser;

pub use universal::{UniversalAst, AstNode, Parameter, Field, EnumVariant, MatchArm, Pattern, Type, Visibility, LoopKind, BinaryOperator, UnaryOperator, Literal};
pub use traversal::{AstVisitor, AstVisitorMut, traverse_ast, traverse_ast_mut, CollectingVisitor, MutatingVisitor};
pub use parser::{Parser, UniversalParser};

// Additional prelude for ease of use
pub mod prelude {
    pub use super::universal::*;
    pub use super::traversal::*;
    pub use super::parser::*;
}