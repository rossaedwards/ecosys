# Fuxyez Extension System Guide

## Overview

Fuxyez supports multiple paradigms through a flexible extension system. Each extension anchors a specific programming paradigm while maintaining core Fuxyez semantics.

## Primary Extensions

### .fux - Pure Fuxyez
Pure language implementation featuring:
- Collapse Rituals
- Pure Sigils
- Lattice operations
- Thread weaving

**Example:**
\\\uxyez
sigil ignite {
    echo "The fire awakens."
}
\\\

### .fuxrs - Rust Symbiosis
Systems programming with Rust FFI integration.

**Example:**
\\\uxyez
import rust_math::fast_sqrt

sigil calculate {
    let result = fast_sqrt(144.0)
    echo result
}
\\\

### .fuxpy - Python Symbiosis
Data science and scripting integration.

**Example:**
\\\uxyez
import numpy as np

sigil analyze {
    let data = np.array([1, 2, 3])
    echo np.mean(data)
}
\\\

### .yez / .yz - Yezian Meta
Declarative meta-programming and oracle queries.

**Example:**
\\\uxyez
oracle DataSource {
    source: "database://prod"
}

query GetUser(id: i32) -> User {
    divine DataSource {
        sql: "SELECT * FROM users WHERE id = {id}"
    }
}
\\\

## Mirrored Configuration Files

Each primary file can have a mirrored config with inverted extension:

| Primary | Mirror | Purpose |
|---------|--------|---------|
| main.fux | main.xuf | Runtime settings, optimization |
| lib.fuxrs | lib.srxuf | FFI mappings, type conversions |
| oracle.yez | oracle.zey | Query schemas, cache settings |

## Creating Custom Extensions

See \uxyez.toml\ for extension registration format.
