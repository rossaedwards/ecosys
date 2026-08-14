//! Language-level transmutation (source → source) for v01d / FUTE.
//!
//! Package packs (vsix→volt) live in the crate root; this module handles
//! **code** symbionts — notably C/C++ → Rust for host-agnostic ports.
//!
//! - Default: structural (regex/line) transmute — always available
//! - Optional: `clang-ast` feature — libclang AST (needs clang-devel)

mod cpp_to_rust;

#[cfg(feature = "clang-ast")]
mod clang_ast;

pub use cpp_to_rust::{
    map_type_name_for_export, to_pascal_export, transmute_c_cpp_to_rust, LangTransmuteReport,
    SourceLang, TargetLang,
};

use crate::{FuteError, Result};
use std::path::Path;

/// Transmute a source file from one language to another on disk.
///
/// Prefers libclang when built with `--features clang-ast` and libclang loads;
/// otherwise uses structural C/C++ → Rust rules.
pub fn transmute_source_file(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    from: SourceLang,
    to: TargetLang,
) -> Result<LangTransmuteReport> {
    let source = source.as_ref();
    let target = target.as_ref();
    let code = std::fs::read_to_string(source)?;

    let (out, report) = match (from, to) {
        (SourceLang::C | SourceLang::Cpp, TargetLang::Rust) => {
            #[cfg(feature = "clang-ast")]
            {
                let is_cpp = matches!(from, SourceLang::Cpp)
                    || source
                        .extension()
                        .and_then(|e| e.to_str())
                        .map(|e| matches!(e, "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx"))
                        .unwrap_or(false);
                if let Some(pair) = clang_ast::try_transmute_with_clang(source, &code, is_cpp) {
                    pair
                } else {
                    let mut r = transmute_c_cpp_to_rust(&code, source);
                    r.1.notes
                        .push("clang-ast unavailable or empty TU — used structural backend".into());
                    r
                }
            }
            #[cfg(not(feature = "clang-ast"))]
            {
                transmute_c_cpp_to_rust(&code, source)
            }
        }
        _ => {
            return Err(FuteError::Transmute(format!(
                "unsupported language pair {:?} → {:?}",
                from, to
            )))
        }
    };

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(target, out)?;
    Ok(report)
}

/// Infer source language from extension.
pub fn detect_source_lang(path: &Path) -> Option<SourceLang> {
    match path
        .extension()
        .and_then(|e| e.to_str())?
        .to_ascii_lowercase()
        .as_str()
    {
        "c" | "h" => Some(SourceLang::C),
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => Some(SourceLang::Cpp),
        "rs" => Some(SourceLang::Rust),
        _ => None,
    }
}

/// Whether this build linked the clang-ast feature (not whether libclang loads).
pub fn clang_ast_feature_enabled() -> bool {
    cfg!(feature = "clang-ast")
}

/// Whether libclang can be loaded at runtime (only with `clang-ast` feature).
pub fn clang_runtime_available() -> bool {
    #[cfg(feature = "clang-ast")]
    {
        clang_ast::clang_available()
    }
    #[cfg(not(feature = "clang-ast"))]
    {
        false
    }
}
