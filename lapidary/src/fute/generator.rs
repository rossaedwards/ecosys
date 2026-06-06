//! Fuxyez Compiler Backend & Artifact Synthesizer
//! Compliant with Symbiotic Xessability Standards.

use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::context::LapidaryContext;
use crate::fute::ast::{AstNode, UniversalAst, ConfigValue};

pub struct VoltSynthesizer {
    output_directory: std::path::PathBuf,
}

impl VoltSynthesizer {
    pub fn new(output_directory: impl AsRef<Path>) -> Self {
        Self {
            output_directory: output_directory.as_ref().to_path_buf(),
        }
    }

    // UPDATED: Signature now accepts &mut self and &mut ctx to allow telemetry logging
    pub fn synthesize_artifacts(&mut self, ast: &UniversalAst, ctx: &mut LapidaryContext) -> Result<()> {
        let (name, version, publisher, description, nodes) = match &ast.root {
            AstNode::ExtensionBundle { name, version, publisher, description, children } => {
                (name, version, publisher, description, children)
            }
            _ => anyhow::bail!("CRITICAL: AST root is not an ExtensionBundle."),
        };

        let safe_plugin_name = name.to_lowercase().replace(' ', "-");
        let target_bundle_dir = self.output_directory.join(&safe_plugin_name);
        let bin_dir = target_bundle_dir.join("bin");

        fs::create_dir_all(&bin_dir)?;

        let mut manifest_buffer = String::new();
        self.build_manifest_meta_block(&mut manifest_buffer, name, version, publisher, description);

        let mut lsp_hook_generated = false;

        for node in nodes {
            match node {
                AstNode::WasmProxyShim { target_script, memory_pages } => {
                    self.generate_wasm_shim(&bin_dir, target_script, *memory_pages)?;
                    manifest_buffer.push_str("\n[lsp]\n");
                    manifest_buffer.push_str(&format!("command = [\"node\", \"{}\", \"--stdio\"]\n", target_script));
                    lsp_hook_generated = true;
                }
                AstNode::QuantumBinary { executable_path, execution_args, .. } => {
                    manifest_buffer.push_str("\n[lsp]\n");
                    manifest_buffer.push_str(&format!("command = [\"{}\"", executable_path));
                    for arg in execution_args {
                        manifest_buffer.push_str(&format!(", \"{}\"", arg));
                    }
                    manifest_buffer.push_str("]\n");
                    lsp_hook_generated = true;
                }
                AstNode::ConfigurationBlock { properties } => {
                    self.build_manifest_config_block(&mut manifest_buffer, properties);
                }
                AstNode::WebviewBloat { view_id, .. } => {
                    ctx.log_warning("Synthesis", format!("Ghost UI Node detected: {}", view_id));
                }
                _ => {}
            }
        }

        if !lsp_hook_generated {
            ctx.log_warning("Synthesis", "No LSP hooks generated.");
        }

        let manifest_path = target_bundle_dir.join("volt.toml");
        let mut manifest_file = File::create(&manifest_path)?;
        manifest_file.write_all(manifest_buffer.as_bytes())?;
        
        Ok(())
    }

    fn build_manifest_meta_block(&self, buffer: &mut String, name: &str, version: &str, publisher: &str, description: &str) {
        buffer.push_str("[meta]\n");
        buffer.push_str(&format!("name = \"{}\"\nversion = \"{}\"\nauthor = \"{}\"\ndisplay-name = \"{}\"\n", 
            name.to_lowercase().replace(' ', "-"), version, publisher, name));
        buffer.push_str(&format!("description = \"{}\"\nwasm = \"bin/lapidary-shim.wasm\"\n", 
            description.replace('"', "\\\"")));
    }

    fn build_manifest_config_block(&self, buffer: &mut String, props: &std::collections::HashMap<String, ConfigValue>) {
        buffer.push_str("\n[config]\n");
        for (key, val) in props {
            match val {
                ConfigValue::String(s) => buffer.push_str(&format!("{} = \"{}\"\n", key, s)),
                ConfigValue::Boolean(b) => buffer.push_str(&format!("{} = {}\n", key, b)),
                ConfigValue::Integer(i) => buffer.push_str(&format!("{} = {}\n", key, i)),
                _ => {}
            }
        }
    }

    fn generate_wasm_shim(&self, bin_dir: &Path, target_script: &str, memory_pages: u32) -> Result<()> {
        let wat = format!("(module (memory (export \"memory\") {}) (func (export \"_start\") nop))", memory_pages);
        let mut f = File::create(bin_dir.join("lapidary-shim.wat"))?;
        f.write_all(wat.as_bytes())?;
        Ok(())
    }
}