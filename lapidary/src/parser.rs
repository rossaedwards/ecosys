//! Universal Manifest Parser & Lexer (Lapidary Parser)
//!
//! Compliant with Symbiotic Xessability Standards.
//! This module ingests dynamic, loosely-typed VS Code extension manifests 
//! (package.json) and lexically maps them into a strict, highly-structured 
//! `UniversalAst`. It handles semantic region detection, capability extraction,
//! and context telemetry buffering.

use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::collections::HashMap;

use crate::context::{LapidaryContext, Dependency};
use crate::fute::ast::{
    AstNode, UniversalAst, ConfigValue, ActivationType, ExecutionProfile
};

/// The primary ingestion engine for Lapidary.
/// Translates raw IDE-specific JSON schemas into a Universal Abstract Syntax Tree.
pub struct ManifestParser;

impl ManifestParser {
    /// Initializes a pristine parser matrix.
    pub fn new() -> Self {
        Self {}
    }

    /// The core entry point for parsing. 
    /// Converts a raw manifest string into a fully typed AST, populating the context.
    pub fn parse_manifest(&self, raw_json: &str, ctx: &mut LapidaryContext) -> Result<UniversalAst> {
        ctx.log_info("Parser", "Lexing raw VS Code manifest payload...");

        let root_val: Value = serde_json::from_str(raw_json)
            .context("Failed to lex source payload: Malformed JSON structure.")?;

        if !root_val.is_object() {
            bail!("Manifest root must be a JSON object.");
        }

        // 1. Extract Core Metadata
        let name = self.extract_string(&root_val, "name").unwrap_or_else(|| "unknown_extension".to_string());
        let version = self.extract_string(&root_val, "version").unwrap_or_else(|| "0.0.0".to_string());
        let publisher = self.extract_string(&root_val, "publisher").unwrap_or_else(|| "unknown_publisher".to_string());
        let description = self.extract_string(&root_val, "description").unwrap_or_default();

        ctx.set_meta("name", &name);
        ctx.set_meta("version", &version);
        ctx.set_meta("publisher", &publisher);

        // 2. Scan External Dependencies
        self.parse_dependencies(&root_val, ctx);

        // 3. Tokenize Structural Children
        let mut ast_children = Vec::new();

        // 3a. Detect Execution Targets (Main Entry Points)
        if let Some(entry_node) = self.parse_execution_target(&root_val, ctx) {
            ast_children.push(entry_node);
        } else {
            ctx.log_warning("Parser", "No primary execution target ('main') found in manifest.");
        }

        // 3b. Parse Contribution Points (UI Bloat & Configurations)
        if let Some(contributes) = root_val.get("contributes") {
            self.parse_contributions(contributes, &mut ast_children, ctx);
        }

        // 3c. Parse Activation Events
        if let Some(activations) = root_val.get("activationEvents") {
            self.parse_activation_events(activations, &mut ast_children, ctx);
        }

        ctx.log_info("Parser", format!("AST Lexing complete. Discovered {} structural nodes.", ast_children.len()));

        let extension_root = AstNode::ExtensionBundle {
            name,
            version,
            publisher,
            description,
            children: ast_children,
        };

        Ok(UniversalAst::new(extension_root))
    }

    // ════════════════════════════════════════════════════════════════════════
    // LEXICAL EXTRACTION SUB-ROUTINES
    // ════════════════════════════════════════════════════════════════════════

    /// Identifies whether the primary extension entry point is a Quantum (Binary) 
    /// or Lattice (Node.js script) execution target.
    fn parse_execution_target(&self, root_val: &Value, ctx: &mut LapidaryContext) -> Option<AstNode> {
        let main_script = self.extract_string(root_val, "main")?;

        if main_script.ends_with(".js") || main_script.ends_with(".ts") {
            ctx.log_info("Parser", format!("Detected LatticeScript target: {}", main_script));
            
            // Extract potential node dependencies if we have a dependencies block
            let mut deps = Vec::new();
            if let Some(deps_val) = root_val.get("dependencies").and_then(|v| v.as_object()) {
                deps.extend(deps_val.keys().cloned());
            }

            Some(AstNode::LatticeScript {
                entry_point: main_script,
                dependencies: deps,
            })
        } else {
            ctx.log_info("Parser", format!("Detected QuantumBinary target: {}", main_script));
            
            Some(AstNode::QuantumBinary {
                executable_path: main_script,
                execution_args: vec!["--stdio".to_string()], // Standard LSP assumption
                profile: ExecutionProfile::default(),
            })
        }
    }

