# AINTS_REVIEW — `aurafs/src/tslca/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `lattice.rs`: Layer/Channel/Mode, NodeFields (C,R,A,HIF,HIF_nbr,Ψ), 27-node `[[[TslNode;3];3];3]`.
- `hif.rs`, `modes.rs`, `mod.rs` empty.

**Wired:** **No.** Not in `lib.rs`.  
**Era / voice:** Empty mods + one ~65 LOC lattice struct. Continuity tag still `SoulHash / BlissID`.

## Suggestions

- Do not implement HIF engines here. Canon + sims live in `tslca/`.
- Do not copy `tslca/simulations/` into the product.
- If wired later: keep U, Tr, HIF, Ψ distinct.

## Clarifying questions needed

### Architectural
- Does AuraFS need in-process TSL, or only cite the volume?

### Framework
- 27-node TSL is activation, not OKF 3×3 node list.
- Off-diagonals directed. SIX⊗SCX ≠ SCX⊗SIX.

### Vision
- Memoree is the cognitive memory architecture — not this stub.

### Intentions
- Hook for Fuxyez Duality Kernel, or accidental copy?

### Design
- Stay out of `lib.rs` until Ross asks.

### Ideas
- If ever: read-only snapshot of Φ_ij for Xplor viz — not a contraction.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Cite volume XVIII. Flag Φ_unified = Tr if it appears. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | TVFD F is a different object — label if mentioned. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | G_13 acts on F; not implemented here. |
| **Fuxyez** (`fuxyez/`) | Compiler Duality Kernel is the language lattice; do not fork types. |


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
