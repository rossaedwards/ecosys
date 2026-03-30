"""Auto-generated figure script for VIM.

Section ID : section_LV
Figure ID  : section_LV_fig55
Label      : Figure 55
Title      : Harmonic geodesic
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate harmonic geodesic paths to Equilibrium Manifold."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_055_055_harmonic_geodesic.png"

    x_c, x_t = 1.0, 1.5
    k_f, k_i = 0.4, 0.3

    def dynamics(state, t):
        x_f, x_i = state
        b = beta(x_f, x_c, x_i, x_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    t = np.linspace(0, 20, 300)
    inits = [(0.6, 2.0), (2.2, 0.6), (1.0, 1.2), (0.8, 0.7)]

    fig, ax = plt.subplots(figsize=(8, 6))
    for x_f0, x_i0 in inits:
        sol = odeint(dynamics, [x_f0, x_i0], t)
        ax.plot(sol[:, 0], sol[:, 1], alpha=0.8)

    x_i_line = np.linspace(0.4, 2.5, 100)
    ax.plot(x_i_line * x_t / x_c, x_i_line, "r-", lw=2.5, label="Harmonic geodesic (β=1)")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Harmonic Geodesic")
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