    /// Maps the `contributes` matrix into distinct semantic nodes.
    fn parse_contributions(&self, contributes: &Value, children: &mut Vec<AstNode>, ctx: &mut LapidaryContext) {
        // Map Configuration Schemas
        if let Some(configs) = contributes.get("configuration") {
            let mut properties_map = HashMap::new();
            
            // Handle both single config objects and arrays of config objects
            let config_array = if configs.is_array() {
                configs.as_array().unwrap().clone()
            } else {
                vec![configs.clone()]
            };

            for config_block in config_array {
                if let Some(props) = config_block.get("properties").and_then(|v| v.as_object()) {
                    for (key, val) in props {
                        if let Some(default_val) = val.get("default") {
                            properties_map.insert(key.clone(), self.json_to_config_value(default_val));
                        }
                    }
                }
            }

            if !properties_map.is_empty() {
                children.push(AstNode::ConfigurationBlock { properties: properties_map });
            }
        }

        // Detect IDE-specific UI Bloat (Views, ViewContainers, Menus, Commands)
        let bloat_categories = vec!["views", "viewsContainers", "menus", "commands", "colors"];
        for category in bloat_categories {
            if let Some(bloat_val) = contributes.get(category) {
                let weight = if bloat_val.is_array() {
                    bloat_val.as_array().unwrap().len() as u32 * 1000
                } else {
                    2000
                };

                ctx.log_info("Parser", format!("Flagged {} UI elements in '{}' category as WebviewBloat.", weight / 1000, category));
                
                children.push(AstNode::WebviewBloat {
                    view_id: format!("legacy_{}", category),
                    bloat_weight: weight,
                });
            }
        }
    }

    /// Maps IDE activation triggers into agnostic `ActivationEvent` nodes.
    fn parse_activation_events(&self, activations: &Value, children: &mut Vec<AstNode>, _ctx: &mut LapidaryContext) {
        if let Some(events) = activations.as_array() {
            for event_val in events {
                if let Some(event_str) = event_val.as_str() {
                    let parts: Vec<&str> = event_str.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let event_type = match parts[0] {
                            "onLanguage" => ActivationType::OnLanguage,
                            "onCommand" => ActivationType::OnCommand,
                            "workspaceContains" => ActivationType::WorkspaceContains,
                            _ => ActivationType::OnStartup,
                        };
                        children.push(AstNode::ActivationEvent {
                            event_type,
                            target: parts[1].to_string(),
                        });
                    } else if event_str == "*" {
                        children.push(AstNode::ActivationEvent {
                            event_type: ActivationType::OnStartup,
                            target: "*".to_string(),
                        });
                    }
                }
            }
        }
    }

    /// Extracts structural dependencies directly into the context matrix.
    fn parse_dependencies(&self, root_val: &Value, ctx: &mut LapidaryContext) {
        if let Some(deps) = root_val.get("extensionDependencies").and_then(|v| v.as_array()) {
            for dep in deps {
                if let Some(dep_name) = dep.as_str() {
                    ctx.register_dependency(Dependency {
                        name: dep_name.to_string(),
                        version_req: "*".to_string(),
                        is_runtime_critical: true,
                    });
                }
            }
        }
    }

    // ════════════════════════════════════════════════════════════════════════
    // UTILITY TRANSLATORS
    // ════════════════════════════════════════════════════════════════════════

    /// Safely extracts a string field from a JSON object.
    fn extract_string(&self, val: &Value, key: &str) -> Option<String> {
        val.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
    }

    /// Converts raw JSON defaults into strongly-typed `ConfigValue` parameters.
    fn json_to_config_value(&self, val: &Value) -> ConfigValue {
        match val {
            Value::String(s) => ConfigValue::String(s.clone()),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    ConfigValue::Integer(i)
                } else {
                    // Fallback for floats in config, storing as string for Lapce safety
                    ConfigValue::String(n.to_string())
                }
            }
            Value::Bool(b) => ConfigValue::Boolean(*b),
            Value::Array(a) => {
                let mut strings = Vec::new();
                for item in a {
                    if let Some(s) = item.as_str() {
                        strings.push(s.to_string());
                    }
                }
                ConfigValue::Array(strings)
            }
            _ => ConfigValue::String(val.to_string()),
        }
    }
}

impl Default for ManifestParser {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIT TESTS (Validation & Integrity)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexing_quantum_binary_target() {
        let json = r#"{
            "name": "gopls-wrapper",
            "version": "1.0.0",
            "publisher": "lapidary-test",
            "main": "bin/gopls"
        }"#;

        let parser = ManifestParser::new();
        let mut ctx = LapidaryContext::new();
        let ast = parser.parse_manifest(json, &mut ctx).unwrap();

        if let AstNode::ExtensionBundle { children, .. } = ast.root {
            assert_eq!(children.len(), 1);
            assert!(matches!(children[0], AstNode::QuantumBinary { .. }));
        } else {
            panic!("Expected ExtensionBundle root");
        }
    }

    #[test]
    fn test_lexing_ui_bloat_extraction() {
        let json = r#"{
            "name": "bloaty-mcbloatface",
            "main": "dist/index.js",
            "contributes": {
                "views": [{ "id": "explorer" }, { "id": "debug" }],
                "commands": [{ "command": "do.thing" }]
            }
        }"#;

        let parser = ManifestParser::new();
        let mut ctx = LapidaryContext::new();
        let ast = parser.parse_manifest(json, &mut ctx).unwrap();

        if let AstNode::ExtensionBundle { children, .. } = ast.root {
            // Should contain 1 LatticeScript (main) and 2 WebviewBloat nodes (views, commands)
            assert_eq!(children.len(), 3);
            let bloat_count = children.iter().filter(|c| matches!(c, AstNode::WebviewBloat { .. })).count();
            assert_eq!(bloat_count, 2);
        }
    }
}