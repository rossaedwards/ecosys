# AINTS_REVIEW — `aurafs/src/model_slice/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- Fractal weight slicing, optimizer↔NodeManager, `pytorch.rs` FFI stub, empty extern `PyTorchModel`.

**Wired:** Yes. Not in `main.rs`.  
**Era / voice:** Love + QDiv + TRL-4 tags.

## Suggestions

- Do not claim PyTorch works. `unsafe_code = forbid` vs raw pointers — conflict if compiled.
- This is not Memoree and not Audry.

## Clarifying questions needed

### Architectural
- Does AuraFS store model shards, or run them?

### Framework
- Welcome Chakra DataCore / DataOrb — different products.

### Vision
- Arora on-device models via AuraFS?

### Intentions
- Keep FFI out of default features.

### Design
- cfg-gate `pytorch.rs`.

### Ideas
- If sliced weights are shards, they follow Void→Trap→Aura.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Weight tensors are not Φ_ij. |
| **FTQC** (`ftqc/`) | Do not equate layer count with D_f scaling. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Model exfil is a SAGES concern; no new guardian. |
| **Fuxyez** (`fuxyez/`) | N/A. |


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
