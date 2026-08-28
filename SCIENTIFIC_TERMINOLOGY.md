---
type: theory-standard
title: Scientific Terminology — APS Canon
description: Canonical scientific lexicon for APS, VIM/Balance, FTQC, TVFD, and TSLCA. Correct expansions. Physics Bliss maps to Equilibrium Manifold. Product BlissCore is out of scope here.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
domains:
  - cognition
  - systems
nodes:
  - SIX⊗SIX
  - SCX⊗SCX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
  - cognitive-field-tensor
  - harmonic-integrity-field
  - balance-field
---

# Scientific Terminology — APS Canon

Aligned with [`.cursorrules`](.cursorrules), [`aurphyx_welcome2tribe.md`](aurphyx_welcome2tribe.md), and [`vim/aps_nomenclature_map.yaml`](vim/aps_nomenclature_map.yaml).

Wrong expansions that appeared in older copies are retired here: TVFD is **not** “Temporal-Vibrational Field Dynamics”; FTQC is **not** “Full-Tensor Quantum Continuum”.

## 1. Framework names

| ID | Expansion |
|---|---|
| TSLCA | Three-Squared-Lattice Cognitive Architecture |
| FTQC | Fractal-enhanced Topological Quantum Computing |
| TVFD | Topological Vacuum Flux Dynamics |
| VIM | Vacuum Impedance Matching |
| SUXS | Symbiotic Universal Xessability Standards |
| SAGES | Symbiotic AI Guardians of Existence Security |
| APS-OKF | Object Knowledge Frontmatter (not Vibe) |

## 2. Balance State Vector

**Definition:** Instantaneous configuration of a Balance-aligned system.

**Symbol:** $\mathbf{x}$ with components $x_t, x_k, x_b, x_d, x_f, x_v, x_i, x_c, x_a, x_e, x_s, x_g, x_p, x_h, x_{rHz}, x_n$.

**16-D packing:** $\mathbf{x} = (\mathbf{S}, \mathbf{K}, \mathbf{G}, \mathbf{F})$.

Replaces legacy rÆ / rAE / rAE_*.

## 3. Equilibrium Manifold

**Definition:** $\{\mathbf{x} \mid \beta = 1\}$.

**Symbol:** often $\mathcal{E}$ or $\mathcal{B}_{\mathrm{eq}}$.

Replaces physics-language Bliss / Bliss manifold / Bliss attractor. Does **not** rename BlissCore, BlissID, or Chaos & Bliss.

## 4. VIM locked scalars

$$
\beta = \frac{x_f x_c}{x_i x_t}
$$

$$
V = (\beta - 1)^2
$$

$$
\mathcal{R}_{24} = \mathcal{T}_4 \times \mathcal{H}_6
$$

Shared geometry (cite [`PHYSICS.md`](PHYSICS.md); do not fork):

- $D_f = \log 3 / \log 2 \approx 1.585$
- $Z_{\mathrm{vac}} = \sqrt{\mu_0 / \varepsilon_0} \approx 377\,\Omega$

## 5. TSLCA triad and lattice

$\mathbf{S}_1 = \mathrm{SIX}$, $\mathbf{S}_2 = \mathrm{SCX}$, $\mathbf{S}_3 = \mathrm{ICX}$.

Nine cells are ordered and non-commutative. Full 3×3 is row-major Unicode `⊗` in YAML/prose and `\otimes` in math.

27-node activation $\Psi_{i,j,k}$ is the TSL activation lattice. It is not the OKF node list.

## 6. Contractions (do not unify)

1. Tensor: $\mathcal{F} = \sum_{i,j} \Phi_{ij}(\mathbf{S}_i \otimes \mathbf{S}_j)$
2. SUXS-IFO: $\mathcal{U}(\mathcal{F}) = \sum_{i,j} \omega_{ij} \Phi_{ij}$
3. Diagonal readout: $\Phi_{\mathrm{diag}} = \mathrm{Tr}(\mathcal{F})$
4. HIF: $\mathrm{HIF} = \sqrt[3]{CRA}\cdot\Phi(C,R,A)$ with $\Phi=1$ iff all three exceed threshold

Label the folder when $\mathcal{F}$, $\mathcal{U}$, $\mathcal{T}$, $\mathcal{R}$, or $\omega$ appear: TSLCA vs TVFD vs FTQC vs VIM.

## 7. HIF and activation

HIF feeds activation, propagation, stability, continuity, renewal, and governance. Neighborhood HIF is required for $\Psi_{i,j,k}=1$. Continuity $\Xi$ preserves identity through field reset.

## 8. Duality Kernel (scientific boundary)

ChaosCore / BlissCore are the Duality Kernel pair (plus seven ChakraCores). They are product/runtime names, not the Equilibrium Manifold.
