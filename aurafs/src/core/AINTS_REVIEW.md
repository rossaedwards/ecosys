# AINTS_REVIEW — `aurafs/src/core/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- Locked `merkle.rs`, `shard.rs`. Fat `persistence.rs`, `bliss.rs` (BlissID).
- Enterprise patterns: circuit breaker, rate limiter, health, metrics, tracing.
- `review_suggestion.md` (2026). Empty `lattice.rs`.

**Wired:** Yes. Overlaps `shard/`, `crypto/`, `error.rs`, `network/`.  
**Era / voice:** Almost all Love + QDiv. `core/shard.rs` also TRL-4. `lattice.rs` empty.

## Suggestions

- One Shard type — `core/shard.rs` is the locked file; `src/shard/` must alias or die.
- New identity APIs: SoulKey/SIG. Leave BlissID until an identity PR.
- Do not fill empty `lattice.rs` with TSLCA paper equations.

## Clarifying questions needed

### Architectural
- Is `core/` the μkernel or a junk drawer?

### Framework
- Soulproof vs USIS SoulCrypt — which product owns proof?

### Vision
- Welcome soul journey: does `bliss.rs` become SIG watermark storage?

### Intentions
- `ENTERPRISE_IMPROVEMENTS.md` — Biznyx or AuraFS personal?

### Design
- Stop declaring missing `mod lattice` until there is a type.

### Ideas
- Merkle + Dilithium as ICX provenance for shards.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Empty lattice.rs must not become a 3-vector collapse. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | soulproof is not the 13 crates. |
| **Fuxyez** (`fuxyez/`) | Fuxyez Lattice persist should target shard IDs from this core, once one type exists. |


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
