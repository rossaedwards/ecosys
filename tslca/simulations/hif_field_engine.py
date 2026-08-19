"""
---
type: implementation-note
title: TSLCA Engine — Harmonic Integrity Field
description: Engine contract for HIF = cbrt(C·R·A)·Φ(C,R,A) on the 27-node Creation/Integration/Renewal volume.
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
---

## ** APS‑TSLCA-ENGINE-HIF **
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
    "title": "TSLCA Engine — Harmonic Integrity Field",
    "description": "Engine contract for HIF = cbrt(C·R·A)·Φ(C,R,A) on the 27-node Creation/Integration/Renewal volume.",
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
    ],
}


class HIFFieldEngine:
    def __init__(self, config: dict[str, Any] | None = None) -> None:
        self.config = dict(config or {})
        self.thresholds = {**THRESHOLDS, **self.config.get("thresholds", {})}
        seed = int(self.config.get("seed", 369))
        self.rng = np.random.default_rng(seed)
        fields = seed_fields(self.rng)
        self.C, self.R, self.A = fields["C"], fields["R"], fields["A"]
        self.hif, self.phi = harmonic_integrity(self.C, self.R, self.A, self.thresholds)

    def compute(self) -> dict[str, Any]:
        self.hif, self.phi = harmonic_integrity(self.C, self.R, self.A, self.thresholds)
        return {
            "hif": self.hif,
            "phi": self.phi,
            "summary": summarize(self.C, self.R, self.A, self.hif),
        }

    def set_fields(self, C: np.ndarray, R: np.ndarray, A: np.ndarray) -> None:
        self.C, self.R, self.A = C.copy(), R.copy(), A.copy()
        self.compute()
