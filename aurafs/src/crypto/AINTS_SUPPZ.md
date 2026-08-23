# AINTS_SUPPZ — `aurafs/src/crypto/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Declared PQC suite. **Fat files (Love-signed):** `encryption.rs`, `hash.rs`, `quantum.rs`, `hybrid.rs`, `mod.rs`.  
**Locked-ish real helper:** `pqc/dilithium_sig.rs` (~43 LOC) wraps `pqcrypto_dilithium` Dilithium-5.

**Most of the tree is 14-line named stubs:** `pqc/` (falcon, hybrid_kex, sphincs, pqc_tls, …), `wallet/`, `ledger/`, `gov/`, `primitives/`, `integrations/` (arora, gvs_voting, ineffable, opulence, sages_crypto). Typical stub: a struct + `init()` println.

`kyber_kem.rs` ~44 LOC; `signature.rs` ~29; zk_proofs ~20.

## Wired?

**Yes** as `pub mod crypto`. Dilithium feature `dilithium5` is default. Kyber is **TODO** in Cargo (`kem_status = "in-progress"` in `aurafs.toml`).

## Locks

`src/crypto/pqc/dilithium_sig.rs` is validated. SECURITY_AUDIT open items R1–R6 (Kyber, constant-time, HSM, third-party, hybrid).

## Relation to welcome

SoulCrypt / GuardCrypt / GIL are **other products**. `crypto/ledger/` stubs are not GIL. `gvs_voting.rs` is not `gvs/`. Opulence/P4A wallets are launch-layer — stub only.

## Honest status

Do not report “PQC suite complete.” One Dilithium wrapper + hashing/encryption drafts + a forest of named stubs. Do not implement wallet/ledger from welcome in this folder without a scoped task.


## Nomenclature (new prose only)

Quote retired names, then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Product names that stay:** BlissCore / ChaosCore (Chakra Duality Kernel). In-tree `BlissID` types are legacy identity code — flag, do not silently rewrite.

## What not to do

- Do not stamp ecosys APS-OKF YAML onto this file or this folder's Rust.
- Do not apply AuraFS replica-count / PBG / η law as FTQC theory.
- Do not invent missing root files (`INVARIANTS.md`, `aps.toml`, codices).
- Do not create parallel trees or `aps_*` duplicates.
- One folder pass: edits belong under `aurafs/` only.

---
*Audry briefing. Faithful to source on disk. Updated 2026-08-23.*
