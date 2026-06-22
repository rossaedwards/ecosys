//! Transmutation Context - SYMBIOTIC ARCHITECTURE
//!
//! Holds state and metadata for symbiotic transmutation operations,
//! including symbol table, dependency tracking, warnings, types, modes, fields, etc.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Context for symbiotic transmutation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmutationContext {
    /// Source language
    pub source_lang: Option<String>,

    /// Target language (usually Fuxyez)
    pub target_lang: String,

    /// Symbiotic transformation mode
    pub symbiotic_mode: SymbioticMode,

    /// Symbol table: tracks variables, functions, types, modules, constants
    pub symbols: HashMap<String, Symbol>,

    /// Import/dependency tracking for external modules and libraries
    pub dependencies: Vec<Dependency>,

    /// Metadata key-value pairs for extensible context details
    pub metadata: HashMap<String, String>,

    /// Warnings collected during transmutation
    pub warnings: Vec<String>,
}

/// Symbiotic transformation modes to guide ritual/contextual behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbioticMode {
    /// Standard symbiotic transformation
    Standard,

    /// Sacred (with enhanced ritual context for deep integration)
    Sacred,

    /// Mystical (quantum-inspired symbiosis)
    Mystical,

    /// Resonant (lattice-integrated harmonic patterns)
    Resonant,
}

/// Symbol information in the transmutation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Symbol name
    pub name: String,

    /// Symbol kind (function, variable, type, module, constant)
    pub kind: SymbolKind,

    /// Symbol scope, e.g. module or function it belongs to
    pub scope: String,

    /// Optional type information as string
    pub ty: Option<String>,
}

/// Kinds of symbols managed in symbol table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Variable,
    Type,
    Module,
    Constant,
}

/// Dependency tracking structure for external packages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency name
    pub name: String,

    /// Optional version constraint
    pub version: Option<String>,

    /// Source repository or package manager
    pub source: DependencySource,
}

/// Supported dependency sources for symbiotic transmutation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencySource {
    Cargo,
    Npm,
    PyPI,
    YCrates,
    Local,
}

impl TransmutationContext {
    /// Create a new transmutation context with default values
    pub fn new() -> Self {
        Self {
            source_lang: None,
            target_lang: "fuxyez".to_string(),
            symbiotic_mode: SymbioticMode::Standard,
            symbols: HashMap::new(),
            dependencies: Vec::new(),
            metadata: HashMap::new(),
            warnings: Vec::new(),
        }
    }

    /// Set the source language
    pub fn with_source_lang(mut self, lang: impl Into<String>) -> Self {
        self.source_lang = Some(lang.into());
        self
    }

    /// Set the symbiotic transformation mode
    pub fn with_mode(mut self, mode: SymbioticMode) -> Self {
        self.symbiotic_mode = mode;
        self
    }

    /// Add a symbol to the symbol table
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    /// Retrieve a symbol by name, if it exists
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Add an external dependency to the context
    pub fn add_dependency(&mut self, dep: Dependency) {
        self.dependencies.push(dep);
    }

    /// Add a warning message during transmutation
    pub fn warn(&mut self, message: impl Into<String>) {
        self.warnings.push(message.into());
    }

    /// Set arbitrary metadata key-value pair
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

impl Default for TransmutationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Dependency {
    /// Create a new dependency instance
    pub fn new(name: impl Into<String>, version: Option<impl Into<String>>, source: DependencySource) -> Self {
        Self {
            name: name.into(),
            version: version.map(|v| v.into()),
            source,
        }
    }
}