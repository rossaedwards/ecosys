//! C / C++ → Rust structural transmutation (v01d / FUTE).
//!
//! Maps surface grammar into idiomatic Rust scaffolding. Complex macros and
//! ownership patterns are marked for polish. Used to port Vibe Audio Visualizer
//! (C) into VMP `vmp-viz` (Rust).

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLang {
    C,
    Cpp,
    Rust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetLang {
    Rust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangTransmuteReport {
    pub from: SourceLang,
    pub to: TargetLang,
    pub source_lines: usize,
    pub output_lines: usize,
    pub structs: usize,
    pub functions: usize,
    pub includes: usize,
    pub notes: Vec<String>,
}

/// Transmute C or C++ source text into a Rust module body.
pub fn transmute_c_cpp_to_rust(source: &str, path_hint: &Path) -> (String, LangTransmuteReport) {
    let mut notes = vec![
        "FUTE/v01d C|C++ → Rust structural transmute".into(),
        format!("source: {}", path_hint.display()),
        "Review // [transmute] markers for ownership polish".into(),
    ];

    let mut out = String::new();
    out.push_str("//! Transmuted by v01d (FUTE) — C/C++ → Rust\n");
    out.push_str(&format!("//! Origin: {}\n", path_hint.display()));
    out.push_str("//! Engine: Fuxyez Universal Transmutation Engine\n\n");
    out.push_str("#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]\n\n");

    let mut structs = 0usize;
    let mut functions = 0usize;
    let mut includes = 0usize;

    let cleaned = strip_c_comments(source);
    let lines: Vec<&str> = cleaned.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            out.push('\n');
            i += 1;
            continue;
        }

        if line.starts_with("#include") {
            includes += 1;
            let inc = line.trim_start_matches("#include").trim();
            let name = inc.trim_matches(|c| c == '"' || c == '<' || c == '>');
            let stem = Path::new(name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(name)
                .replace('-', "_");
            if inc.starts_with('"') {
                out.push_str(&format!("// use crate::{stem}; // from #include \"{name}\"\n"));
            } else {
                out.push_str(&format!("// system include: {name}\n"));
            }
            i += 1;
            continue;
        }

        if line.starts_with('#') {
            out.push_str(&format!("// {line}\n"));
            i += 1;
            continue;
        }

        if line.starts_with("typedef struct") {
            let (block, next) = take_braced_block(&lines, i);
            i = next;
            match transmute_typedef_struct(&block) {
                Some(rs) => {
                    structs += 1;
                    out.push_str(&rs);
                    out.push('\n');
                }
                None => {
                    notes.push(format!("unparsed typedef struct near L{i}"));
                    out.push_str(&format!("/* unparsed typedef struct\n{block}*/\n"));
                }
            }
            continue;
        }

        if line.starts_with("struct ") && (line.contains('{') || peek_brace(&lines, i)) {
            let (block, next) = take_braced_block(&lines, i);
            i = next;
            match transmute_named_struct(&block) {
                Some(rs) => {
                    structs += 1;
                    out.push_str(&rs);
                    out.push('\n');
                }
                None => out.push_str(&format!("/* unparsed struct\n{block}*/\n")),
            }
            continue;
        }

        if looks_like_function(line) {
            let (block, next, is_def) = take_function(&lines, i);
            i = next;
            functions += 1;
            out.push_str(&transmute_function(&block, is_def));
            out.push('\n');
            continue;
        }

        out.push_str(&map_c_line_to_rust(line));
        out.push('\n');
        i += 1;
    }

    let from = match path_hint.extension().and_then(|e| e.to_str()) {
        Some("cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx") => SourceLang::Cpp,
        _ => SourceLang::C,
    };

    let report = LangTransmuteReport {
        from,
        to: TargetLang::Rust,
        source_lines: source.lines().count(),
        output_lines: out.lines().count(),
        structs,
        functions,
        includes,
        notes,
    };
    (out, report)
}

fn peek_brace(lines: &[&str], i: usize) -> bool {
    lines.get(i + 1).map(|l| l.trim().starts_with('{')).unwrap_or(false)
}

fn strip_c_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let b = src.as_bytes();
    let mut i = 0;
    let mut line = false;
    let mut block = false;
    while i < b.len() {
        if line {
            out.push(b[i] as char);
            if b[i] == b'\n' {
                line = false;
            }
            i += 1;
            continue;
        }
        if block {
            if b[i] == b'*' && i + 1 < b.len() && b[i + 1] == b'/' {
                block = false;
                i += 2;
                out.push(' ');
                continue;
            }
            if b[i] == b'\n' {
                out.push('\n');
            }
            i += 1;
            continue;
        }
        if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
            line = true;
            out.push_str("//");
            i += 2;
            continue;
        }
        if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
            block = true;
            i += 2;
            continue;
        }
        out.push(b[i] as char);
        i += 1;
    }
    out
}

