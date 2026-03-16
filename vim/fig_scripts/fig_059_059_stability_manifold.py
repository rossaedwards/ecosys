"""Auto-generated figure script for VIM.

Section ID : section_LIX
Figure ID  : section_LIX_fig59
Label      : Figure 59
Title      : Stability manifold
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate stability manifold (β=1) with flow trajectories."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_059_059_stability_manifold.png"

    rAE_c, rAE_t = 1.0, 1.5
    k_f, k_i = 0.35, 0.28

    def dynamics(state, t):
        rAE_f, rAE_i = state
        b = beta(rAE_f, rAE_c, rAE_i, rAE_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    t = np.linspace(0, 25, 400)
    inits = [(0.5, 2.0), (2.2, 0.6), (1.2, 1.2), (0.7, 0.9)]

    fig, ax = plt.subplots(figsize=(8, 6))
    for rAE_f0, rAE_i0 in inits:
        sol = odeint(dynamics, [rAE_f0, rAE_i0], t)
        ax.plot(sol[:, 0], sol[:, 1], alpha=0.8)

    rAE_i_line = np.linspace(0.4, 2.5, 100)
    ax.plot(rAE_i_line * rAE_t / rAE_c, rAE_i_line, "r-", lw=2.5, label="Stability manifold β=1")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Stability Manifold")
    ax.legend()
    ax.set_xlim(0.3, 2.5)
    ax.set_ylim(0.4, 2.2)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
