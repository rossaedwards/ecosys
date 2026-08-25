# AINTS_REVIEW — `aurafs/src/crypto/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- Locked `pqc/dilithium_sig.rs` (`pqcrypto_dilithium`).
- Drafts: `encryption.rs`, `hash.rs`, `hybrid.rs`, `quantum.rs`.
- Named stubs: wallet, ledger, gov, primitives, integrations (arora, gvs, ineffable, opulence, sages).
- Kyber: in-progress / Cargo feature commented (SECURITY_AUDIT R1).

**Wired:** Yes. Dilithium-5 helper real; most of tree is 14-line stubs.  
**Era / voice:** Fat files Love-signed; stubs undated.

## Suggestions

- Finish Kyber-1024 or keep `kem_status` honest.
- Do not implement GIL/Opulence/GVS from filenames.
- SoulCrypt / GuardCrypt are other products — bridge later.

## Clarifying questions needed

### Architectural
- Is `crypto/` the only PQC plane, or does `quantum/` duplicate it?

### Framework
- FTQC is theory; this is classical PQC on Rust.

### Vision
- HSM / third-party audit (R3–R5) before any token wallet?

### Intentions
- Wallet stubs — AuraFS fees or Opulence launch 3?

### Design
- Delete or `#[cfg]` stub forests so `cargo check` stays meaningful.

### Ideas
- Hybrid KEX only after Dilithium+Kyber both real.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | N/A. |
| **FTQC** (`ftqc/`) | Do not claim Shor-resistance from FTQC papers; use PQC crates. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | `sages_crypto.rs` stub ≠ SAGES_Framework. |
| **Fuxyez** (`fuxyez/`) | Signed lattice blobs should use Dilithium-5, not a new scheme. |


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
