"""
---
type: implementation-note
title: TSLCA Phase 1 — Cognitive Field Tensor
description: Simulates the cognitive-field-tensor F = sum Φ_ij (S_i ⊗ S_j) on the 3x3 SIX/SCX/ICX lattice without collapsing off-diagonal directionality.
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
  - cognitive-field-tensor
  - harmonic-integrity-field
---

## ** APS‑TSLCA-SIM-P1-FIELD-TENSOR **
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
    "title": "TSLCA Phase 1 — Cognitive Field Tensor",
    "description": "Simulates the cognitive-field-tensor F = sum Φ_ij (S_i ⊗ S_j) on the 3x3 SIX/SCX/ICX lattice without collapsing off-diagonal directionality.",
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
        "cognitive-field-tensor",
        "harmonic-integrity-field",
    ],
}


def run(seed: int = 369, steps: int = 1) -> dict[str, Any]:
    rng = np.random.default_rng(seed)
    fields = seed_fields(rng)
    hif, _ = harmonic_integrity(fields["C"], fields["R"], fields["A"])
    phi_ij = collapse_volume_to_cells(hif)
    F = field_tensor(phi_ij)
    return {
        "okf": OKF,
        "phi_ij": phi_ij.tolist(),
        "F": F.tolist(),
        "cells": {NODES[i * 3 + j]: float(phi_ij[i, j]) for i in range(3) for j in range(3)},
        "diag": [float(phi_ij[i, i]) for i in range(3)],
        "asymmetric": bool(not np.allclose(phi_ij, phi_ij.T)),
        "trace_readout": float(np.trace(F)),
        "fusion": FUSION_OPERATOR,
        "note": "trace is readout, not SUXS-IFO fusion",
        "seed": seed,
        "steps_ignored": steps,
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
