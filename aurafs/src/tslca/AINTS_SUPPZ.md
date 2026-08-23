# AINTS_SUPPZ — `aurafs/src/tslca/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

**Mostly empty stubs.** `mod.rs`, `hif.rs`, `modes.rs` are empty. `lattice.rs` (~65 LOC) defines Layer / Channel / Mode, `NodeFields` (C, R, A, HIF, HIF_nbr, Ψ), `NodeContinuity` (mem, tag `SoulHash / BlissID`, invariants), `TslLattice` as `[[[TslNode; 3]; 3]; 3]` — the **27-node activation lattice**.

## Wired?

**No.** Not in `lib.rs`.

## Locks (cite TSLCA volume — do not dump papers)

Contractions stay distinct: \(\mathcal{F}\), SUXS-IFO \(\mathcal{U}\), \(\mathrm{Tr}\), HIF \(\Phi(C,R,A)\), activation \(\Psi\). 27-node TSL is **not** the OKF 3×3 node list. Off-diagonals are directed. Do not collapse to a 3-vector.

Canon lives in `tslca/` (markdown + tex + sims). `tslca/PROJECT_CONTEXT.md`, `tslca/INVARIANTS.md`, `tslca/README.md`, and `tslca/AINTS_SUPPZ.md` are **missing** on this tree.

## Relation to welcome

TSLCA is APS. AuraFS may **use** geometry; this folder is a thin type sketch, not Volume XVIII.

## Honest status

Empty module root + one lattice struct file. Do not implement HIF engines here. Do not copy `tslca/simulations/` into this product.


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
