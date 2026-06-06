#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - PART 5 (API + CLI + BIN)
#>
param([string]$RepoRoot = (Get-Location).Path)
$ErrorActionPreference = "Stop"

$crate = Join-Path $RepoRoot "afs\tts"
$src   = Join-Path $crate "src"
$api   = Join-Path $src "api"
$cli   = Join-Path $src "cli"
$bin   = Join-Path $src "bin"

New-Item -ItemType Directory -Path $api -Force | Out-Null
New-Item -ItemType Directory -Path $cli -Force | Out-Null
New-Item -ItemType Directory -Path $bin -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $api "http") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $api "grpc") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $cli "commands") -Force | Out-Null

@"
pub mod http;
pub mod grpc;

pub mod dto;
pub mod routes;
pub mod server;
pub mod errors;
"@ | Set-Content -Path (Join-Path $api "mod.rs") -Encoding UTF8

@"
pub mod commands;

pub mod args;
pub mod output;
pub mod wiring;
"@ | Set-Content -Path (Join-Path $cli "mod.rs") -Encoding UTF8

# ---------- Ensure clap dependency exists in afs/tts/Cargo.toml ----------
$cargoToml = Join-Path $crate "Cargo.toml"
if (Test-Path $cargoToml) {
  $raw = Get-Content $cargoToml -Raw
  if ($raw -notmatch "(?m)^\s*clap\s*=") {
    # Insert under [dependencies] (simple insertion)
    $lines = Get-Content $cargoToml
    $out = New-Object System.Collections.Generic.List[string]
    $inserted = $false
    foreach ($line in $lines) {
      $out.Add($line)
      if (-not $inserted -and $line -match "^\[dependencies\]\s*$") {
        $out.Add('clap = { workspace = true }')
        $inserted = $true
      }
    }
    Set-Content -Path $cargoToml -Value ($out -join "`n") -Encoding UTF8
  }
}

# ---------- Create API/CLI placeholder modules ----------
$files = @(
"api/dto.rs","api/routes.rs","api/server.rs","api/errors.rs",
"api/http/mod.rs","api/http/health.rs","api/http/speak.rs","api/http/stream.rs","api/http/voices.rs",
"api/grpc/mod.rs","api/grpc/tts_service.rs","api/grpc/health.rs",
"cli/args.rs","cli/output.rs","cli/wiring.rs",
"cli/commands/mod.rs","cli/commands/speak.rs","cli/commands/stream.rs","cli/commands/voices.rs","cli/commands/cache.rs","cli/commands/voicepack.rs"
)

foreach ($f in $files) {
  $p = Join-Path $src $f
  $dir = Split-Path $p -Parent
  New-Item -ItemType Directory -Path $dir -Force | Out-Null
  if (-not (Test-Path $p)) { "pub struct Placeholder;" | Set-Content -Path $p -Encoding UTF8 }
}

# ---------- Binary: src/bin/tts_cli.rs ----------
$ttsCli = Join-Path $bin "tts_cli.rs"

@"
use clap::{Parser, Subcommand};
use anyhow::Result;

use afs_tts::core::{Tts, TtsEngine, TtsVoice};

#[derive(Parser, Debug)]
#[command(name = "tts-cli", version, about = "AURAFS TTS CLI (afs-tts crate)")]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Speak {
        #[arg(long, default_value = "Default")]
        voice: String,
        #[arg()]
        text: String,
    },
    Voices,
}

fn parse_voice(v: &str) -> TtsVoice {
    match v.to_lowercase().as_str() {
        "ross" => TtsVoice::Ross,
        "audry" => TtsVoice::Audry,
        "quantumaura" | "quantum_aura" | "quantum" => TtsVoice::QuantumAura,
        _ => TtsVoice::Default,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::Speak { voice, text } => {
            let engine = TtsEngine { voice: parse_voice(&voice) };
            engine.speak(&text).await?;
        }
        Command::Voices => {
            println!("Default");
            println!("Ross");
            println!("Audry");
            println!("QuantumAura");
        }
    }

    Ok(())
}
"@ | Set-Content -Path $ttsCli -Encoding UTF8 -Force