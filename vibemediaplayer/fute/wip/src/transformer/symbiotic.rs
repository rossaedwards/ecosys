//! Mythical Symbiotic Ceremonial Transformer for Fuxyez
//!
//! Converts Universal AST into quantum/lattice/ritual-rich Fuxyez ceremonial patterns.
//! Async-ready, rule/combinator-driven, region- and pattern-pack aware, with full traceability.

use anyhow::Result;
use async_trait::async_trait;
use crate::{
    ast::*,
    patterns::{
        DetectedPattern, TransformationHint, PatternDetector, PatternLibrary, PatternMatch,
        SemanticRegion, PatternDomain, PatternMatcherEngine, rules::PatternRuleEngine,
    },
    core::context::{TransmutationContext, CeremonialMode},
};

/// Advanced ceremonial transformer with plugin rules, context-awareness, tracing, and async support.
pub struct CeremonialTransformer {
    mode: CeremonialMode,
    pattern_library: PatternLibrary,
    rule_engine: Option<PatternRuleEngine>,
}

impl CeremonialTransformer {
    pub fn new() -> Self {
        Self {
            mode: CeremonialMode::Standard,
            pattern_library: PatternLibrary::new(),
            rule_engine: None,
        }
    }

    /// Attach pattern packs, pattern rules, and transformer configuration for mega extension support.
    pub fn with_patterns_and_rules(
        mut self,
        library: PatternLibrary,
        rule_engine: PatternRuleEngine,
        mode: CeremonialMode,
    ) -> Self {
        self.pattern_library = library;
        self.rule_engine = Some(rule_engine);
        self.mode = mode;
        self
    }

    /// Transform Universal AST to ceremonial representation leveraging patterns/rules/regions.
    pub async fn transform(
        &self,
        ast: UniversalAst,
        context: &mut TransmutationContext,
    ) -> Result<UniversalAst> {
        log::info!("✨ [Mythic] Ceremonial transformation begins ({:?} mode)...", self.mode);
        context.ceremonial_mode = self.mode;
        let pattern_matcher = PatternMatcherEngine::new(&self.pattern_library);
        let patterns = pattern_matcher.match_ast(&ast);

        // Optionally evaluate custom rules and trigger chained transformations
        if let Some(engine) = &self.rule_engine {
            let mut rule_ctx = crate::patterns::rules::RuleContext {
                library: &self.pattern_library,
                engine_metadata: context.metadata.clone(),
                audit_log: engine.telemetry.clone(),
                context_metadata: context.metadata.clone(),
            };
            let _ = engine.evaluate_bulk_async(&ast, &mut rule_ctx).await;
        }

        // Scan for semantic regions (quantum/lattice/hybrid)
        let detector = PatternDetector::new(&self.pattern_library);
        let quantum_regions = detector.detect_quantum_regions(&ast, context);
        let lattice_regions = detector.detect_lattice_regions(&ast, context);

        // Main traversal/transformation
        let transformed_root = self
            .transform_node(ast.root, &patterns, &quantum_regions, &lattice_regions, context)
            .await?;

        Ok(UniversalAst {
            root: transformed_root,
            metadata: ast.metadata,
        })
    }

