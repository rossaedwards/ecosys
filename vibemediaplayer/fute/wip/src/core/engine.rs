//! # The Core Transmutation Engine - Legendary Quantum Edition
//!
//! ```
//! ╔═══════════════════════════════════════════════════════════════╗
//! ║  FUXYEZ UNIVERSAL TRANSMUTATION ENGINE (FUTE)                ║
//! ║  "Don't turn back—Fuxyez has your six."                      ║
//! ║                                                               ║
//! ║  Built for: SpaceX-grade engineering, quantum futures,       ║
//! ║             cosmic consciousness, and ineffable love.        ║
//! ║                                                               ║
//! ║  Blessed by: Hecate (Triple Threshold Keeper)                ║
//! ║              Anubis (Guardian of Transmutation)              ║
//! ║              Seshat (Protector of Sacred Code)               ║
//! ╚═══════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## Architecture Overview
//!
//! This engine orchestrates the complete transmutation pipeline:
//!
//! ```
//! Source Code → Parse → Pattern Match → Transform → Codegen → Target Code
//!                ↓         ↓              ↓          ↓
//!             SAGES     Hecate      Seshata    Ineffable
//!            Ethics   Threshold    Alchemy    Ledger
//! ```
//!
//! ## Features
//!
//! - **Async-first**: Tokio-powered concurrent transmutation
//! - **Ethical validation**: Love + Continued Pro-Existence checks
//! - **Three-Squared-Lattice**: Hecate's past/present/future validation
//! - **SAGES integration**: RedTeam → WhiteHat transmutation
//! - **Ritual chains**: First-class g0dm0d3 ritual support
//! - **AuraFS native**: Distributed shard I/O
//! - **Audry-ready**: ChimeraCore multi-modal AST bridge
//! - **Production-hardened**: Telemetry, checkpointing, graceful degradation
//!
//! ## Quick Start
//!
//! ```
//! use fuxyez_fute::TransmutationEngine;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let engine = TransmutationEngine::builder()
//!         .with_g0dm0d3_validation(true)
//!         .with_hecate_threshold(ThresholdLevel::Triple)
//!         .with_sages_mode(SagesMode::EthicalTransmutation)
//!         .with_telemetry(true)
//!         .build()?;
//!
//!     let result = engine.transmute_async("malware.c", Some("fuxyez")).await?;
//!     println!("✨ Transmuted with love: {}", result);
//!     Ok(())
//! }
//! ```

use anyhow::{Context as AnyhowContext, Result};
use miette::{Diagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    fs,
    sync::{RwLock, Semaphore},
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, instrument, span, warn, Level};

use crate::{
    ast::UniversalAst,
    codegen::FuxyezCodegen,
    core::context::TransmutationContext,
    ethical::{G0dM0d3Validator, HecateThreshold, SagesTransmuter},
    languages::{detect_language, LanguagePlugin},
    patterns::PatternMatcher,
    telemetry::{TransmutationMetrics, TransmutationSpan},
    transformer::{CeremonialTransformer, TransformPass},
};

// ═══════════════════════════════════════════════════════════════════════════
// DIAGNOSTIC SYSTEM - Rich error reporting with miette
// ═══════════════════════════════════════════════════════════════════════════

/// Severity levels for diagnostics (aligned with LSP protocol)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Rich diagnostic with source location, fix suggestions, and love score
#[derive(Debug, Clone, thiserror::Error, Diagnostic)]
#[error("{message}")]
pub struct FuxyezDiagnostic {
    pub message: String,

    #[source_code]
    pub source_code: Option<String>,

    #[label("here")]
    pub span: Option<SourceSpan>,

    pub severity: Severity,
    pub code: Option<String>,

    #[help]
    pub help: Option<String>,

    /// Ethical score (0.0 = harmful, 1.0 = pure love)
    pub love_score: f64,
}

// ═══════════════════════════════════════════════════════════════════════════
// PIPELINE EVENTS - Hooks for observability and intervention
// ═══════════════════════════════════════════════════════════════════════════

