# AINTS_REVIEW — `aurafs/src/shard/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- id, metadata, data, storage, index, audit. Void→Trap→Aura language in `mod.rs`.
- Declares `tests` without `tests.rs` (risk).

**Wired:** Yes. Used by `storage/`. Dual with locked `core/shard.rs`.  
**Era / voice:** Love + QDiv; several TRL-4. `review_suggestion.md` 2026.

## Suggestions

- Lifecycle names are law — do not rename Trap-State to cache.
- Resolve type collision with `core/shard.rs` (locked).
- Aethornyx/Casino shard-as-currency is design — no ledger rules here.

## Clarifying questions needed

### Architectural
- Which Shard is the atom?

### Framework
- Welcome: AuraFS atom. Fuxyez persist targets this atom.

### Vision
- Bio-Resonant Reality Forge — product poetry or a pipeline name?

### Intentions
- Audit submodule vs `src/audit/`?

### Design
- One ShardId newtype everywhere.

### Ideas
- Metadata holds codec + SoulKey + replica η result.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | A shard is not a lattice cell. |
| **FTQC** (`ftqc/`) | Replica count is AuraFS η, not FTQC. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Shard rewrite is auditable. |
| **Fuxyez** (`fuxyez/`) | `integrations/aurafs/shard.rs` must match this ID once stable. |


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
