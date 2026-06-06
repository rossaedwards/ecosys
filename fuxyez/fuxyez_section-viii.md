### 8 Tooling and Developer Experience  
Fuxyez provides a tooling ecosystem designed to make the symbiotic model practical for real developers while remaining analyzable for language researchers. The tools emphasize clarity, reproducibility, and multi‑language workflows, enabling developers to move fluidly between Fux, Yez, and external ecosystems.

---

### CLI commands for real-world workflows  
The command-line interface is implemented in the `fute-cli` crate and exposes the full transmutation pipeline. Each command corresponds to a stage in the Universal AST lifecycle and is designed to be predictable, scriptable, and reproducible.

- **`transmute` — Convert source code into the Universal AST and emit a target artifact.**  
  This is the core operation: Python → U‑AST → Fux, JS → U‑AST → WASM, Rust → U‑AST → Yez, etc.

- **`harmonize` — Normalize and reconcile multi-language modules.**  
  Useful for polyglot projects where Python, Rust, and JS modules must share types or control flow.

- **`weave` — Combine multiple U‑ASTs into a single coherent program.**  
  Enables distributed or modular transmutation.

- **`collapse` — Reduce symbolic constructs into deterministic Fux equivalents.**  
  Used when migrating dynamic Yez code into structural Fux code.

- **`divine` — Apply symbolic inference and ritual semantics.**  
  This is the most expressive mode, used for research, symbolic computation, and advanced transformations.

- **`inspect` — Display the U‑AST, metadata, and transformation traces.**  
  Essential for debugging and academic analysis.

- **`fmt` — Format Fux and Yez code using canonical templates.**  
  Ensures consistency across teams and runtimes.

These commands form a complete workflow for building, analyzing, and deploying symbiotic programs.

---

### Editor and IDE support  
Fuxyez is designed to integrate with modern development environments through:

- **Language Server Protocol (LSP)** for syntax highlighting, autocompletion, and inline diagnostics.  
- **AST visualization tools** that display the Universal AST in real time.  
- **Transformation trace viewers** that show how code evolves through FUTE passes.  
- **Inline YezL adapters** that allow Python, JS, or WASM blocks to be edited with native syntax highlighting.

This tooling makes the dual‑runtime model approachable for everyday developers.

---

### Testing and debugging  
Fuxyez includes a unified testing framework that supports:

- **structural tests** for Fux code  
- **dynamic tests** for Yez code  
- **cross‑language tests** for YezL adapters  
- **transmutation tests** that verify semantic continuity across transformations  

Debugging is supported through:

- **coherence trace logs**  
- **symbolic evaluation traces**  
- **U‑AST diffing tools**  
- **runtime introspection APIs**  

These tools allow developers to reason about both structural and symbolic behavior.

---

### Package and ecosystem integration  
Fuxyez integrates with major package ecosystems through the FUTE bridge layer:

- **Cargo** for Rust  
- **NPM** for JavaScript  
- **PyPI** for Python  
- **NuGet** for C#  

This enables developers to import external libraries, transmute them into the Universal AST, and execute them symbiotically within the Fux–Yez runtime.

---

### Developer experience summary  
The tooling ecosystem is designed to support:

- polyglot development  
- reproducible transformations  
- symbolic and structural debugging  
- cross‑ecosystem integration  
- research‑grade analysis  

This makes Fuxyez practical for real-world engineering while remaining grounded in formal language theory.