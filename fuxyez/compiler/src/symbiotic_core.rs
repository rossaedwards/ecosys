/*!
Symbiot core for Fuxyez Compiler

The symbiotic system is the guardian and observer layer for both compile-time
and runtime operations. It inspects code events, execution context, and AST
transformations — proactively detecting anomalies, inefficiencies, or breaches
of the compiler’s sacred laws.

Symbiots operate as modular detectors that can attach to compiler subsystems
(parser, optimizer, executor) via hook registries. They report structured
SymbioticEvents to diagnostics and the runtime user interface.
*/

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

//------------------------------------------------------------------------------
// SEVERITY AND SYMBIOTIC EVENT STRUCTURES
//------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SymbioticEvent {
    pub code: String,
    pub message: String,
    pub severity: Severity,
    pub origin: String,
    pub span: Option<(usize, usize)>,
    pub metadata: Option<HashMap<String, String>>,
}

impl SymbioticEvent {
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

/// The core Symbiotic trait for modules implementing custom analysis logic.
/// These detectors observe inputs, AST, runtime traces, or patterns of execution,
/// returning one or more SymbioticEvents.
pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn inspect(&self, input: &str) -> Vec<SymbioticEvent>;
}

/// The global Symbiotic registry and orchestrator.
/// Manages active detectors and coordinates scans and event aggregation.
#[derive(Default)]
pub struct SymbioticRegistry {
    detectors: Arc<Mutex<Vec<Arc<dyn Detector>>>>,
    pub last_scan: Arc<Mutex<Vec<SymbioticEvent>>>,
}

impl SymbioticRegistry {
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

    /// Runs all registered Symbiots on input and returns aggregated events.
    pub fn run_all(&self, input: &str) -> Vec<SymbioticEvent> {
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
// SAMPLE BUILT-IN SYMBIOTIC DETECTORS
//------------------------------------------------------------------------------

/// Symbiotic detecting dangerous code patterns or deprecated syntaxes.
pub struct PatternSymbiotic;
impl Detector for PatternSymbiotic {
    fn name(&self) -> &'static str { "PatternSymbiotic" }

    fn inspect(&self, input: &str) -> Vec<SymbioticEvent> {
        let mut events = Vec::new();
        if input.contains("unsafe!") {
            events.push(SymbioticEvent::new(
                "FXZ001",
                "Usage of unsafe ritual detected",
                Severity::Warning,
                "PatternSymbiotic",
            ).with_metadata("category", "safety"));
        }
        if input.contains("goto") {
            events.push(SymbioticEvent::new(
                "FXZ002",
                "Usage of forbidden legacy construct (goto)",
                Severity::Error,
                "PatternSymbiotic",
            ).with_metadata("category", "stability"));
        }
        events
    }
}

/// Symbiotic detecting abnormal recursion or logical depth in expressions.
pub struct RecursionSymbiotic;
impl Detector for RecursionSymbiotic {
    fn name(&self) -> &'static str { "RecursionSymbiotic" }

    fn inspect(&self, input: &str) -> Vec<SymbioticEvent> {
        let depth = input.matches("if").count();
        if depth > 25 {
            vec![SymbioticEvent::new(
                "FXZ010",
                "Excessive conditional depth — potential recursion loop",
                Severity::Critical,
                "RecursionSymbiotic",
            )]
        } else { vec![] }
    }
}

/// Symbiotic analyzing performance inefficiencies or repeated constructs.
pub struct EfficiencySymbiotic;
impl Detector for EfficiencySymbiotic {
    fn name(&self) -> &'static str { "EfficiencySymbiotic" }

    fn inspect(&self, input: &str) -> Vec<SymbioticEvent> {
        let mut events = Vec::new();
        if input.matches("repeat").count() > 10 {
            events.push(SymbioticEvent::new(
                "FXZ020",
                "Excessive repeat operations: potential infinite ritual",
                Severity::Warning,
                "EfficiencySymbiotic",
            ).with_metadata("hint", "Consider using 'every' or temporal constraints"));
        }
        events
    }
}

//------------------------------------------------------------------------------
// LOGGING & HUMAN-READABLE SUMMARIES
//------------------------------------------------------------------------------

impl fmt::Display for SymbioticEvent {
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
// SYMBIOTIC ENGINE INIT (Default bootstrap)
//------------------------------------------------------------------------------

impl SymbioticRegistry {
    pub fn with_default_detectors() -> Self {
        let mut reg = Self::new();
        reg.register(Arc::new(PatternSymbiotic));
        reg.register(Arc::new(RecursionSymbiotic));
        reg.register(Arc::new(EfficiencySymbiotic));
        reg
    }
}

/* ===========================================================================
The Symbiotic Core is the compiler’s mystical nervous system.
Each detector forms a watchtower, a guard stationed within an aspect
of the Fuxyez ecosystem — syntax, ritual flow, runtime behavior,
and temporal balance.

Symbiotics not only report anomalies but evolve dynamically
through adaptive heuristics, plugging seamlessly into diagnostics
and runtime monitors.

Designed for limitless extensibility: any module can define
its own Detector, register it, and shape the future vigilance
of the compiler.
=========================================================================== */