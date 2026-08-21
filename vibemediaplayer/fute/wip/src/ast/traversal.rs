//! AST Traversal Utilities for Fuxyez Fute
//!
//! Provides robust, extensible traversal and visitor patterns for Universal AST nodes.
//! Enables easy querying, mutation, and data extraction with deep support for both immutable and mutable traversals.
//! Supports visitor chaining, early termination, and selective node filtering as diamond-grade essential toolkit.

use crate::ast::universal::AstNode;
use anyhow::Result;

/// Immutable AST visitor trait supporting recursive traversal with optional early termination.
pub trait AstVisitor {
    /// Called on each node during traversal.
    /// Return false to stop descending into this node’s children.
    fn visit(&mut self, node: &AstNode) -> Result<bool>;
}

/// Mutable AST visitor trait allowing in-place mutations during traversal.
pub trait AstVisitorMut {
    /// Called on each node during traversal.
    /// Return false to skip visiting children of this node.
    fn visit_mut(&mut self, node: &mut AstNode) -> Result<bool>;
}

/// Recursive AST traversal with immutable visitor.
pub fn traverse_ast<V: AstVisitor>(node: &AstNode, visitor: &mut V) -> Result<()> {
    if !visitor.visit(node)? {
        return Ok(()); // Skip children if visit returns false
    }

    match node {
        AstNode::Module { items, .. } => {
            for item in items {
                traverse_ast(item, visitor)?;
            }
        }
        AstNode::Function { body, .. } => {
            for stmt in body {
                traverse_ast(stmt, visitor)?;
            }
        }
        AstNode::Block(stmts) => {
            for stmt in stmts {
                traverse_ast(stmt, visitor)?;
            }
        }
        AstNode::If { then_branch, else_branch, .. } => {
            for stmt in then_branch {
                traverse_ast(stmt, visitor)?;
            }
            if let Some(else_branch) = else_branch {
                for stmt in else_branch {
                    traverse_ast(stmt, visitor)?;
                }
            }
        }
        AstNode::Class { methods, .. } => {
            for method in methods {
                traverse_ast(method, visitor)?;
            }
        }
        AstNode::Loop { body, .. } => {
            for stmt in body {
                traverse_ast(stmt, visitor)?;
            }
        }
        AstNode::Match { arms, .. } => {
            for arm in arms {
                for stmt in &arm.body {
                    traverse_ast(stmt, visitor)?;
                }
            }
        }
        AstNode::Assignment { target, value }
        | AstNode::Call { function: target, args: _ }
        | AstNode::BinaryOp { left: target, op: _, right: _ }
        | AstNode::UnaryOp { op: _, operand: target } => {
            traverse_ast(target, visitor)?;
        }
        AstNode::VarDecl { value, .. } => {
            if let Some(val) = value {
                traverse_ast(val, visitor)?;
            }
        }
        AstNode::Return(expr) => {
            if let Some(e) = expr {
                traverse_ast(e, visitor)?;
            }
        }
        AstNode::AnnotationNode { inner, .. } => {
            traverse_ast(inner, visitor)?;
        }
        _ => {}
    }
    Ok(())
}

/// Recursive AST traversal with mutable visitor allowing in-place edits.
pub fn traverse_ast_mut<V: AstVisitorMut>(node: &mut AstNode, visitor: &mut V) -> Result<()> {
    if !visitor.visit_mut(node)? {
        return Ok(()); // Skip children if visit_mut returns false
    }

    match node {
        AstNode::Module { items, .. } => {
            for item in items {
                traverse_ast_mut(item, visitor)?;
            }
        }
        AstNode::Function { body, .. } => {
            for stmt in body {
                traverse_ast_mut(stmt, visitor)?;
            }
        }
        AstNode::Block(stmts) => {
            for stmt in stmts {
                traverse_ast_mut(stmt, visitor)?;
            }
        }
        AstNode::If { then_branch, else_branch, .. } => {
            for stmt in then_branch {
                traverse_ast_mut(stmt, visitor)?;
            }
            if let Some(else_branch) = else_branch {
                for stmt in else_branch {
                    traverse_ast_mut(stmt, visitor)?;
                }
            }
        }
        AstNode::Class { methods, .. } => {
            for method in methods {
                traverse_ast_mut(method, visitor)?;
            }
        }
        AstNode::Loop { body, .. } => {
            for stmt in body {
                traverse_ast_mut(stmt, visitor)?;
            }
        }
        AstNode::Match { arms, .. } => {
            for arm in arms {
                for stmt in &mut arm.body {
                    traverse_ast_mut(stmt, visitor)?;
                }
            }
        }
        AstNode::Assignment { target, value }
        | AstNode::Call { function: target, args: _ }
        | AstNode::BinaryOp { left: target, op: _, right: _ }
        | AstNode::UnaryOp { op: _, operand: target } => {
            traverse_ast_mut(target, visitor)?;
        }
        AstNode::VarDecl { value, .. } => {
            if let Some(val) = value {
                traverse_ast_mut(val, visitor)?;
            }
        }
        AstNode::Return(expr) => {
            if let Some(e) = expr {
                traverse_ast_mut(e, visitor)?;
            }
        }
        AstNode::AnnotationNode { inner, .. } => {
            traverse_ast_mut(inner, visitor)?;
        }
        _ => {}
    }
    Ok(())
}

