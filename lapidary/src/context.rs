//! Lapidary Transmutation Context (Aurphyx SUXS Core)
//!
//! Compliant with Aurphyx Symbiotic Universal Xessability Standards (SUXS).
//! This module acts as the central nervous system for the Lapidary pipeline.
//! It carries the operational state, telemetry, symbol tables, and diagnostic 
//! buffers across the asynchronous deconstruction, transformation, and synthesis stages.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Defines the operational frequency and strictness of the transmutation engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum XessMode {
    /// Standard pass-through logic. Fast, but less strict on layout bloat.
    #[default]
    Standard,
    /// The Aurphyx standard. Strips all IDE-specific UI, enforces pure LSP boundaries.
    Sacred,
    /// Highly experimental. Attempts to transpile unsupported TS constructs natively.
    Mystical,
}

/// Tracks external dependencies required by the source VSIX package.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version_req: String,
    pub is_runtime_critical: bool,
}

/// Tracks isolated architectural symbols (e.g., binaries, scripts, WASM shims).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub identifier: String,
    pub symbol_type: SymbolType,
    pub original_path: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolType {
    BinaryExecutable,
    NodeScript,
    WasmProxy,
    ConfigurationMap,
}

/// Defines the severity of pipeline diagnostics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Critical,
}

/// A structured telemetry event emitted during the transmutation lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub component: String,
}

/// The Master Context Matrix. 
/// Passed mutably through the pipeline stages to aggregate state and artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LapidaryContext {
    /// The target runtime architecture (Defaults to "lapce-volt").
    pub target_runtime: String,
    /// The strictness level of the transformation passes.
    pub mode: XessMode,
    /// Extracted runtime variables and manifest metadata (name, version, etc).
    pub metadata: HashMap<String, String>,
    /// Tracked execution targets and structural components.
    pub symbol_table: HashMap<String, Symbol>,
    /// Dependencies parsed from the original VS Code manifest.
    pub dependencies: Vec<Dependency>,
    /// The telemetry buffer capturing all pipeline diagnostic events.
    pub diagnostics: Vec<Diagnostic>,
}

impl LapidaryContext {
    /// Initializes a pristine, empty context ready for the pipeline.
    pub fn new() -> Self {
        Self {
            target_runtime: "lapce-volt".to_string(),
            mode: XessMode::default(),
            metadata: HashMap::new(),
            symbol_table: HashMap::new(),
            dependencies: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Builder pattern hook to enforce a specific Xessability mode.
    pub fn with_mode(mut self, mode: XessMode) -> Self {
        self.mode = mode;
        self
    }

    /// Builder pattern hook to override the default target runtime.
    pub fn with_target_runtime(mut self, target: impl Into<String>) -> Self {
        self.target_runtime = target.into();
        self
    }

    // ════════════════════════════════════════════════════════════════════════
    // METADATA & STATE MANAGEMENT
    // ════════════════════════════════════════════════════════════════════════

    /// Safely writes a metadata key-value pair into the matrix.
    pub fn set_meta(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Retrieves a metadata value, returning an empty string if undefined.
    pub fn get_meta_or_default(&self, key: &str) -> String {
        self.metadata.get(key).cloned().unwrap_or_default()
    }

    /// Registers a newly discovered structural symbol (e.g., a language server binary).
    pub fn register_symbol(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol.identifier.clone(), symbol);
    }

    /// Retrieves a registered symbol by its identifier.
    pub fn get_symbol(&self, identifier: &str) -> Option<&Symbol> {
        self.symbol_table.get(identifier)
    }

    /// Registers a detected external dependency.
    pub fn register_dependency(&mut self, dep: Dependency) {
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
    }

    // ════════════════════════════════════════════════════════════════════════
    // TELEMETRY & DIAGNOSTICS
    // ════════════════════════════════════════════════════════════════════════

    /// Logs an informational event from a specific pipeline component.
    pub fn log_info(&mut self, component: &str, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: message.into(),
            component: component.to_string(),
        });
    }

    /// Logs a non-fatal warning requiring developer attention or engine fallback.
    pub fn log_warning(&mut self, component: &str, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            component: component.to_string(),
        });
    }

    /// Logs a critical failure event that compromises the structural output.
    pub fn log_critical(&mut self, component: &str, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Critical,
            message: message.into(),
            component: component.to_string(),
        });
    }

    /// Analyzes the telemetry buffer to determine if the pipeline suffered a fatal error.
    pub fn has_critical_failures(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::Critical)
    }
    
    /// Flushes all warnings and critical diagnostics to the standard output buffer.
    pub fn flush_diagnostics_to_stdout(&self) {
        for diag in &self.diagnostics {
            match diag.level {
                DiagnosticLevel::Critical => eprintln!("❌ [CRITICAL | {}] {}", diag.component, diag.message),
                DiagnosticLevel::Warning => println!("⚠️  [WARNING  | {}] {}", diag.component, diag.message),
                DiagnosticLevel::Info => println!("ℹ️  [INFO     | {}] {}", diag.component, diag.message),
            }
        }
    }
}

impl Default for LapidaryContext {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIT TESTS (Validation & Integrity)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_initialization_and_mode_switching() {
        let ctx = LapidaryContext::new().with_mode(XessMode::Sacred);
        assert_eq!(ctx.mode, XessMode::Sacred);
        assert_eq!(ctx.target_runtime, "lapce-volt");
    }

    #[test]
    fn test_metadata_matrix_operations() {
        let mut ctx = LapidaryContext::new();
        ctx.set_meta("engine_version", "1.4.2");
        
        assert_eq!(ctx.metadata.get("engine_version").unwrap(), "1.4.2");
        assert_eq!(ctx.get_meta_or_default("missing_key"), "");
    }

    #[test]
    fn test_symbol_registration_integrity() {
        let mut ctx = LapidaryContext::new();
        let sym = Symbol {
            identifier: "rust-analyzer-core".to_string(),
            symbol_type: SymbolType::BinaryExecutable,
            original_path: "bin/rust-analyzer".to_string(),
        };

        ctx.register_symbol(sym.clone());
        
        let retrieved = ctx.get_symbol("rust-analyzer-core").unwrap();
        assert_eq!(retrieved.original_path, "bin/rust-analyzer");
        assert_eq!(retrieved.symbol_type, SymbolType::BinaryExecutable);
    }

    #[test]
    fn test_telemetry_diagnostic_buffering() {
        let mut ctx = LapidaryContext::new();
        
        ctx.log_info("Parser", "Deconstruction initiated.");
        ctx.log_warning("Transformer", "Ghost UI Node detected.");
        assert!(!ctx.has_critical_failures());

        ctx.log_critical("Generator", "WASI compilation failed.");
        assert!(ctx.has_critical_failures());
        
        assert_eq!(ctx.diagnostics.len(), 3);
    }
}