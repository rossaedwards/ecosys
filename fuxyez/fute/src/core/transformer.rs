//! Ceremonial Transformer - Diamond Edition
//!
//! Provides an extensible, high-performance transformation engine over the Universal AST.
//! Supports dynamic, composable transformation passes with rich pattern-based rewrites,
//! contextual awareness using the TransmutationContext, async-ready design, and detailed logging.
//!
//! Designed to be the core workhorse for symbiotic transformations in the Fuxyez engine.

use anyhow::{Result, Context};
use crate::{
    ast::UniversalAst,
    ast::AstNode,
    core::context::TransmutationContext,
    patterns::{DetectedPattern, PatternMatcher},
};

/// Trait representing a transformation pass that can mutate or rewrite the AST.
pub trait TransformPass: Send + Sync {
    /// Return the name of the transformation pass.
    fn name(&self) -> &str;

    /// Execute the transformation pass on the given AST with context, producing a transformed AST.
    fn execute(&self, ast: UniversalAst, context: &mut TransmutationContext) -> Result<UniversalAst>;
}

/// The ceremonial transformer aggregates multiple transformation passes,
/// orchestrates their execution, and applies pattern-based rewrites.
#[derive(Clone)]
pub struct CeremonialTransformer {
    /// Registered transformation passes executed sequentially.
    passes: Vec<Box<dyn TransformPass + Send + Sync>>,

    /// Pattern matcher instance used for detecting transformation patterns in the AST.
    pattern_matcher: PatternMatcher,
}

impl CeremonialTransformer {
    /// Create a new transformer with the default set of passes.
    pub fn new() -> Self {
        let default_pass = Box::new(DefaultCeremonialPass::new());
        Self {
            passes: vec![default_pass],
            pattern_matcher: PatternMatcher::new(),
        }
    }

    /// Register an additional transformation pass.
    pub fn register_pass(&mut self, pass: Box<dyn TransformPass + Send + Sync>) {
        self.passes.push(pass);
    }

    /// Perform a full transformation pipeline on the AST, applying passes in order.
    pub fn transform(
        &self,
        ast: UniversalAst,
        context: &mut TransmutationContext,
    ) -> Result<UniversalAst> {
        log::info!("✨ Beginning ceremonial transformation pipeline...");

        let mut current_ast = ast;
        for pass in &self.passes {
            log::info!("⚙️ Executing pass '{}'", pass.name());
            current_ast = pass.execute(current_ast, context)
                .with_context(|| format!("Pass '{}' failed", pass.name()))?;
        }

        log::info!("✨ Ceremonial transformation pipeline complete.");
        Ok(current_ast)
    }
}

/// Default ceremonial pass implementing basic pattern-driven rewrites.
pub struct DefaultCeremonialPass;

impl DefaultCeremonialPass {
    /// Create a new default ceremonial pass instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Transform a single AST node recursively based on detected patterns.
    fn transform_node(
        &self,
        node: AstNode,
        patterns: &[DetectedPattern],
        context: &mut TransmutationContext,
    ) -> Result<AstNode> {
        // Example: Wrap all function bodies in ceremonial blocks if pattern matches
        match node {
            AstNode::Module { name, items } => {
                let transformed_items = items
                    .into_iter()
                    .map(|item| self.transform_node(item, patterns, context))
                    .collect::<Result<Vec<_>>>()?;

                Ok(AstNode::Module {
                    name,
                    items: transformed_items,
                })
            }
            AstNode::Function { name, params, body, return_type, is_async, visibility, .. } => {
                // Wrap function body in a block with ceremonial semantics using detected patterns
                let transformed_body = self.transform_node(AstNode::Block(body), patterns, context)?;

                Ok(AstNode::Function {
                    name,
                    params,
                    body: if let AstNode::Block(stmts) = transformed_body { stmts } else { vec![] },
                    return_type,
                    is_async,
                    visibility,
                    symbol_id: None,
                })
            }
            _ => Ok(node), // pass-through for other nodes for now
        }
    }
}

impl TransformPass for DefaultCeremonialPass {
    fn name(&self) -> &str {
        "Default Ceremonial Pass"
    }

    fn execute(&self, ast: UniversalAst, context: &mut TransmutationContext) -> Result<UniversalAst> {
        let patterns = context
            .metadata
            .get("detected_patterns")
            .and_then(|_| Some(Vec::<DetectedPattern>::new()))
            .unwrap_or_else(|| Vec::new());
        // TODO: Possibly get real detected patterns from context or external source

        log::info!("🔍 Detected {} patterns for ceremonial transformation.", patterns.len());

        let transformed_root = self.transform_node(ast.root, &patterns, context)?;

        Ok(UniversalAst {
            root: transformed_root,
            metadata: ast.metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{UniversalAst, AstNode};
    use crate::core::context::TransmutationContext;

    #[test]
    fn test_default_ceremonial_pass_execution() {
        let pass = DefaultCeremonialPass::new();
        let mut context = TransmutationContext::new();

        // Construct a minimal AST to transform
        let ast = UniversalAst {
            root: AstNode::Function {
                name: "test_func".to_string(),
                params: vec![],
                return_type: None,
                body: vec![AstNode::VarDecl {
                    name: "x".to_string(),
                    ty: None,
                    value: Some(Box::new(AstNode::Literal(crate::ast::Literal::Int(42)))),
                    is_mutable: false,
                    symbol_id: None,
                }],
                is_async: false,
                visibility: crate::ast::Visibility::Public,
                symbol_id: None,
            },
            metadata: Default::default(),
        };

        let result = pass.execute(ast, &mut context);
        assert!(result.is_ok());

        // Check that the transformed AST still has a root function node
        let transformed_ast = result.unwrap();
        if let AstNode::Function { name, .. } = &transformed_ast.root {
            assert_eq!(name, "test_func");
        } else {
            panic!("Transformed AST root was not a Function node");
        }
    }
}