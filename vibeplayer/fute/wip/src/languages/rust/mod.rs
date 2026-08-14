//! Rust Language Plugin
//! 
//! Parses Rust code using syn and generates Rust code from Universal AST

use anyhow::{Result, Context as AnyhowContext};
use syn::{File as SynFile, Item};
use quote::ToTokens;
use crate::{
    ast::*,
    languages::traits::*,
    core::context::TransmutationContext,
};

mod parser;
mod generator;

pub use parser::RustParser;
pub use generator::RustGenerator;

/// Rust language plugin
pub struct RustPlugin {
    parser: RustParser,
    generator: RustGenerator,
}

impl RustPlugin {
    pub fn new() -> Self {
        Self {
            parser: RustParser::new(),
            generator: RustGenerator::new(),
        }
    }
}

impl LanguagePlugin for RustPlugin {
    fn name(&self) -> &str {
        "Rust"
    }
    
    fn version(&self) -> &str {
        "1.70+"
    }
    
    fn file_extensions(&self) -> Vec<&str> {
        vec!["rs"]
    }
    
    fn parse(&self, source: &str, context: &mut TransmutationContext) -> Result<UniversalAst> {
        log::info!("🦀 Parsing Rust code...");
        
        // Parse with syn
        let syntax_tree: SynFile = syn::parse_str(source)
            .context("Failed to parse Rust code")?;
        
        // Convert to Universal AST
        let ast = self.parser.convert_to_universal(&syntax_tree, context)?;
        
        log::info!("✅ Rust parsing complete");
        Ok(ast)
    }
    
    fn generate(&self, ast: &UniversalAst, context: &TransmutationContext) -> Result<String> {
        log::info!("🦀 Generating Rust code...");
        
        let code = self.generator.generate_code(ast, context)?;
        
        log::info!("✅ Rust generation complete");
        Ok(code)
    }
    
    fn validate(&self, source: &str) -> Result<Vec<ValidationError>> {
        let result = syn::parse_str::<SynFile>(source);
        
        match result {
            Ok(_) => Ok(vec![]),
            Err(e) => {
                let err = ValidationError {
                    line: e.span().start().line,
                    column: e.span().start().column,
                    message: e.to_string(),
                    severity: Severity::Error,
                };
                Ok(vec![err])
            }
        }
    }
    
    fn metadata(&self) -> LanguageMetadata {
        LanguageMetadata {
            name: "Rust".to_string(),
            paradigm: vec![
                Paradigm::Imperative,
                Paradigm::Functional,
                Paradigm::Concurrent,
            ],
            typing: TypingSystem::Static,
            memory_model: MemoryModel::Ownership,
            concurrency: ConcurrencyModel::AsyncAwait,
        }
    }
}

impl Default for RustPlugin {
    fn default() -> Self {
        Self::new()
    }
}