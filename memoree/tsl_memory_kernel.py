## ** APS-TSLCA-MEMOREE-KERNEL **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — TSLCA Memory Kernel & Harmonic Integrity Field (HIF) Runtime
═══════════════════════════════════════════════════════════════════════════════
Implements the mathematical foundation of the Three-Squared-Lattice Cognitive
Architecture (TSLCA) for memory gating, tensor contraction, and invariant
verification.

Governing Equations:
───────────────────
  1. Cognitive Field Tensor:
     F = ∑_{i,j=1}^3 Φ_{ij} (S_i ⊗ S_j)   where S = (SIX, SCX, ICX)

  2. Harmonic Integrity Field (HIF):
     HIF(x,t) = (C(x,t) · R(x,t) · A(x,t))^(1/3) · Φ(C,R,A)
     where Φ(C,R,A) = 1 if C >= C_θ, R >= R_θ, A >= A_θ else 0 (TTG)

  3. SUXS-IFO / USAIC Contraction:
     Φ_unified = Tr(F) = Φ_11 + Φ_22 + Φ_33  (with off-diagonal bridge weights)
═══════════════════════════════════════════════════════════════════════════════
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional, Tuple

from schemas import (
    MemoryType,
    _now,
)

# ─────────────────────────────────────────────────────────────────────────────
# Canonical Lattice Matrix Mapping
# ─────────────────────────────────────────────────────────────────────────────

# 3 Axes
AXIS_SIX = "SIX"  # Somatic Coherence aXis
AXIS_SCX = "SCX"  # Systemic Coherence aXis
AXIS_ICX = "ICX"  # Identity Coherence aXis

TSL_CELLS: Dict[Tuple[str, str], MemoryType] = {
    (AXIS_SIX, AXIS_SIX): MemoryType.SENSORY,      # Pure aXis: Perception & resonance
    (AXIS_SIX, AXIS_SCX): MemoryType.WORKING,      # Bridge: Active session buffer
    (AXIS_SIX, AXIS_ICX): MemoryType.EPISODIC,     # Bridge: Timed contact with self
    (AXIS_SCX, AXIS_SIX): MemoryType.SEMANTIC,     # Bridge: Project knowledge graph
    (AXIS_SCX, AXIS_SCX): MemoryType.META,         # Pure aXis: Invariants & verified axioms
    (AXIS_SCX, AXIS_ICX): MemoryType.QUANTUM,      # Bridge: Physics & simulation state
    (AXIS_ICX, AXIS_SIX): MemoryType.IDENTITY,     # Bridge: SoulJourney lineage
    (AXIS_ICX, AXIS_SCX): MemoryType.PROCEDURAL,   # Bridge: Repeatable workflows
    (AXIS_ICX, AXIS_ICX): MemoryType.GOVERNANCE,   # Pure aXis: Archivus ledger & mandates
}

TYPE_TO_CELL: Dict[MemoryType, Tuple[str, str]] = {
    v: k for k, v in TSL_CELLS.items()
}
# Map legacy alias
TYPE_TO_CELL[MemoryType.CREATIVE] = (AXIS_SCX, AXIS_SIX)


# ─────────────────────────────────────────────────────────────────────────────
# Harmonic Integrity Field (HIF) & Triple Threshold Gate (TTG)
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class TTGThresholds:
    """Triple Threshold Gate operational limits."""
    c_theta: float = 0.50  # Coherence floor
    r_theta: float = 0.50  # Resonance floor
    a_theta: float = 0.50  # Alignment floor

    h_create: float = 0.65     # Gated write threshold
    h_integrate: float = 0.55  # Gated recall threshold
    h_renew: float = 0.35      # Dissolution / decay to Q12 residual threshold


DEFAULT_TTG = TTGThresholds()


def calculate_hif(
    coherence: float,
    resonance: float,
    alignment: float,
    thresholds: TTGThresholds = DEFAULT_TTG,
) -> float:
    """
    Calculate the Harmonic Integrity Field (HIF) value.

    HIF(x,t) = (C · R · A)^(1/3) · Φ(C,R,A)
    """
    c = max(0.0, min(1.0, float(coherence)))
    r = max(0.0, min(1.0, float(resonance)))
    a = max(0.0, min(1.0, float(alignment)))

    # Triple Threshold Gate evaluation
    if c < thresholds.c_theta or r < thresholds.r_theta or a < thresholds.a_theta:
        return 0.0

    # Geometric mean
    product = c * r * a
    if product <= 0.0:
        return 0.0
    return math.pow(product, 1.0 / 3.0)


