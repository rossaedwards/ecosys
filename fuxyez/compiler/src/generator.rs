use crate::uir::UirNode;
use futures::future::{BoxFuture, FutureExt};
use std::pin::Pin;

/// The advanced async generator layered with recursive async traversal.
pub struct Generator {
    pub bytecode: Vec<String>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    /// Generate code from a UIR node asynchronously.
    pub fn generate<'a>(&'a mut self, node: &'a UirNode) -> BoxFuture<'a, Result<(), String>> {
        async move {
            match node {
                UirNode::Sigil { name, params } => {
                    self.bytecode.push(format!("sigil {} {:?}", name, params));
                    Ok(())
                }
                UirNode::Echo { event, response } => {
                    self.bytecode.push(format!("echo {}", event));
                    self.generate(response).await?;
                    Ok(())
                }
                UirNode::Oracle { query, output } => {
                    self.bytecode.push(format!("oracle {}", query));
                    if let Some(resp) = output {
                        self.generate(resp).await?;
                    }
                    Ok(())
                }
                UirNode::Spinon { binding, value } => {
                    self.bytecode.push(format!("spinon {} {}", binding, value));
                    Ok(())
                }
                UirNode::Glyph { key, value } => {
                    self.bytecode.push(format!("glyph {} {}", key, value));
                    Ok(())
                }
                UirNode::Group(nodes) => {
                    for n in nodes {
                        self.generate(n).await?;
                    }
                    Ok(())
                }
                UirNode::Condition { cond, then_branch, else_branch } => {
                    self.bytecode.push("cond".to_string());
                    self.generate(cond).await?;
                    for n in then_branch {
                        self.generate(n).await?;
                    }
                    if let Some(else_nodes) = else_branch {
                        self.bytecode.push("else".to_string());
                        for n in else_nodes {
                            self.generate(n).await?;
                        }
                    }
                    Ok(())
                }
                UirNode::Logical { op, nodes } => {
                    self.bytecode.push(format!("logical {:?}", op));
                    for n in nodes {
                        self.generate(n).await?;
                    }
                    Ok(())
                }
                UirNode::PatternMatch { target, arms } => {
                    self.bytecode.push("pattern_match".to_string());
                    self.generate(target).await?;
                    for (pat, nodelist) in arms {
                        self.bytecode.push(format!("pattern {:?}", pat));
                        for n in nodelist {
                            self.generate(n).await?;
                        }
                    }
                    Ok(())
                }
                UirNode::Modifier { kind, target } => {
                    self.bytecode.push(format!("modifier {:?}", kind));
                    self.generate(target).await?;
                    Ok(())
                }
                UirNode::AsyncBlock(nodes) => {
                    self.bytecode.push("async_block".to_string());
                    for n in nodes {
                        self.generate(n).await?;
                    }
                    Ok(())
                }
                UirNode::ParallelBlock(nodes) => {
                    self.bytecode.push("parallel_block".to_string());
                    for n in nodes {
                        self.generate(n).await?;
                    }
                    Ok(())
                }
                UirNode::Temporal { kind, nodes } => {
                    self.bytecode.push(format!("temporal {:?}", kind));
                    for n in nodes {
                        self.generate(n).await?;
                    }
                    Ok(())
                }
                UirNode::OptimizedExpr(expr) => {
                    self.bytecode.push(format!("optimized {}", expr));
                    Ok(())
                }
            }
        }.boxed()
    }

