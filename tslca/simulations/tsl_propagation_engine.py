"""
---
type: implementation-note
title: TSLCA Engine — Propagation
description: Engine contract for C, R, A neighborhood updates under conservation α+β+γ=1.
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

## ** APS‑TSLCA-ENGINE-PROPAGATION **
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

from typing import Any

import numpy as np

try:
    from .lattice_kernel import (
        CORES,
        FUSION_OPERATOR,
        SAGES_INVARIANTS,
        THRESHOLDS,
        activation_gate,
        collapse_volume_to_cells,
        continuity_state,
        harmonic_integrity,
        lattice_mode,
        neighbor_mean,
        node_hif_mode,
        propagate_cra,
        seed_fields,
        stability_report,
        summarize,
        suxs_ifo,
    )
except ImportError:
    from lattice_kernel import (
        CORES,
        FUSION_OPERATOR,
        SAGES_INVARIANTS,
        THRESHOLDS,
        activation_gate,
        collapse_volume_to_cells,
        continuity_state,
        harmonic_integrity,
        lattice_mode,
        neighbor_mean,
        node_hif_mode,
        propagate_cra,
        seed_fields,
        stability_report,
        summarize,
        suxs_ifo,
    )

OKF = {
    "type": "implementation-note",
    "title": "TSLCA Engine — Propagation",
    "description": "Engine contract for C, R, A neighborhood updates under conservation α+β+γ=1.",
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


class TSLPropagationEngine:
    def __init__(self, config: dict[str, Any] | None = None) -> None:
        self.config = dict(config or {})
        self.thresholds = {**THRESHOLDS, **self.config.get("thresholds", {})}

    def step_cra(
        self, C: np.ndarray, R: np.ndarray, A: np.ndarray
    ) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        C2, R2, A2, _, _ = propagate_cra(C, R, A, self.thresholds)
        return C2, R2, A2
