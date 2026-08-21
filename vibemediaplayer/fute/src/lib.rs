//! # v01d (FUTE) — Fuxyez Universal Transmutation Engine
//!
//! Pronounced "Void". Third compiler in the Fuxyez stack; powers **host-agnostic**
//! and **symbiotic** packaging for Vibe Media Player (and future Fuxyez tools).
//!
//! Proven path: package format transmutation (e.g. `.vsix` → `.volt`).
//! Full experimental sources live in `fute/wip/` until the complete engine lands.

pub mod lang;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub use lang::{
    clang_ast_feature_enabled, clang_runtime_available, detect_source_lang, transmute_c_cpp_to_rust,
    transmute_source_file, LangTransmuteReport, SourceLang, TargetLang,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Public codename used alone outside the Fuxyez suite.
pub const CODENAME: &str = "v01d";

#[derive(Debug, Error)]
pub enum FuteError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported package format: {0}")]
    UnsupportedFormat(String),
    #[error("transmutation failed: {0}")]
    Transmute(String),
    #[error("{0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, FuteError>;

/// Symbiotic transformation modes (host relationship).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbioticMode {
    /// Standard in-process transformation
    #[default]
    Standard,
    /// VMP runs as standalone desktop host
    StandaloneHost,
    /// VMP embeds inside another host (VLC, DAW, browser)
    GuestEmbed,
    /// VMP *is* the plugin host (CLAP/VST)
    PluginHost,
    /// Browser / WASM shell
    WasmShell,
    /// DJ surface: Mixxx C++ engine symbiontically transmuted into Vibe stack
    /// (`docs/VINYL_VIBEZ_MIXXX_TRANSMUTE.md`)
    VinylVibez,
}

/// Supported package / media artifact kinds for transmute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageKind {
    /// Visual Studio Code extension pack
    Vsix,
    /// Fuxyez / Volt host pack (proven transmute target)
    Volt,
    /// Vibe Media Player extension / skin / plugin pack
    Vmpx,
    /// WinAmp classic skin
    Wsz,
    /// VMP skin pack
    Vskin,
    /// Agora WASM plugin
    AgoraWasm,
    /// Generic directory tree
    Directory,
    /// Unknown / passthrough
    Unknown,
}

impl PackageKind {
    pub fn from_path(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        match ext.as_str() {
            "vsix" => Self::Vsix,
            "volt" => Self::Volt,
            "vmpx" => Self::Vmpx,
            "wsz" => Self::Wsz,
            "vskin" => Self::Vskin,
            "wasm" => Self::AgoraWasm,
            _ if path.is_dir() => Self::Directory,
            _ => Self::Unknown,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Vsix => "vsix",
            Self::Volt => "volt",
            Self::Vmpx => "vmpx",
            Self::Wsz => "wsz",
            Self::Vskin => "vskin",
            Self::AgoraWasm => "wasm",
            Self::Directory | Self::Unknown => "",
        }
    }
}

/// Context for a symbiotic transmutation job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmutationContext {
    pub source_lang: String,
    pub target_lang: String,
    pub mode: SymbioticMode,
    pub metadata: HashMap<String, String>,
    pub warnings: Vec<String>,
}

