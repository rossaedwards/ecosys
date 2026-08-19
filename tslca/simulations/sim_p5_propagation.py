"""
---
type: implementation-note
title: TSLCA Phase 5 — Propagation Rules
description: Updates C, R, and A by self-retention, orthogonal-neighborhood coupling, and gradient correction across all 27 lattice nodes.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - Aura
  - AuraFS
  - Fuxyez
  - SAGES
  - SoulSync
  - GVS
domains:
  - cognition
  - xessability
  - identity
  - semantics
  - provenance
  - ethics
  - systems
  - governance
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
  - harmonic-integrity-field
  - cognitive-field-tensor
---

## ** APS‑TSLCA-SIM-P5-PROPAGATION **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Symbiotic Universal Xessability Standards **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 3.69 **

AGPLv3 | Aurphyx LLC | SAGES | Pro-Existence | Version 3.69
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import numpy as np

import sys
_ROOT = Path(__file__).resolve().parents[1]
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from lattice_kernel import (
    CORES,
    FUSION_OPERATOR,
    NODES,
    SAGES_INVARIANTS,
    THRESHOLDS,
    activation_gate,
    collapse_volume_to_cells,
    coherence_potential,
    discrete_laplacian,
    energy_functional,
    field_tensor,
    harmonic_integrity,
    layer_mean,
    neighbor_mean,
    orthonormal_cores,
    propagate_cra,
    sages_report,
    seed_fields,
    stability_report,
    summarize,
    suxs_ifo,
    triple_threshold_gate,
)

OKF = {
    "type": "implementation-note",
    "title": "TSLCA Phase 5 — Propagation Rules",
    "description": "Updates C, R, and A by self-retention, orthogonal-neighborhood coupling, and gradient correction across all 27 lattice nodes.",
    "workspaces": "rossaedwards/ecosys, aurphyx/ecosys",
    "services": [
        "Audry",
        "Aura",
        "AuraFS",
        "Fuxyez",
        "SAGES",
        "SoulSync",
        "GVS",
    ],
    "domains": [
        "cognition",
        "xessability",
        "identity",
        "semantics",
        "provenance",
        "ethics",
        "systems",
        "governance",
    ],
    "nodes": [
        "SIX⊗SIX",
        "SIX⊗SCX",
        "SIX⊗ICX",
        "SCX⊗SIX",
        "SCX⊗SCX",
        "SCX⊗ICX",
        "ICX⊗SIX",
        "ICX⊗SCX",
        "ICX⊗ICX",
    ],
    "cores": [
        "SIX",
        "SCX",
        "ICX",
    ],
    "fields": [
        "harmonic-integrity-field",
        "cognitive-field-tensor",
    ],
}


def run(seed: int = 369, steps: int = 64) -> dict[str, Any]:
    rng = np.random.default_rng(seed)
    fields = seed_fields(rng)
    C, R, A = fields["C"], fields["R"], fields["A"]
    trail = []
    for t in range(steps):
        C, R, A, hif, psi = propagate_cra(C, R, A)
        trail.append({
            "t": t,
            "HIF_mean": float(np.mean(hif)),
            "psi_fraction": float(np.mean(psi)),
        })
    hif, _ = harmonic_integrity(C, R, A)
    return {
        "okf": OKF,
        "steps": steps,
        "summary": summarize(C, R, A, hif),
        "coeff_law": "C:α>β>γ  R:β>α>γ  A:γ>β>α  α+β+γ=1",
        "trail": trail[:: max(steps // 8, 1)],
        "seed": seed,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=OKF["title"])
    parser.add_argument("--seed", type=int, default=369)
    parser.add_argument("--steps", type=int, default=64)
    parser.add_argument("--out", type=str, default="")
    args = parser.parse_args()
    result = run(seed=args.seed, steps=args.steps)
    text = json.dumps(result, indent=2, default=str)
    print(text)
    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(text)
