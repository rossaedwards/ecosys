# AuraFS Contribution & Governance Standards

## 1. The S.A.G.E.S. Protocol
All code modifications to `src/gov/` or `src/core/` must be validated against the 13 Sentinel AI Guardians.
- **Vyrellix Compliance:** Initial verification of logic gates.
- **Archivus Logging:** Every change must be double-verified and logged to the Ineffable Ledger.

## 2. Physics Compliance
- **No Magic Numbers:** All constants MUST be imported from `crate::physics`.
- **Theorem Citations:** New functions must include a docstring citing the relevant section of the Aurphyx Thesis.
- **Windows-First:** All pathing must use `std::path::PathBuf` or Windows backslashes to ensure Dokany compatibility.

## 3. Error Handling
Generic `unwrap()` or `panic!()` is prohibited in the storage path. Use `PhysicsViolationError` to trigger autonomous recovery loops.