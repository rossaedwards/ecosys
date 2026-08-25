# AINTS_REVIEW — `aurafs/src/crypto/wallet/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `backup_manager.rs` — 14 LOC (stub)
- `hd_wallet.rs` — 14 LOC (stub)
- `mod.rs` — 14 LOC (stub)
- `multi_sig.rs` — 14 LOC (stub)
- `node_shards.rs` — 14 LOC (stub)
- `recovery_sharding.rs` — 14 LOC (stub)
- `shard_vault.rs` — 14 LOC (stub)
- `signing_engine.rs` — 14 LOC (stub)
- `vault_storage.rs` — 14 LOC (stub)

Named HD wallet / vault / recovery files. Almost certainly stubs. Not Opulence.

## Suggestions

- Do not implement a wallet in AuraFS unless Ross scopes finance.
- Opulence / P4A are other products.

## Clarifying questions needed

### Architectural
- Does AuraFS ever custody keys, or only store sealed shards?

### Framework
- SoulKey is identity, not a BIP-44 wallet.

### Vision
- One purchase SoulKey vs software HD wallet — which is v1?

### Intentions
- Filenames are placeholders.

### Design
- Keep out of default features.

### Ideas
- If ever: shard-sealed backup of SIG material, not a token wallet.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | N/A. |
| **FTQC** | N/A. |
| **TVFD** | N/A. |
| **SAGES** | Key custody is a SAGES concern. |
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
