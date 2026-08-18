# Fuxyez Examples

This directory contains example programs demonstrating Fuxyez's features and extension system.

## Examples

### hello_world
Basic Fuxyez program demonstrating sigils and rituals.

\\\ash
fuxyez run examples/hello_world/main.fux
\\\

### rust_ffi
Rust FFI integration example showing how to call Rust functions from Fuxyez.

### python_script
Python integration example using NumPy and Pandas.

### oracle_query
Yezian meta-script demonstrating oracle queries and divination.

## Running Examples

\\\ash
# Pure Fuxyez
fuxyez run examples/hello_world/main.fux

# With Rust FFI (requires Rust toolchain)
cd examples/rust_ffi/rust_lib && cargo build --release
fuxyez run examples/rust_ffi/bridge.fuxrs

# With Python (requires Python 3.10+)
fuxyez run examples/python_script/data_process.fuxpy

# Oracle query
fuxyez run examples/oracle_query/prophecy.yez
\\\