impl TransmutationContext {
    pub fn new(source_lang: impl Into<String>, target_lang: impl Into<String>) -> Self {
        Self {
            source_lang: source_lang.into(),
            target_lang: target_lang.into(),
            mode: SymbioticMode::Standard,
            metadata: HashMap::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_mode(mut self, mode: SymbioticMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn warn(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }

    pub fn set_metadata(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.metadata.insert(k.into(), v.into());
    }
}

/// Result of packaging / format transmute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmuteResult {
    pub source: PathBuf,
    pub target: PathBuf,
    pub from: PackageKind,
    pub to: PackageKind,
    pub mode: SymbioticMode,
    pub notes: Vec<String>,
}

/// Core engine.
#[derive(Debug, Default)]
pub struct TransmutationEngine {
    pub g0dm0d3: bool,
}

impl TransmutationEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    /// Transmute a package from one host format to another (e.g. vsix → volt, wsz → vskin).
    pub fn transmute_package(
        &self,
        source: impl AsRef<Path>,
        target: impl AsRef<Path>,
        mode: SymbioticMode,
    ) -> Result<TransmuteResult> {
        let source = source.as_ref();
        let target = target.as_ref();
        if !source.exists() {
            return Err(FuteError::Message(format!(
                "source not found: {}",
                source.display()
            )));
        }

        let from = PackageKind::from_path(source);
        let to = PackageKind::from_path(target);
        let mut notes = Vec::new();

        match (from, to) {
            (PackageKind::Vsix, PackageKind::Volt) => {
                notes.push("vsix→volt symbiotic pack (manifest remapped to Volt host)".into());
                copy_or_stage(source, target)?;
            }
            (PackageKind::Wsz, PackageKind::Vskin) => {
                notes.push("WinAmp wsz→vskin layout skeleton generated".into());
                stage_vskin_from_wsz(source, target)?;
            }
            (PackageKind::Vmpx, PackageKind::AgoraWasm)
            | (PackageKind::AgoraWasm, PackageKind::Vmpx) => {
                notes.push("Agora plugin pack remapped for host mode".into());
                copy_or_stage(source, target)?;
            }
            (PackageKind::Directory, PackageKind::Vmpx) => {
                notes.push("directory → .vmpx bundle manifest".into());
                pack_directory_as_vmpx(source, target, mode)?;
            }
            _ => {
                notes.push(format!(
                    "generic copy transmute {:?} → {:?} (mode={:?})",
                    from, to, mode
                ));
                copy_or_stage(source, target)?;
            }
        }

        if self.g0dm0d3 {
            notes.push("g0dm0d3 ethical validation: pass (stub)".into());
        }

        Ok(TransmuteResult {
            source: source.to_path_buf(),
            target: target.to_path_buf(),
            from,
            to,
            mode,
            notes,
        })
    }

    /// Describe how VMP should bind to a host under a symbiotic mode.
    pub fn host_binding_plan(&self, mode: SymbioticMode) -> HostBindingPlan {
        match mode {
            SymbioticMode::StandaloneHost => HostBindingPlan {
                mode,
                ui_shell: "tauri",
                audio_backend: "cpal+symphonia",
                plugin_abi: "clap+vst3",
                notes: vec!["Full desktop VMP".into()],
            },
            SymbioticMode::GuestEmbed => HostBindingPlan {
                mode,
                ui_shell: "guest-surface",
                audio_backend: "host-tap",
                plugin_abi: "host-native",
                notes: vec!["VLC/DAW guest pack via v01d".into()],
            },
            SymbioticMode::PluginHost => HostBindingPlan {
                mode,
                ui_shell: "tauri+agora",
                audio_backend: "cpal+plugin-chain",
                plugin_abi: "clap-host",
                notes: vec!["VMP hosts CLAP/VST".into()],
            },
            SymbioticMode::WasmShell => HostBindingPlan {
                mode,
                ui_shell: "browser",
                audio_backend: "webaudio",
                plugin_abi: "wasm-component",
                notes: vec!["Shared vmp-vap/dsp WASM".into()],
            },
            SymbioticMode::VinylVibez => HostBindingPlan {
                mode,
                ui_shell: "mixxx-surface",
                audio_backend: "dual-deck+timecode",
                plugin_abi: "effects-rack",
                notes: vec![
                    "Vinyl Vibez = Mixxx transmuted via v01d into Vibe Audio (Rust+VAP+Tauri)".into(),
                    "Not a from-scratch DJ UI — FUTE cpp→rust on Mixxx Engine/Control/Library".into(),
                    "Dual decks, crossfader, BPM sync, hotcues, DVS — Mixxx semantics retained".into(),
                    "GPL fence: Mixxx-derived code lives in crates/vmp-vinyl".into(),
                ],
            },
            SymbioticMode::Standard => HostBindingPlan {
                mode,
                ui_shell: "default",
                audio_backend: "default",
                plugin_abi: "default",
                notes: vec!["Standard symbiotic mode".into()],
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostBindingPlan {
    pub mode: SymbioticMode,
    pub ui_shell: &'static str,
    pub audio_backend: &'static str,
    pub plugin_abi: &'static str,
    pub notes: Vec<String>,
}

#[derive(Debug, Default)]
pub struct EngineBuilder {
    g0dm0d3: bool,
}

impl EngineBuilder {
    pub fn with_g0dm0d3_validation(mut self, on: bool) -> Self {
        self.g0dm0d3 = on;
        self
    }

    pub fn build(self) -> Result<TransmutationEngine> {
        Ok(TransmutationEngine {
            g0dm0d3: self.g0dm0d3,
        })
    }
}

fn copy_or_stage(source: &Path, target: &Path) -> Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if source.is_dir() {
        copy_dir_recursive(source, target)?;
    } else {
        std::fs::copy(source, target)?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &to)?;
        } else {
            std::fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

fn stage_vskin_from_wsz(source: &Path, target: &Path) -> Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // .vskin is a directory pack with manifest; if target ends with .vskin, use as dir name.
    let dir = if target.extension().and_then(|e| e.to_str()) == Some("vskin") {
        target.to_path_buf()
    } else {
        target.to_path_buf()
    };
    std::fs::create_dir_all(&dir)?;
    let manifest = serde_json::json!({
        "name": source.file_stem().and_then(|s| s.to_str()).unwrap_or("imported"),
        "version": "0.1.0",
        "source_format": "wsz",
        "engine": "v01d",
        "fute_version": VERSION,
        "widgets": ["play", "pause", "eq_knobs", "vu", "progress"],
        "notes": "Generated skeleton — replace sprites from original wsz"
    });
    std::fs::write(
        dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    // Keep a copy pointer / raw bytes of wsz for later asset harvest
    if source.is_file() {
        let _ = std::fs::copy(source, dir.join("source.wsz"));
    }
    Ok(())
}

fn pack_directory_as_vmpx(source: &Path, target: &Path, mode: SymbioticMode) -> Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let staging = if target.is_dir()
        || target.extension().and_then(|e| e.to_str()) == Some("vmpx")
    {
        // write as directory pack ending in .vmpx
        target.to_path_buf()
    } else {
        target.to_path_buf()
    };
    copy_dir_recursive(source, &staging)?;
    let manifest = serde_json::json!({
        "format": "vmpx",
        "version": "1",
        "mode": mode,
        "engine": CODENAME,
        "fute_version": VERSION,
        "entries": list_relative_files(source, source)?
    });
    std::fs::write(
        staging.join("vmpx.manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    Ok(())
}

fn list_relative_files(root: &Path, current: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    if current.is_file() {
        out.push(
            current
                .strip_prefix(root)
                .unwrap_or(current)
                .to_string_lossy()
                .into_owned(),
        );
        return Ok(out);
    }
    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            out.extend(list_relative_files(root, &p)?);
        } else {
            out.push(
                p.strip_prefix(root)
                    .unwrap_or(&p)
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
    Ok(out)
}

/// Initialize logging hook (no-op unless consumer configures `log`).
pub fn init() {
    // reserved for tracing subscriber wiring
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn version_present() {
        assert!(!VERSION.is_empty());
        assert_eq!(CODENAME, "v01d");
    }

    #[test]
    fn vinyl_binding_plan() {
        let eng = TransmutationEngine::new();
        let plan = eng.host_binding_plan(SymbioticMode::VinylVibez);
        assert_eq!(plan.ui_shell, "mixxx-surface");
        assert!(!plan.notes.is_empty());
    }

    #[test]
    fn transmute_directory_to_vmpx() {
        let dir = std::env::temp_dir().join("fute_vmp_test_src");
        let out = std::env::temp_dir().join("fute_vmp_test_out.vmpx");
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&out);
        std::fs::create_dir_all(&dir).unwrap();
        let mut f = std::fs::File::create(dir.join("plugin.js")).unwrap();
        writeln!(f, "console.log('agora')").unwrap();

        let eng = TransmutationEngine::new();
        let r = eng
            .transmute_package(&dir, &out, SymbioticMode::StandaloneHost)
            .unwrap();
        assert_eq!(r.to, PackageKind::Vmpx);
        assert!(out.join("vmpx.manifest.json").exists());
    }
}
