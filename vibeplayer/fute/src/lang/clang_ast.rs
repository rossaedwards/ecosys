//! libclang-backed C/C++ AST extraction for FUTE.
//!
//! Feature: `clang-ast` (system: clang-devel / libclang).
//! Only emits entities from the **main file** (not system headers).

use clang::{Clang, Entity, EntityKind, Index, TypeKind};
use std::collections::HashSet;
use std::path::Path;

use super::cpp_to_rust::{
    map_type_name_for_export, to_pascal_export, LangTransmuteReport, SourceLang, TargetLang,
};

/// Try to build a Rust scaffold from libclang AST. Returns None if clang cannot load/parse
/// or the main file yields nothing useful (caller falls back to structural).
pub fn try_transmute_with_clang(
    source_path: &Path,
    source_text: &str,
    is_cpp: bool,
) -> Option<(String, LangTransmuteReport)> {
    let clang = Clang::new().ok()?;
    let index = Index::new(&clang, false, false);

    let std_flag = if is_cpp { "-std=c++17" } else { "-std=c11" };
    let lang = if is_cpp { "c++" } else { "c" };

    // Parent dir as -I so local includes resolve
    let parent = source_path
        .parent()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| ".".into());
    let include_flag = format!("-I{parent}");
    // Also try sibling vap/ for VAV headers
    let vap_inc = source_path
        .parent()
        .and_then(|p| p.parent())
        .map(|p| format!("-I{}/vap", p.display()))
        .unwrap_or_default();

    let mut args = vec![
        "-x".to_string(),
        lang.to_string(),
        std_flag.to_string(),
        "-Wno-everything".to_string(),
        include_flag,
    ];
    if !vap_inc.is_empty() {
        args.push(vap_inc);
    }

    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let tu = index
        .parser(source_path)
        .arguments(&arg_refs)
        .parse()
        .ok()?;

    let mut structs = Vec::new();
    let mut functions = Vec::new();
    let mut seen_struct = HashSet::new();
    let mut seen_fn = HashSet::new();

    walk(
        tu.get_entity(),
        &mut structs,
        &mut functions,
        &mut seen_struct,
        &mut seen_fn,
    );

    if structs.is_empty() && functions.is_empty() {
        return None;
    }

    let mut out = String::new();
    out.push_str("//! Transmuted by v01d (FUTE) — C/C++ → Rust\n");
    out.push_str("//! Backend: libclang AST (main-file only)\n");
    out.push_str(&format!("//! Origin: {}\n\n", source_path.display()));
    out.push_str("#![allow(dead_code, non_snake_case, unused_variables)]\n\n");

    for s in &structs {
        out.push_str(s);
        out.push('\n');
    }
    for f in &functions {
        out.push_str(f);
        out.push('\n');
    }

    let report = LangTransmuteReport {
        from: if is_cpp {
            SourceLang::Cpp
        } else {
            SourceLang::C
        },
        to: TargetLang::Rust,
        source_lines: source_text.lines().count(),
        output_lines: out.lines().count(),
        structs: structs.len(),
        functions: functions.len(),
        includes: 0,
        notes: vec![
            "libclang AST backend (main-file only)".into(),
            format!("-I {parent}"),
            "Method bodies: todo! stubs — polish in target crate".into(),
        ],
    };

    Some((out, report))
}

fn is_main_file(entity: &Entity<'_>) -> bool {
    entity
        .get_location()
        .map(|loc| loc.is_in_main_file())
        .unwrap_or(false)
}

