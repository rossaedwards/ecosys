#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - PART 4 (Voicepacks + Cache)
#>
param([string]$RepoRoot = (Get-Location).Path)
$ErrorActionPreference = "Stop"

$src = Join-Path $RepoRoot "afs\tts\src"

$vp = Join-Path $src "voicepacks"
$cache = Join-Path $src "cache"
New-Item -ItemType Directory -Path $vp -Force | Out-Null
New-Item -ItemType Directory -Path $cache -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $vp "registry") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $vp "formats") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $cache "store") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $cache "policy") -Force | Out-Null

@"
pub mod registry;
pub mod formats;

pub mod voicepack;
pub mod manifest;
pub mod download;
pub mod verify;
pub mod licensing;
pub mod tags;
pub mod locale;
"@ | Set-Content -Path (Join-Path $vp "mod.rs") -Encoding UTF8

@"
pub mod store;
pub mod policy;

pub mod key;
pub mod index;
pub mod fs_store;
pub mod eviction;
pub mod hashing;
"@ | Set-Content -Path (Join-Path $cache "mod.rs") -Encoding UTF8

$files = @(
"voicepack.rs","manifest.rs","download.rs","verify.rs","licensing.rs","tags.rs","locale.rs",
"registry/mod.rs","registry/local.rs","registry/remote.rs","registry/ipfs.rs","registry/http.rs",
"formats/mod.rs","formats/piper.rs","formats/elevenlabs.rs","formats/coqui.rs",
"key.rs","index.rs","fs_store.rs","eviction.rs","hashing.rs",
"store/mod.rs","store/disk.rs","store/memory.rs","store/object_store.rs",
"policy/mod.rs","policy/ttl.rs","policy/lru.rs","policy/pin.rs"
)

foreach ($f in $files) {
  $p = Join-Path $src (Join-Path "voicepacks" $f)
  if ($f -like "store/*" -or $f -like "policy/*" -or $f -like "key.rs" -or $f -like "index.rs" -or $f -like "fs_store.rs" -or $f -like "eviction.rs" -or $f -like "hashing.rs") {
    $p = Join-Path $src (Join-Path "cache" $f)
  }
  $dir = Split-Path $p -Parent
  New-Item -ItemType Directory -Path $dir -Force | Out-Null
  if (-not (Test-Path $p)) { "pub struct Placeholder;" | Set-Content -Path $p -Encoding UTF8 }
}