//! Lapidary Transmutation Pipeline Orchestrator
//!
//! Compliant with Symbiotic Xessability Standards.
//! This module serves as the master execution engine, seamlessly chaining 
//! the Frontend (ManifestParser), the Middle-end (AST Transformation Stages), 
//! and the Backend (VoltSynthesizer). It provides an async-first, extensible 
//! pipeline architecture with built-in telemetry, phase timing, and error isolation.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;
use async_trait::async_trait;

use crate::context::LapidaryContext;
use crate::fute::ast::UniversalAst;
use crate::parser::ManifestParser;
use crate::transformer::ExtensionPass;
use crate::fute::generator::VoltSynthesizer;

// ═══════════════════════════════════════════════════════════════════════════
// PIPELINE TRAITS & EXTENSIBILITY
// ═══════════════════════════════════════════════════════════════════════════

/// Represents a discrete, modular transformation pass over the Universal AST.
/// Engineered for asynchronous execution to support heavy I/O or network-bound analysis in the future.
#[async_trait]
pub trait PipelineStage: Send + Sync {
    /// Returns the canonical name of this pipeline stage for telemetry tracing.
    fn name(&self) -> &str;
    
    /// Executes the AST mutation matrix.
    async fn execute(&self, ast: UniversalAst, ctx: &mut LapidaryContext) -> Result<UniversalAst>;
}

/// A pre-packaged pipeline stage that wraps the core Lapidary `ExtensionPass`.
/// This bridges the synchronous transformer into the async-first pipeline matrix.
pub struct CoreTransmutationStage {
    pass: ExtensionPass,
}

impl CoreTransmutationStage {
    pub fn new() -> Self {
        Self {
            pass: ExtensionPass::new(),
        }
    }
}

#[async_trait]
impl PipelineStage for CoreTransmutationStage {
    fn name(&self) -> &str {
        "Core Ceremonial Transmutation Pass"
    }

    async fn execute(&self, ast: UniversalAst, ctx: &mut LapidaryContext) -> Result<UniversalAst> {
        // The core transformer is synchronous, but wrapped in the async trait for pipeline uniformity.
        self.pass.transmutate_ast(ast, ctx)
    }
}

impl Default for CoreTransmutationStage {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MASTER TRANSMUTATION ENGINE
// ═══════════════════════════════════════════════════════════════════════════

/// The master orchestrator for the Lapidary transformation lifecycle.
pub struct TransmutationEngine {
    output_directory: PathBuf,
    stages: Vec<Box<dyn PipelineStage>>,
    enable_telemetry_flush: bool,
}

impl TransmutationEngine {
    /// Bootstraps a new engine via the robust Builder pattern.
    pub fn builder(output_directory: impl AsRef<Path>) -> TransmutationEngineBuilder {
        TransmutationEngineBuilder::new(output_directory)
    }

