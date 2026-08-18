use pest::Parser;
use pest::iterators::Pairs;
use crate::ast::*;
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
            Rule::ritual_block => {
                let mut inner = pair.into_inner();
                let open = RitualOpen::DoubleColon; // Example, parse from tokens properly
                let nodes = build_nodes(inner.next().unwrap().into_inner())?;
                let close = RitualClose::TripleColon; // Example, same as open
                Some(RitualNode::RitualBlock { open, nodes, close })
            }
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

pub fn parse_ritual(input: &str) -> Result<Vec<RitualNode>, String> {
    let pairs = RitualParser::parse(Rule::ritual, input)
        .map_err(|e| format!("Parse error: {}", e))?;
    build_nodes(pairs)
}

// Helper functions: build_sigil, build_echo, build_oracle, build_spinon,
// build_glyph, build_condition, build_logical, build_pattern_match,
// build_modifier, and build_temporal should follow your previous logic,
// adapted for consistency with your AST.