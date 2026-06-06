//! Lapidary: Universal VSIX ➔ Volt Transmutation Engine
//!
//! The master CLI driver orchestrating the ingestion, transformation, 
//! and synthesis of closed-ecosystem VS Code extensions into highly 
//! optimized Lapce-compatible artifacts.
//!
//! Engineered to strictly comply with Symbiotic Xessability Standards.

pub mod context;
pub mod parser;
pub mod pipeline;
pub mod transformer;

// Integrated Fuxyez Core Ecosystem Matrix
pub mod fute {
    pub mod ast;
    pub mod generator;
}

use anyhow::{Context, Result, bail};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::context::{LapidaryContext, XessMode};
use crate::pipeline::TransmutationEngine;

#[tokio::main]
async fn main() -> Result<()> {
    print_cli_header();

    // 1. Resolve Target Paths & Arguments
    let (target_vsix, output_dir) = resolve_environment_paths()
        .context("Failed to resolve workspace environment paths.")?;

    // 2. Initialize Telemetry & State Context
    // Enforcing Sacred mode by default for maximum bloat stripping
    let mut ctx = LapidaryContext::new().with_mode(XessMode::Sacred);
    ctx.log_info("CLI", format!("Lapidary initialized. Target Architecture: {}", ctx.target_runtime));

    // 3. Extract the Raw Manifest Payload
    let raw_manifest = extract_manifest_from_vsix(&target_vsix, &mut ctx)?;

    // 4. Bootstrap the Asynchronous Transmutation Pipeline
    let engine = TransmutationEngine::builder(&output_dir)
        .with_default_transformer() // Registers the ExtensionPass automatically
        .with_telemetry_flush(true) // Ensure diagnostics hit stdout
        .build();

    // 5. Ignite the Master Orchestrator
    ctx.log_info("CLI", "Handing off execution to the Transmutation Engine pipeline...");
    engine.execute(&raw_manifest, &mut ctx).await?;

    print_cli_footer();
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// ENVIRONMENT RESOLUTION & I/O
// ═══════════════════════════════════════════════════════════════════════════

/// Dynamically resolves the target .vsix file and the output compilation directory.
fn resolve_environment_paths() -> Result<(PathBuf, PathBuf)> {
    let current_dir = env::current_dir().context("Failed to detect the current working directory.")?;
    let out_dir = current_dir.join("extz");
    
    let args: Vec<String> = env::args().collect();
    
    let target_vsix = if args.len() > 1 {
        // User provided an explicit path via CLI arguments
        PathBuf::from(&args[1])
    } else {
        // Fallback to the local testing default
        current_dir.join("sample.vsix")
    };

    Ok((target_vsix, out_dir))
}

/// Safely decompresses the `.vsix` archive and extracts the internal `package.json`.
fn extract_manifest_from_vsix(vsix_path: &Path, ctx: &mut LapidaryContext) -> Result<String> {
    if !vsix_path.exists() {
        ctx.log_critical("CLI", format!("Target archive not found at: {:?}", vsix_path));
        bail!(
            "Missing Target: Provide a valid path via `cargo run -- <path>` or ensure `sample.vsix` is in the root directory."
        );
    }

    ctx.log_info("Archive", format!("Unbinding internal zip matrix for {:?}", vsix_path));

    let file = File::open(vsix_path)
        .with_context(|| format!("Failed to open target package at {:?}", vsix_path))?;
        
    let mut archive = zip::ZipArchive::new(file)
        .context("Failed to read the ZIP archive topology.")?;

    let mut manifest_buffer = String::new();

    // Scan the internal zip topology for the target metadata matrix
    for i in 0..archive.len() {
        let mut internal_file = archive.by_index(i)?;
        
        // In standard VSIX topologies, the manifest lives at `extension/package.json`
        if internal_file.name() == "extension/package.json" {
            internal_file.read_to_string(&mut manifest_buffer)
                .context("Failed to stream the manifest slice into memory.")?;
            break;
        }
    }

    if manifest_buffer.is_empty() {
        ctx.log_critical("Archive", "Manifest (package.json) was not found in the extension region.");
        bail!("Invalid VSIX Topology: `extension/package.json` is missing or unreadable.");
    }

    ctx.log_info("Archive", format!("Successfully extracted manifest payload ({} bytes).", manifest_buffer.len()));
    Ok(manifest_buffer)
}

// ═══════════════════════════════════════════════════════════════════════════
// TERMINAL UI
// ═══════════════════════════════════════════════════════════════════════════

fn print_cli_header() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║             LAPIDARY TRANSMUTATION ENGINE (CLI)               ║");
    println!("║           Symbiotic Xessability Standards Compliant           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_cli_footer() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║             TRANSMUTATION CYCLE FINISHED SUCCESSFULLY         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}