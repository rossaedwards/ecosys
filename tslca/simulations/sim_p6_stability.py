"""
---
type: implementation-note
title: TSLCA Phase 6 — Stability Conditions
description: Measures node, layer, and lattice stability from the discrete Laplacian of HIF and classifies Creation-ready, Integration, and Renewal basins.
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
  - SAGES-governance-field
---

## ** APS‑TSLCA-SIM-P6-STABILITY **
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
    "title": "TSLCA Phase 6 — Stability Conditions",
    "description": "Measures node, layer, and lattice stability from the discrete Laplacian of HIF and classifies Creation-ready, Integration, and Renewal basins.",
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
        "SAGES-governance-field",
    ],
}


def run(seed: int = 369, steps: int = 1) -> dict[str, Any]:
    rng = np.random.default_rng(seed)
    fields = seed_fields(rng)
    hif, _ = harmonic_integrity(fields["C"], fields["R"], fields["A"])
    report = stability_report(hif)
    return {
        "okf": OKF,
        "S_lattice": report["S_lattice"],
        "S_layer": report["S_layer"],
        "class": report["class"],
        "n_unstable_nodes": report["n_unstable_nodes"],
        "n_isolated": report["n_isolated"],
        "HIF_mean": float(np.mean(hif)),
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