fn take_braced_block(lines: &[&str], start: usize) -> (String, usize) {
    let mut buf = String::new();
    let mut depth = 0i32;
    let mut seen = false;
    let mut i = start;
    while i < lines.len() {
        let l = lines[i];
        buf.push_str(l);
        buf.push('\n');
        for c in l.chars() {
            match c {
                '{' => {
                    depth += 1;
                    seen = true;
                }
                '}' => depth -= 1,
                _ => {}
            }
        }
        i += 1;
        if seen && depth <= 0 {
            break;
        }
    }
    (buf, i)
}

fn take_function(lines: &[&str], start: usize) -> (String, usize, bool) {
    let first = lines[start].trim();
    if first.ends_with(';') && !first.contains('{') {
        return (first.to_string(), start + 1, false);
    }
    let (block, next) = take_braced_block(lines, start);
    (block, next, true)
}

fn looks_like_function(line: &str) -> bool {
    if line.starts_with("typedef")
        || line.starts_with("struct")
        || line.starts_with('#')
        || line.starts_with("if ")
        || line.starts_with("for ")
        || line.starts_with("while ")
        || line.starts_with("switch ")
        || line.starts_with("return ")
        || line.starts_with("else")
    {
        return false;
    }
    let has_paren = line.contains('(') && line.contains(')');
    let type_like = [
        "void ", "int ", "float ", "double ", "static ", "unsigned ", "uint", "char ", "bool ",
        "size_t ", "const ",
    ]
    .iter()
    .any(|p| line.starts_with(p));
    has_paren && type_like
}

fn transmute_typedef_struct(block: &str) -> Option<String> {
    let close = block.rfind('}')?;
    let after = block[close + 1..].trim().trim_end_matches(';').trim();
    let name = if after.is_empty() {
        "AnonymousStruct"
    } else {
        after
    };
    let body = extract_body(block)?;
    Some(format_rust_struct(name, &body))
}

fn transmute_named_struct(block: &str) -> Option<String> {
    let first = block.lines().next()?.trim();
    let name = first
        .trim_start_matches("struct ")
        .split(|c: char| c == '{' || c.is_whitespace())
        .next()?
        .trim();
    let body = extract_body(block)?;
    Some(format_rust_struct(name, &body))
}

fn extract_body(block: &str) -> Option<String> {
    let open = block.find('{')?;
    let close = block.rfind('}')?;
    Some(block[open + 1..close].to_string())
}

fn format_rust_struct(name: &str, body: &str) -> String {
    let rust_name = to_pascal_case(name.trim_end_matches("_t"));
    let mut s = String::new();
    s.push_str("#[derive(Debug, Clone)]\n");
    s.push_str(&format!("pub struct {rust_name} {{\n"));
    for line in body.lines() {
        let t = line.trim().trim_end_matches(';').trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("//") {
            s.push_str(&format!("    {t}\n"));
            continue;
        }
        if let Some((ty, field)) = split_c_decl(t) {
            let rty = map_c_type(&ty);
            let fname = field.trim_start_matches('*').trim();
            if let Some((base, len)) = parse_array_field(fname) {
                s.push_str(&format!(
                    "    pub {}: [{}; {}],\n",
                    to_snake_safe(base),
                    rty,
                    len
                ));
            } else {
                s.push_str(&format!("    pub {}: {},\n", to_snake_safe(fname), rty));
            }
        } else {
            s.push_str(&format!("    // unparsed field: {t}\n"));
        }
    }
    s.push_str("}\n");
    s
}

