# Getting Started with Fuxyez

Welcome to Fuxyez, where code collapses into reality.

## Installation

\\\ash
# Clone the repository
git clone https://github.com/aurphyx/fuxyez.git
cd fuxyez

# Build the compiler
cd fuxyez_compiler
cargo build --release

# Add to PATH
export PATH=$PATH:$(pwd)/target/release
\\\

## Your First Fuxyez Program

Create \hello.fux\:

\\\uxyez
sigil greet {
    echo "Hello, Fuxyez!"
}

ritual main {
    collapse greet
}
\\\

Run it:

\\\ash
fuxyez run hello.fux
\\\

## Next Steps

- Read the [Language Reference](LANGUAGE_REFERENCE.md)
- Explore [Examples](../examples/)
- Learn about [Extensions](EXTENSION_GUIDE.md)
