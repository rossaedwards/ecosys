#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - PART 3 (Backends)
#>
param([string]$RepoRoot = (Get-Location).Path)
$ErrorActionPreference = "Stop"

$src = Join-Path $RepoRoot "afs\tts\src"
$back = Join-Path $src "backends"
New-Item -ItemType Directory -Path $back -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $back "system") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $back "elevenlabs") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $back "piper") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $back "coqui") -Force | Out-Null

@"
pub mod system;
pub mod elevenlabs;
pub mod piper;
pub mod coqui;

pub mod selector;
pub mod capabilities;
pub mod errors;
"@ | Set-Content -Path (Join-Path $back "mod.rs") -Encoding UTF8

# System backend: bring your old logic in later; we just create the slot now.
@"
pub mod sapi;
pub mod say;
pub mod process;
pub mod voices;
pub mod detection;
"@ | Set-Content -Path (Join-Path $back "system\mod.rs") -Encoding UTF8

# ElevenLabs: WebSocket streaming will live here (docs support TTS WS streaming). [web:44]
@"
pub mod client;
pub mod websocket;
pub mod models;
pub mod auth;
pub mod voices;
pub mod streaming;
pub mod errors;
"@ | Set-Content -Path (Join-Path $back "elevenlabs\mod.rs") -Encoding UTF8

@"
pub mod cli;
pub mod models;
pub mod runner;
pub mod voices;
pub mod errors;
"@ | Set-Content -Path (Join-Path $back "piper\mod.rs") -Encoding UTF8

@"
pub mod server_client;
pub mod models;
pub mod auth;
pub mod voices;
pub mod errors;
"@ | Set-Content -Path (Join-Path $back "coqui\mod.rs") -Encoding UTF8

$files = @(
"selector.rs","capabilities.rs","errors.rs",
"system/sapi.rs","system/say.rs","system/process.rs","system/voices.rs","system/detection.rs",
"elevenlabs/client.rs","elevenlabs/websocket.rs","elevenlabs/models.rs","elevenlabs/auth.rs","elevenlabs/voices.rs","elevenlabs/streaming.rs","elevenlabs/errors.rs",
"piper/cli.rs","piper/models.rs","piper/runner.rs","piper/voices.rs","piper/errors.rs",
"coqui/server_client.rs","coqui/models.rs","coqui/auth.rs","coqui/voices.rs","coqui/errors.rs"
)

foreach ($f in $files) {
  $p = Join-Path $back $f
  $dir = Split-Path $p -Parent
  New-Item -ItemType Directory -Path $dir -Force | Out-Null
  if (-not (Test-Path $p)) { "pub struct Placeholder;" | Set-Content -Path $p -Encoding UTF8 }
}