    /// Finalize the bytecode vector.
    pub fn finalize(self) -> Vec<String> {
        self.bytecode
    }
}
// -- Temporal Schedulers
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalKind {
    Delay(u64), // milliseconds
    Interval(u64), // milliseconds
}
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalNode {
    pub kind: TemporalKind,
    pub nodes: Vec<RitualNode>,
}
// The top-level RitualNode enum representing all possible AST nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum RitualNode {
    Sigil(SigilNode),
    Echo(EchoNode),
    Oracle(OracleNode),
    Spinon(SpinonNode),
    Glyph(GlyphNode),
    RitualGroup(Vec<RitualNode>),
    RitualBlock { nodes: Vec<RitualNode> },
    Condition(ConditionNode),
    LogicalNode(LogicalNode),
    PatternMatch(PatternMatchNode),
    Modifier(ModifierNode),
    AsyncBlock(Vec<RitualNode>),
    ParallelBlock(Vec<RitualNode>),
    Temporal(TemporalNode),
}
#[derive(Debug, Clone, PartialEq)]
pub struct SigilNode {
    pub name: String,
    pub params: Vec<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct EchoNode {
    pub event: String,
    pub response: Box<RitualNode>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OracleNode {
    pub query: String,
    pub output: Option<Box<RitualNode>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SpinonNode {
    pub binding: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GlyphNode {
    pub key: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionNode {
    pub cond: Box<RitualNode>,
    pub then_branch: Vec<RitualNode>,
    pub else_branch: Option<Vec<RitualNode>>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
    Not,
}
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalNode {
    pub op: LogicalOp,
    pub nodes: Vec<RitualNode>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PatternMatchNode {
    pub target: Box<RitualNode>,
    pub arms: Vec<(Pattern, Vec<RitualNode>)>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Literal(String),
    Identifier(String),
    // Add more pattern types as needed
}
#[derive(Debug, Clone, PartialEq)]
pub struct ModifierNode {
    pub kind: ModifierKind,
    pub target: Box<RitualNode>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ModifierKind {
    Optional,
    Repeated,
    // Add more modifier types as needed
}
// -- Parser Implementation
use pest::Parser;
use pest::iterators::Pairs;
use crate::ast::{RitualNode};
// Import the generated parser and Rule enum from your pest grammar
use crate::parser::{RitualParser, Rule};
fn build_nodes(mut pairs: Pairs<Rule>) -> Result<Vec<RitualNode>, String> {
    let mut nodes = Vec::new();

    while let Some(pair) = pairs.next() {
        let node = match pair.as_rule() {
            Rule::sigil => Some(build_sigil(pair)?),
            Rule::echo => Some(build_echo(pair)?),
            Rule::oracle => Some(build_oracle(pair)?),
            Rule::spinon => Some(build_spinon(pair)?),
            Rule::glyph => Some(build_glyph(pair)?),
            Rule::ritual_group => Some(RitualNode::RitualGroup(build_nodes(pair.into_inner())?)),
            Rule::ritual_block => Some(RitualNode::RitualBlock{nodes: build_nodes(pair.into_inner())?}),
            Rule::condition => Some(build_condition(pair)?),
            Rule::logical => Some(build_logical(pair)?),
            Rule::pattern_match => Some(build_pattern_match(pair)?),
            Rule::modifier => Some(build_modifier(pair)?),
            Rule::async_block => Some(RitualNode::AsyncBlock(build_nodes(pair.into_inner())?)),
            Rule::parallel_block => Some(RitualNode::ParallelBlock(build_nodes(pair.into_inner())?)),
            Rule::temporal => Some(build_temporal(pair)?),
            _ => None,
        };

        if let Some(n) = node {
            nodes.push(n);
        }
    }

    Ok(nodes)
}
/// Parses a ritual from the given input string and returns a vector of RitualNode AST nodes.
/// Note: For deeply nested rituals, `build_nodes` uses recursion and may cause high stack usage.
/// Consider profiling or refactoring to an iterative approach for very large inputs.
/// # Arguments
/// *  `input` - A string slice containing the ritual source code to be parsed.
/// # Errors
/// Returns a `String` error message if parsing fails.
/// # Examples
/// ```
/// let ritual_code = r#"   sigil my_sigil(param1, param2)
///   echo "event_name" {
///       spinon my_var = "value"
///   } "#;
/// let ast_nodes = parse_ritual(ritual_code).expect("Failed to parse ritual");
/// assert!(!ast_nodes.is_empty());
/// ```
pub fn parse_ritual(input: &str) -> Result<Vec<RitualNode>, String> {
    let pairs = RitualParser::parse(Rule::ritual, input)
        .map_err(|e| format!("Parse error while parsing rule 'ritual' with input '{}': {}", input, e))?;
    build_nodes(pairs)
}
// Helper functions to build specific AST nodes from pest pairs
fn build_sigil(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or("Missing sigil name")?.as_str().to_string();
    let params_pair = inner.next().ok_or("Missing sigil params")?;
    let params = params_pair.into_inner().map(|p| p.as_str().to_string()).collect();
    Ok(RitualNode::Sigil(SigilNode { name, params }))
}
fn build_echo(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let event = inner.next().ok_or("Missing echo event")?.as_str().to_string();
    let response_pair = inner.next().ok_or("Missing echo response")?;
    let response = Box::new(build_nodes(response_pair.into_inner())?.into_iter().next().ok_or("Empty echo response")?);
    Ok(RitualNode::Echo(EchoNode { event, response }))
}
fn build_oracle(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let query = inner.next().ok_or("Missing oracle query")?.as_str().to_string();
    let output = if let Some(output_pair) = inner.next() {
        Some(Box::new(build_nodes(output_pair.into_inner())?.into_iter().next().ok_or("Empty oracle output")?))
    } else {
        None
    };
    Ok(RitualNode::Oracle(OracleNode { query, output }))
}
fn build_spinon(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let binding = inner.next().ok_or("Missing spinon binding")?.as_str().to_string();
    let value = inner.next().ok_or("Missing spinon value")?.as_str().to_string();
    Ok(RitualNode::Spinon(SpinonNode { binding, value }))
}
fn build_glyph(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let key = inner.next().ok_or("Missing glyph key")?.as_str().to_string();
    let value = inner.next().ok_or("Missing glyph value")?.as_str().to_string();
    Ok(RitualNode::Glyph(GlyphNode { key, value }))
}
fn build_condition(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let cond_pair = inner.next().ok_or("Missing condition expression")?;
    let cond = Box::new(build_nodes(cond_pair.into_inner())?.into_iter().next().ok_or("Empty condition expression")?);
    let then_pair = inner.next().ok_or("Missing then branch")?;
    let then_branch = build_nodes(then_pair.into_inner())?;
    let else_branch = if let Some(else_pair) = inner.next() {
        Some(build_nodes(else_pair.into_inner())?)
    } else {
        None
    };
    Ok(RitualNode::Condition(ConditionNode { cond, then_branch, else_branch }))
}
fn build_logical(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let op_pair = inner.next().ok_or("Missing logical operator")?;
    let op = match op_pair.as_str() {
        "and" => LogicalOp::And,
        "or" => LogicalOp::Or,
        "not" => LogicalOp::Not,
        _ => return Err("Unknown logical operator".to_string()),
    };
    let nodes = inner.map(|p| build_nodes(p.into_inner()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(RitualNode::LogicalNode(LogicalNode { op, nodes }))
}
fn build_pattern_match(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let target_pair = inner.next().ok_or("Missing pattern match target")?;
    let target = Box::new(build_nodes(target_pair.into_inner())?.into_iter().next().ok_or("Empty pattern match target")?);
    let mut arms = Vec::new();
    for arm_pair in inner {
        let mut arm_inner = arm_pair.into_inner();
        let pat_pair = arm_inner.next().ok_or("Missing pattern in arm")?;
        let pat = match pat_pair.as_rule() {
            Rule::wildcard => Pattern::Wildcard,
            Rule::literal => Pattern::Literal(pat_pair.as_str().to_string()),
            Rule::identifier => Pattern::Identifier(pat_pair.as_str().to_string()),
            _ => return Err("Unknown pattern type".to_string()),
        };
        let nodes = build_nodes(arm_inner)?.into_iter().collect();
        arms.push((pat, nodes));
    }
    Ok(RitualNode::PatternMatch(PatternMatchNode { target, arms }))
}
fn build_modifier(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let kind_pair = inner.next().ok_or("Missing modifier kind")?;
    let kind = match kind_pair.as_str() {
        "optional" => ModifierKind::Optional,
        "repeated" => ModifierKind::Repeated,
        _ => return Err("Unknown modifier kind".to_string()),
    };
    let target_pair = inner.next().ok_or("Missing modifier target")?;
    let target = Box::new(build_nodes(target_pair.into_inner())?.into_iter().next().ok_or("Empty modifier target")?);
    Ok(RitualNode::Modifier(ModifierNode { kind, target }))
}
fn build_temporal(pair: pest::iterators::Pair<Rule>) -> Result<RitualNode, String> {
    let mut inner = pair.into_inner();
    let kind_pair = inner.next().ok_or("Missing temporal kind")?;
    let kind = match kind_pair.as_rule() {
        Rule::delay => {
            let ms_str = kind_pair.into_inner().next().ok_or("Missing delay duration")?.as_str();
            let ms = ms_str.parse::<u64>().map_err(|_| "Invalid delay duration")?;
            TemporalKind::Delay(ms)
        }
        Rule::interval => {
            let ms_str = kind_pair.into_inner().next().ok_or("Missing interval duration")?.as_str();
            let ms = ms_str.parse::<u64>().map_err(|_| "Invalid interval duration")?;
            TemporalKind::Interval(ms)
        }
        _ => return Err("Unknown temporal kind".to_string()),
    };
    let nodes_pair = inner.next().ok_or("Missing temporal nodes")?;
    let nodes = build_nodes(nodes_pair.into_inner())?;
    Ok(RitualNode::Temporal(TemporalNode { kind, nodes }))
}
// -- UIR Implementation
use crate::ast::{LogicalOp, ModifierKind, Pattern, RitualNode, TemporalKind};
use std::collections::HashMap;
// The UIR node enum representing the unified intermediate representation.
#[derive(Debug, Clone, PartialEq)]
pub enum UirNode {
    Sigil { name: String, params: Vec<String> },
    Echo { event: String, response: Box<UirNode> },
    Oracle { query: String, output: Option<Box<UirNode>> },
    Spinon { binding: String, value: String },
    Glyph { key: String, value: String },
    Group(Vec<UirNode>),
    Condition {
        then_branch: Vec<UirNode>,
        else_branch: Option<Vec<UirNode>>,
        cond: Box<UirNode>,
    },
    Logical { op: LogicalOp, nodes: Vec<UirNode> },
    PatternMatch { target: Box<UirNode>, arms: Vec<(Pattern, Vec<UirNode>)> },
    Modifier { kind: ModifierKind, target: Box<UirNode> },
    AsyncBlock(Vec<UirNode>),
    ParallelBlock(Vec<UirNode>),
    Temporal { kind: TemporalKind, nodes: Vec<UirNode> },
    OptimizedExpr(String), // Placeholder for optimized expressions
}
/// Builder for converting AST nodes to UIR nodes, with a symbol table.
pub struct UirBuilder {
    symbol_table: HashMap<String, String>,
}