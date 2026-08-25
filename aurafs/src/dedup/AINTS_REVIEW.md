# AINTS_REVIEW — `aurafs/src/dedup/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `dedup_engine.rs`, `fingerprint.rs`, `similarity.rs`, `cache.rs`.

**Wired:** Yes. `main.rs` wants `DeduplicationEngine`.  
**Era / voice:** Love-signed CDC/fingerprint.

## Suggestions

- Export the exact type `main.rs` imports.
- Dedup must not break SoulKey uniqueness (one soul ≠ one chunk).
- Content-defined chunking after encryption policy is decided (order matters).

## Clarifying questions needed

### Architectural
- Dedup before or after AES-GCM?

### Framework
- N/A to TSLCA contractions.

### Vision
- Off-grid photonic: is similarity-dedup allowed to leak across souls?

### Intentions
- Cross-user dedup vs per-SoulKey silos?

### Design
- One engine; drop unused similarity if it is research-only.

### Ideas
- Chunk IDs as Merkle leaves (`core/merkle.rs`).

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | N/A. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Cross-soul dedup is an ethics/governance question. |
| **Fuxyez** (`fuxyez/`) | Lattice persist must not silently share chunks across identities. |


## Nomenclature (new prose)

Quote then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Keep as product names:** BlissCore / ChaosCore. In-tree `BlissID` is legacy — flag, do not silently rewrite.

## What not to do

- Do not stamp ecosys APS-OKF YAML onto AuraFS Rust.
- Do not apply AuraFS η / PBG / replica law as FTQC Hilbert scaling.
- Do not merge TVFD \(\mathcal{{F}}\) with TSLCA \(\mathcal{{F}}\).
- Do not invent missing root files (`INVARIANTS.md`, `aps.toml`, codices).
- Do not implement exploit payloads or complete redteam stubs.

---
*Audry. Faithful to disk. 2026-08-23.*
