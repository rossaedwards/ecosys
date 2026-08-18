# Rust Yezl Bridge

Rust integration for Fuxyez enabling \.fuxrs\ files.

## Features

- Call Rust functions with C ABI
- Zero-copy data sharing where possible
- Full type safety with compile-time checks
- Async/await integration

## Usage

\\\ust
// rust_lib/src/lib.rs
#[no_mangle]
pub extern "C" fn calculate_fibonacci(n: i32) -> u64 {
    // Rust implementation
}
\\\

\\\uxyez
// bridge.fuxrs
import rust_lib::calculate_fibonacci

sigil fib_demo {
    let result = calculate_fibonacci(10)
    echo "Fib(10) = {result}"
}
\\\

## Configuration

Use mirrored \.srxuf\ config files to specify Rust crate paths and linking options.
