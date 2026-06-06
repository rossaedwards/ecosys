#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - PART 2 (Streaming)
#>
param([string]$RepoRoot = (Get-Location).Path)
$ErrorActionPreference = "Stop"

$src = Join-Path $RepoRoot "afs\tts\src"
$stream = Join-Path $src "streaming"
New-Item -ItemType Directory -Path $stream -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stream "buffer") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stream "formats") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stream "playback") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stream "transport") -Force | Out-Null

@"
pub mod buffer;
pub mod formats;
pub mod playback;
pub mod transport;

pub mod chunk;
pub mod stream;
pub mod events;
pub mod backpressure;
pub mod jitter;
pub mod resampler;
pub mod timestamps;
pub mod stats;
"@ | Set-Content -Path (Join-Path $stream "mod.rs") -Encoding UTF8

# 35 module files (skeleton-stubs but real modules, no "sample" nonsense)
$files = @(
"chunk.rs","stream.rs","events.rs","backpressure.rs","jitter.rs","resampler.rs","timestamps.rs","stats.rs",
"buffer/mod.rs","buffer/ring.rs","buffer/queue.rs","buffer/window.rs","buffer/allocator.rs","buffer/pool.rs","buffer/limits.rs","buffer/spill.rs",
"formats/mod.rs","formats/wav.rs","formats/flac.rs","formats/opus.rs","formats/pcm_s16le.rs","formats/pcm_f32.rs",
"playback/mod.rs","playback/device.rs","playback/sink.rs","playback/latency.rs","playback/underrun.rs","playback/mixer.rs",
"transport/mod.rs","transport/http.rs","transport/websocket.rs","transport/ipc.rs","transport/stdio.rs"
)

foreach ($f in $files) {
  $p = Join-Path $stream $f
  $dir = Split-Path $p -Parent
  New-Item -ItemType Directory -Path $dir -Force | Out-Null
  if (-not (Test-Path $p)) { "pub struct Placeholder;" | Set-Content -Path $p -Encoding UTF8 }
}