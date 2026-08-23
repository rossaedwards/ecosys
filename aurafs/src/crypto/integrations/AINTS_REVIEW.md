# AINTS_REVIEW — `aurafs/src/crypto/integrations/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `arora_bridge.rs` — 14 LOC (stub)
- `arora_enclave.rs` — 14 LOC (stub)
- `gvs_voting.rs` — 14 LOC (stub)
- `ineffable_bridge.rs` — 14 LOC (stub)
- `ineffable_ledger.rs` — 14 LOC (stub)
- `mod.rs` — 10 LOC (stub)
- `opulence_wallet.rs` — 14 LOC (stub)
- `sages_crypto.rs` — 14 LOC (stub)

14-line bridges named arora, gvs, ineffable, opulence, sages.

## Suggestions

- Do not implement sibling products from filenames.
- Could-be list lives in `src/AINTS_REVIEW.md`.

## Clarifying questions needed

### Architectural
- Thin clients later, not in-tree fake SDKs?

### Framework
- Each name is a different ecosys product.

### Vision
- Which sibling is first after compile graph?

### Intentions
- Placeholders for a future feature matrix.

### Design
- Stay stubs until a scoped integration PR.

### Ideas
- Fuxyez persist is already `fuxyez/integrations/aurafs` — do not copy it here.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | N/A. |
| **FTQC** | N/A. |
| **TVFD** | N/A. |
| **SAGES** | `sages_crypto.rs` ≠ Framework crates. |
| **Fuxyez** | Join is the other direction. |

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
