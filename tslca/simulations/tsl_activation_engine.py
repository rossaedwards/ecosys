"""
---
type: implementation-note
title: TSLCA Engine — Activation
description: Engine contract for the Triple Threshold Gate and the activation field Ψ acting on HIF.
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

## ** APS‑TSLCA-ENGINE-ACTIVATION **
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
    "title": "TSLCA Engine — Activation",
    "description": "Engine contract for the Triple Threshold Gate and the activation field Ψ acting on HIF.",
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


class TSLActivationEngine:
    def __init__(self, config: dict[str, Any] | None = None) -> None:
        self.config = dict(config or {})
        self.thresholds = {**THRESHOLDS, **self.config.get("thresholds", {})}

    def activate(self, hif: np.ndarray, phi: np.ndarray) -> dict[str, Any]:
        psi = activation_gate(hif, self.thresholds)
        mode = np.empty(hif.shape, dtype=object)
        for i in range(hif.shape[0]):
            for j in range(hif.shape[1]):
                for k in range(hif.shape[2]):
                    mode[i, j, k] = node_hif_mode(float(hif[i, j, k]), self.thresholds)
        silent = psi < 0.5
        n_silent = int(np.sum(silent))
        return {
            "psi": psi,
            "mode": mode,
            "n_create": int(np.sum(mode == "create")),
            "n_integrate": int(np.sum(mode == "integrate")),
            "n_renew": int(np.sum(mode == "renew")),
            "n_liminal": int(np.sum(mode == "liminal")),
            "n_silent": n_silent,
            "lattice_mode": lattice_mode(float(np.mean(hif)), self.thresholds),
        }