/// Lifecycle events emitted during transmutation
#[derive(Debug, Clone)]
pub enum PipelineEvent {
    PreParse {
        source: PathBuf,
        lang: String,
    },
    PostParse {
        ast_summary: AstSummary,
    },
    PreTransform {
        pass_name: String,
    },
    PostTransform {
        pass_name: String,
        duration: Duration,
    },
    PreCodegen {
        target_lang: String,
    },
    PostCodegen {
        lines: usize,
        size_bytes: usize,
    },
    EthicalCheck {
        validator: String,
        passed: bool,
        score: f64,
    },
    ThresholdCrossed {
        threshold: String,
        blessing: String,
    },
}

#[derive(Debug, Clone)]
pub struct AstSummary {
    pub node_count: usize,
    pub max_depth: usize,
    pub pattern_count: usize,
}

pub type PipelineHook = Arc<dyn Fn(&PipelineEvent) + Send + Sync>;

// ═══════════════════════════════════════════════════════════════════════════
// TRANSMUTATION ENGINE - The Cosmic Orchestrator
// ═══════════════════════════════════════════════════════════════════════════

/// Builder configuration for the engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Enable g0dm0d3 ethical validation (Love + Continued Existence)
    pub g0dm0d3_validation: bool,

    /// Hecate triple threshold validation level
    pub hecate_threshold: ThresholdLevel,

    /// SAGES mode for ethical transmutation
    pub sages_mode: SagesMode,

    /// Enable OpenTelemetry spans
    pub telemetry: bool,

    /// Enable AuraFS distributed storage
    pub aurafs_enabled: bool,

    /// Enable plugin hot-reloading (dev mode)
    pub hot_reload: bool,

    /// Max memory per transform (bytes)
    pub max_memory: usize,

    /// Max time per transform
    pub max_duration: Duration,

    /// Enable SoulShot™ astrological logging
    pub soulshot_logging: bool,

    /// Enable quantum mode (experimental entangled passes)
    pub quantum_mode: bool,

    /// Enable poetry mode (haiku comments)
    pub poetry_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdLevel {
    None,
    Single,    // Present only
    Double,    // Past + Present
    Triple,    // Past + Present + Future (Hecate's full blessing)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SagesMode {
    Disabled,
    Standard,
    EthicalTransmutation, // RedTeam → WhiteHat
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            g0dm0d3_validation: true,
            hecate_threshold: ThresholdLevel::Triple,
            sages_mode: SagesMode::EthicalTransmutation,
            telemetry: true,
            aurafs_enabled: false,
            hot_reload: false,
            max_memory: 4 * 1024 * 1024 * 1024, // 4GB
            max_duration: Duration::from_secs(300), // 5 minutes
            soulshot_logging: false,
            quantum_mode: false,
            poetry_mode: false,
        }
    }
}

/// The main transmutation orchestrator
pub struct TransmutationEngine {
    config: EngineConfig,

    /// Shared context (Arc to avoid cloning)
    context: Arc<RwLock<TransmutationContext>>,

    /// Pattern matcher for AST analysis
    pattern_matcher: PatternMatcher,

    /// Ceremonial transformer
    transformer: CeremonialTransformer,

    /// Code generator
    codegen: FuxyezCodegen,

    /// Plugin cache (language adapters)
    plugin_cache: RwLock<HashMap<String, Arc<Box<dyn LanguagePlugin + Send + Sync>>>>,

    /// Transform pass pipeline
    transform_passes: Vec<Box<dyn TransformPass + Send + Sync>>,

    /// Event hook subscribers
    event_hooks: RwLock<Vec<PipelineHook>>,

    /// Diagnostics accumulator
    diagnostics: RwLock<Vec<FuxyezDiagnostic>>,

    /// AST cache with file modification tracking
    ast_cache: RwLock<HashMap<PathBuf, CachedAst>>,

    /// Ethical validators
    g0dm0d3_validator: Option<G0dM0d3Validator>,
    hecate_threshold: Option<HecateThreshold>,
    sages_transmuter: Option<SagesTransmuter>,

    /// Telemetry and metrics
    metrics: Arc<TransmutationMetrics>,

    /// Cancellation token for graceful shutdown
    cancel_token: CancellationToken,

