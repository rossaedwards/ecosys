# AINTS_REVIEW — `aurafs/src/crypto/pqc/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `dilithium_sig.rs` — 43 LOC (thin)
- `falcon_sig.rs` — 14 LOC (stub)
- `hybrid_kex.rs` — 14 LOC (stub)
- `kyber_kem.rs` — 44 LOC (thin)
- `mod.rs` — 15 LOC (stub)
- `pq_hashes.rs` — 14 LOC (stub)
- `pqc_kdf.rs` — 14 LOC (stub)
- `pqc_tls.rs` — 14 LOC (stub)
- `sphincs_sig.rs` — 14 LOC (stub)

Locked real helper: `dilithium_sig.rs`. Kyber/Falcon/SPHINCS/TLS mostly thin or stub. `mod.rs` re-exports Kyber names that may not be real KEM.

## Suggestions

- Treat Dilithium-5 as the only shipping PQC until Kyber R1 is actually `pqcrypto-kyber`.
- Do not implement wallet/ledger from sibling stub forests.

## Clarifying questions needed

### Architectural
- Is `crypto/pqc` the only PQC plane, or does `src/quantum/` duplicate it?

### Framework
- FTQC is theory in `ftqc/`. This folder is classical PQC on Rust.

### Vision
- Hybrid KEX only after both Dilithium and Kyber are real?

### Intentions
- SECURITY_AUDIT R1–R6 still open — confirm order.

### Design
- Feature-gate unfinished algs so `cargo check` is meaningful.

### Ideas
- Fuxyez signed lattice blobs should call Dilithium-5 here, not a new scheme.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | N/A. |
| **FTQC** | Do not claim Shor-resistance from FTQC papers. |
| **TVFD** | N/A. |
| **SAGES** | N/A. |
| **Fuxyez** | Use Dilithium-5 for persist signatures. |

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
