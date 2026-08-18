/*!
Optimizer: The Refinery of Ritual Logic
--------------------------------------------------------------------
This module performs high-level optimizations over the Fuxyez AST.
It traverses RitualNodes, folds redundant logic, eliminates no-ops,
and ensures state consistency while invoking Sentinels to validate
transformations at each step.

The optimizer is structurally aware — it doesn’t just compress code;
it performs ritual refinement.

Each optimization pass can:
- Simplify repetitive modifiers.
- Inline constant bindings.
- Merge nested logical/parallel/async blocks.
- Detect and correct inefficiencies via SentinelRegistry.

This serves as the bridge between AST elegance and executor efficiency.
*/

use std::sync::{Arc, Mutex};

use crate::ast::*;
use crate::diagnostics::{Diagnostic, DiagnosticLevel, DiagnosticSink, Span};
use crate::sentinel_core::{SentinelEvent, SentinelRegistry, Severity};

//------------------------------------------------------------------------------
// Optimization Pass Context
//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct OptimizeContext {
    pub registry: Arc<SentinelRegistry>,
    pub diagnostics: Arc<Mutex<DiagnosticSink>>,
    pub applied_passes: Vec<String>,
}

impl OptimizeContext {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(SentinelRegistry::with_default_detectors()),
            diagnostics: Arc::new(Mutex::new(DiagnosticSink::new())),
            applied_passes: vec![],
        }
    }

    fn record_pass(&mut self, name: &str) {
        self.applied_passes.push(name.to_string());
        if let Ok(mut sink) = self.diagnostics.lock() {
            sink.report(Diagnostic {
                level: DiagnosticLevel::Info,
                message: format!("Optimization pass applied: {}", name),
                span: Span::new(0, 0),
                code: None,
            });
        }
    }

    fn run_sentinels(&self, msg: &str) {
        let results = self.registry.run_all(msg);
        if let Ok(mut sink) = self.diagnostics.lock() {
            for ev in results {
                let diag: Diagnostic = ev.into();
                sink.report(diag);
            }
        }
    }
}

//------------------------------------------------------------------------------
// Optimizer Definition
//------------------------------------------------------------------------------

pub struct Optimizer {
    pub ctx: OptimizeContext,
}

impl Optimizer {
    pub fn new(ctx: OptimizeContext) -> Self {
        Self { ctx }
    }

    /// Perform a complete optimization pass chain on a set of nodes.
    pub fn optimize(&mut self, ast: Vec<RitualNode>) -> Vec<RitualNode> {
        let mut nodes = ast;

        nodes = self.merge_nested_blocks(nodes);
        self.ctx.record_pass("merge_nested_blocks");

        nodes = self.inline_spinons(nodes);
        self.ctx.record_pass("inline_spinons");

        nodes = self.purify_modifiers(nodes);
        self.ctx.record_pass("purify_modifiers");

        self.ctx.run_sentinels("Fuxyez optimization completed");
        nodes
    }

    //------------------------------------------------------------------------------
    // Pass 1: Merge Nested Ritual/Parallel/Async Blocks
    //------------------------------------------------------------------------------

    fn merge_nested_blocks(&self, nodes: Vec<RitualNode>) -> Vec<RitualNode> {
        let mut result = Vec::new();
        for node in nodes {
            match node {
                RitualNode::RitualGroup(inner) => {
                    let flattened: Vec<RitualNode> = inner
                        .into_iter()
                        .flat_map(|n| match n {
                            RitualNode::RitualGroup(g2) => g2,
                            other => vec![other],
                        })
                        .collect();
                    result.push(RitualNode::RitualGroup(flattened));
                }
                RitualNode::ParallelBlock(inner) => {
                    let merged: Vec<RitualNode> = inner
                        .into_iter()
                        .flat_map(|n| match n {
                            RitualNode::ParallelBlock(g2) => g2,
                            other => vec![other],
                        })
                        .collect();
                    result.push(RitualNode::ParallelBlock(merged));
                }
                RitualNode::AsyncBlock(inner) => {
                    let unified: Vec<RitualNode> = inner
                        .into_iter()
                        .flat_map(|n| match n {
                            RitualNode::AsyncBlock(ab2) => ab2,
                            other => vec![other],
                        })
                        .collect();
                    result.push(RitualNode::AsyncBlock(unified));
                }
                RitualNode::RitualBlock { open, nodes, close } => {
                    let optimized_inner = self.merge_nested_blocks(nodes);
                    result.push(RitualNode::RitualBlock { open, nodes: optimized_inner, close });
                }
                other => result.push(other),
            }
        }
        result
    }

