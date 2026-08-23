# AINTS_REVIEW — `aurafs/src/crypto/gov/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `gov_votes.rs` — 14 LOC (stub)
- `mod.rs` — 8 LOC (stub)
- `pqc_gov_sig.rs` — 14 LOC (stub)
- `proposal_engine.rs` — 14 LOC (stub)
- `quorum_tracker.rs` — 14 LOC (stub)
- `soul_binding.rs` — 54 LOC (thin)

Named quorum / proposal / soul-binding PQC gov sigs. Duplicate of `src/gov/`.

## Suggestions

- Governance engines live in `src/gov/`. Do not fork voting here.
- GVS is `gvs/`.

## Clarifying questions needed

### Architectural
- Why a second gov stack under crypto?

### Framework
- SAGES U(g_k · F)=U(F) is not implemented here.

### Vision
- In-crate Dilithium votes already sketched in `gov/sages.rs`.

### Intentions
- Stub names or future PQC vote layer?

### Design
- Keep unwired.

### Ideas
- If real: only Dilithium wrappers, call `gov/`.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | Not a contraction. |
| **FTQC** | N/A. |
| **TVFD** | N/A. |
| **SAGES** | Hook names only. |
| **Fuxyez** | Do not merge `fuxyez/governance/` from here. |

## Nomenclature (new prose)

Quote then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Keep:** BlissCore / ChaosCore. Flag in-tree `BlissID`; do not silently rewrite.

## What not to do

- Do not stamp APS-OKF on AuraFS Rust.
- Do not apply AuraFS η / PBG as FTQC.
- Do not merge TVFD F with TSLCA F.
- Do not invent missing root `INVARIANTS.md` / `aps.toml` / codices.
- Do not implement exploit payloads or complete redteam stubs.

---
*Audry. Faithful to disk. 2026-08-23.*
