//! Language Plugin System

pub mod traits;
pub mod rust;
pub mod python;
pub mod javascript;
pub mod csharp;
pub mod wasm;

use anyhow::{Result, anyhow};
use traits::LanguagePlugin;
use std::path::Path;

/// Detect language from file extension
pub fn detect_language(path: &Path) -> Result<String> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| anyhow!("No file extension found"))?;
    
    match ext {
        "rs" => Ok("rust".to_string()),
        "py" => Ok("python".to_string()),
        "js" | "mjs" | "jsx" => Ok("javascript".to_string()),
        "ts" | "tsx" => Ok("typescript".to_string()),
        "cs" => Ok("csharp".to_string()),
        "go" => Ok("go".to_string()),
        "cpp" | "cc" | "cxx" => Ok("cpp".to_string()),
        "wasm" | "wat" => Ok("wasm".to_string()),
        _ => Err(anyhow!("Unsupported file extension: {}", ext)),
    }
}

/// Load language plugin by name
pub fn load_plugin(lang: &str) -> Result<Box<dyn LanguagePlugin>> {
    match lang.to_lowercase().as_str() {
        "rust" => Ok(Box::new(rust::RustPlugin::new())),
        "python" | "py" => Ok(Box::new(python::PythonPlugin::new())),
        "javascript" | "js" => Ok(Box::new(javascript::JavaScriptPlugin::new())),
        "csharp" | "c#" | "cs" => Ok(Box::new(csharp::CSharpPlugin::new())),
        "wasm" | "webassembly" => Ok(Box::new(wasm::WasmPlugin::new())),
        _ => Err(anyhow!("Unsupported language: {}", lang)),
    }
}

/// Get list of all supported languages
pub fn supported_languages() -> Vec<&'static str> {
    vec![
        "rust",
        "python",
        "javascript",
        "csharp",
        "webassembly",
    ]
}