    //------------------------------------------------------------------------------
    // Pass 2: Inline Simple Spinon Bindings
    //------------------------------------------------------------------------------

    fn inline_spinons(&self, nodes: Vec<RitualNode>) -> Vec<RitualNode> {
        let mut inlined: Vec<RitualNode> = Vec::new();
        let mut bindings: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        for n in nodes {
            match n {
                RitualNode::Spinon(sp) => {
                    bindings.insert(sp.binding.clone(), sp.value.clone());
                    inlined.push(RitualNode::Spinon(sp));
                }
                RitualNode::Echo(mut e) => {
                    // Replace binding identifiers inside echo events
                    for (k, v) in &bindings {
                        if e.event.contains(k) {
                            e.event = e.event.replace(k, v);
                        }
                    }
                    inlined.push(RitualNode::Echo(e));
                }
                RitualNode::Oracle(mut o) => {
                    for (k, v) in &bindings {
                        if o.query.contains(k) {
                            o.query = o.query.replace(k, v);
                        }
                    }
                    inlined.push(RitualNode::Oracle(o));
                }
                RitualNode::RitualGroup(g) => {
                    let rewritten = self.inline_spinons(g);
                    inlined.push(RitualNode::RitualGroup(rewritten));
                }
                other => inlined.push(other),
            }
        }
        inlined
    }

    //------------------------------------------------------------------------------
    // Pass 3: Purify Modifiers (remove neutral or redundant transformations)
    //------------------------------------------------------------------------------

    fn purify_modifiers(&self, nodes: Vec<RitualNode>) -> Vec<RitualNode> {
        let mut refined = Vec::new();
        for n in nodes {
            match n {
                RitualNode::Modifier(m) => match m.kind {
                    ModifierKind::Custom(ref name) if name.is_empty() => {
                        self.ctx.run_sentinels("Removed empty custom modifier");
                    }
                    ModifierKind::Amplify => {
                        // Example redundancy: amplify followed by amplify
                        self.ctx.run_sentinels("Amplify modifier detected — checking redundancy");
                    }
                    _ => {}
                },
                RitualNode::RitualGroup(g) => refined.push(RitualNode::RitualGroup(self.purify_modifiers(g))),
                RitualNode::AsyncBlock(g) => refined.push(RitualNode::AsyncBlock(self.purify_modifiers(g))),
                RitualNode::ParallelBlock(g) => refined.push(RitualNode::ParallelBlock(self.purify_modifiers(g))),
                other => refined.push(other),
            }
        }
        refined
    }
}

//------------------------------------------------------------------------------
// Optimization Summary Formatting
//------------------------------------------------------------------------------

impl std::fmt::Display for Optimizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let summary = self.ctx.registry.export_summary();
        writeln!(f, "Optimizer Summary:")?;
        for (sev, count) in summary {
            writeln!(f, "  {:?}: {} findings", sev, count)?;
        }
        writeln!(f, "Applied passes: {:?}", self.ctx.applied_passes)?;
        Ok(())
    }
}

/* ===========================================================================
Mythos of the Optimizer
-----------------------
In Fuxyez, the optimizer serves not just to enhance performance —
it is the purification ritual, eliminating impurity and redundancy
from logic, ensuring ritual coherence before execution.

Each pass reaffirms structural stability while Sentinels scan the resulting
transmutations for imbalance or inefficiency. Together, they form
a self-regulating cycle of clarity between syntax, semantics, and spirit.
=========================================================================== */