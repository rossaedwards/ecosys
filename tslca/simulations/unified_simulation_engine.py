"""
---
type: implementation-note
title: TSLCA Engine — Unified Simulation
description: Orchestrates HIF, activation, propagation, stability, continuity, and Balance Manifold into one lattice epoch.
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

## ** APS‑TSLCA-ENGINE-UNIFIED **
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

import json
from pathlib import Path
from typing import Any

import numpy as np

try:
    from .lattice_kernel import (
        CORES,
        FUSION_OPERATOR,
        collapse_volume_to_cells,
        harmonic_integrity,
        summarize,
        suxs_ifo,
    )
except ImportError:
    from lattice_kernel import (
        CORES,
        FUSION_OPERATOR,
        collapse_volume_to_cells,
        harmonic_integrity,
        summarize,
        suxs_ifo,
    )

OKF = {
    "type": "implementation-note",
    "title": "TSLCA Engine — Unified Simulation",
    "description": "Orchestrates HIF, activation, propagation, stability, continuity, and Balance Manifold into one lattice epoch.",
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


class UnifiedSimulationEngine:
    def __init__(
        self,
        hif,
        activation,
        propagation,
        stability,
        continuity,
        balance,
        config: dict[str, Any] | None = None,
    ) -> None:
        self.hif = hif
        self.activation = activation
        self.propagation = propagation
        self.stability = stability
        self.continuity = continuity
        self.balance = balance
        self.config = dict(config or {})
        self.history: list[dict[str, Any]] = []

    def run(self, steps: int = 100) -> dict[str, Any]:
        self.history.clear()
        self.continuity.snapshot(self.hif.C, self.hif.R, self.hif.A, self.hif.hif)
        last_act: dict[str, Any] = {}
        last_stab: dict[str, Any] = {}
        last_bal: dict[str, Any] = {}
        for t in range(int(steps)):
            self.hif.C, self.hif.R, self.hif.A = self.propagation.step_cra(
                self.hif.C, self.hif.R, self.hif.A
            )
            computed = self.hif.compute()
            last_act = self.activation.activate(self.hif.hif, self.hif.phi)
            last_stab = self.stability.assess(self.hif.hif)
            last_bal = self.balance.probe(self.hif.C, self.hif.R, self.hif.A, self.hif.hif)
            self.history.append(
                {
                    "t": t,
                    "HIF_mean": computed["summary"]["HIF_mean"],
                    "class": last_stab["class"],
                    "symmetry_index": last_bal["symmetry_index"],
                }
            )
        cells = collapse_volume_to_cells(self.hif.hif)
        unified = suxs_ifo(cells)
        trace = float(np.trace(cells))
        result = {
            "steps": int(steps),
            "summary": summarize(self.hif.C, self.hif.R, self.hif.A, self.hif.hif),
            "activation": {
                k: last_act[k]
                for k in ("n_create", "n_integrate", "n_renew", "n_silent")
            },
            "stability": {
                "S_lattice": last_stab.get("S_lattice"),
                "class": last_stab.get("class"),
            },
            "balance": last_bal,
            "continuity": self.continuity.continuity_state(),
            "U": unified,
            "trace": trace,
            "fusion": FUSION_OPERATOR,
            "cores": list(CORES),
            "history": self.history,
        }
        return result

    def export(self, out_dir: str) -> None:
        path = Path(out_dir)
        path.mkdir(parents=True, exist_ok=True)
        payload = {
            "okf": OKF,
            "fusion": FUSION_OPERATOR,
            "history": self.history,
            "C": self.hif.C.tolist(),
            "R": self.hif.R.tolist(),
            "A": self.hif.A.tolist(),
            "hif": self.hif.hif.tolist(),
        }
        (path / "unified_epoch.json").write_text(json.dumps(payload, indent=2))

    def get_state(self) -> dict[str, Any]:
        return {
            "C": self.hif.C.copy(),
            "R": self.hif.R.copy(),
            "A": self.hif.A.copy(),
            "history": list(self.history),
        }

    def set_state(self, state: dict[str, Any]) -> None:
        self.hif.set_fields(state["C"], state["R"], state["A"])
        self.history = list(state.get("history", []))