def evaluate_gate(
    hif: float,
    operation: str = "create",
    thresholds: TTGThresholds = DEFAULT_TTG,
) -> Tuple[bool, str]:
    """
    Evaluate if an operation is permitted under HIF constraints.

    Returns:
        (permitted, reason_message)
    """
    if operation == "create":
        if hif >= thresholds.h_create:
            return True, f"HIF {hif:.3f} >= H_create ({thresholds.h_create:.2f})"
        return False, f"HIF {hif:.3f} refused: below H_create threshold ({thresholds.h_create:.2f})"

    elif operation == "integrate" or operation == "recall":
        if hif >= thresholds.h_integrate:
            return True, f"HIF {hif:.3f} >= H_integrate ({thresholds.h_integrate:.2f})"
        return False, f"HIF {hif:.3f} suppressed: below H_integrate threshold ({thresholds.h_integrate:.2f})"

    elif operation == "renew" or operation == "dissolve":
        if hif < thresholds.h_renew:
            return True, f"HIF {hif:.3f} < H_renew ({thresholds.h_renew:.2f}): eligible for residual decay"
        return False, f"HIF {hif:.3f} maintains active integrity (>= {thresholds.h_renew:.2f})"

    return True, f"Operation '{operation}' evaluated (HIF: {hif:.3f})"


# ─────────────────────────────────────────────────────────────────────────────
# 3×3 Lattice State & SUXS-IFO Prompt Contraction
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class LatticeCellState:
    """State descriptor for a single cell in the 3x3 cognitive lattice."""
    core_i: str
    core_j: str
    memory_type: MemoryType
    count: int = 0
    mean_hif: float = 1.0
    last_updated: datetime = field(default_factory=_now)


@dataclass
class LatticeSnapshot:
    """Complete 3x3 Cognitive Field Tensor snapshot."""
    timestamp: datetime = field(default_factory=_now)
    cells: Dict[str, LatticeCellState] = field(default_factory=dict)
    total_memories: int = 0
    unified_field_trace: float = 0.0

    @classmethod
    def create_empty(cls) -> "LatticeSnapshot":
        snap = cls()
        for (i, j), m_type in TSL_CELLS.items():
            cell_key = f"{i}⊗{j}"
            snap.cells[cell_key] = LatticeCellState(
                core_i=i,
                core_j=j,
                memory_type=m_type,
            )
        return snap


def contract_lattice_context(
    memories_by_type: Dict[MemoryType, List[Dict[str, Any]]],
    active_project: Optional[str] = None,
    max_tokens: int = 4000,
) -> Dict[str, Any]:
    """
    Contract 9-cell memory contents into a unified context payload
    via SUXS-IFO / USAIC tensor contraction.

    Order of priority:
      1. Axiomatic & Meta Invariants (SCX ⊗ SCX)
      2. Active Working Buffer & Open Loops (SIX ⊗ SCX)
      3. Identity & Soul Continuum (ICX ⊗ SIX)
      4. Semantic Knowledge & Project Graphs (SCX ⊗ SIX)
      5. Recent Episodic Turns (SIX ⊗ ICX)
      6. Procedural Recipes & Governance Mandates (ICX ⊗ SCX, ICX ⊗ ICX)
      7. Sensory & Quantum Logs (SIX ⊗ SIX, SCX ⊗ ICX)
    """
    contracted: Dict[str, Any] = {
        "unified_trace": 0.0,
        "active_project": active_project,
        "timestamp": _now().isoformat(),
        "layers": {},
        "total_items": 0,
    }

    total_count = 0
    for m_type in MemoryType:
        items = memories_by_type.get(m_type, [])
        total_count += len(items)
        cell = TYPE_TO_CELL.get(m_type, ("SCX", "SCX"))
        cell_key = f"{cell[0]}⊗{cell[1]}"
        contracted["layers"][m_type.value] = {
            "cell": cell_key,
            "count": len(items),
            "records": items,
        }

    contracted["total_items"] = total_count
    # Trace calculation across diagonal elements
    trace_types = [MemoryType.SENSORY, MemoryType.META, MemoryType.GOVERNANCE]
    trace_val = sum(len(memories_by_type.get(t, [])) for t in trace_types)
    contracted["unified_field_trace"] = float(trace_val)

    return contracted