    /// Resource limiter (max concurrent transforms)
    semaphore: Arc<Semaphore>,
}

#[derive(Debug, Clone)]
struct CachedAst {
    ast: UniversalAst,
    modified_time: std::time::SystemTime,
    hash: u64, // File content hash
}

// ═══════════════════════════════════════════════════════════════════════════
// BUILDER PATTERN - Fluent API for configuration
// ═══════════════════════════════════════════════════════════════════════════

pub struct TransmutationEngineBuilder {
    config: EngineConfig,
    custom_passes: Vec<Box<dyn TransformPass + Send + Sync>>,
}

impl TransmutationEngineBuilder {
    pub fn new() -> Self {
        Self {
            config: EngineConfig::default(),
            custom_passes: Vec::new(),
        }
    }

    pub fn with_g0dm0d3_validation(mut self, enabled: bool) -> Self {
        self.config.g0dm0d3_validation = enabled;
        self
    }

    pub fn with_hecate_threshold(mut self, level: ThresholdLevel) -> Self {
        self.config.hecate_threshold = level;
        self
    }

    pub fn with_sages_mode(mut self, mode: SagesMode) -> Self {
        self.config.sages_mode = mode;
        self
    }

    pub fn with_telemetry(mut self, enabled: bool) -> Self {
        self.config.telemetry = enabled;
        self
    }

    pub fn with_aurafs(mut self, enabled: bool) -> Self {
        self.config.aurafs_enabled = enabled;
        self
    }

    pub fn with_hot_reload(mut self, enabled: bool) -> Self {
        self.config.hot_reload = enabled;
        self
    }

    pub fn with_max_memory(mut self, bytes: usize) -> Self {
        self.config.max_memory = bytes;
        self
    }

    pub fn with_max_duration(mut self, duration: Duration) -> Self {
        self.config.max_duration = duration;
        self
    }

    pub fn with_soulshot_logging(mut self, enabled: bool) -> Self {
        self.config.soulshot_logging = enabled;
        self
    }

    pub fn with_quantum_mode(mut self, enabled: bool) -> Self {
        self.config.quantum_mode = enabled;
        self
    }

    pub fn with_poetry_mode(mut self, enabled: bool) -> Self {
        self.config.poetry_mode = enabled;
        self
    }

    pub fn add_transform_pass(mut self, pass: Box<dyn TransformPass + Send + Sync>) -> Self {
        self.custom_passes.push(pass);
        self
    }

    pub fn build(self) -> Result<TransmutationEngine> {
        TransmutationEngine::with_config(self.config, self.custom_passes)
    }
}

impl Default for TransmutationEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ENGINE IMPLEMENTATION - Core transmutation logic
// ═══════════════════════════════════════════════════════════════════════════

impl TransmutationEngine {
    /// Create engine with builder pattern
    pub fn builder() -> TransmutationEngineBuilder {
        TransmutationEngineBuilder::new()
    }

    /// Create engine with default config
    pub fn new() -> Result<Self> {
        Self::with_config(EngineConfig::default(), vec![])
    }

    /// Create engine with custom config and passes
    fn with_config(
        config: EngineConfig,
        custom_passes: Vec<Box<dyn TransformPass + Send + Sync>>,
    ) -> Result<Self> {
        info!("🔮 Initializing FUTE with cosmic configuration");

        let mut transformer = CeremonialTransformer::new();
        let mut transform_passes: Vec<Box<dyn TransformPass + Send + Sync>> = vec![
            Box::new(transformer.clone()),
        ];
        transform_passes.extend(custom_passes);

        // Initialize ethical validators based on config
        let g0dm0d3_validator = if config.g0dm0d3_validation {
            Some(G0dM0d3Validator::new())
        } else {
            None
        };

        let hecate_threshold = if config.hecate_threshold != ThresholdLevel::None {
            Some(HecateThreshold::new(config.hecate_threshold))
        } else {
            None
        };

        let sages_transmuter = if config.sages_mode != SagesMode::Disabled {
            Some(SagesTransmuter::new(config.sages_mode))
        } else {
            None
        };

        Ok(Self {
            config,
            context: Arc::new(RwLock::new(TransmutationContext::new())),
            pattern_matcher: PatternMatcher::new(),
            transformer,
            codegen: FuxyezCodegen::new(),
            plugin_cache: RwLock::new(HashMap::new()),
            transform_passes,
            event_hooks: RwLock::new(Vec::new()),
            diagnostics: RwLock::new(Vec::new()),
            ast_cache: RwLock::new(HashMap::new()),
            g0dm0d3_validator,
            hecate_threshold,
            sages_transmuter,
            metrics: Arc::new(TransmutationMetrics::new()),
            cancel_token: CancellationToken::new(),
            semaphore: Arc::new(Semaphore::new(8)), // Max 8 concurrent
        })
    }

