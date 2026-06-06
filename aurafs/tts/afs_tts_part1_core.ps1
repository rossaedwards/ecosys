#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - PART 1 (Core)
#>

param([string]$RepoRoot = (Get-Location).Path)
$ErrorActionPreference = "Stop"

$crate = Join-Path $RepoRoot "afs\tts"
$src   = Join-Path $crate "src"

New-Item -ItemType Directory -Path $src -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $src "core") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $src "errors") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $src "types") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $src "util") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $crate "config") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $crate "scripts") -Force | Out-Null

@"
[package]
name = "afs-tts"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
uuid = { workspace = true }
async-trait = "0.1"

# Optional deps (enabled later when used)
reqwest = { version = "0.12", default-features = true, features = ["json", "stream"], optional = true }
tokio-tungstenite = { version = "0.24", optional = true }
futures-util = { version = "0.3", optional = true }

[features]
default = ["system"]
system = []
elevenlabs = ["dep:reqwest", "dep:tokio-tungstenite", "dep:futures-util"]
piper = []
coqui = ["dep:reqwest"]
"@ | Set-Content -Path (Join-Path $crate "Cargo.toml") -Encoding UTF8

@"
//! afs-tts
//! No samples. Only weapons-grade scaffolding.

pub mod core;
pub mod errors;
pub mod types;
pub mod util;

pub mod streaming;
pub mod backends;
pub mod voicepacks;
pub mod cache;
pub mod api;
pub mod cli;
pub mod integrations;
pub mod testsupport;

pub use core::{Tts, TtsEngine, TtsVoice};
"@ | Set-Content -Path (Join-Path $src "lib.rs") -Encoding UTF8

@"
pub mod engine;
pub mod traits;
pub mod registry;
pub mod routing;
pub mod policy;
pub mod health;
pub mod metrics;
pub mod runtime;
pub mod config;
pub mod shard_bridge;
pub mod prelude;

pub use engine::{TtsEngine, TtsVoice};
pub use traits::Tts;
"@ | Set-Content -Path (Join-Path $src "core\mod.rs") -Encoding UTF8

@"
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Tts: Send + Sync {
    async fn speak(&self, text: &str) -> Result<()>;
}
"@ | Set-Content -Path (Join-Path $src "core\traits.rs") -Encoding UTF8

@"
#[derive(Clone, Debug)]
pub enum TtsVoice {
    Default,
    Ross,
    Audry,
    QuantumAura,
}
"@ | Set-Content -Path (Join-Path $src "core\engine.rs") -Encoding UTF8

@"
pub struct EngineRegistry;
"@ | Set-Content -Path (Join-Path $src "core\registry.rs") -Encoding UTF8

@"
pub struct EngineRouter;
"@ | Set-Content -Path (Join-Path $src "core\routing.rs") -Encoding UTF8

@"
pub struct TtsPolicy;
"@ | Set-Content -Path (Join-Path $src "core\policy.rs") -Encoding UTF8

@"
pub struct TtsHealth;
"@ | Set-Content -Path (Join-Path $src "core\health.rs") -Encoding UTF8

@"
pub struct TtsMetrics;
"@ | Set-Content -Path (Join-Path $src "core\metrics.rs") -Encoding UTF8

@"
pub struct TtsRuntime;
"@ | Set-Content -Path (Join-Path $src "core\runtime.rs") -Encoding UTF8

@"
#[derive(Clone, Debug)]
pub struct TtsConfig {
    pub engine: String,
}
"@ | Set-Content -Path (Join-Path $src "core\config.rs") -Encoding UTF8

@"
pub struct ShardBridge;
"@ | Set-Content -Path (Join-Path $src "core\shard_bridge.rs") -Encoding UTF8

@"
pub mod v1;
"@ | Set-Content -Path (Join-Path $src "core\prelude.rs") -Encoding UTF8

@"
pub use crate::core::{Tts, TtsEngine, TtsVoice};
"@ | Set-Content -Path (Join-Path $src "core\prelude\v1.rs") -Encoding UTF8

@"
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("TTS backend error: {0}")]
    Backend(String),
    #[error("TTS config error: {0}")]
    Config(String),
    #[error("TTS streaming error: {0}")]
    Streaming(String),
}
"@ | Set-Content -Path (Join-Path $src "errors\mod.rs") -Encoding UTF8

@"
pub struct AudioBytes(pub Vec<u8>);
"@ | Set-Content -Path (Join-Path $src "types\audio_bytes.rs") -Encoding UTF8

@"
pub struct TextChunk(pub String);
"@ | Set-Content -Path (Join-Path $src "types\text_chunk.rs") -Encoding UTF8

@"
pub struct VoiceId(pub String);
"@ | Set-Content -Path (Join-Path $src "types\voice_id.rs") -Encoding UTF8

@"
pub mod audio_bytes;
pub mod text_chunk;
pub mod voice_id;
"@ | Set-Content -Path (Join-Path $src "types\mod.rs") -Encoding UTF8

@"
pub fn now_unix_ms() -> u64 { 0 }
"@ | Set-Content -Path (Join-Path $src "util\time.rs") -Encoding UTF8

@"
pub mod time;
"@ | Set-Content -Path (Join-Path $src "util\mod.rs") -Encoding UTF8

@"
# afs-tts
Config lives in config/*.toml, scripts in scripts/.
"@ | Set-Content -Path (Join-Path $crate "README.md") -Encoding UTF8

@"
engine = "system"
"@ | Set-Content -Path (Join-Path $crate "config\default.toml") -Encoding UTF8

@"
#!/usr/bin/env bash
set -e
cargo check -p afs-tts
"@ | Set-Content -Path (Join-Path $crate "scripts\check.sh") -Encoding UTF8

@"
param()
cargo check -p afs-tts
"@ | Set-Content -Path (Join-Path $crate "scripts\check.ps1") -Encoding UTF8