fn walk(
    entity: Entity<'_>,
    structs: &mut Vec<String>,
    functions: &mut Vec<String>,
    seen_struct: &mut HashSet<String>,
    seen_fn: &mut HashSet<String>,
) {
    // Only emit from the translation unit's main file
    let main = is_main_file(&entity);

    match entity.get_kind() {
        EntityKind::StructDecl | EntityKind::ClassDecl if main => {
            if let Some(s) = emit_struct(&entity) {
                // key by rust name
                let key = s.lines().find(|l| l.contains("pub struct")).unwrap_or("").to_string();
                if !key.is_empty() && seen_struct.insert(key) {
                    structs.push(s);
                }
            }
        }
        EntityKind::TypedefDecl if main => {
            // typedef struct { ... } name_t; — may re-export the struct name
            if let Some(s) = emit_typedef_struct(&entity) {
                let key = s.lines().find(|l| l.contains("pub struct")).unwrap_or("").to_string();
                if !key.is_empty() && seen_struct.insert(key) {
                    structs.push(s);
                }
            }
        }
        EntityKind::FunctionDecl if main => {
            if let Some(f) = emit_function(&entity) {
                if let Some(name) = entity.get_name() {
                    if seen_fn.insert(name) {
                        functions.push(f);
                    }
                }
            }
        }
        EntityKind::Method | EntityKind::Constructor if main => {
            if let Some(f) = emit_function(&entity) {
                let name = entity.get_name().unwrap_or_default();
                if !name.is_empty() && seen_fn.insert(name) {
                    functions.push(f);
                }
            }
        }
        _ => {}
    }

    for child in entity.get_children() {
        walk(child, structs, functions, seen_struct, seen_fn);
    }
}

fn emit_typedef_struct(entity: &Entity<'_>) -> Option<String> {
    let name = entity.get_name()?;
    // underlying type may be a record
    let ty = entity.get_type()?;
    let canon = ty.get_canonical_type();
    if canon.get_kind() != TypeKind::Record {
        return None;
    }
    // Find the struct declaration entity
    let decl = canon.get_declaration()?;
    emit_struct_named(&decl, &name)
}

fn emit_struct(entity: &Entity<'_>) -> Option<String> {
    let name = entity.get_name().filter(|n| !n.is_empty() && !n.contains('('))?;
    // Skip anonymous display names
    if name.contains("unnamed") || name.contains("anonymous") {
        return None;
    }
    emit_struct_named(entity, &name)
}

fn emit_struct_named(entity: &Entity<'_>, name: &str) -> Option<String> {
    let rust_name = to_pascal_export(name.trim_end_matches("_t"));
    let mut s = String::new();
    s.push_str("#[derive(Debug, Clone)]\n");
    s.push_str(&format!("pub struct {rust_name} {{\n"));
    let mut fields = 0;
    for child in entity.get_children() {
        if child.get_kind() != EntityKind::FieldDecl {
            continue;
        }
        let fname = child
            .get_name()
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| format!("field_{fields}"));
        let ty = child
            .get_type()
            .map(|t| map_clang_type(&t))
            .unwrap_or_else(|| "()".into());
        s.push_str(&format!("    pub {}: {},\n", sanitize_ident(&fname), ty));
        fields += 1;
    }
    s.push_str("}\n");
    if fields == 0 {
        return None;
    }
    Some(s)
}

fn emit_function(entity: &Entity<'_>) -> Option<String> {
    let name = entity.get_name()?;
    if name.is_empty() || name.starts_with('_') {
        return None;
    }
    // skip operators
    if name.starts_with("operator") {
        return None;
    }

    let ret = entity
        .get_result_type()
        .map(|t| map_clang_type(&t))
        .unwrap_or_else(|| "()".into());
    let ret = if ret == "()" {
        String::new()
    } else {
        format!(" -> {ret}")
    };

    let args = entity.get_arguments().unwrap_or_default();
    let mut params = Vec::new();
    for (i, arg) in args.into_iter().enumerate() {
        let pname = arg
            .get_name()
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| format!("arg{i}"));
        let ty = arg
            .get_type()
            .map(|t| map_clang_type(&t))
            .unwrap_or_else(|| "()".into());
        params.push(format!("{}: {}", sanitize_ident(&pname), ty));
    }

    Some(format!(
        "pub fn {}({}){ret} {{\n    todo!(\"FUTE scaffold — polish body\")\n}}\n",
        sanitize_ident(&name),
        params.join(", ")
    ))
}

