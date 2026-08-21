//! Fuxyez Complete Standard Library
pub mod aurafs;
pub mod chains;
pub mod echoes;
pub mod io;
pub mod oracle;
pub mod rituals;

pub use aurafs::*;
pub use chains::*;
pub use echoes::*;
pub use io::*;
pub use oracle::*;
pub use rituals::*;
use crate::core::rituals::{QuantumRitualContext, QuantumRitualEngine, QuantumRitualPhase};
use std::time::Duration;
use std::time::Instant;
impl QuantumRitualEngine {
    pub async fn execute_ceremony<F>(
        &self,
        name: &str,
        ritual: F,
    ) -> QuantumCeremonyResult
    where
        F: FnOnce() + Send + 'static,
    {
        let mut context = QuantumRitualContext::new(name);
        let start = Instant::now();
        context.begin();
        ritual();
        context.complete();
        QuantumCeremonyResult {
            context,
            duration: start.elapsed(),
            successful: true,
        }
    }
}