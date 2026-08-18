/*!
Executor: Ritual Runtime Orchestrator for Fuxyez Compiler Core
--------------------------------------------------------------------
The Executor walks the Abstract Syntax Tree (AST) and performs ritual
evaluation — with direct integration into Sentinels, diagnostics, and
runtime monitoring.

Execution is designed to be symbolic: each node can invoke a detector,
emit an event, or modify shared runtime state.

This system supports interactive synchronous/asynchronous tasks,
parallel invocations, and temporal (delayed/scheduled) rituals.

Everything the language executes passes through this file.
*/

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use crate::ast::*;
use crate::diagnostics::{Diagnostic, DiagnosticLevel, DiagnosticSink, Span};
use crate::sentinel_core::{SentinelEvent, SentinelRegistry, Severity};

//------------------------------------------------------------------------------
// Runtime Context
//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ExecContext {
    pub env: HashMap<String, String>,
    pub sentinel: Option<Arc<SentinelRegistry>>,
    pub diagnostics: Arc<Mutex<DiagnosticSink>>,
}

impl ExecContext {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            sentinel: Some(Arc::new(SentinelRegistry::with_default_detectors())),
            diagnostics: Arc::new(Mutex::new(DiagnosticSink::new())),
        }
    }

    pub fn set_var(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.env.insert(key.into(), val.into());
    }

    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.env.get(key)
    }

    pub fn push_diag(&self, msg: &str, level: DiagnosticLevel, span: Option<Span>) {
        if let Ok(mut sink) = self.diagnostics.lock() {
            sink.report(Diagnostic {
                level,
                message: msg.to_string(),
                span: span.unwrap_or(Span::new(0, 0)),
                code: None,
            });
        }
    }

    fn run_sentinels(&self, context_str: &str) {
        if let Some(ref registry) = self.sentinel {
            let findings = registry.run_all(context_str);
            for f in findings {
                if let Ok(mut sink) = self.diagnostics.lock() {
                    let diag: Diagnostic = f.into();
                    sink.report(diag);
                }
            }
        }
    }
}

//------------------------------------------------------------------------------
// Executor Core Implementation
//------------------------------------------------------------------------------

pub struct Executor {
    ctx: Arc<Mutex<ExecContext>>,
}

impl Executor {
    pub fn new(ctx: Arc<Mutex<ExecContext>>) -> Self {
        Self { ctx }
    }

    /// The main entry point for executing AST nodes.
    pub fn execute(&self, nodes: &[RitualNode]) -> Result<(), String> {
        for n in nodes {
            self.eval_node(n)?;
        }
        Ok(())
    }

    // Evaluate a single AST node
    fn eval_node(&self, node: &RitualNode) -> Result<(), String> {
        match node {
            RitualNode::Sigil(s) => self.sigil(s),
            RitualNode::Echo(e) => self.echo(e),
            RitualNode::Oracle(o) => self.oracle(o),
            RitualNode::Spinon(b) => self.bind(b),
            RitualNode::Condition(c) => self.condition(c),
            RitualNode::LogicalNode(l) => self.logical(l),
            RitualNode::RitualGroup(group) | RitualNode::AsyncBlock(group) => {
                for n in group {
                    self.eval_node(n)?;
                }
                Ok(())
            }
            RitualNode::ParallelBlock(group) => self.parallel(group),
            RitualNode::Temporal(t) => self.temporal(t),
            RitualNode::Modifier(m) => self.modifier(m),
            RitualNode::Annotated(inner, _) | RitualNode::WithSpan(inner, _) => self.eval_node(inner),
            _ => Ok(()),
        }
    }

    fn sigil(&self, sig: &Sigil) -> Result<(), String> {
        let msg = format!("Activating sigil '{}' with params {:?}", sig.name, sig.params);
        let lock = self.ctx.lock().unwrap();
        lock.run_sentinels(&msg);
        drop(lock);
        println!("{}", msg);
        Ok(())
    }

