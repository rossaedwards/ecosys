use crate::ast::*;
use futures::future::{BoxFuture, FutureExt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum UirNode {
    Sigil { name: String, params: Vec<String> },
    Echo { event: String, response: Box<UirNode> },
    Oracle { query: String, output: Option<Box<UirNode>> },
    Spinon { binding: String, value: String },
    Glyph { key: String, value: String },
    Group(Vec<UirNode>),
    Condition { cond: Box<UirNode>, then_branch: Vec<UirNode>, else_branch: Option<Vec<UirNode>> },
    Logical { op: LogicalOp, nodes: Vec<UirNode> },
    PatternMatch { target: Box<UirNode>, arms: Vec<(Pattern, Vec<UirNode>)> },
    Modifier { kind: ModifierKind, target: Box<UirNode> },
    AsyncBlock(Vec<UirNode>),
    ParallelBlock(Vec<UirNode>),
    Temporal { kind: TemporalKind, nodes: Vec<UirNode> },
    OptimizedExpr(String),
}

pub struct UirBuilder {
    symbol_table: HashMap<String, String>,
}

impl UirBuilder {
    pub fn new() -> Self {
        UirBuilder {
            symbol_table: HashMap::new(),
        }
    }

    pub fn from_ast(&mut self, node: &RitualNode) -> Result<UirNode, String> {
        match node {
            RitualNode::Sigil(s) => Ok(UirNode::Sigil { name: s.name.clone(), params: s.params.clone() }),
            RitualNode::Echo(e) => Ok(UirNode::Echo { event: e.event.clone(), response: Box::new(self.from_ast(&e.response)?) }),
            RitualNode::Oracle(o) => Ok(UirNode::Oracle { query: o.query.clone(), output: o.output.as_ref().map(|o| self.from_ast(o)).transpose()? }),
            RitualNode::Spinon(s) => Ok(UirNode::Spinon { binding: s.binding.clone(), value: s.value.clone() }),
            RitualNode::Glyph(g) => Ok(UirNode::Glyph { key: g.key.clone(), value: g.value.clone() }),
            RitualNode::RitualGroup(nodes) | RitualNode::RitualBlock { nodes, .. } => {
                let children = nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()?;
                Ok(UirNode::Group(children))
            }
            RitualNode::Condition { cond, then_branch, else_branch } => {
                Ok(UirNode::Condition {
                    cond: Box::new(self.from_ast(cond)?),
                    then_branch: then_branch.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()?,
                    else_branch: else_branch.as_ref().map(|nodes| nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()).transpose()?,
                })
            }
            RitualNode::LogicalNode(l) => Ok(UirNode::Logical { op: l.op.clone(), nodes: l.nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()? }),
            RitualNode::PatternMatch { target, arms } => Ok(UirNode::PatternMatch {
                target: Box::new(self.from_ast(target)?),
                arms: arms.iter().map(|(pat, nodes)| {
                    Ok((pat.clone(), nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()?))
                }).collect::<Result<Vec<_>, _>>()?,
            }),
            RitualNode::Modifier { kind, target } => Ok(UirNode::Modifier { kind: kind.clone(), target: Box::new(self.from_ast(target)?) }),
            RitualNode::AsyncBlock(nodes) => Ok(UirNode::AsyncBlock(nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()?)),
            RitualNode::ParallelBlock(nodes) => Ok(UirNode::ParallelBlock(nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()?)),
            RitualNode::Temporal { kind, nodes } => Ok(UirNode::Temporal { kind: kind.clone(), nodes: nodes.iter().map(|n| self.from_ast(n)).collect::<Result<Vec<_>, _>>()? }),
            RitualNode::Annotated(boxed, _) | RitualNode::WithSpan(boxed, _) => self.from_ast(boxed),
            _ => Err("Unsupported AST node".into())
        }
    }
}