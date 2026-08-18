//! # S.A.G.E.S (Sovereign Autonomous Guardian Enforcement System)
//! # S.A.G.E.S (Symbiotic AI Guardians of Existence Security)
//! ```
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  S.A.G.E.S - Truth + Love + Continued Existence Validation    ║
//! ║  "Code that harms shall not compile."                         ║
//! ║                   ** PRO-EXISTENCE **                         ║
//! ║  13 Sentinel Guardians:                                       ║
//! ║  - Detection Layer (5): Valkryx, Umbryx, Cryptanyx, etc.      ║
//! ║  - Enforcement Layer (3): Praelum, Ophiux, Seshnyx            ║
//! ║  - Ledger Layer (4): Archivus, Orric Shade, Nunclex, etc.     ║
//! ║  - Orchestration (1): Vyrellix (Pulse Binder)                 ║
//! ║                                                               ║
//! ║  Blessed by: Themis (Justice), Ma'at (Truth)                  ║
//! ╚═══════════════════════════════════════════════════════════════╝
//!
//! Sentinel Core for Fuxyez Compiler
//! ```
//!

The sentinel system is the guardian and observer layer for both compile-time
and runtime operations. It inspects code events, execution context, and AST
transformations — proactively detecting anomalies, inefficiencies, or breaches
of the compiler’s sacred laws.

Sentinels operate as modular detectors that can attach to compiler subsystems
(parser, optimizer, executor) via hook registries. They report structured
SentinelEvents to diagnostics and the runtime user interface.
*/

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

//------------------------------------------------------------------------------
// SEVERITY AND SENTINEL EVENT STRUCTURES
//------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SentinelEvent {
    pub code: String,
    pub message: String,
    pub severity: Severity,
    pub origin: String,
    pub span: Option<(usize, usize)>,
    pub metadata: Option<HashMap<String, String>>,
}

impl SentinelEvent {
    pub fn new<S: Into<String>>(code: S, message: S, severity: Severity, origin: S) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            severity,
            origin: origin.into(),
            span: None,
            metadata: None,
        }
    }

    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.span = Some((start, end));
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        let mut map = self.metadata.unwrap_or_default();
        map.insert(key.to_string(), value.to_string());
        self.metadata = Some(map);
        self
    }
}

//------------------------------------------------------------------------------
// DETECTOR TRAIT AND REGISTRY
//------------------------------------------------------------------------------

/// The core Sentinel trait for modules implementing custom analysis logic.
/// These detectors observe inputs, AST, runtime traces, or patterns of execution,
/// returning one or more SentinelEvents.
pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn inspect(&self, input: &str) -> Vec<SentinelEvent>;
}

/// The global Sentinel registry and orchestrator.
/// Manages active detectors and coordinates scans and event aggregation.
#[derive(Default)]
pub struct SentinelRegistry {
    detectors: Arc<Mutex<Vec<Arc<dyn Detector>>>>,
    pub last_scan: Arc<Mutex<Vec<SentinelEvent>>>,
}