/// Convenience visitor to collect all nodes matching a predicate immutably.
pub struct CollectingVisitor<F>
where
    F: Fn(&AstNode) -> bool,
{
    pub predicate: F,
    pub matches: Vec<AstNode>,
}

impl<F> CollectingVisitor<F>
where
    F: Fn(&AstNode) -> bool,
{
    pub fn new(predicate: F) -> Self {
        Self {
            predicate,
            matches: Vec::new(),
        }
    }
}

impl<F> AstVisitor for CollectingVisitor<F>
where
    F: Fn(&AstNode) -> bool,
{
    fn visit(&mut self, node: &AstNode) -> Result<bool> {
        if (self.predicate)(node) {
            self.matches.push(node.clone());
        }
        Ok(true)
    }
}

/// Convenience visitor for mutable in-place node transformation.
pub struct MutatingVisitor<F>
where
    F: Fn(&mut AstNode) -> Result<()>,
{
    pub transform: F,
}

impl<F> MutatingVisitor<F>
where
    F: Fn(&mut AstNode) -> Result<()>,
{
    pub fn new(transform: F) -> Self {
        Self { transform }
    }
}

impl<F> AstVisitorMut for MutatingVisitor<F>
where
    F: Fn(&mut AstNode) -> Result<()>,
{
    fn visit_mut(&mut self, node: &mut AstNode) -> Result<bool> {
        (self.transform)(node)?;
        Ok(true)
    }
}
use crate::ast::universal::UniversalAst;
use crate::core::context::{Dependency, SymbioticMode, Symbol};
use std::collections::HashMap;
/// Context for transmutation process, holding symbols, dependencies, and metadata.
pub struct TransmutationContext {
    /// Optional source language
    pub source_lang: Option<String>,
    /// Symbiotic transformation mode
    pub symbiotic_mode: SymbioticMode,
    /// Symbol table
    pub symbols: HashMap<String, Symbol>,
    /// External dependencies
    pub dependencies: Vec<Dependency>,
    /// Arbitrary metadata
    pub metadata: HashMap<String, String>,
    /// Warnings collected during transmutation
    pub warnings: Vec<String>,
}
impl TransmutationContext {
    /// Create new transmutation context
    pub fn new() -> Self {
        Self {
            source_lang: None,
            symbiotic_mode: SymbioticMode::Standard,
            metadata: HashMap::new(),
            warnings: Vec::new(),
        }
    }
    /// Set source language
    pub fn with_source_lang(mut self, lang: impl Into<String>) -> Self {
        self.source_lang = Some(lang.into());
        self
    }
    /// Set symbiotic mode
    pub fn with_mode(mut self, mode: SymbioticMode) -> Self {
        self.symbiotic_mode = mode;
        self
    }
    /// Add symbol to symbol table
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }
    /// Retrieve symbol by name
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
    /// Add external dependency
    pub fn add_dependency(&mut self, dep: Dependency) {
        self.dependencies.push(dep);
    }
    /// Add warning message
    pub fn warn(&mut self, message: impl Into<String>) {
        self.warnings.push(message.into());
    }
    /// Set metadata key-value pair
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
impl Default for TransmutationContext {
    fn default() -> Self {
        Self::new()
    }
}
use anyhow::Result;
use crate::{
    ast::*,
    patterns::{DetectedPattern, TransformationHint},
    core::context::{TransmutationContext, CeremonialMode},
};
/// Ceremonial transformer
pub struct CeremonialTransformer {
    mode: CeremonialMode,
}
impl CeremonialTransformer {
    pub fn new() -> Self {
        Self {
            mode: CeremonialMode::Standard,
        }
    }
    
