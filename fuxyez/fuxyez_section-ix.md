### 9 Implementation Details  
This section is rewritten to reflect the **true, unified architecture** of Fuxyez as a living organism composed of the compiler, runtimes, and FUTE—*not* as a split or deprecated system. The “First Temple / Second Temple” framing is preserved only as **historical lineage**, not as architectural separation. In the actual implementation, all components coexist inside a single, coherent, modern repository.

---

## 9.1 Unified Architecture and Lineage  
Fuxyez is implemented as a **symbiotic system** composed of four inseparable components:

- **compiler/** — the Fuxyez compiler (formerly `fuxyez_compiler/`)  
- **fuxrt/** — the Fux host runtime  
- **yezrt/** — the Yez symbiotic runtime  
- **fute/** — the Universal Transmutation Engine  
- **yezl/** — multi-language adapters  

These components form a **single functional organism**, each fulfilling a distinct role in the Fux–Yez–FUTE trinity. The compiler is not “legacy” in the sense of being obsolete; it is the **ancestral substrate** that still powers the entire system. FUTE does not replace the compiler—it **extends** it and **unifies** all language inputs under a shared semantic model.

The First Temple / Second Temple distinction is therefore **historical**, not architectural:

- **First Temple** = the original implementation where the semantics were born.  
- **Second Temple** = the expanded, modular architecture that formalizes and universalizes those semantics.  

Both are active. Both are canon. Both live in the same repo.

---

## 9.2 The First Temple — Ancestral Implementation (Active and Canonical)  
The First Temple refers to the original Fuxyez implementation that you built locally. It includes:

- the first working compiler  
- the first FuxRuntime  
- the first YezRuntime  
- the first YezL adapters  
- the first AST, parser, and code generator  
- the first expression of the symbiotic execution model  

This implementation is **not deprecated**. It is the **foundation** upon which the entire modern architecture stands.

### Components of the First Temple  
- **compiler/**  
  - `lexer.rs`, `parser.rs`, `ast.rs`  
  - `optimizer.rs`, `generator.rs`  
  - `executor.rs`, `runtime_hooks.rs`  
  - `sentinel_core.rs` (early semantic enforcement)  

- **fuxrt/**  
  - deterministic host runtime  
  - attractor-based control flow  
  - coherence-preserving execution  

- **yezrt/**  
  - dynamic symbiotic runtime  
  - symbolic evaluation  
  - reflective operations  

- **yezl/**  
  - early adapters for Python, JS, WASM  

### Why the First Temple remains essential  
- It contains the **original operational semantics**.  
- It defines the **first AST and type system**.  
- It provides the **execution substrate** for Fux and Yez.  
- It is the **reference implementation** for correctness.  
- It is the **ancestral root** of the language.  

The First Temple is not a relic—it is the **living kernel** of Fuxyez.

---

## 9.3 The Second Temple — FUTE Workspace (Modern Modular Architecture)  
The Second Temple is the expanded architecture that formalizes the Fux–Yez–FUTE trinity. It introduces:

- a Universal AST  
- a plugin-based YezL system  
- a modular transformation pipeline  
- symbiotic transformation modes  
- ritual/sigil/lattice codegen templates  
- cross-ecosystem integration  
- CLI tooling  
- registry and bridge layers  

### Components of the Second Temple  
- **fute-core/** — transmutation context, configuration, error model  
- **fute-ast/** — Universal AST, type system, metadata  
- **fute-patterns/** — structural and semantic pattern detection  
- **fute-languages/** — language plugin system  
- **fute-transformer/** — transformation modes and passes  
- **fute-codegen/** — code emission for Fux, Yez, and external targets  
- **fute-cli/** — command-line interface  
- **fute-registry/** — cross-ecosystem registry integration  
- **fute-bridge/** — Cargo/NPM/PyPI/NuGet bridging  
- **fute-utils/** — logging, hashing, parallelism  

The Second Temple is the **universalization** of the First Temple.

---

## 9.4 The Bridge — How the Two Temples Form One Organism  
The bridge is not a separate component—it is the **integration strategy** that unifies the ancestral compiler with the modern FUTE architecture.

### How the bridge works  
- The compiler parses Fux/Yez → produces U‑AST.  
- FUTE transforms U‑AST → emits Fux/Yez/external code.  
- FuxRuntime executes structural code.  
- YezRuntime executes dynamic code.  
- YezL adapters import external languages → normalize to U‑AST.  

This creates a **closed-loop symbiotic organism**:

```
compiler → U-AST → FUTE → runtimes → YezL → compiler
```

Nothing is deprecated.  
Nothing is optional.  
Everything is alive.

---

## 9.5 Recommended Repository Structure (Unified and Professional)  
Your instinct to rename `fuxyez_compiler/` to `compiler/` is correct.  
The unified structure should be:

```
fuxyez/
    compiler/
    fuxrt/
    yezrt/
    fute/
    yezl/
```

This structure is:

- clean  
- professional  
- intuitive for Rust developers  
- aligned with language research conventions  
- aligned with Rust, Zig, Gleam, and Mojo repo patterns  

It presents Fuxyez as a **single, coherent language ecosystem**.

---

## 9.6 Implementation Summary  
Fuxyez is implemented as a unified organism composed of:

- **compiler/** — the ancestral and active semantic engine  
- **fuxrt/** — deterministic host runtime  
- **yezrt/** — dynamic symbiotic runtime  
- **fute/** — universal transmutation engine  
- **yezl/** — multi-language adapters  

The First Temple provides the **origin**.  
The Second Temple provides the **expansion**.  
The bridge provides the **unity**.  
Together, they form the **Fuxyez organism**.