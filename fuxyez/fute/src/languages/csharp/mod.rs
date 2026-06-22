//! C# Language Plugin
//! 
//! Parses C# code and generates C# from Universal AST

use anyhow::Result;
use crate::{
    ast::*,
    languages::traits::*,
    core::context::TransmutationContext,
};

mod parser;
mod generator;

pub use parser::CSharpParser;
pub use generator::CSharpGenerator;

/// C# language plugin
pub struct CSharpPlugin {
    parser: CSharpParser,
    generator: CSharpGenerator,
}

impl CSharpPlugin {
    pub fn new() -> Self {
        Self {
            parser: CSharpParser::new(),
            generator: CSharpGenerator::new(),
        }
    }
}

impl LanguagePlugin for CSharpPlugin {
    fn name(&self) -> &str {
        "C#"
    }
    
    fn version(&self) -> &str {
        "11.0+"
    }
    
    fn file_extensions(&self) -> Vec<&str> {
        vec!["cs"]
    }
    
    fn parse(&self, source: &str, context: &mut TransmutationContext) -> Result<UniversalAst> {
        log::info!("💜 Parsing C# code...");
        
        let ast = self.parser.parse_source(source, context)?;
        
        log::info!("✅ C# parsing complete");
        Ok(ast)
    }
    
    fn generate(&self, ast: &UniversalAst, context: &TransmutationContext) -> Result<String> {
        log::info!("💜 Generating C# code...");
        
        let code = self.generator.generate_code(ast, context)?;
        
        log::info!("✅ C# generation complete");
        Ok(code)
    }
    
    fn validate(&self, _source: &str) -> Result<Vec<ValidationError>> {
        // TODO: Implement C# validation using Roslyn API
        Ok(vec![])
    }
    
    fn metadata(&self) -> LanguageMetadata {
        LanguageMetadata {
            name: "C#".to_string(),
            paradigm: vec![
                Paradigm::ObjectOriented,
                Paradigm::Functional,
                Paradigm::Imperative,
            ],
            typing: TypingSystem::Static,
            memory_model: MemoryModel::GarbageCollected,
            concurrency: ConcurrencyModel::AsyncAwait,
        }
    }
}

impl Default for CSharpPlugin {
    fn default() -> Self {
        Self::new()
    }
}