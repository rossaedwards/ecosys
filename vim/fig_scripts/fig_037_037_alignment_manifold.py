"""Auto-generated figure script for VIM.

Section ID : section_XXXVII
Figure ID  : section_XXXVII_fig37
Label      : Figure 37
Title      : Alignment manifold
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate alignment manifold (β=1) with trajectories."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_037_037_alignment_manifold.png"

    x_c, x_t = 1.0, 1.5
    k_f, k_i = 0.35, 0.28

    def dynamics(state, t):
        x_f, x_i = state
        b = beta(x_f, x_c, x_i, x_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    t = np.linspace(0, 30, 500)
    inits = [(0.5, 2.2), (2.3, 0.6), (1.0, 1.0), (0.9, 0.8), (2.0, 1.5)]

    fig, ax = plt.subplots(figsize=(8, 6))
    for x_f0, x_i0 in inits:
        sol = odeint(dynamics, [x_f0, x_i0], t)
        ax.plot(sol[:, 0], sol[:, 1], alpha=0.8)

    x_i_line = np.linspace(0.4, 2.5, 100)
    x_f_bliss = x_i_line * x_t / x_c
    ax.plot(x_f_bliss, x_i_line, "r-", lw=2.5, label="Alignment manifold β=1")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Alignment Manifold")
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
