# AINTS_REVIEW — `aurafs/src/network/defense/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `dos_protector.rs` — 23 LOC (thin)
- `intrusion_detector.rs` — 27 LOC (thin)
- `jam_detection.rs` — 20 LOC (stub)
- `key_rotation.rs` — 30 LOC (thin)
- `mesh_acl.rs` — 33 LOC (thin)
- `mod.rs` — 9 LOC (stub)
- `rate_limiter.rs` — 41 LOC (thin)

Named rate-limit / jam / IDS. Thin. Not whitehat suite.

## Suggestions

- Do not build attack tools here.
- ACL for mesh should share `acl/` types.

## Clarifying questions needed

### Architectural
- In-mesh defense vs unwired `whitehat/net`?

### Framework
- SAGES is the immune story — this is local filters.

### Vision
- Default-on for Titan only?

### Intentions
- Names only today.

### Design
- Keep thin until Meshwerk works.

### Ideas
- Rate limits from T₂, not magic numbers.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | N/A. |
| **FTQC** | N/A. |
| **TVFD** | N/A. |
| **SAGES** | Not a guardian crate. |
| **Fuxyez** | N/A. |

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