    // -- Recursive async pattern-driven transform, with region and rule hooks --
    async fn transform_node(
        &self,
        node: AstNode,
        patterns: &[PatternMatch],
        quantum_regions: &[SemanticRegion],
        lattice_regions: &[SemanticRegion],
        context: &mut TransmutationContext,
    ) -> Result<AstNode> {
        match node {
            AstNode::Module { name, items } => {
                let mut transformed_items = Vec::with_capacity(items.len());
                for item in items {
                    transformed_items.push(self.transform_node(
                        item, patterns, quantum_regions, lattice_regions, context
                    ).await?);
                }
                Ok(AstNode::Module { name, items: transformed_items })
            }
            AstNode::Function { name, params, return_type, body, is_async, visibility, symbol_id } => {
                let transformation = self.determine_function_transformation(
                    &name, is_async, patterns
                );
                match transformation {
                    FunctionTransformation::Ritual => {
                        log::debug!("Mythic: Function '{}' → Ritual", name);
                        self.transform_to_ritual(
                            name, params, return_type, body, is_async, visibility, symbol_id, context
                        ).await
                    }
                    FunctionTransformation::Sigil => {
                        log::debug!("Mythic: Function '{}' → Sigil", name);
                        self.transform_to_sigil(
                            name, params, return_type, body, visibility, symbol_id
                        ).await
                    }
                    FunctionTransformation::Preserve => {
                        let mut transformed_body = Vec::with_capacity(body.len());
                        for stmt in body {
                            transformed_body.push(self.transform_node(
                                stmt, patterns, quantum_regions, lattice_regions, context
                            ).await?);
                        }
                        Ok(AstNode::Function {
                            name, params, return_type, body: transformed_body,
                            is_async, visibility, symbol_id,
                        })
                    }
                }
            }
            AstNode::Struct { name, fields, visibility, symbol_id } => {
                if Self::is_lattice_struct(&name, lattice_regions) {
                    log::debug!("Mythic: Struct '{}' → Lattice", name);
                    self.transform_to_lattice(name, fields, visibility, symbol_id).await
                } else {
                    Ok(AstNode::Struct { name, fields, visibility, symbol_id })
                }
            }
            AstNode::Class { name, fields, methods, base_class, visibility, symbol_id } => {
                let mut transformed_methods = Vec::with_capacity(methods.len());
                for method in methods {
                    transformed_methods.push(self.transform_node(
                        method, patterns, quantum_regions, lattice_regions, context
                    ).await?);
                }
                Ok(AstNode::Class {
                    name, fields, methods: transformed_methods, base_class, visibility, symbol_id,
                })
            }
            // Add region-driven or other mythic pattern-aware cases as desired
            _ => Ok(node),
        }
    }

    fn determine_function_transformation(
        &self,
        name: &str,
        is_async: bool,
        patterns: &[PatternMatch],
    ) -> FunctionTransformation {
        for pm in patterns {
            let hint = &pm.pattern.hint;
            match hint {
                TransformationHint::Ritual { .. } if pm.pattern.confidence() > 0.8 => {
                    return FunctionTransformation::Ritual
                }
                TransformationHint::Sigil { .. } => {
                    return FunctionTransformation::Sigil
                }
                _ => {}
            }
        }
        // Extra context/heuristics if patterns aren't conclusive:
        if is_async { return FunctionTransformation::Ritual }
        if name.len() < 8 { return FunctionTransformation::Sigil }
        FunctionTransformation::Preserve
    }

    async fn transform_to_ritual(
        &self,
        name: String, params: Vec<Parameter>,
        return_type: Option<Type>, body: Vec<AstNode>,
        is_async: bool, visibility: Visibility, symbol_id: Option<usize>,
        _context: &mut TransmutationContext,
    ) -> Result<AstNode> {
        Ok(AstNode::Function {
            name: format!("ritual_{}", name),
            params, return_type, body,
            is_async, visibility, symbol_id
        })
    }

    async fn transform_to_sigil(
        &self,
        name: String, params: Vec<Parameter>,
        return_type: Option<Type>, body: Vec<AstNode>,
        visibility: Visibility, symbol_id: Option<usize>
    ) -> Result<AstNode> {
        Ok(AstNode::Function {
            name: format!("sigil_{}", name),
            params, return_type, body,
            is_async: false, visibility, symbol_id
        })
    }

    async fn transform_to_lattice(
        &self,
        name: String, fields: Vec<Field>,
        visibility: Visibility, symbol_id: Option<usize>
    ) -> Result<AstNode> {
        Ok(AstNode::Struct {
            name: format!("Lattice{}", name),
            fields, visibility, symbol_id
        })
    }

    fn is_lattice_struct(name: &str, lattice_regions: &[SemanticRegion]) -> bool {
        let lower = name.to_lowercase();
        lower.contains("lattice") || lattice_regions.iter().any(|r| r.start_node_name() == name)
    }
}

impl Default for CeremonialTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
enum FunctionTransformation {
    Ritual,
    Sigil,
    Preserve,
}

// --- Helper trait implementations for region detection / start_node extraction ---
trait SemanticRegionExt {
    fn start_node_name(&self) -> &str;
}
impl SemanticRegionExt for SemanticRegion<'_> {
    fn start_node_name(&self) -> &str {
        match self.start_node {
            AstNode::Struct { name, .. } => name,
            AstNode::Function { name, .. } => name,
            _ => "",
        }
    }
}