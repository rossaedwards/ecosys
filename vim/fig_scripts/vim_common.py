"""
VIM Common — Shared physics and plotting utilities for VIM figure scripts.

Equations from appendix_a, vim_section_lxx, vim_section_lxxi, EQUATION_EXTRACTION_SUPPLEMENT.
"""

from __future__ import annotations

import numpy as np
from typing import Tuple

# Physical constants (MASTER_SYNC_CONVERG)
D_S = 1.37  # Topological spectral dimension
T_2 = 1600e-6  # Coherence window (s)
ETA = 5.3  # Fractal scaling bias
PBG = 0.21  # Photonic band gap


def beta(rAE_f: np.ndarray, rAE_c: np.ndarray, rAE_i: np.ndarray, rAE_t: np.ndarray) -> np.ndarray:
    """Balance coefficient: β = (rAE_f * rAE_c) / (rAE_i * rAE_t). Bliss when β=1."""
    return (rAE_f * rAE_c) / np.maximum(rAE_i * rAE_t, 1e-12)


def phi_hif(C: np.ndarray, R: np.ndarray, A: np.ndarray) -> np.ndarray:
    """HIF potential: Φ_HIF = -ln(∛(C·R·A)). High HIF = low potential."""
    return -np.log(np.maximum(np.cbrt(C * R * A), 1e-12))


def balance_potential(beta_val: np.ndarray) -> np.ndarray:
    """Balance potential: V = (β - 1)²."""
    return (beta_val - 1) ** 2


def edwards_flow_2d(C: np.ndarray, R: np.ndarray, A: np.ndarray, dx: float = 0.1, dy: float = 0.1) -> Tuple[np.ndarray, np.ndarray]:
    """Edwards Flow: ∇_u u^a = -g^{ab} ∇_b Φ_HIF. Returns (u_x, u_y) on 2D grid."""
    phi = phi_hif(C, R, A)
    grad_y, grad_x = np.gradient(phi, dy, dx)
    return -grad_x, -grad_y


def hrd_control_step(rAE_f: float, rAE_i: float, beta_val: float, k_f: float = 0.4, k_i: float = 0.3) -> Tuple[float, float]:
    """HRD control law: drAE_f/dt = k_f(1-β), drAE_i/dt = -k_i(1-β)."""
    err = 1.0 - beta_val
    return rAE_f + k_f * err, rAE_i - k_i * err
