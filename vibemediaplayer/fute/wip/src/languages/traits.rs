//! Language Plugin Traits
//! 
//! Defines the interface all language plugins must implement

use anyhow::Result;
use crate::{
    ast::UniversalAst,
    core::context::TransmutationContext,
};

/// Language plugin trait
pub trait LanguagePlugin: Send + Sync {
    /// Language name
    fn name(&self) -> &str;
    
    /// Language version supported
    fn version(&self) -> &str;
    
    /// File extensions this plugin handles
    fn file_extensions(&self) -> Vec<&str>;
    
    /// Parse source code to Universal AST
    fn parse(&self, source: &str, context: &mut TransmutationContext) -> Result<UniversalAst>;
    
    /// Generate source code from Universal AST
    fn generate(&self, ast: &UniversalAst, context: &TransmutationContext) -> Result<String>;
    
    /// Validate source code
    fn validate(&self, source: &str) -> Result<Vec<ValidationError>>;
    
    /// Get language-specific metadata
    fn metadata(&self) -> LanguageMetadata;
}

/// Language metadata
#[derive(Debug, Clone)]
pub struct LanguageMetadata {
    pub name: String,
    pub paradigm: Vec<Paradigm>,
    pub typing: TypingSystem,
    pub memory_model: MemoryModel,
    pub concurrency: ConcurrencyModel,
}

/// Programming paradigms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paradigm {
    Imperative,
    Functional,
    ObjectOriented,
    Procedural,
    Declarative,
    Concurrent,
    EventDriven,
}

/// Typing system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingSystem {
    Static,
    Dynamic,
    Gradual,
}

/// Memory model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryModel {
    Manual,
    GarbageCollected,
    ReferenceCounted,
    Ownership,
}

/// Concurrency model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcurrencyModel {
    Threads,
    AsyncAwait,
    ActorModel,
    CSP, // Communicating Sequential Processes
    None,
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: Severity,
}

/// Error severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}