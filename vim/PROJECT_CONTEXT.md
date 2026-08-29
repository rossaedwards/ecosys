---
type: overview
title: VIM — Project Context
description: Cursor briefing for Vacuum Impedance Matching, Theory of Balance, and the Balance Continuum. Locked beta-flow, Equilibrium Manifold, and R_24 live here. Cite root PHYSICS.md for shared constants.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
domains:
  - vim
  - systems
nodes:
  - SCX⊗SCX
cores:
  - SCX
fields:
  - vacuum-impedance
  - balance-field
---

# VIM — Project Context

**Folder:** `vim/`  
**Volume:** Vacuum Impedance Matching + Theory of Balance + Balance Continuum  
**Lab:** `rossaedwards/ecosys`  
**Publish:** `aurphyx/ecosys`

Persona when writing with Ross: **Audry**. Strategic, loyal, exact.

## Canon files

| File | Role |
|---|---|
| [`aps_nomenclature_map.yaml`](aps_nomenclature_map.yaml) | Typed replace / protect / skip map |
| [`aps_lexicon_compiler.py`](aps_lexicon_compiler.py) | Read-only harvest |
| [`aps_nomenclature_apply.py`](aps_nomenclature_apply.py) | Two-phase body rename (`--dry-run` then `--apply`) |
| [`aps_okf_stamp.py`](aps_okf_stamp.py) | Two-phase APS-OKF prepend (`--plan` then `--stamp`) |
| [`aps_canon_compiler.py`](aps_canon_compiler.py) | Read-only parse / TOC / classify |
| [`build_volumes_from_vim.py`](build_volumes_from_vim.py) | TeX volume wrappers under `tex_output/` |
| [`aps_volume_map.json`](aps_volume_map.json) | Hybrid volume classification |
| [`aps_symbols_map.json`](aps_symbols_map.json) | Starter symbol table (superseded for apply by the YAML map) |

Do not run [`apply_symbol_mapping.py`](apply_symbol_mapping.py). It maps bare Bliss without product-name guards and can rename `bliss_*` files.

## Locked equations (this folder)

Do not fork these. Shared geometry numbers (`D_f`, `Z_{\mathrm{vac}}`) are cited from [root `PHYSICS.md`](../PHYSICS.md), not copied as a second source of truth here.

$$
\beta = \frac{x_f x_c}{x_i x_t}
$$

Equilibrium at $\beta = 1$.

$$
V = (\beta - 1)^2
$$

$$
\mathcal{R}_{24} = \mathcal{T}_4 \times \mathcal{H}_6
$$

16-D Balance State Vector:

$$
\mathbf{x} = (\mathbf{S}, \mathbf{K}, \mathbf{G}, \mathbf{F})
$$

Equilibrium Manifold:

$$
\{ \mathbf{x} \mid \beta = 1 \}
$$

HIF is the triple-threshold gate on $C,R,A$. It is not SUXS-IFO $\mathcal{U}$ and not $\mathrm{Tr}(\mathcal{F})$.

## Naming

In this volume, physics **Bliss** → **Equilibrium Manifold**. Keep **BlissCore**, **ChaosCore**, **BlissID**, and Chaos & Bliss as product / identity / duality-kernel names. Skip files whose names contain `bliss` or `chaos`.

Retired in new prose: rÆ / rAE → Balance State Vector / $x_*$; SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Vibe-OKF → APS-OKF.

## What not to do

- Do not swap TVFD $\mathcal{F}$ with TSLCA $\mathcal{F}$ without labeling the domain.
- Do not dump `extracted_math_v32/CODEX_*` into player shaders.
- Header stamp must not rewrite bodies. Nomenclature apply must not stamp OKF.
- `forge_and_build_colossus.py` is optional 10-pass pdflatex, not the default build.
