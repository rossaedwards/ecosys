//! ═══════════════════════════════════════════════════════════════════
//! AI Module - Fractal Orchestration & Intelligence
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

// Core AI modules that exist
pub mod error;
pub mod fractal_orchestrator;

// Re-exports
pub use error::AiError;
pub use fractal_orchestrator::FractalOrchestrator;

/// Initialize the AI subsystem
pub fn init() {
    tracing::info!("🧠 AI subsystem initialized");
}
