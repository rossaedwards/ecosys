//! Vibe Audio Visualizer runtime — **transmuted** from C via v01d (FUTE).
//!
//! Origin tree: `main/vibe-audio-visualizer/`
//!   - `vap_runtime.c/h`
//!   - `dsp_engine.c`
//!   - `vap/vap_photometric.h`, `vap_affective.h`
//!
//! The FUTE language pipeline (`v01d lang --from c --to rust`) produces structural
//! scaffolding; this crate is the polished, compile-ready symbiont integrated into VMP.

mod photometric;
mod runtime;

pub use photometric::*;
pub use runtime::*;

/// Provenance stamp for symbiotic packaging.
pub const TRANSMUTE_ORIGIN: &str = "vibe-audio-visualizer";
pub const TRANSMUTE_ENGINE: &str = "v01d/FUTE";
