//! Thin bridge: Vibe Media Player ↔ v01d (FUTE).

use fute::{
    HostBindingPlan, SymbioticMode, TransmutationEngine, TransmuteResult, CODENAME, VERSION,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum V01dError {
    #[error(transparent)]
    Fute(#[from] fute::FuteError),
}

pub type Result<T> = std::result::Result<T, V01dError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V01dInfo {
    pub codename: String,
    pub version: String,
    pub role: String,
}

pub fn info() -> V01dInfo {
    V01dInfo {
        codename: CODENAME.into(),
        version: VERSION.into(),
        role: "Fuxyez third compiler — symbiotic packaging for VMP".into(),
    }
}

pub fn engine() -> TransmutationEngine {
    TransmutationEngine::new()
}

pub fn binding_for_player() -> HostBindingPlan {
    engine().host_binding_plan(SymbioticMode::StandaloneHost)
}

pub fn binding_for_vinyl_vibez() -> HostBindingPlan {
    engine().host_binding_plan(SymbioticMode::VinylVibez)
}

pub fn transmute_pack(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    mode: SymbioticMode,
) -> Result<TransmuteResult> {
    Ok(engine().transmute_package(source, target, mode)?)
}

/// Map VMP app mode string → symbiotic mode.
pub fn mode_from_ui(name: &str) -> SymbioticMode {
    let key: String = name
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    match key.as_str() {
        "vinyl" | "vinylvibez" | "mixxx" | "dj" => SymbioticMode::VinylVibez,
        "guest" | "vlc" => SymbioticMode::GuestEmbed,
        "wasm" | "browser" => SymbioticMode::WasmShell,
        "host" | "pluginhost" => SymbioticMode::PluginHost,
        _ => SymbioticMode::StandaloneHost,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vinyl_mode() {
        assert!(matches!(
            mode_from_ui("Vinyl Vibez"),
            SymbioticMode::VinylVibez
        ));
        let plan = binding_for_vinyl_vibez();
        assert_eq!(plan.ui_shell, "mixxx-surface");
    }
}