fn map_clang_type(t: &clang::Type<'_>) -> String {
    match t.get_kind() {
        TypeKind::Void => "()".into(),
        TypeKind::Bool => "bool".into(),
        TypeKind::CharU | TypeKind::UChar | TypeKind::CharS | TypeKind::SChar => "u8".into(),
        TypeKind::UShort => "u16".into(),
        TypeKind::UInt => "u32".into(),
        TypeKind::ULong | TypeKind::ULongLong => "u64".into(),
        TypeKind::Short => "i16".into(),
        TypeKind::Int => "i32".into(),
        TypeKind::Long | TypeKind::LongLong => "i64".into(),
        TypeKind::Float => "f32".into(),
        TypeKind::Double | TypeKind::LongDouble => "f64".into(),
        // typedefs like uint32_t often show as elaborated/typedef — handled below
        TypeKind::Pointer => {
            if let Some(pointee) = t.get_pointee_type() {
                let inner = map_clang_type(&pointee);
                let display = t.get_display_name();
                if display.contains("const") {
                    format!("&{inner}")
                } else {
                    format!("&mut {inner}")
                }
            } else {
                "*mut ()".into()
            }
        }
        TypeKind::ConstantArray | TypeKind::IncompleteArray | TypeKind::VariableArray => {
            if let Some(elem) = t.get_element_type() {
                let inner = map_clang_type(&elem);
                if let Some(n) = t.get_size() {
                    format!("[{inner}; {n}]")
                } else {
                    format!("Vec<{inner}>")
                }
            } else {
                "()".into()
            }
        }
        TypeKind::Record | TypeKind::Typedef | TypeKind::Elaborated => {
            let name = t
                .get_declaration()
                .and_then(|d| d.get_name())
                .unwrap_or_else(|| t.get_display_name());
            // strip "struct " prefix
            let name = name.trim_start_matches("struct ").trim_start_matches("class ");
            if name.contains("unnamed") || name.contains("anonymous") {
                return "() /* anonymous */".into();
            }
            map_type_name_for_export(name.trim_end_matches("_t"))
        }
        _ => {
            let d = t.get_display_name();
            if d.contains('*') {
                let inner = d.replace("const ", "").replace('*', "").trim().to_string();
                format!("&mut {}", map_type_name_for_export(&inner))
            } else {
                map_type_name_for_export(&d)
            }
        }
    }
}

fn sanitize_ident(s: &str) -> String {
    let s = s.replace('-', "_").replace(' ', "_");
    match s.as_str() {
        "type" | "ref" | "mut" | "fn" | "mod" | "impl" | "self" | "Self" | "box" | "move" => {
            format!("{s}_")
        }
        _ => s,
    }
}

/// Probe whether libclang is loadable on this machine.
pub fn clang_available() -> bool {
    Clang::new().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn clang_parses_simple_header() {
        if !clang_available() {
            return;
        }
        let dir = std::env::temp_dir().join("fute_clang_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("sample.h");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(
            f,
            r#"
typedef struct {{
    float bpm_raw;
    int count;
    char key[8];
}} vap_sample_t;

void vap_init(vap_sample_t *v);
float vap_get_bpm(const vap_sample_t *v);
"#
        )
        .unwrap();

        let src = std::fs::read_to_string(&path).unwrap();
        let (out, rep) = try_transmute_with_clang(&path, &src, false).expect("clang transmute");
        assert!(
            out.contains("pub struct VapSample") || out.contains("VapSample"),
            "out was:\n{out}"
        );
        assert!(rep.structs >= 1 || rep.functions >= 1, "{rep:?}");
        assert!(out.contains("pub fn vap_init") || out.contains("vap_init"));
    }
}
