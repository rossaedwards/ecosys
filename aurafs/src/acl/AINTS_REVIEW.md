# AINTS_REVIEW — `aurafs/src/acl/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `manager.rs` (~267 LOC) path/owner RBAC manager.
- `acl_config.json` sample policy.
- `ENTERPRISE_IMPROVEMENTS.md` wishlist (not runtime).

**Wired:** Yes (`pub mod acl`). `main.rs` wants `AclManager`.  
**Era / voice:** Love-signed RBAC.

## Suggestions

- Bind ACL subjects to welcome SoulKey → SIG (one soul, one account, one vote) instead of leftover BlissID strings.
- Share one enforcer with `shard_server/acl.rs` — do not keep two policy languages.
- If ZK/quantum-safe claims stay in comments, either implement Dilithium verify via `crypto/pqc/dilithium_sig.rs` or drop the claim.

## Clarifying questions needed

### Architectural
- Is ACL the AuraFS authorization plane, or does `gov/policy_enforcer.rs` win?

### Framework
- TSLCA/ICX: should access decisions carry identity continuity (Ξ) rather than path strings?
- SAGES: is a deny a guardian action or a local RBAC miss?

### Vision
- Should Xplor (`xpl0r`) show ACL as SoulKey/SIG, never BlissID?

### Intentions
- Is `acl_config.json` the human-editable policy, or generated from GuardTable?

### Design
- One `AclManager` type exported from `lib.rs` so `main.rs` stops guessing.

### Ideas
- Map ACL verbs onto SIX (mount/read), SCX (namespace invariants), ICX (who may persist).

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Identity cells (ICX⊗*) — do not treat ACL as a 3-vector sum. |
| **FTQC** (`ftqc/`) | No replica-count law here. |
| **TVFD** (`tvfd/`) | No vacuum impedance in ACL. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Quorum-13 overrides stay in `gov/`; ACL enforces local allow/deny. |
| **Fuxyez** (`fuxyez/`) | Fuxyez `integrations/aurafs` persist/load must respect this ACL, not bypass it. |


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
