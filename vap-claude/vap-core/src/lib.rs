//! vap-core — V.A.P. v3.1 runtime, live DSP, and sidecar loader.
//!
//! Rust port of `vap/`, `src/dsp_engine.*`, `src/vap_runtime.*`, and
//! `src/vap_loader.*` from the reference C implementation in
//! `vibemediaplayer/visualizer/`. See that tree's `src/gl_renderer.c` for
//! the exact uniform-upload contract this crate's outputs feed.

pub mod dsp_engine;
pub mod nexus_vibez;
pub mod vap_affective;
pub mod vap_loader;
pub mod vap_photometric;
pub mod vap_runtime;

pub use dsp_engine::{apply_frame, DspEngine, DspFrame};
pub use nexus_vibez::{NexusVibezEngine, NexusVibezFrame};
pub use vap_affective::VapAffective;
pub use vap_loader::{load as load_vap, parse_json as parse_vap_json, LoadSource};
pub use vap_photometric::{VapPhotometric, CHROMATIC_MAP};
pub use vap_runtime::VapRuntime;