    /// Set ceremonial mode
    pub fn with_mode(mut self, mode: CeremonialMode) -> Self {
        self.mode = mode;
        self
    }
    
    /// Transform Universal AST to ceremonial representation
    pub fn transform(
        &self,
        ast: UniversalAst,
        patterns: Vec<DetectedPattern>,
        context: &mut TransmutationContext,
    ) -> Result<UniversalAst> {
        log::info!("✨ Beginning ceremonial transformation...");
        
        context.ceremonial_mode = self.mode;
        
        let transformed_root = self.transform_node(ast.root, &patterns, context)?;
        
        Ok(UniversalAst {
            root: transformed_root,
            metadata: ast.metadata,
        })
    }
    fn transform_node(
        &self,
        node: AstNode,
        patterns: &[DetectedPattern],
        context: &mut TransmutationContext,
    ) -> Result<AstNode> {
        match node {
            AstNode::Module { name, items } => {
                let transformed_items = items.into_iter()
                    .map(|item| self.transform_node(item, patterns, context))
                    .collect::<Result<Vec<_>>>()?;
                
                Ok(AstNode::Module {
                    name,
                    items: transformed_items,
                })
            }
            AstNode::Function { name, params, body } => {
                let transformed_body = body.into_iter()
                    .map(|stmt| self.transform_node(stmt, patterns, context))
                    .collect::<Result<Vec<_>>>()?;
                
                Ok(AstNode::Function {
                    name,
                    params,
                    body: transformed_body,
                })
            }
            AstNode::Block(stmts) => {
                let transformed_stmts = stmts.into_iter()
                    .map(|stmt| self.transform_node(stmt, patterns, context))
                    .collect::<Result<Vec<_>>>()?;
                
                Ok(AstNode::Block(transformed_stmts))
            }
            // Handle other node types similarly...
            _ => Ok(node), // For simplicity, return unmodified node for unhandled types
        }
    }
}
/// Complete transformation pipeline
pub struct TransformationPipeline {
    stages: Vec<Box<dyn PipelineStage>>,
}
impl TransformationPipeline {
    /// Create new pipeline
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
        }
    }
    
    /// Add stage to pipeline
    pub fn add_stage(mut self, stage: Box<dyn PipelineStage>) -> Self {
        self.stages.push(stage);
        self
    }
    
    /// Execute entire pipeline
    pub fn execute(&self, mut ast: UniversalAst, context: &mut TransmutationContext) -> Result<UniversalAst> {
        for stage in &self.stages {
            log::debug!("Executing pipeline stage: {}", stage.name());
            ast = stage.execute(ast, context)?;
        }
        Ok(ast)
    }
}
impl Default for TransformationPipeline {
    fn default() -> Self {
        Self::new()
    }
}
/// Trait for a pipeline stage
pub trait PipelineStage {
    /// Stage name
    fn name(&self) -> &str;
    
    /// Execute this stage
    fn execute(&self, ast: UniversalAst, context: &mut TransmutationContext) -> Result<UniversalAst>;
}
/// Example stage: Type inference
pub struct TypeInferenceStage;
impl PipelineStage for TypeInferenceStage {
    fn name(&self) -> &str {
        "Type Inference"
    }
    fn execute(&self, ast: UniversalAst, _context: &mut TransmutationContext) -> Result<UniversalAst> {
        Self {
            target_lang: None,
            symbols: Vec::new(),
            dependencies: Vec::new(),
            patterns: Vec::new(),
        }
    }       Ok(ast)
    }
use anyhow::Result;
use crate::{
    ast::UniversalAst,
    core::context::TransmutationContext,
};
/// Transformation pipeline stage
pub trait PipelineStage: Send + Sync {
    /// Stage name
    fn name(&self) -> &str;
    
    /// Execute this stage
    fn execute(&self, ast: UniversalAst, context: &mut TransmutationContext) -> Result<UniversalAst>;
}
/// Complete transformation pipeline
pub struct TransformationPipeline {
    stages: Vec<Box<dyn PipelineStage>>,
}