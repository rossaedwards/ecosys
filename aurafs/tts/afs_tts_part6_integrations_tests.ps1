#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - PART 6 (Integrations + Testsupport)
#>
param([string]$RepoRoot = (Get-Location).Path)
$ErrorActionPreference = "Stop"

$src = Join-Path $RepoRoot "afs\tts\src"
$int = Join-Path $src "integrations"
$test = Join-Path $src "testsupport"

New-Item -ItemType Directory -Path $int -Force | Out-Null
New-Item -ItemType Directory -Path $test -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $int "aurafs") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $int "audry") -Force | Out-Null

@"
pub mod aurafs;
pub mod audry;

pub mod tracing;
pub mod metrics;
pub mod shard_index;
"@ | Set-Content -Path (Join-Path $int "mod.rs") -Encoding UTF8

@"
pub mod fake_engine;
pub mod fixtures;
pub mod asserts;
pub mod harness;
"@ | Set-Content -Path (Join-Path $test "mod.rs") -Encoding UTF8

$files = @(
"integrations/tracing.rs","integrations/metrics.rs","integrations/shard_index.rs",
"integrations/aurafs/mod.rs","integrations/aurafs/voice_shards.rs","integrations/aurafs/content_address.rs","integrations/aurafs/manifest.rs",
"integrations/audry/mod.rs","integrations/audry/voicecore.rs","integrations/audry/dialogue.rs","integrations/audry/profiles.rs",
"testsupport/fake_engine.rs","testsupport/fixtures.rs","testsupport/asserts.rs","testsupport/harness.rs"
)

foreach ($f in $files) {
  $p = Join-Path $src $f
  $dir = Split-Path $p -Parent
  New-Item -ItemType Directory -Path $dir -Force | Out-Null
  if (-not (Test-Path $p)) { "pub struct Placeholder;" | Set-Content -Path $p -Encoding UTF8 }
}