fn transmute_function(block: &str, is_def: bool) -> String {
    let header = block.lines().next().unwrap_or("").trim().trim_end_matches('{').trim();
    let header = header.trim_end_matches(';').trim();
    // strip static inline
    let header = header
        .trim_start_matches("static ")
        .trim_start_matches("inline ")
        .trim_start_matches("static ");

    let open = match header.find('(') {
        Some(i) => i,
        None => return format!("// [transmute] unparsed fn: {header}\n"),
    };
    let close = match header.rfind(')') {
        Some(i) => i,
        None => return format!("// [transmute] unparsed fn: {header}\n"),
    };
    let before = header[..open].trim();
    let params_c = &header[open + 1..close];
    let mut parts = before.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        return format!("// [transmute] empty fn header\n");
    }
    let name = parts.pop().unwrap().trim_start_matches('*');
    let ret_c = parts.join(" ");
    let ret = if ret_c == "void" || ret_c.is_empty() {
        String::new()
    } else {
        format!(" -> {}", map_c_type(&ret_c))
    };

    let mut rust_params = Vec::new();
    if !params_c.trim().is_empty() && params_c.trim() != "void" {
        for p in params_c.split(',') {
            let p = p.trim();
            if p.is_empty() {
                continue;
            }
            if let Some((ty, name)) = split_c_decl(p) {
                let is_ptr = ty.contains('*') || name.starts_with('*');
                let base = map_c_type(&ty.replace('*', ""));
                let n = to_snake_safe(name.trim_start_matches('*'));
                if is_ptr {
                    if ty.contains("const") {
                        rust_params.push(format!("{n}: &[{base}] /* [transmute] ptr→slice */"));
                    } else {
                        rust_params.push(format!("{n}: &mut {base}"));
                    }
                } else {
                    rust_params.push(format!("{n}: {base}"));
                }
            } else {
                rust_params.push(format!("/* {p} */"));
            }
        }
    }

    let mut s = String::new();
    if !is_def {
        s.push_str(&format!(
            "pub fn {name}({}){ret}; // declaration\n",
            rust_params.join(", ")
        ));
        return s;
    }

    s.push_str(&format!(
        "pub fn {name}({}){ret} {{\n",
        rust_params.join(", ")
    ));
    // body
    if let Some(body) = extract_body(block) {
        for line in body.lines() {
            let t = line.trim();
            if t.is_empty() {
                s.push('\n');
                continue;
            }
            s.push_str("    ");
            s.push_str(&map_c_line_to_rust(t));
            s.push('\n');
        }
    }
    s.push_str("}\n");
    s
}

fn map_c_line_to_rust(line: &str) -> String {
    let mut l = line.to_string();
    // common replacements
    l = l.replace("NULL", "None /* was NULL */");
    l = l.replace("true", "true");
    l = l.replace("false", "false");
    l = l.replace("->", ".");
    // float suffixes
    l = regex_replace_float_suffix(&l);
    // math
    l = l.replace("sqrtf(", "(");
    l = l.replace("fminf(", "(");
    l = l.replace("fmaxf(", "(");
    l = l.replace("fabsf(", "(");
    l = l.replace("sinf(", "(");
    l = l.replace("cosf(", "(");
    // for loops
    if l.starts_with("for (") || l.starts_with("for(") {
        if let Some(mapped) = map_for_loop(&l) {
            return mapped;
        }
        return format!("// [transmute] {l}");
    }
    if l.starts_with("if (") || l.starts_with("if(") {
        let inner = l
            .trim_start_matches("if")
            .trim()
            .trim_start_matches('(')
            .trim_end_matches('{')
            .trim()
            .trim_end_matches(')')
            .trim();
        return format!("if {inner} {{");
    }
    if l.starts_with("while (") || l.starts_with("while(") {
        let inner = l
            .trim_start_matches("while")
            .trim()
            .trim_start_matches('(')
            .trim_end_matches('{')
            .trim()
            .trim_end_matches(')')
            .trim();
        return format!("while {inner} {{");
    }
    if l.starts_with("return ") {
        return l.trim_end_matches(';').to_string() + ";";
    }
    if l == "}" || l == "{" {
        return l.to_string();
    }
    // strip trailing semicolon for expression statements that become statements
    if l.ends_with(';') {
        return l;
    }
    l
}