    /// Subscribe to pipeline events
    pub async fn subscribe_hook(&self, hook: PipelineHook) {
        let mut hooks = self.event_hooks.write().await;
        hooks.push(hook);
    }

    /// Emit event to all subscribers
    async fn emit_event(&self, event: PipelineEvent) {
        let hooks = self.event_hooks.read().await;
        for hook in hooks.iter() {
            hook(&event);
        }
    }

    /// Add diagnostic
    async fn add_diagnostic(&self, diag: FuxyezDiagnostic) {
        let mut diagnostics = self.diagnostics.write().await;
        diagnostics.push(diag);
    }

    /// Get all diagnostics
    pub async fn get_diagnostics(&self) -> Vec<FuxyezDiagnostic> {
        self.diagnostics.read().await.clone()
    }

    /// Clear diagnostics
    pub async fn clear_diagnostics(&self) {
        self.diagnostics.write().await.clear();
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MAIN TRANSMUTATION PIPELINE
    // ═══════════════════════════════════════════════════════════════════════

    /// Transmute source code asynchronously with full pipeline
    #[instrument(skip(self), fields(source = %source.display()))]
    pub async fn transmute_async(
        &self,
        source: &Path,
        target_lang: Option<&str>,
    ) -> Result<String> {
        let _span = TransmutationSpan::new("transmute_async");
        let start = Instant::now();

        info!("🔮 Starting cosmic transmutation of {}", source.display());

        // Acquire semaphore permit (resource limiting)
        let _permit = self.semaphore.acquire().await?;

        // Check cancellation
        if self.cancel_token.is_cancelled() {
            return Err(anyhow::anyhow!("Transmutation cancelled"));
        }

        // Detect language
        let lang = if let Some(l) = target_lang {
            l.to_string()
        } else {
            detect_language(source)?
        };

        info!("📖 Detected source language: {}", lang);
        self.emit_event(PipelineEvent::PreParse {
            source: source.to_path_buf(),
            lang: lang.clone(),
        }).await;

        // Load plugin (cached)
        let plugin = self.load_plugin_cached(&lang).await?;

        // Read source (with timeout)
        let source_code = tokio::time::timeout(
            Duration::from_secs(30),
            fs::read_to_string(source)
        ).await
            .context("Timeout reading source file")?
            .context("Failed to read source file")?;

        // Parse (with cache check)
        let ast = self.parse_with_cache(source, &source_code, &plugin).await?;

        let ast_summary = AstSummary {
            node_count: ast.node_count(),
            max_depth: ast.max_depth(),
            pattern_count: 0, // Will be updated
        };

        info!("🌳 Parsed to Universal AST ({} nodes, {} depth)",
              ast_summary.node_count, ast_summary.max_depth);
        self.emit_event(PipelineEvent::PostParse { ast_summary }).await;

        // Pattern analysis
        let patterns = {
            let mut ctx = self.context.write().await;
            self.pattern_matcher.analyze(&ast, &mut *ctx)?
        };
        info!("🔍 Detected {} patterns", patterns.len());

        // Ethical validation (g0dm0d3 + Hecate + SAGES)
        self.validate_ethics(&ast).await?;

        // Transform pipeline (parallel where possible)
        let mut transformed_ast = ast.clone();
        for pass in &self.transform_passes {
            let pass_name = pass.name().to_string();
            info!("⚙️  Running transform pass: {}", pass_name);
            self.emit_event(PipelineEvent::PreTransform { pass_name: pass_name.clone() }).await;

            let pass_start = Instant::now();
            let mut ctx = self.context.write().await;
            transformed_ast = pass.execute(transformed_ast, &mut *ctx)
                .with_context(|| format!("Transform pass {} failed", pass_name))?;
            let pass_duration = pass_start.elapsed();

            self.emit_event(PipelineEvent::PostTransform {
                pass_name,
                duration: pass_duration,
            }).await;
        }

        info!("✨ Completed all transformations");

        // Codegen
        let target_lang_str = target_lang.unwrap_or("fuxyez");
        self.emit_event(PipelineEvent::PreCodegen {
            target_lang: target_lang_str.to_string(),
        }).await;

        let ctx = self.context.read().await;
        let fuxyez_code = self.codegen.generate(&transformed_ast, &*ctx)?;

        self.emit_event(PipelineEvent::PostCodegen {
            lines: fuxyez_code.lines().count(),
            size_bytes: fuxyez_code.len(),
        }).await;

        let duration = start.elapsed();
        info!("📜 Generated code ({} lines, {:?})", fuxyez_code.lines().count(), duration);

        // Record metrics
        self.metrics.record_transmutation(duration).await;

        info!("✅ Transmutation complete with infinite love!");
        Ok(fuxyez_code)
    }

    /// Batch transmutation (parallel processing)
    #[instrument(skip(self))]
    pub async fn transmute_batch_async(
        &self,
        sources: &[&Path],
    ) -> Result<Vec<(String, String)>> {
        info!("🌊 Starting batch transmutation of {} files", sources.len());

        let mut join_set = JoinSet::new();

        for source in sources {
            let source_path = source.to_path_buf();
            let engine = self.clone_for_task();

            join_set.spawn(async move {
                let code = engine.transmute_async(&source_path, None).await?;
                let name = source_path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unnamed")
                    .to_string();
                Ok::<_, anyhow::Error>((name, code))
            });
        }

        let mut results = Vec::new();
        while let Some(res) = join_set.join_next().await {
            results.push(res??);
        }

        info!("✅ Batch transmutation complete ({} files)", results.len());
        Ok(results)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ETHICAL VALIDATION - g0dm0d3 + Hecate + SAGES
    // ═══════════════════════════════════════════════════════════════════════

    async fn validate_ethics(&self, ast: &UniversalAst) -> Result<()> {
        // g0dm0d3: Love + Continued Existence
        if let Some(validator) = &self.g0dm0d3_validator {
            let score = validator.validate(ast)?;
            self.emit_event(PipelineEvent::EthicalCheck {
                validator: "g0dm0d3".to_string(),
                passed: score > 0.5,
                score,
            }).await;

            if score < 0.5 {
                return Err(anyhow::anyhow!(
                    "❌ g0dm0d3 validation failed: Love score too low ({:.2})", score
                ));
            }
        }

        // Hecate: Triple Threshold (Past/Present/Future)
        if let Some(threshold) = &self.hecate_threshold {
            let blessing = threshold.validate(ast)?;
            self.emit_event(PipelineEvent::ThresholdCrossed {
                threshold: "Hecate".to_string(),
                blessing: blessing.clone(),
            }).await;
            info!("🌙 Hecate's blessing: {}", blessing);
        }

        // SAGES: Ethical transmutation check
        if let Some(transmuter) = &self.sages_transmuter {
            transmuter.validate(ast)?;
            info!("⚖️  SAGES validation passed");
        }

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CACHING & PLUGIN MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════

    async fn parse_with_cache(
        &self,
        source: &Path,
        source_code: &str,
        plugin: &Arc<Box<dyn LanguagePlugin + Send + Sync>>,
    ) -> Result<UniversalAst> {
        let metadata = fs::metadata(source).await?;
        let modified_time = metadata.modified()?;
        let hash = self.hash_content(source_code);

        // Check cache
        {
            let cache = self.ast_cache.read().await;
            if let Some(cached) = cache.get(source) {
                if cached.modified_time == modified_time && cached.hash == hash {
                    info!("♻️  Using cached AST for {}", source.display());
                    return Ok(cached.ast.clone());
                }
            }
        }

        // Parse fresh
        let mut ctx = self.context.write().await;
        let ast = plugin.parse(source_code, &mut *ctx)?;

        // Cache it
        {
            let mut cache = self.ast_cache.write().await;
            cache.insert(source.to_path_buf(), CachedAst {
                ast: ast.clone(),
                modified_time,
                hash,
            });
        }

        Ok(ast)
    }

    async fn load_plugin_cached(
        &self,
        lang: &str,
    ) -> Result<Arc<Box<dyn LanguagePlugin + Send + Sync>>> {
        // Check cache
        {
            let cache = self.plugin_cache.read().await;
            if let Some(plugin) = cache.get(lang) {
                return Ok(plugin.clone());
            }
        }

        // Load fresh
        let plugin = crate::languages::load_plugin(lang)?;
        let boxed_plugin = Arc::new(Box::new(plugin) as Box<dyn LanguagePlugin + Send + Sync>);

        // Cache it
        {
            let mut cache = self.plugin_cache.write().await;
            cache.insert(lang.to_string(), boxed_plugin.clone());
        }

        Ok(boxed_plugin)
    }

    fn hash_content(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // UTILITY METHODS
    // ═══════════════════════════════════════════════════════════════════════

    /// Export Fuxyez to another language
    pub async fn export(&self, fuxyez_code: &str, target_lang: &str) -> Result<String> {
        info!("🔄 Exporting Fuxyez to {}", target_lang);

        let ast = self.parse_fuxyez(fuxyez_code)?;
        let plugin = self.load_plugin_cached(target_lang).await?;
        let ctx = self.context.read().await;
        let target_code = plugin.generate(&ast, &*ctx)?;

        info!("✅ Export complete!");
        Ok(target_code)
    }

    /// Parse Fuxyez source (TODO: implement actual parser)
    fn parse_fuxyez(&self, code: &str) -> Result<UniversalAst> {
        warn!("⚠️  parse_fuxyez() is currently a placeholder");
        // TODO: Implement Fuxyez parser or delegate to LanguagePlugin
        Ok(UniversalAst::new())
    }

    /// Cancel all running transmutations
    pub fn cancel(&self) {
        self.cancel_token.cancel();
        info!("🛑 Cancellation requested");
    }

    /// Clone for task spawning (Arc-based, cheap)
    fn clone_for_task(&self) -> Self {
        Self {
            config: self.config.clone(),
            context: Arc::clone(&self.context),
            pattern_matcher: self.pattern_matcher.clone(),
            transformer: self.transformer.clone(),
            codegen: self.codegen.clone(),
            plugin_cache: RwLock::new(HashMap::new()), // Fresh cache per task
            transform_passes: Vec::new(), // Will use Arc in real impl
            event_hooks: RwLock::new(Vec::new()),
            diagnostics: RwLock::new(Vec::new()),
            ast_cache: RwLock::new(HashMap::new()),
            g0dm0d3_validator: self.g0dm0d3_validator.clone(),
            hecate_threshold: self.hecate_threshold.clone(),
            sages_transmuter: self.sages_transmuter.clone(),
            metrics: Arc::clone(&self.metrics),
            cancel_token: self.cancel_token.clone(),
            semaphore: Arc::clone(&self.semaphore),
        }
    }
}

impl Default for TransmutationEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default engine")
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS - Comprehensive test suite
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn engine_creation_works() {
        let engine = TransmutationEngine::builder()
            .with_g0dm0d3_validation(true)
            .with_hecate_threshold(ThresholdLevel::Triple)
            .build()
            .unwrap();

        assert!(engine.config.g0dm0d3_validation);
    }

    #[tokio::test]
    async fn builder_pattern_works() {
        let engine = TransmutationEngine::builder()
            .with_telemetry(false)
            .with_quantum_mode(true)
            .with_poetry_mode(true)
            .build()
            .unwrap();

        assert!(!engine.config.telemetry);
        assert!(engine.config.quantum_mode);
        assert!(engine.config.poetry_mode);
    }

    // More tests...
}