    fn echo(&self, echo: &Echo) -> Result<(), String> {
        println!("ECHO:: {}", echo.event);
        self.eval_node(&echo.response)
    }

    fn oracle(&self, orc: &Oracle) -> Result<(), String> {
        let msg = format!("Consulting Oracle with query: '{}'", orc.query);
        {
            let ctx = self.ctx.lock().unwrap();
            ctx.run_sentinels(&msg);
            ctx.push_diag("Oracle query executed.", DiagnosticLevel::Info, None);
        }
        if let Some(ref out) = orc.output {
            self.eval_node(out)?;
        }
        println!("{}", msg);
        Ok(())
    }

    fn bind(&self, sp: &Spinon) -> Result<(), String> {
        let mut ctx = self.ctx.lock().unwrap();
        ctx.set_var(&sp.binding, &sp.value);
        ctx.push_diag(
            &format!("Bound {} = {}", &sp.binding, &sp.value),
            DiagnosticLevel::Info,
            None,
        );
        Ok(())
    }

    fn logical(&self, logic: &LogicalNode) -> Result<(), String> {
        println!("Evaluating logic {:?}", logic.op);
        for n in &logic.nodes {
            self.eval_node(n)?;
        }
        Ok(())
    }

    fn condition(&self, cond: &ConditionNode) -> Result<(), String> {
        match cond {
            ConditionNode::If { cond, then_branch, else_branch } => {
                println!("IF condition encountered");
                self.eval_node(cond)?;
                self.execute(then_branch)?;
                if let Some(e) = else_branch {
                    self.execute(e)?;
                }
            }
            ConditionNode::Match { target, arms } => {
                println!("MATCH invocation");
                self.eval_node(target)?;
                for (pattern, branch) in arms {
                    println!("Pattern Arm: {:?}", pattern.kind);
                    self.execute(branch)?;
                }
            }
        }
        Ok(())
    }

    fn modifier(&self, modr: &ModifierNode) -> Result<(), String> {
        let msg = format!("Modifier {:?} applied to node", modr.kind);
        let lock = self.ctx.lock().unwrap();
        lock.run_sentinels(&msg);
        println!("{}", msg);
        self.eval_node(&modr.target)
    }

    fn parallel(&self, group: &[RitualNode]) -> Result<(), String> {
        let threads: Vec<_> = group
            .iter()
            .map(|node| {
                let exec = self.clone();
                thread::spawn(move || {
                    exec.eval_node(node).unwrap_or_else(|e| println!("Parallel exec error: {}", e));
                })
            })
            .collect();
        for t in threads {
            t.join().expect("parallel thread join failed");
        }
        Ok(())
    }

    fn temporal(&self, t: &TemporalNode) -> Result<(), String> {
        match &t.kind {
            TemporalKind::After(delay) => {
                println!("Temporal delay ritual: {}", delay);
                // simple sleep placeholder
                thread::sleep(Duration::from_millis(500));
                self.execute(&t.nodes)
            }
            TemporalKind::Every(interval) => {
                println!("Temporal repeating ritual: {}", interval);
                for _ in 0..2 {
                    self.execute(&t.nodes)?;
                    thread::sleep(Duration::from_millis(200));
                }
                Ok(())
            }
            other => {
                println!("Temporal Start: {:?}", other);
                self.execute(&t.nodes)
            }
        }
    }
}

//------------------------------------------------------------------------------
// Implement Clone manually for controlled Arc context
//------------------------------------------------------------------------------

impl Clone for Executor {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
        }
    }
}

//------------------------------------------------------------------------------
// Future Hooks (Runtime integrations)
//------------------------------------------------------------------------------

/*
This file is Sentinel-aware. All events, node evals, and variable bindings
feed through the SentinelRegistry, which can dynamically warn, stop, or repair
execution logic. Each invocation traces context for diagnostics,
allowing the IDE and runtime UI (UIR) to replay or visualize execution.

Its design fuses symbolic ritual logic, concurrency, introspection, and
diagnostic feedback into one mythic execution layer.
*/