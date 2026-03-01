### 5 Architecture Overview  
This section presents the structural architecture of the Fuxyez ecosystem in a formal, research‑grade manner. It describes the runtime model, the Universal AST, the transformation pipeline, and the code‑generation subsystems that together constitute the operational core of the language. The architecture is intentionally modular: each subsystem is independently analyzable yet semantically interdependent, reflecting the symbiotic design principles established earlier.

---

### 5.1 Runtime Architecture  
The Fuxyez runtime architecture consists of three cooperating subsystems: **FuxRuntime**, **YezRuntime**, and **FUTE Core**. Each subsystem fulfills a distinct semantic role while participating in a unified execution model.

- **FuxRuntime** provides the deterministic substrate for structural execution. It implements attractor‑based control flow, coherence‑preserving state transitions, and type‑sound evaluation of Universal AST nodes.  
- **YezRuntime** provides dynamic evaluation, symbolic computation, and multi‑language embedding. It is optimized for reflective operations and dynamic dispatch while remaining constrained by the invariants enforced by the host runtime.  
- **FUTE Core** mediates between the two runtimes by normalizing source programs into a shared intermediate representation and coordinating the execution of transmuted artifacts.

The interaction between these subsystems forms a closed semantic loop in which static and dynamic semantics coexist without compromising safety or expressiveness. This dual‑runtime model is a defining characteristic of the language.

---

### 5.2 Universal AST  
The Universal AST (U‑AST) is the central intermediate representation used throughout the Fuxyez ecosystem. It is designed to be:

- **language‑agnostic**, supporting frontends for Python, JavaScript, Rust, C#, WASM, and others  
- **structurally expressive**, capturing control flow, type information, and symbolic metadata  
- **semantically stable**, enabling reproducible transformations and deterministic code generation  
- **extensible**, supporting domain‑specific constructs and ritual semantics  

The U‑AST is implemented in the `fute-ast` crate and includes:

- a node hierarchy representing expressions, statements, declarations, and symbolic constructs  
- a type system that supports structural, nominal, and symbiotic types  
- metadata channels for source mapping, annotations, and ritual semantics  
- validation and traversal mechanisms for static analysis and transformation passes  

The U‑AST serves as the canonical representation for all transmutation operations.

---

### 5.3 Symbiotic Transformation Modes  
FUTE implements a multi‑stage transformation pipeline that converts source programs into executable artifacts. This pipeline is parameterized by **symbiotic modes**, each representing a distinct balance between structural strictness and symbolic flexibility:

- **Standard Mode**: conservative transformations that preserve structural semantics with minimal symbolic inference.  
- **Sacred Mode**: transformations that incorporate ritual semantics and symbolic annotations while maintaining type safety.  
- **Mystical Mode**: exploratory transformations that permit symbolic inference, pattern‑based restructuring, and non‑local optimizations.  
- **Resonant Mode**: transformations that align structural and symbolic semantics through coherence‑based heuristics.

These modes are implemented in the `fute-transformer` crate and provide developers with fine‑grained control over the transmutation process.

---

### 5.4 Code Generation  
The code‑generation subsystem is responsible for emitting executable artifacts from the U‑AST. It is implemented in the `fute-codegen` crate and includes:

- **Fux codegen**, which emits artifacts optimized for the FuxRuntime  
- **Yez codegen**, which emits dynamic constructs for the YezRuntime  
- **external codegen**, which emits artifacts for WASM, Rust, JavaScript, and other ecosystems  
- **ritual templates**, which encode symbolic constructs and semantic intent  
- **sigil templates**, which encode structural invariants and coherence constraints  
- **lattice templates**, which encode multi‑node or distributed execution patterns  

The code‑generation subsystem ensures that transmuted programs retain semantic continuity across runtimes and ecosystems.

---

### 5.5 Multi‑Ecosystem Integration  
Fuxyez is designed to operate across heterogeneous programming ecosystems through the YezL adapter layer. Each adapter provides:

- a parser for the source language  
- a mapping to the U‑AST  
- a type‑mapping layer  
- optional code‑generation backends  

This architecture enables Fuxyez to serve as a universal intermediary, harmonizing programs written in different languages under a single semantic framework.

---

### 5.6 Architectural Summary  
The Fuxyez architecture is defined by five core components:

- a dual‑runtime execution model  
- a universal intermediate representation  
- a multi‑mode transformation pipeline  
- a modular code‑generation subsystem  
- a multi‑ecosystem adapter layer  

Together, these components form a coherent, extensible, and semantically rigorous language architecture suitable for research, high‑assurance systems, and cross‑ecosystem development.