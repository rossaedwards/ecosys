# v01d (FUTE) — Fuxyez Universal Transmutation Engine

Pronounced **“Void”**. Host-agnostic / symbiotic packaging engine used by **Vibe Media Player**.

Full experimental tree preserved under `wip/`. This package is the **compilable** v01d surface VMP depends on today.

## CLI

```bash
cargo run -p fute --bin v01d -- version
cargo run -p fute --bin v01d -- transmute ./plugin.vsix ./plugin.volt
cargo run -p fute --bin v01d -- bind vinyl-vibez

# C/C++ → Rust language transmute
cargo run -p fute --bin v01d -- lang path/to/file.c -o out.rs --from c
cargo run -p fute --bin v01d -- lang path/to/file.cpp -o out.rs --from cpp
cargo run -p fute --bin v01d -- detect path/to/file.hpp
```

### Language pairs (current)

| From | To | Status |
|------|-----|--------|
| C (`.c`, `.h`) | Rust | structural scaffold ✅ |
| C++ (`.cpp`, `.hpp`, …) | Rust | structural scaffold ✅ |
| C/C++ + **libclang** | Rust | AST backend via `--features clang-ast` |

```bash
# Fedora: install toolchain FUTE plugs into
sudo dnf install -y clang clang-devel llvm-devel gcc-c++ cmake ninja-build

# Build FUTE with AST backend
cargo test -p fute --features clang-ast
cargo run -p fute --bin v01d --features clang-ast -- version
```

See [`docs/FUTE_CPP_TOOLCHAIN.md`](../docs/FUTE_CPP_TOOLCHAIN.md).

Raw scaffolds land with markers; production ports (`vmp-viz`, `vmp-vinyl`) polish ownership.

## Modes

- `StandaloneHost` — Tauri desktop VMP  
- `GuestEmbed` — VLC / foreign host guest  
- `PluginHost` — CLAP/VST host  
- `WasmShell` — browser  
- `VinylVibez` — Mixxx-class DJ surface  
