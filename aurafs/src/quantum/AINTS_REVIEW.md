# AINTS_REVIEW — `aurafs/src/quantum/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- Placeholder Kyber-shaped Vec<u8> KEX, entanglement sim, QRNG sim.
- `lib.rs` “Majorana-1 simulation” is a **name**, not hardware.

**Wired:** Declared. Not used by `main.rs`.  
**Era / voice:** Love-signed single `mod.rs` (~278) with inline sim modules.

## Suggestions

- Real PQC stays in `crypto/pqc`.
- Do not present this as FTQC or Majorana-1.

## Clarifying questions needed

### Architectural
- Why two quantum folders (`quantum/` vs `crypto/quantum.rs`)?

### Framework
- FTQC volume is docs/sims in `ftqc/`.

### Vision
- Educational hooks vs shipping crypto?

### Intentions
- Keep default-off if it confuses TRL-4 claims.

### Design
- Rename comments to “simulation only”.

### Ideas
- QRNG: only if a real entropy source is specified.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | N/A. |
| **FTQC** (`ftqc/`) | Do not copy Hilbert-dimension law into this file. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | N/A. |
| **Fuxyez** (`fuxyez/`) | Fuxyez is the language; this is not FUTE quantum. |


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