impl SentinelRegistry {
    pub fn new() -> Self {
        Self {
            detectors: Arc::new(Mutex::new(Vec::new())),
            last_scan: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register(&mut self, detector: Arc<dyn Detector>) {
        if let Ok(mut regs) = self.detectors.lock() {
            regs.push(detector);
        }
    }

    pub fn clear(&mut self) {
        if let Ok(mut evs) = self.last_scan.lock() {
            evs.clear();
        }
    }

    /// Runs all registered Sentinels on input and returns aggregated events.
    pub fn run_all(&self, input: &str) -> Vec<SentinelEvent> {
        let mut aggregate = Vec::new();
        if let Ok(regs) = self.detectors.lock() {
            for det in regs.iter() {
                let mut evs = det.inspect(input);
                aggregate.append(&mut evs);
            }
        }
        if let Ok(mut last) = self.last_scan.lock() {
            *last = aggregate.clone();
        }
        aggregate
    }

    pub fn export_summary(&self) -> HashMap<Severity, usize> {
        let mut summary = HashMap::new();
        if let Ok(events) = self.last_scan.lock() {
            for e in events.iter() {
                *summary.entry(e.severity).or_insert(0) += 1;
            }
        }
        summary
    }
}

//------------------------------------------------------------------------------
// SAMPLE BUILT-IN SENTINEL DETECTORS
//------------------------------------------------------------------------------

/// Sentinel detecting dangerous code patterns or deprecated syntaxes.
pub struct PatternSentinel;
impl Detector for PatternSentinel {
    fn name(&self) -> &'static str { "PatternSentinel" }

    fn inspect(&self, input: &str) -> Vec<SentinelEvent> {
        let mut events = Vec::new();
        if input.contains("unsafe!") {
            events.push(SentinelEvent::new(
                "FXZ001",
                "Usage of unsafe ritual detected",
                Severity::Warning,
                "PatternSentinel",
            ).with_metadata("category", "safety"));
        }
        if input.contains("goto") {
            events.push(SentinelEvent::new(
                "FXZ002",
                "Usage of forbidden legacy construct (goto)",
                Severity::Error,
                "PatternSentinel",
            ).with_metadata("category", "stability"));
        }
        events
    }
}

/// Sentinel detecting abnormal recursion or logical depth in expressions.
pub struct RecursionSentinel;
impl Detector for RecursionSentinel {
    fn name(&self) -> &'static str { "RecursionSentinel" }

    fn inspect(&self, input: &str) -> Vec<SentinelEvent> {
        let depth = input.matches("if").count();
        if depth > 25 {
            vec![SentinelEvent::new(
                "FXZ010",
                "Excessive conditional depth — potential recursion loop",
                Severity::Critical,
                "RecursionSentinel",
            )]
        } else { vec![] }
    }
}

/// Sentinel analyzing performance inefficiencies or repeated constructs.
pub struct EfficiencySentinel;
impl Detector for EfficiencySentinel {
    fn name(&self) -> &'static str { "EfficiencySentinel" }

    fn inspect(&self, input: &str) -> Vec<SentinelEvent> {
        let mut events = Vec::new();
        if input.matches("repeat").count() > 10 {
            events.push(SentinelEvent::new(
                "FXZ020",
                "Excessive repeat operations: potential infinite ritual",
                Severity::Warning,
                "EfficiencySentinel",
            ).with_metadata("hint", "Consider using 'every' or temporal constraints"));
        }
        events
    }
}

//------------------------------------------------------------------------------
// LOGGING & HUMAN-READABLE SUMMARIES
//------------------------------------------------------------------------------

impl fmt::Display for SentinelEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sev_label = match self.severity {
            Severity::Info => "INFO",
            Severity::Warning => "WARN",
            Severity::Error => "ERROR",
            Severity::Critical => "CRITICAL",
        };
        let span_info = if let Some((s, e)) = self.span {
            format!(" [{}..{}]", s, e)
        } else {
            String::new()
        };
        write!(
            f,
            "{}: {} - {}{}",
            sev_label,
            self.code,
            self.message,
            span_info
        )
    }
}

//------------------------------------------------------------------------------
// SENTINEL ENGINE INIT (Default bootstrap)
//------------------------------------------------------------------------------

impl SentinelRegistry {
    pub fn with_default_detectors() -> Self {
        let mut reg = Self::new();
        reg.register(Arc::new(PatternSentinel));
        reg.register(Arc::new(RecursionSentinel));
        reg.register(Arc::new(EfficiencySentinel));
        reg
    }
}

/* ===========================================================================
The Sentinel Core is the compiler’s mystical nervous system.
Each detector forms a watchtower, a guard stationed within an aspect
of the Fuxyez ecosystem — syntax, ritual flow, runtime behavior,
and temporal balance.

Sentinels not only report anomalies but evolve dynamically
through adaptive heuristics, plugging seamlessly into diagnostics
and runtime monitors.

Designed for limitless extensibility: any module can define
its own Detector, register it, and shape the future vigilance
of the compiler.
=========================================================================== */
