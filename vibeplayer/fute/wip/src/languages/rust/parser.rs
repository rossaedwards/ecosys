//! Rust AST to Universal AST converter

use anyhow::Result;
use syn::{File as SynFile, Item, ItemFn, Visibility as SynVis};
use crate::{
    ast::*,
    core::context::{TransmutationContext, Symbol, SymbolKind},
};

pub struct RustParser;

impl RustParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn convert_to_universal(&self, syntax_tree: &SynFile, context: &mut TransmutationContext) -> Result<UniversalAst> {
        let mut items = Vec::new();
        
        for item in &syntax_tree.items {
            if let Some(node) = self.convert_item(item, context) {
                items.push(node);
            }
        }
        
        let root = AstNode::Module {
            name: "root".to_string(),
            items,
        };
        
        Ok(UniversalAst {
            root,
            metadata: AstMetadata {
                source_language: Some("rust".to_string()),
                line_count: syntax_tree.items.len(),
                ..Default::default()
            },
        })
    }
    
    fn convert_item(&self, item: &Item, context: &mut TransmutationContext) -> Option<AstNode> {
        match item {
            Item::Fn(func) => Some(self.convert_function(func, context)),
            Item::Struct(s) => Some(self.convert_struct(s, context)),
            Item::Enum(e) => Some(self.convert_enum(e, context)),
            Item::Mod(m) => Some(self.convert_module(m, context)),
            _ => None,
        }
    }
    
    fn convert_function(&self, func: &ItemFn, context: &mut TransmutationContext) -> AstNode {
        let name = func.sig.ident.to_string();
        
        // Add to symbol table
        context.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Function,
            scope: "root".to_string(),
            ty: None,
        });
        
        let params = func.sig.inputs.iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    Some(Parameter {
                        name: quote::quote!(#pat_type.pat).to_string(),
                        ty: Type::Unknown, // TODO: Convert Rust type
                        default: None,
                    })
                } else {
                    None
                }
            })
            .collect();
        
        let is_async = func.sig.asyncness.is_some();
        let visibility = self.convert_visibility(&func.vis);
        
        AstNode::Function {
            name,
            params,
            return_type: None, // TODO: Convert return type
            body: vec![], // TODO: Convert body
            is_async,
            visibility,
        }
    }
    
    fn convert_struct(&self, s: &syn::ItemStruct, context: &mut TransmutationContext) -> AstNode {
        let name = s.ident.to_string();
        
        context.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Type,
            scope: "root".to_string(),
            ty: Some("struct".to_string()),
        });
        
        let fields = s.fields.iter()
            .filter_map(|f| {
                f.ident.as_ref().map(|ident| Field {
                    name: ident.to_string(),
                    ty: Type::Unknown,
                    visibility: self.convert_visibility(&f.vis),
                })
            })
            .collect();
        
        AstNode::Struct {
            name,
            fields,
            visibility: self.convert_visibility(&s.vis),
        }
    }
    
    fn convert_enum(&self, e: &syn::ItemEnum, context: &mut TransmutationContext) -> AstNode {
        let name = e.ident.to_string();
        
        context.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Type,
            scope: "root".to_string(),
            ty: Some("enum".to_string()),
        });
        
        let variants = e.variants.iter()
            .map(|v| EnumVariant {
                name: v.ident.to_string(),
                fields: None, // TODO: Convert variant fields
            })
            .collect();
        
        AstNode::Enum {
            name,
            variants,
            visibility: self.convert_visibility(&e.vis),
        }
    }
    
    fn convert_module(&self, m: &syn::ItemMod, _context: &mut TransmutationContext) -> AstNode {
        let name = m.ident.to_string();
        
        AstNode::Module {
            name,
            items: vec![], // TODO: Convert module contents
        }
    }
    
    fn convert_visibility(&self, vis: &SynVis) -> Visibility {
        match vis {
            SynVis::Public(_) => Visibility::Public,
            SynVis::Restricted(_) => Visibility::Internal,
            SynVis::Inherited => Visibility::Private,
        }
    }
}

impl Default for RustParser {
    fn default() -> Self {
        Self::new()
    }
}