"""
---
type: implementation-note
title: TSLCA Series 1 — Architecture
description: Series run of lattice grammar, cognitive-field-tensor, and SUXS-IFO contraction — the architectural spine of TSLCA.
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
  - unified-cognitive-field
---

## ** APS‑TSLCA-SERIES-S1-ARCHITECTURE **
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
import sys
from pathlib import Path
from typing import Any

_HERE = Path(__file__).resolve().parent
_ROOT = _HERE.parent
for _p in (_HERE, _ROOT):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

OKF = {
    "type": "implementation-note",
    "title": "TSLCA Series 1 — Architecture",
    "description": "Series run of lattice grammar, cognitive-field-tensor, and SUXS-IFO contraction — the architectural spine of TSLCA.",
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
        "unified-cognitive-field",
    ],
}

PHASES = ['p0', 'p1', 'p2']
APS_ID = 'APS‑TSLCA-SERIES-S1-ARCHITECTURE'

import sim_p0_lattice_okf as _p0
import sim_p1_cognitive_field_tensor as _p1
import sim_p2_suxs_ifo_contraction as _p2

def run(seed: int = 369, steps: int = 64) -> dict[str, Any]:
    results = {
        "p0": _p0.run(seed=seed, steps=steps),
        "p1": _p1.run(seed=seed, steps=steps),
        "p2": _p2.run(seed=seed, steps=steps),
    }
    return {
        "okf": OKF,
        "aps_id": APS_ID,
        "series": 's1',
        "phases": PHASES,
        "seed": seed,
        "steps": steps,
        "results": results,
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
