//! JavaScript/TypeScript Language Plugin

use anyhow::Result;
use crate::{
    ast::*,
    languages::traits::*,
    core::context::TransmutationContext,
};

mod parser;
mod generator;

pub use parser::JavaScriptParser;
pub use generator::JavaScriptGenerator;

/// JavaScript language plugin
pub struct JavaScriptPlugin {
    parser: JavaScriptParser,
    generator: JavaScriptGenerator,
}

impl JavaScriptPlugin {
    pub fn new() -> Self {
        Self {
            parser: JavaScriptParser::new(),
            generator: JavaScriptGenerator::new(),
        }
    }
}

impl LanguagePlugin for JavaScriptPlugin {
    fn name(&self) -> &str {
        "JavaScript"
    }
    
    fn version(&self) -> &str {
        "ES2022+"
    }
    
    fn file_extensions(&self) -> Vec<&str> {
        vec!["js", "mjs", "jsx"]
    }
    
    fn parse(&self, source: &str, context: &mut TransmutationContext) -> Result<UniversalAst> {
        log::info!("🟨 Parsing JavaScript code...");
        
        let ast = self.parser.parse_source(source, context)?;
        
        log::info!("✅ JavaScript parsing complete");
        Ok(ast)
    }
    
    fn generate(&self, ast: &UniversalAst, context: &TransmutationContext) -> Result<String> {
        log::info!("🟨 Generating JavaScript code...");
        
        let code = self.generator.generate_code(ast, context)?;
        
        log::info!("✅ JavaScript generation complete");
        Ok(code)
    }
    
    fn validate(&self, _source: &str) -> Result<Vec<ValidationError>> {
        // TODO: Implement JS validation
        Ok(vec![])
    }
    
    fn metadata(&self) -> LanguageMetadata {
        LanguageMetadata {
            name: "JavaScript".to_string(),
            paradigm: vec![
                Paradigm::Functional,
                Paradigm::ObjectOriented,
                Paradigm::EventDriven,
            ],
            typing: TypingSystem::Dynamic,
            memory_model: MemoryModel::GarbageCollected,
            concurrency: ConcurrencyModel::AsyncAwait,
        }
    }
}

impl Default for JavaScriptPlugin {
    fn default() -> Self {
        Self::new()
    }
}