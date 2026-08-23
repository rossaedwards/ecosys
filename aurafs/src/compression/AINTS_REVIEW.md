# AINTS_REVIEW — `aurafs/src/compression/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- LZ4, Zstd, quantum-named wavelet, manager, stats.
- Locked lattice compressor (product lattice, not TSLCA \(\mathcal{F}\)).

**Wired:** Yes. **`config.rs` empty** while `mod.rs` uses `CompressionConfig` — compile break.  
**Era / voice:** Love + QDiv. `lattice.rs` + `manager.rs` locked.

## Suggestions

- Fill `config.rs` or stop importing `CompressionConfig`.
- Feature-gate zstd/lz4 to match Cargo optional features.
- Do not call wavelet output SUXS-IFO.

## Clarifying questions needed

### Architectural
- Is compression before or after Void→Trap→Aura?

### Framework
- TSLCA lattice.rs in `src/tslca/` is 27-node activation — different object.

### Vision
- Entropy-adaptive as product feature or marketing name?

### Intentions
- Quantum wavelet — sim or real codec?

### Design
- One CompressionManager exported.

### Ideas
- Store codec id in shard metadata (`shard/metadata.rs`).

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Do not contract nine cells in a compressor. |
| **FTQC** (`ftqc/`) | Fractal compression ≠ Hilbert-space scaling. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | N/A. |
| **Fuxyez** (`fuxyez/`) | Persist compressed blobs; Fuxyez lattice types stay typed on load. |


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
