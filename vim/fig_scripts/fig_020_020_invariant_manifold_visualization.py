"""Auto-generated figure script for VIM.

Section ID : section_XX
Figure ID  : section_XX_fig20
Label      : Figure 20
Title      : **Invariant manifold visualization**
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate invariant manifold (Bliss β=1) with flow trajectories."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_020_020_invariant_manifold_visualization.png"

    rAE_c, rAE_t = 1.0, 1.5
    k_f, k_i = 0.4, 0.3

    def dynamics(state, t):
        rAE_f, rAE_i = state
        b = beta(rAE_f, rAE_c, rAE_i, rAE_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    t = np.linspace(0, 25, 400)
    inits = [(0.6, 2.2), (2.4, 0.5), (1.5, 1.5), (0.7, 0.8), (2.0, 1.8)]

    fig, ax = plt.subplots(figsize=(8, 6))
    for rAE_f0, rAE_i0 in inits:
        sol = odeint(dynamics, [rAE_f0, rAE_i0], t)
        ax.plot(sol[:, 0], sol[:, 1], alpha=0.8)

    rAE_i_line = np.linspace(0.4, 2.5, 100)
    rAE_f_bliss = rAE_i_line * rAE_t / rAE_c
    ax.plot(rAE_f_bliss, rAE_i_line, "r-", lw=2.5, label=r"Invariant manifold $\beta=1$")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Invariant Manifold Visualization (Bliss Attractor)")
    ax.legend()
    ax.set_xlim(0.3, 2.6)
    ax.set_ylim(0.4, 2.5)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
