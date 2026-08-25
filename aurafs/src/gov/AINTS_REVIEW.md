# AINTS_REVIEW — `aurafs/src/gov/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- SoulSync engine, voting, proposals, policy, identity verifier, audit, FastAPI-shaped stubs.
- `sages.rs`: 13 **in-crate role names** (Vyrellix, Archivus, Sentry, …) — **≠** `SAGES_Framework/` crate list.
- Legacy `blissid_manager.rs`.

**Wired:** Yes. Locked `sages.rs`.  
**Era / voice:** Love + several Phase II / TRL-4 engines.

## Suggestions

- Do not invent a mapping between in-crate sentinels and Framework crates until Ross writes it.
- GVS lives at `gvs/` — do not grow a second global voting system here.
- Identity: new APIs SoulKey/SIG; BlissID stays until identity PR.

## Clarifying questions needed

### Architectural
- In-process hooks vs 13 guardian processes — which ships in AuraFS v1?

### Framework
- SAGES: U(g_k · F) = U(F). This folder does not implement group action on TSLCA F.
- Welcome: SAGES at kernel through meshwrk.

### Vision
- Is AuraFS gov the filesystem immune system, or a stub until `sages-4dm1n`?

### Intentions
- Quorum 13 in toml — product law or placeholder?

### Design
- `server.rs` FastAPI-shaped Rust — pick axum/actix once, or delete.

### Ideas
- Dilithium already used in sages.rs — keep that as the only vote signature.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Governance field is SAGES, not a ninth contraction. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Docs in `sages/`; crates in `SAGES_Framework/`; this is a hook. Flag name drift. |
| **Fuxyez** (`fuxyez/`) | `fuxyez/governance/` looks like a sibling copy — do not merge from AuraFS. |


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
