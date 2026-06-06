### 6 Language Specification  
This section uses a **hybrid style**: formal enough for programming‑language researchers, but clear and accessible for Rust ecosystem developers. It defines the syntax, semantics, type system, and operational behavior of Fuxyez across the Fux–Yez–FUTE trinity.

---

## 6.1 Syntax  
Fuxyez syntax is defined as a **two‑layer grammar**:  
- **Fux syntax** for structural, deterministic constructs.  
- **Yez syntax** for dynamic, symbolic, and embedded multi‑language constructs.

Both layers compile into the Universal AST (U‑AST), but they differ in expressiveness and constraints.

#### Fux Syntax  
Fux syntax emphasizes structural clarity, explicit control flow, and type stability. It resembles a blend of Rust‑like determinism with a more declarative, lattice‑oriented structure.

Key characteristics include:  
- explicit block structure  
- deterministic evaluation order  
- typed bindings  
- attractor‑based control constructs  
- sigil‑annotated declarations for ritual semantics

Example (illustrative only):  
```
let x: Int = 5;
sigil @align {
    let y = x * 2;
}
```

#### Yez Syntax  
Yez syntax is intentionally flexible and symbolic. It supports dynamic constructs, reflective operations, and embedded multi‑language fragments.

Characteristics include:  
- dynamic binding  
- symbolic expressions  
- inline YezL blocks (e.g., Python, JS, WASM)  
- ritual annotations for transformation modes

Example:  
```
yez {
    python: """
    def add(a, b): return a + b
    """
}
```

---

## 6.2 Semantics  
Fuxyez semantics are defined by the **dual‑runtime model** and the **universal transmutation pipeline**.

### Structural Semantics (Fux)  
Fux semantics enforce:  
- deterministic evaluation  
- type‑sound execution  
- coherence‑preserving state transitions  
- attractor‑based control flow (a formalization of stable execution patterns)

Fux semantics are strict: violations result in compile‑time or transmutation‑time errors.

### Dynamic Semantics (Yez)  
Yez semantics support:  
- dynamic dispatch  
- symbolic evaluation  
- runtime reflection  
- multi‑language embedding via YezL

Yez semantics are flexible but bounded by the host runtime. Dynamic constructs cannot violate Fux invariants.

### Transmutation Semantics (FUTE)  
FUTE defines how programs move between languages and runtimes. Its semantics include:  
- normalization into U‑AST  
- structural and symbolic pattern matching  
- mode‑dependent transformation rules  
- deterministic code emission

The semantics guarantee **intent preservation** across transformations.

---

## 6.3 Type System  
The Fuxyez type system is **hybrid**: structural types for Fux, symbolic types for Yez, and universal types for cross‑language transmutation.

### Structural Types (Fux)  
These include:  
- primitive types (Int, Float, Bool, etc.)  
- composite types (Struct, Enum, Tuple)  
- lattice types for distributed constructs  
- sigil‑annotated types for ritual semantics

Structural types must be fully resolved at transmutation time.

### Symbolic Types (Yez)  
Symbolic types allow:  
- dynamic typing  
- reflective type queries  
- symbolic unions  
- multi‑language type mapping (via YezL)

Symbolic types are resolved lazily, but must be compatible with Fux invariants.

### Universal Types (FUTE)  
Universal types unify structural and symbolic types into a single representation. They support:  
- cross‑language type mapping  
- type inference during normalization  
- type preservation across transformations  
- compatibility checks for multi‑runtime execution

The Universal Type System is implemented in `fute-ast/types.rs`.

---

## 6.4 Operational Semantics  
Operational semantics describe how programs execute across the Fux–Yez–FUTE trinity.

### FuxRuntime Execution  
FuxRuntime executes U‑AST nodes deterministically.  
Key properties:  
- small‑step operational semantics  
- attractor‑based control transitions  
- coherence‑preserving memory model  
- strict error propagation

### YezRuntime Execution  
YezRuntime executes symbolic and dynamic constructs.  
Key properties:  
- dynamic dispatch  
- reflective evaluation  
- multi‑language embedding  
- cooperative scheduling with FuxRuntime

### FUTE Execution  
FUTE does not execute programs directly; it **transmutes** them.  
Its operational semantics define:  
- normalization rules  
- transformation passes  
- mode‑dependent rewriting  
- code emission strategies

Together, these define the **symbiotic execution model**.

---

## 6.5 Error Model  
Fuxyez uses a **three‑tier error model**:

- **Structural Errors** (Fux): type mismatches, invariant violations, coherence errors.  
- **Symbolic Errors** (Yez): unresolved symbols, dynamic dispatch failures.  
- **Transmutation Errors** (FUTE): normalization failures, unsupported constructs, mode conflicts.

Errors are represented uniformly in the U‑AST metadata, enabling cross‑runtime debugging.

---

## 6.6 Summary  
The Fuxyez language specification defines:  
- a dual‑layer syntax  
- a hybrid semantic model  
- a universal type system  
- a multi‑runtime operational model  
- a unified error system  

This hybrid specification supports both formal analysis and practical implementation, making Fuxyez suitable for research, high‑assurance systems, and multi‑ecosystem development.