fn map_for_loop(line: &str) -> Option<String> {
    // for (int i = 0; i < n; ++i) {
    let inner = line
        .trim()
        .trim_start_matches("for")
        .trim()
        .trim_start_matches('(');
    let end = inner.find(')')?;
    let parts: Vec<&str> = inner[..end].split(';').map(|s| s.trim()).collect();
    if parts.len() != 3 {
        return None;
    }
    // init: int i = 0
    let init = parts[0];
    let var = init
        .split_whitespace()
        .rev()
        .nth(2)
        .or_else(|| init.split('=').next())?
        .trim()
        .trim_end_matches('=')
        .split_whitespace()
        .last()?;
    // cond: i < n
    let cond = parts[1];
    if let Some(rest) = cond.strip_prefix(&format!("{var} < ")) {
        return Some(format!("for {var} in 0..({rest}) {{"));
    }
    if let Some(rest) = cond.strip_prefix(&format!("{var}<=")) {
        return Some(format!("for {var} in 0..=({rest}) {{"));
    }
    None
}

fn regex_replace_float_suffix(s: &str) -> String {
    // 1.0f / 0.5f / 180.0f → 1.0 / 0.5 / 180.0
    let mut out = String::with_capacity(s.len());
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            let start = i;
            while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
                i += 1;
            }
            out.push_str(&s[start..i]);
            if i < b.len() && (b[i] == b'f' || b[i] == b'F') {
                // skip f suffix if previous was a number
                if start < i {
                    i += 1;
                    continue;
                }
            }
            continue;
        }
        out.push(b[i] as char);
        i += 1;
    }
    out
}

fn split_c_decl(decl: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = decl.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    let name = parts[parts.len() - 1].to_string();
    let ty = parts[..parts.len() - 1].join(" ");
    Some((ty, name))
}

fn parse_array_field(name: &str) -> Option<(&str, usize)> {
    let open = name.find('[')?;
    let close = name.find(']')?;
    let base = &name[..open];
    let len: usize = name[open + 1..close].parse().ok()?;
    Some((base, len))
}

fn map_c_type(ty: &str) -> String {
    map_type_name_for_export(ty)
}

/// Shared with clang-ast backend.
pub fn map_type_name_for_export(ty: &str) -> String {
    let t = ty
        .replace("const ", "")
        .replace("static ", "")
        .replace("unsigned ", "u")
        .replace('*', "")
        .trim()
        .to_string();
    match t.as_str() {
        "void" => "()".into(),
        "float" => "f32".into(),
        "double" => "f64".into(),
        "int" | "int32_t" => "i32".into(),
        "uint32_t" | "u32" | "Uint32" | "uint32" => "u32".into(),
        "uint64_t" | "Uint64" => "u64".into(),
        "int64_t" => "i64".into(),
        "size_t" | "Size" | "std::size_t" => "usize".into(),
        "char" | "Char" => "u8".into(),
        "bool" | "_Bool" | "Bool" => "bool".into(),
        "uint8_t" => "u8".into(),
        "int8_t" => "i8".into(),
        "uint16_t" => "u16".into(),
        "int16_t" => "i16".into(),
        other if other.ends_with("_t") => to_pascal_case(other.trim_end_matches("_t")),
        other => to_pascal_case(other),
    }
}

/// Shared with clang-ast backend.
pub fn to_pascal_export(s: &str) -> String {
    to_pascal_case(s)
}

fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn to_snake_safe(s: &str) -> String {
    let s = s.replace('-', "_");
    if s == "type" || s == "ref" || s == "mut" || s == "fn" || s == "mod" {
        format!("{s}_")
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn struct_and_fn_transmute() {
        let src = r#"
#include "vap_runtime.h"
typedef struct {
    float bpm_raw;
    int spatial_width;
    char key[8];
} vap_runtime_t;

static float compute_centroid(const float *mag, int n, int sr) {
    float num = 0.0f;
    for (int i = 1; i < n; ++i) {
        num += mag[i];
    }
    return num;
}
"#;
        let (out, rep) = transmute_c_cpp_to_rust(src, Path::new("test.c"));
        assert!(out.contains("pub struct VapRuntime"));
        assert!(out.contains("pub bpm_raw: f32"));
        assert!(out.contains("pub key: [u8; 8]"));
        assert!(out.contains("pub fn compute_centroid"));
        assert!(rep.structs >= 1);
        assert!(rep.functions >= 1);
    }
}