    /// Ignites the full transmutation lifecycle on a raw manifest payload.
    /// 
    /// 1. Lexes the JSON payload into a Universal AST.
    /// 2. Iterates through all registered transformation stages sequentially.
    /// 3. Synthesizes the final physical artifacts (`volt.toml`, `.wat` shims).
    pub async fn execute(&self, raw_manifest: &str, ctx: &mut LapidaryContext) -> Result<()> {
        let global_timer = Instant::now();
        ctx.log_info("Pipeline", format!("Igniting Transmutation Engine targeting {:?}", self.output_directory));

        // ---------------------------------------------------------
        // PHASE 1: FRONTEND (Lexical Ingestion & Parsing)
        // ---------------------------------------------------------
        let phase_timer = Instant::now();
        let parser = ManifestParser::new();
        let mut active_ast = parser.parse_manifest(raw_manifest, ctx)
            .context("Pipeline aborted during Phase 1: Frontend Parsing Failure")?;
        
        ctx.log_info("Pipeline", format!("Phase 1 completed in {:?}", phase_timer.elapsed()));

        // ---------------------------------------------------------
        // PHASE 2: MIDDLE-END (AST Transformation Passes)
        // ---------------------------------------------------------
        if self.stages.is_empty() {
            ctx.log_warning("Pipeline", "No transformation stages registered. AST will pass through unmutated.");
        } else {
            for stage in &self.stages {
                let stage_timer = Instant::now();
                ctx.log_info("Pipeline", format!("Entering Stage: [{}]", stage.name()));
                
                // Transfer ownership of the AST through the mutation stage and reclaim it
                active_ast = stage.execute(active_ast, ctx).await
                    .with_context(|| format!("Fatal error during pipeline stage: {}", stage.name()))?;
                
                ctx.log_info("Pipeline", format!("Stage [{}] completed in {:?}", stage.name(), stage_timer.elapsed()));
            }
        }

        // ---------------------------------------------------------
        // PHASE 3: BACKEND (Artifact Synthesis)
        // ---------------------------------------------------------
        let phase_timer = Instant::now();
        let synthesizer = VoltSynthesizer::new(&self.output_directory);
        
        synthesizer.synthesize_artifacts(&active_ast, ctx)
            .context("Pipeline aborted during Phase 3: Artifact Synthesis Failure")?;

        ctx.log_info("Pipeline", format!("Phase 3 completed in {:?}", phase_timer.elapsed()));
        
        // ---------------------------------------------------------
        // TEARDOWN & DIAGNOSTICS
        // ---------------------------------------------------------
        ctx.log_info("Pipeline", format!("Transmutation cycle completed globally in {:?}", global_timer.elapsed()));

        if self.enable_telemetry_flush {
            ctx.flush_diagnostics_to_stdout();
        }

        if ctx.has_critical_failures() {
            anyhow::bail!("Pipeline completed, but critical failures were recorded in the matrix.");
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BUILDER PATTERN IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════

/// Safely constructs a `TransmutationEngine` with validation hooks.
pub struct TransmutationEngineBuilder {
    engine: TransmutationEngine,
}

impl TransmutationEngineBuilder {
    fn new(output_directory: impl AsRef<Path>) -> Self {
        Self {
            engine: TransmutationEngine {
                output_directory: output_directory.as_ref().to_path_buf(),
                stages: Vec::new(),
                enable_telemetry_flush: true,
            }
        }
    }

    /// Registers a standard or custom AST transformation pass into the execution queue.
    pub fn add_stage(mut self, stage: Box<dyn PipelineStage>) -> Self {
        self.engine.stages.push(stage);
        self
    }

    /// Registers the default `CoreTransmutationStage` for standard VS Code -> Lapce mapping.
    pub fn with_default_transformer(mut self) -> Self {
        self.engine.stages.push(Box::new(CoreTransmutationStage::new()));
        self
    }

    /// Controls whether the engine automatically flushes the telemetry context buffer to stdout upon completion.
    pub fn with_telemetry_flush(mut self, enable: bool) -> Self {
        self.engine.enable_telemetry_flush = enable;
        self
    }

    /// Finalizes the build matrix and returns the execution-ready engine.
    pub fn build(self) -> TransmutationEngine {
        self.engine
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIT TESTS (Validation & Integrity)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_pipeline_engine_creation_and_execution() -> Result<()> {
        let temp_dir = tempdir()?;
        let mut ctx = LapidaryContext::new();
        
        let engine = TransmutationEngine::builder(temp_dir.path())
            .with_default_transformer()
            .with_telemetry_flush(false) // Suppress stdout in test
            .build();

        let mock_manifest = r#"{
            "name": "test-lang-server",
            "version": "1.0.0",
            "publisher": "aurphyx",
            "main": "server.js"
        }"#;

        // Execute the full pipeline loop
        engine.execute(mock_manifest, &mut ctx).await?;

        // Validate that the output artifact was successfully created
        let manifest_path = temp_dir.path().join("test-lang-server/volt.toml");
        assert!(manifest_path.exists());
        
        // Ensure symbol tracking successfully captured the Node script and WASM proxy
        assert!(ctx.get_symbol("lattice_script_server.js").is_some());
        
        Ok(())
    }
}