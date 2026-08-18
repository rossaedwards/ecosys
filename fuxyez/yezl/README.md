# Yezl - Fuxyez Language Library

**Yezl** (pronounced "yes-el") is the symbiosis layer connecting Fuxyez to other programming languages. Each subdirectory contains FFI bridges, bindings, and integration code for specific languages.

## Structure

- **python/** - Python FFI (.fuxpy support)
- **rust/** - Rust FFI (.fuxrs support)
- **javascript/** - JavaScript/Node.js FFI (.fuxjs support)
- **csharp/** - C# .NET FFI
- **webassembly/** - WASM compilation target
- **go/** - Go FFI
- **cpp/** - C++ FFI
- **java/** - Java JNI bindings
- **elixir/** - Elixir/Erlang BEAM FFI

## Philosophy

Yezl enables **true language symbiosis** - not just FFI, but deep integration where Fuxyez code can seamlessly invoke and be invoked by other languages while maintaining the mystical semantics of Fuxyez.

## Example: Python Integration

\\\uxyez
// file.fuxpy
import numpy as np

sigil analyze_data {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0]
    let arr = np.array(data)
    let mean = np.mean(arr)
    echo "Mean: {mean}"
}
\\\

The \.fuxpy\ extension automatically loads Python runtime and bindings from \yezl/python/\.

## Adding New Language Support

1. Create directory: \yezl/[language]/\
2. Implement FFI bridge in Rust
3. Add grammar support to compiler
4. Create example programs
5. Document in language-specific README
