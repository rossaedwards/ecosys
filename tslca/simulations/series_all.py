"""
---
type: implementation-note
title: TSLCA Series Runner — All Campaigns
description: Runs S1 architecture, S2 field physics, S3 governance, and S4 manuscript epoch in order, each carrying aligned APS-OKF v3.69.
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
  - SAGES-governance-field
  - harmonic-integrity-field
---

## ** APS‑TSLCA-SERIES-ALL **
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
if str(_HERE) not in sys.path:
    sys.path.insert(0, str(_HERE))

import series_s1_architecture as s1
import series_s2_field_physics as s2
import series_s3_governance as s3
import series_s4_manuscript_epoch as s4

OKF = {
    "type": "implementation-note",
    "title": "TSLCA Series Runner — All Campaigns",
    "description": "Runs S1 architecture, S2 field physics, S3 governance, and S4 manuscript epoch in order, each carrying aligned APS-OKF v3.69.",
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
        "SAGES-governance-field",
        "harmonic-integrity-field",
    ],
}

def run(seed: int = 369, steps: int = 32) -> dict[str, Any]:
    return {
        "okf": OKF,
        "aps_id": "APS‑TSLCA-SERIES-ALL",
        "s1": s1.run(seed=seed, steps=steps),
        "s2": s2.run(seed=seed, steps=steps),
        "s3": s3.run(seed=seed, steps=steps),
        "s4": s4.run(seed=seed, steps=steps),
    }

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=OKF["title"])
    parser.add_argument("--seed", type=int, default=369)
    parser.add_argument("--steps", type=int, default=32)
    parser.add_argument("--out", type=str, default="")
    args = parser.parse_args()
    result = run(seed=args.seed, steps=args.steps)
    text = json.dumps(result, indent=2, default=str)
    print(text)
    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(text)
