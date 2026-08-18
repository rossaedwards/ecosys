//! Fuxyez standard library

pub mod rituals;
pub mod echoes;

#[cfg(feature = "chains")]
pub mod chains;

#[cfg(feature = "oracles")]
pub mod oracle;

pub use rituals::{RitualContext, RitualPhase, RitualBuilder};
pub use echoes::{Echo, EchoLevel, EchoSystem, echo, info, warn, error};

#[cfg(feature = "chains")]
pub use chains::{Chain, ChainMode};

#[cfg(feature = "oracles")]
pub use oracle::{Oracle, OracleSource};