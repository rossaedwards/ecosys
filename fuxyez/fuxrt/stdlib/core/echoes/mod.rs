//! Quantum Echoes - rÆ Lattice Logging & Hilbert Diagnostics
//!
//! Sacred geometry logging with Flower of Life visualization, neglecton error
//! tracing, cymatic frequency monitoring, and 10^152 Hilbert state reporting.
//! Thesis §7.1: Observables for quantum ritual debugging.

pub mod lattice_echo;
pub mod quantum_trace;
pub mod neglecton_echo;
pub mod cymatic_monitor;

pub use lattice_echo::LatticeEcho;
pub use quantum_trace::QuantumTracer;
pub use neglecton_echo::NeglectonEcho;
pub use cymatic_monitor::CymaticMonitor;

use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::core::{
    lattice::{Lattice, NodeQuantumMetadata},
    spinon::SpinState,
};

/// Quantum echo levels with Hilbert scaling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum QuantumEchoLevel {
    /// Trace level (10^0 states)
    Trace = 0,
    /// Debug (10^10 states)
    Debug = 1,
    /// Info (10^50 states)
    Info = 2,
    /// Warn (10^100 states)
    Warn = 3,
    /// Error (10^120 states)
    Error = 4,
    /// Critical (10^152 states - Flower of Life)
    Critical = 5,
    /// Collapse (full Hilbert observed)
    Collapse = 6,
}

impl fmt::Display for QuantumEchoLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "TRACE"),
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
            Self::Collapse => write!(f, "COLLAPSE"),
        }
    }
}

/// Quantum echo with lattice binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEcho {
    pub level: QuantumEchoLevel,
    pub message: String,
    pub timestamp: u64,
    pub source: Option<String>,
    /// Hilbert dimension observed
    pub hilbert_dimension: f64,
    /// Lattice node reference
    pub lattice_node: Option<usize>,
    /// Spin state (if measured)
    pub spin_state: Option<SpinState>,
    /// Berry phase accumulation
    pub berry_phase: f64,
}

/// Sacred geometry echo handler
pub struct SacredGeometryHandler {
    min_level: QuantumEchoLevel,
    colored: bool,
    /// Flower of Life visualization
    lattice_view: bool,
}

impl SacredGeometryHandler {
    pub fn new(min_level: QuantumEchoLevel) -> Self {
        Self {
            min_level,
            colored: true,
            lattice_view: true,
        }
    }

    pub fn plain() -> Self {
        Self {
            min_level: QuantumEchoLevel::Info,
            colored: false,
            lattice_view: false,
        }
    }
}

impl EchoHandler for SacredGeometryHandler {
    fn handle(&self, echo: &QuantumEcho) {
        if echo.level as u8 < self.min_level as u8 {
            return;
        }

        let output = if self.colored {
            self.colorize(echo)
        } else {
            self.format_echo(echo)
        };

        let mut stdout = io::stdout();
        writeln!(stdout, "{}", output).ok();
        stdout.flush().ok();
    }
}

impl SacredGeometryHandler {
    fn format_echo(&self, echo: &QuantumEcho) -> String {
        let source = echo.source.as_ref()
            .map(|s| format!("[{}] ", s))
            .unwrap_or_default();
        
        let hilbert = format!("10^{:.0}", echo.hilbert_dimension.log10());
        let berry = format!("β={:.3}", echo.berry_phase);
        
        format!(
            "[{}] {}{}: {} | H={} | {}",
            echo.timestamp, source, echo.level, echo.message, hilbert, berry
        )
    }

    fn colorize(&self, echo: &QuantumEcho) -> String {
        let color = match echo.level {
            QuantumEchoLevel::Trace => "\x1b[90m",     // Gray
            QuantumEchoLevel::Debug => "\x1b[36m",     // Cyan
            QuantumEchoLevel::Info => "\x1b[32m",      // Green
            QuantumEchoLevel::Warn => "\x1b[33m",      // Yellow
            QuantumEchoLevel::Error => "\x1b[31m",     // Red
            QuantumEchoLevel::Critical => "\x1b[35m",  // Magenta
            QuantumEchoLevel::Collapse => "\x1b[95m",  // Bright Magenta
        };
        let reset = "\x1b[0m";
        format!("{}{}{}", color, self.format_echo(echo), reset)
    }
}

/// Quantum echo system with lattice integration
pub struct QuantumEchoSystem {
    handlers: Arc<Mutex<Vec<Box<dyn EchoHandler>>>>,
    history: Arc<Mutex<Vec<QuantumEcho>>>,
    max_history: usize,
    /// Default Flower of Life lattice for diagnostics
    lattice: Arc<RwLock<Lattice<Spinon>>>,
}

