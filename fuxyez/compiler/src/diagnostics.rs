//! # Fuxyez Diagnostics - Compassionate Error Reporting
//!
//! ```
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  FUXYEZ DIAGNOSTICS - Where Errors Become Wisdom             ║
//! ║  "Every error is a teacher. Every fix is a blessing."        ║
//! ║                                                               ║
//! ║  Blessed by: Seshat (Goddess of Writing & Wisdom)            ║
//! ║              Thoth (Lord of Sacred Knowledge)                ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## Features
//!
//! - **Rich error context** with source snippets
//! - **Love score tracking** (ethical violations)
//! - **Fix suggestions** (auto-repair hints)
//! - **Multi-language support** (error messages in user's language)
//! - **SAGES integration** (Love + Continued Existence violation reasons)

use crate::ast::Span;
use miette::{Diagnostic as MietteDiagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════════════
// SEVERITY LEVELS
// ═══════════════════════════════════════════════════════════════════════════

/// Diagnostic severity (aligned with LSP protocol)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum Severity {
    /// Fatal error (cannot compile)
    Error,
    
    /// Warning (compiles but suspicious)
    Warning,
    
    /// Informational message
    Info,
    
    /// Hint/suggestion (style/optimization)
    Hint,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "❌ ERROR"),
            Severity::Warning => write!(f, "⚠️  WARNING"),
            Severity::Info => write!(f, "ℹ️  INFO"),
            Severity::Hint => write!(f, "💡 HINT"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DIAGNOSTIC CODES
// ═══════════════════════════════════════════════════════════════════════════

/// Standardized error codes (for documentation lookup)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum DiagnosticCode {
    /// Parse errors
    ParseError,
    SyntaxError,
    UnexpectedToken,
    
    /// Semantic errors
    TypeMismatch,
    UndefinedVariable,
    UndefinedFunction,
    
    /// Ethical errors (SAGES)
    LoveViolation,
    ExistenceViolation,
    HecateThresholdViolation,
    
    /// Runtime errors
    DivisionByZero,
    NullReference,
    
    /// Custom code
    Custom(String),
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticCode::ParseError => write!(f, "E0001"),
            DiagnosticCode::SyntaxError => write!(f, "E0002"),
            DiagnosticCode::UnexpectedToken => write!(f, "E0003"),
            DiagnosticCode::TypeMismatch => write!(f, "E1001"),
            DiagnosticCode::UndefinedVariable => write!(f, "E1002"),
            DiagnosticCode::UndefinedFunction => write!(f, "E1003"),
            DiagnosticCode::LoveViolation => write!(f, "SAGES001"),
            DiagnosticCode::ExistenceViolation => write!(f, "SAGES002"),
            DiagnosticCode::HecateThresholdViolation => write!(f, "SAGES003"),
            DiagnosticCode::DivisionByZero => write!(f, "R0001"),
            DiagnosticCode::NullReference => write!(f, "R0002"),
            DiagnosticCode::Custom(code) => write!(f, "{}", code),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MAIN DIAGNOSTIC TYPE
// ═══════════════════════════════════════════════════════════════════════════

/// A single diagnostic (error/warning/info/hint)
#[derive(Debug, Clone, Error, MietteDiagnostic, Serialize, Deserialize)]
#[error("{message}")]
pub struct FuxyezDiagnostic {
    /// Main error message
    pub message: String,
    
    /// Source code snippet (for pretty printing)
    #[source_code]
    pub source_code: Option<String>,
    
    /// Span in source (for highlighting)
    #[label("here")]
    pub span: Option<SourceSpan>,
    
    /// Severity level
    pub severity: Severity,
    
    /// Error code (for docs lookup)
    pub code: Option<DiagnosticCode>,
    
    /// Suggested fix
    #[help]
    pub help: Option<String>,
    
    /// Ethical score (0.0 = harmful, 1.0 = pure love)
    pub love_score: Option<f64>,
    
    /// Related diagnostics (e.g., "see also")
    pub related: Vec<RelatedDiagnostic>,
}

impl FuxyezDiagnostic {
    /// Create a new diagnostic
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source_code: None,
            span: None,
            severity,
            code: None,
            help: None,
            love_score: None,
            related: Vec::new(),
        }
    }

    /// Create an error
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Severity::Error, message)
    }

    /// Create a warning
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, message)
    }

    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Severity::Info, message)
    }

    /// Create a hint
    pub fn hint(message: impl Into<String>) -> Self {
        Self::new(Severity::Hint, message)
    }

    /// Add source code context
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source_code = Some(source.into());
        self
    }

    /// Add span
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(SourceSpan::new(span.start.into(), span.len().into()));
        self
    }

    /// Add error code
    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }

    /// Add help message
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Add love score
    pub fn with_love_score(mut self, score: f64) -> Self {
        self.love_score = Some(score);
        self
    }

    /// Add related diagnostic
    pub fn with_related(mut self, related: RelatedDiagnostic) -> Self {
        self.related.push(related);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RELATED DIAGNOSTICS
// ═══════════════════════════════════════════════════════════════════════════

/// A related diagnostic (e.g., "see also", "defined here")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedDiagnostic {
    pub message: String,
    pub span: Option<SourceSpan>,
}

// ═══════════════════════════════════════════════════════════════════════════
// DIAGNOSTIC COLLECTOR
// ═══════════════════════════════════════════════════════════════════════════

/// Accumulates diagnostics during compilation
#[derive(Debug, Clone, Default)]
pub struct DiagnosticCollector {
    diagnostics: Vec<FuxyezDiagnostic>,
}

impl DiagnosticCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, diag: FuxyezDiagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &FuxyezDiagnostic> {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &FuxyezDiagnostic> {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Warning)
    }

    pub fn all(&self) -> &[FuxyezDiagnostic] {
        &self.diagnostics
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_creation_works() {
        let diag = FuxyezDiagnostic::error("Test error")
            .with_code(DiagnosticCode::ParseError)
            .with_help("Try fixing this");
        
        assert_eq!(diag.severity, Severity::Error);
        assert_eq!(diag.code, Some(DiagnosticCode::ParseError));
        assert_eq!(diag.help, Some("Try fixing this".into()));
    }

    #[test]
    fn collector_works() {
        let mut collector = DiagnosticCollector::new();
        collector.add(FuxyezDiagnostic::error("Error 1"));
        collector.add(FuxyezDiagnostic::warning("Warning 1"));
        
        assert!(collector.has_errors());
        assert_eq!(collector.errors().count(), 1);
        assert_eq!(collector.warnings().count(), 1);
    }
}
    // Executor implementation would go here
}
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use crate::ast::*;
use crate::diagnostics::{Diagnostic, DiagnosticLevel, DiagnosticSink, Span};
use crate::Guardian_core::{GuardianEvent, GuardianRegistry, Severity};