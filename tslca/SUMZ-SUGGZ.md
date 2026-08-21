---
type: implementation-note
title: TSLCA — Summary and Suggestions
description: Working map of the Three-Squared-Lattice Cognitive Architecture volume and how the 3x3 plus nine VASP pillars must drive Vibe orbs without collapsing fusion, trace, and HIF.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - Fuxyez
  - SAGES
domains:
  - cognition
  - xessability
  - identity
nodes:
  - SIX⊗SIX
  - SIX⊗SCX
  - SIX⊗ICX
  - SCX⊗SIX
  - SCX⊗SCX
  - SCX⊗ICX
  - ICX⊗SIX
  - ICX⊗SCX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
  - cognitive-field-tensor
  - unified-cognitive-field
  - SAGES-governance-field
  - harmonic-integrity-field
---

# TSLCA — SUMZ / SUGGZ

**Folder:** `tslca/`  
**Role:** Volume XVIII. Canon for the 3×3 cognitive lattice. This folder occupies the **whole lattice**, so all nine nodes are declared.

**Gap:** `PROJECT_CONTEXT.md`, folder `INVARIANTS.md`, folder `PHYSICS.md` are missing. Root `.cursorrules` §11 already locks this volume. Prefer creating those overlays over rewriting section bodies.

---

## What this is

Perception, semantics, and identity are orthonormal aXes:

- **SIX** Sensorimotor Integration aXis  
- **SCX** Systemic Coherence aXis  
- **ICX** Soul Identity aXis  

Off-diagonal cells are directed duals. `SIX⊗SCX` is not `SCX⊗SIX`. Do not force Φ_ij = Φ_ji.

Contractions stay distinct: tensor field F, SUXS-IFO fusion U, diagonal Tr, HIF gate on C,R,A, activation Ψ. If a file writes Φ_unified = Tr(F), flag it — do not rewrite until Ross approves.

27-node TSL (i,j,k ∈ {1,2,3}) is the **activation** lattice. OKF nodes are the 3×3 cells. VASP’s nine pillars are the **audio identity** readout, not a replacement 3-vector for F.

## What exists

- Markdown sections I–X plus abstract, HIF spec, TTG, continuity, stability, propagation, references.
- Parallel `tex/` (do not convert working TeX to Markdown).
- Simulations: HIF, activation, propagation, stability, continuity, balance manifold, series S1–S4, unified engine.
- Figures under `figures/output/group1/` (lattice, aXes, off-diagonal asymmetry, HIF-related diagrams).
- aXis specs: SIX / SCX / ICX / SUXS-IFO markdown + tex.

This volume is **docs + Python sims**, not a player crate. Simulations are **blocked**: every engine imports `lattice_kernel.py`, which **does not exist**. No in-folder `requirements.txt`.

## How Vibe must use it (without inventing physics)

| Player need | TSLCA object | Rule |
|---|---|---|
| Orb 3×3 field | Φ_ij visualization | Nine cells, directed. Do not symmetrize for pretty graphics. |
| Nine VASP pillars | Experiential identity of a *track* | Map pillars onto lattice **readouts** in a table in player docs. Do not claim pillars = Φ_ij. |
| Bloom / glow intensity | Not HIF unless C,R,A are actually measured | Analyser energy is SIX-ish. Do not label it HIF. |
| Skinz “Lattice” pack | Aesthetic named after the 3×3 | Colors only, unless a vis-plug reads Φ_ij. |

Suggested pillar→aXis *display* map (documentation only, not a contraction):

- SIX-heavy: STRUCTURAL, TIMBRAL, KINETIC  
- SCX-heavy: TONAL, LINGUISTIC, CONTEXTUAL  
- ICX-heavy: AFFECTIVE, PHOTOMETRIC, GENEALOGICAL  

Off-diagonals can modulate vis coupling (e.g. kinetic×affective). Keep the 3×3 visible in the orb, not a 3-bar equalizer.

## Suggestions (this folder)

1. Add `PROJECT_CONTEXT.md`: canon file list, locked equations, “do not collapse U/Tr/HIF.”
2. Overlay `PHYSICS.md` that **cites** root `D_f`, `d_s`, `Z_vac` — no new constants.
3. One short `VIBE_BINDING.md` (or a section in PROJECT_CONTEXT) that the orb implementers must read. Players should not scrape section-v.tex.
4. **Add `simulations/lattice_kernel.py`** (CORES, `suxs_ifo`, HIF, collapse_volume_to_cells) so `sim_p0`–`sim_p9` can run. Then `requirements.txt` + `series_all.py` smoke.
5. Formal VASP-pillar ↔ 9-cell mapping table lives in player docs, not as a new contraction.

## What not to do

- Do not resurrect SIC/SCC/ICC in new headers.
- Do not dump this volume into `.cursorrules`.
- Do not treat MYTHIC_CODEX as YAML for the orb.
