#!/usr/bin/env pwsh
<#
.SYNOPSIS
AURAFS TTS - MASTER FORGE (Part 15)
.DESCRIPTION
Runs all Part 15 TTS scaffold scripts in order and ensures workspace membership is updated.
Works no matter where it is executed from.
#>

param(
  [string]$RepoRoot = ""
)

$ErrorActionPreference = "Stop"

function Find-WorkspaceRoot {
  $cur = $PWD.Path
  if ([string]::IsNullOrWhiteSpace($cur)) { throw "PWD.Path is empty; cannot locate workspace root." }

  while ($true) {
    $candidate = Join-Path $cur "Cargo.toml"
    if (Test-Path $candidate) { return $cur }

    $parent = Split-Path -Path $cur -Parent
    if ([string]::IsNullOrWhiteSpace($parent) -or ($parent -eq $cur)) {
      throw "Could not find workspace Cargo.toml above current directory."
    }
    $cur = $parent
  }
}

function Assert-Path([string]$p) {
  if ([string]::IsNullOrWhiteSpace($p)) { throw "Assert-Path got empty path." }
  if (-not (Test-Path $p)) { throw "Missing path: $p" }
}

function Ensure-WorkspaceMember([string]$cargoTomlPath, [string]$memberPath) {
  $raw = Get-Content $cargoTomlPath -Raw
  if ($raw -match [regex]::Escape("""$memberPath""")) {
    Write-Host "Workspace already includes $memberPath" -ForegroundColor DarkGreen
    return
  }

  $lines = Get-Content $cargoTomlPath
  $out = New-Object System.Collections.Generic.List[string]
  $inMembers = $false
  $inserted = $false

  foreach ($line in $lines) {
    if ($line -match "members\s*=\s*\[") { $inMembers = $true }

    if ($inMembers -and -not $inserted -and ($line -match "^\s*\]")) {
      $out.Add("    ""$memberPath"",")
      $inserted = $true
    }

    $out.Add($line)

    if ($inMembers -and ($line -match "^\s*\]")) { $inMembers = $false }
  }

  if (-not $inserted) { throw "Could not insert workspace member automatically. Add ""$memberPath"" manually." }

  Set-Content -Path $cargoTomlPath -Value ($out -join "`n") -Encoding UTF8
  Write-Host "Added workspace member: $memberPath" -ForegroundColor Green
}

function Resolve-PartScript([string]$repoRoot, [string]$fileName) {
  $a = Join-Path $repoRoot $fileName
  $b = Join-Path $repoRoot ("afs\tts\" + $fileName)

  if (Test-Path $a) { return $a }
  if (Test-Path $b) { return $b }

  throw "Missing part script in both locations: $a OR $b"
}

if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
  $RepoRoot = Find-WorkspaceRoot
}

$rootCargo = Join-Path $RepoRoot "Cargo.toml"
Assert-Path $rootCargo

# Ensure target crate directory exists
$ttsCrate = Join-Path $RepoRoot "afs\tts"
New-Item -ItemType Directory -Path $ttsCrate -Force | Out-Null

Ensure-WorkspaceMember -cargoTomlPath $rootCargo -memberPath "afs/tts"

$parts = @(
  "afs_tts_part1_core.ps1",
  "afs_tts_part2_streaming.ps1",
  "afs_tts_part3_backends.ps1",
  "afs_tts_part4_voicepacks_cache.ps1",
  "afs_tts_part5_api_cli.ps1",
  "afs_tts_part6_integrations_tests.ps1"
)

foreach ($p in $parts) {
  $scriptPath = Resolve-PartScript -repoRoot $RepoRoot -fileName $p
  Write-Host "`nFORGING: $p" -ForegroundColor Cyan
  & $scriptPath -RepoRoot $RepoRoot
}

Write-Host "`nAURAFS TTS PART 15 FORGE COMPLETE." -ForegroundColor Magenta
Write-Host "Next: cargo check -p afs-tts" -ForegroundColor Yellow