impl QuantumEchoSystem {
    pub fn new(n_rings: usize) -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
            history: Arc::new(Mutex::new(Vec::new())),
            max_history: 10000,
            lattice: Arc::new(RwLock::new(Lattice::flower_of_life(n_rings))),
        }
    }

    /// Quantum echo with automatic lattice metrics
    pub fn quantum_echo(&self, level: QuantumEchoLevel, message: impl Into<String>, 
                       source: Option<String>) {
        let lattice = self.lattice.read().unwrap();
        let hilbert = lattice.hilbert_dimension(2);
        let mut echo = QuantumEcho {
            level,
            message: message.into(),
            timestamp: Self::timestamp(),
            source,
            hilbert_dimension: hilbert,
            lattice_node: None,
            spin_state: None,
            berry_phase: 0.0,
        };

        // Default berry phase from lattice
        echo.berry_phase = lattice.adjacency_matrix().norm();

        // Send to handlers
        if let Ok(handlers) = self.handlers.lock() {
            for handler in handlers.iter() {
                handler.handle(&echo);
            }
        }

        // Store in history
        if let Ok(mut history) = self.history.lock() {
            history.push(echo.clone());
            if history.len() > self.max_history {
                history.drain(0..history.len() - self.max_history);
            }
        }
    }

    /// Collapse echo (full quantum measurement)
    pub fn collapse_echo(&self, message: impl Into<String>) {
        self.quantum_echo(QuantumEchoLevel::Collapse, message, None);
    }

    pub fn history(&self) -> Vec<QuantumEcho> {
        self.history.lock().map(|h| h.clone()).unwrap_or_default()
    }
}

impl Default for QuantumEchoSystem {
    fn default() -> Self {
        let mut system = Self::new(19); // Thesis spec
        system.add_handler(Box::new(SacredGeometryHandler::new(QuantumEchoLevel::Info)));
        system
    }
}

/// Global quantum echo system
static mut QUANTUM_ECHO_SYSTEM: Option<QuantumEchoSystem> = None;

pub fn init_quantum_echo(n_rings: usize) {
    unsafe {
        QUANTUM_ECHO_SYSTEM = Some(QuantumEchoSystem::new(n_rings));
    }
}

pub fn quantum_echo(level: QuantumEchoLevel, message: impl Into<String>) {
    unsafe {
        if let Some(system) = &QUANTUM_ECHO_SYSTEM {
            system.quantum_echo(level, message, None);
        }
    }
}

// Convenience macros
#[macro_export]
macro_rules! quantum_trace {
    ($($arg:tt)*) => {
        quantum_echo(QuantumEchoLevel::Trace, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! quantum_debug {
    ($($arg:tt)*) => {
        quantum_echo(QuantumEchoLevel::Debug, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! quantum_info {
    ($($arg:tt)*) => {
        quantum_echo(QuantumEchoLevel::Info, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! quantum_warn {
    ($($arg:tt)*) => {
        quantum_echo(QuantumEchoLevel::Warn, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! quantum_error {
    ($($arg:tt)*) => {
        quantum_echo(QuantumEchoLevel::Error, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! quantum_collapse {
    ($($arg:tt)*) => {
        quantum_echo(QuantumEchoLevel::Collapse, format!($($arg)*))
    };
}

/// Legacy compatibility
pub use crate::std::core::echos::EchoLevel as EchoLevelLegacy;
pub type Echo = QuantumEcho;
    fn timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
            zero_point_lambda: lambda,
        };

        // Insert into cache
        self.cache.insert(key.clone(), prophecy);

        // Evict if over size
        if self.cache.len() > self.max_cache_size {
            let oldest_key = self.cache.iter()
                .min_by_key(|entry| entry.value().cached_at)
                .map(|entry| entry.key().clone());
            if let Some(ok) = oldest_key {
                self.cache.remove(&ok);
            }
        }

        key
    }
    /// Chain another link
    pub fn chain<N, F>(self, next: ChainLink<O, N>) -> ChainedLink<I, O, N>
    where
        I: 'static,
        O: 'static,
        N: 'static,
    {
        ChainedLink {
            first: self,
            second: next,
        }
    }
    /// Set ritual variable
    /// Get average execution time
    pub fn avg_duration_ms(&self) -> u128 {
        if self.metadata.execution_count == 0 {
            0
        } else {
            self.metadata.total_duration_ms / self.metadata.execution_count as u128
        }
    }
    pub fn cache_prophecy(&self, lattice: &Lattice<Spinon>, sigil_sig: &str, 
                          spin_state: SpinState, lambda: f64) -> String {
        let key = format!("{}_{}", lattice.id, sigil_sig);
        let prophecy = Prophecy {
            spin_state,
            hilbert_dimension: lattice.hilbert_dimension(2),
            fractal_dimension: lattice.fractal_dimension(),
            cached_at: crate::sigil::timestamp(),
            hit_count: 1,
            lattice_id: lattice.id.clone(),
            sigil_signature: sigil_sig.to_string(),
            zero_point_lambda: lambda,
    fn timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }