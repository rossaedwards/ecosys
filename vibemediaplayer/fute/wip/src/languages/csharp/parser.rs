//! C# Parser

use anyhow::Result;
use crate::{
    ast::*,
    core::context::{TransmutationContext, Symbol, SymbolKind},
};

pub struct CSharpParser;

impl CSharpParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse_source(&self, source: &str, context: &mut TransmutationContext) -> Result<UniversalAst> {
        let mut items = Vec::new();
        
        // Simplified C# parsing
        for line in source.lines() {
            let trimmed = line.trim();
            
            if trimmed.contains("class ") {
                if let Some(class) = self.parse_class(line, context) {
                    items.push(class);
                }
            } else if trimmed.contains("void ") || trimmed.contains("async ") || trimmed.ends_with(')') {
                if let Some(func) = self.parse_method(line, context) {
                    items.push(func);
                }
            }
        }
        
        let root = AstNode::Module {
            name: "root".to_string(),
            items,
        };
        
        Ok(UniversalAst {
            root,
            metadata: AstMetadata {
                source_language: Some("csharp".to_string()),
                line_count: source.lines().count(),
                ..Default::default()
            },
        })
    }
    
    fn parse_class(&self, line: &str, context: &mut TransmutationContext) -> Option<AstNode> {
        // Extract class name from "public class ClassName"
        let parts: Vec<&str> = line.split_whitespace().collect();
        let class_idx = parts.iter().position(|&p| p == "class")?;
        let name = parts.get(class_idx + 1)?.trim_end_matches('{').to_string();
        
        context.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Type,
            scope: "root".to_string(),
            ty: Some("class".to_string()),
        });
        
        Some(AstNode::Class {
            name,
            fields: vec![],
            methods: vec![],
            base_class: None,
            visibility: if line.contains("public") {
                Visibility::Public
            } else {
                Visibility::Internal
            },
        })
    }
    
    fn parse_method(&self, line: &str, context: &mut TransmutationContext) -> Option<AstNode> {
        let parts: Vec<&str> = line.split('(').collect();
        if parts.is_empty() {
            return None;
        }
        
        let signature: Vec<&str> = parts[0].split_whitespace().collect();
        let name = signature.last()?.to_string();
        
        context.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Function,
            scope: "root".to_string(),
            ty: None,
        });
        
        let is_async = line.contains("async");
        
        Some(AstNode::Function {
            name,
            params: vec![],
            return_type: None,
            body: vec![],
            is_async,
            visibility: if line.contains("public") {
                Visibility::Public
            } else if line.contains("private") {
                Visibility::Private
            } else {
                Visibility::Internal
            },
        })
    }
}

impl Default for CSharpParser {
    fn default() -> Self {
        Self::new()
    }
}