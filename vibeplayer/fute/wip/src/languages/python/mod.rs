//! Python Language Plugin
//! 
//! Parses Python code and generates Python from Universal AST

use anyhow::{Result, Context as AnyhowContext};
use crate::{
    ast::*,
    languages::traits::*,
    core::context::TransmutationContext,
};

mod parser;
mod generator;

pub use parser::PythonParser;
pub use generator::PythonGenerator;

/// Python language plugin
pub struct PythonPlugin {
    parser: PythonParser,
    generator: PythonGenerator,
}

impl PythonPlugin {
    pub fn new() -> Self {
        Self {
            parser: PythonParser::new(),
            generator: PythonGenerator::new(),
        }
    }
}

impl LanguagePlugin for PythonPlugin {
    fn name(&self) -> &str {
        "Python"
    }
    
    fn version(&self) -> &str {
        "3.10+"
    }
    
    fn file_extensions(&self) -> Vec<&str> {
        vec!["py"]
    }
    
    fn parse(&self, source: &str, context: &mut TransmutationContext) -> Result<UniversalAst> {
        log::info!("🐍 Parsing Python code...");
        
        // Use tree-sitter for Python parsing
        let ast = self.parser.parse_source(source, context)?;
        
        log::info!("✅ Python parsing complete");
        Ok(ast)
    }
    
    fn generate(&self, ast: &UniversalAst, context: &TransmutationContext) -> Result<String> {
        log::info!("🐍 Generating Python code...");
        
        let code = self.generator.generate_code(ast, context)?;
        
        log::info!("✅ Python generation complete");
        Ok(code)
    }
    
    fn validate(&self, source: &str) -> Result<Vec<ValidationError>> {
        // Basic syntax validation using tree-sitter
        match self.parser.validate_syntax(source) {
            Ok(_) => Ok(vec![]),
            Err(e) => Ok(vec![ValidationError {
                line: 0,
                column: 0,
                message: e.to_string(),
                severity: Severity::Error,
            }]),
        }
    }
    
    fn metadata(&self) -> LanguageMetadata {
        LanguageMetadata {
            name: "Python".to_string(),
            paradigm: vec![
                Paradigm::ObjectOriented,
                Paradigm::Imperative,
                Paradigm::Functional,
            ],
            typing: TypingSystem::Dynamic,
            memory_model: MemoryModel::GarbageCollected,
            concurrency: ConcurrencyModel::AsyncAwait,
        }
    }
}

impl Default for PythonPlugin {
    fn default() -> Self {
        Self::new()
    }
}