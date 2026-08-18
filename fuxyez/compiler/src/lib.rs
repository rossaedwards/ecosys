pub mod ast;
pub mod diagnostics;
pub mod generator;
pub mod lexer;
pub mod parser;
pub mod executor;
pub mod optimizer;
pub mod runtime_hooks;
pub mod uir;
// Use conditional mod imports for `main` only in binary crates, not in library crate

// Public re-exports for core types
pub use ast::*;
pub use diagnostics::*;
pub use lexer::*;