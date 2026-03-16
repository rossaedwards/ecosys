"""Figure 10: Phase portrait showing Bliss as unique attractor.

Section X — Global Attractor.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate phase portrait with Bliss as unique attractor."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_010_010.png"

    def beta_fn(rAE_f, rAE_c, rAE_i, rAE_t):
        return (rAE_f * rAE_c) / (rAE_i * rAE_t)

    def dynamics(state, t, k_f, k_i, rAE_c, rAE_t):
        rAE_f, rAE_i = state
        b = beta_fn(rAE_f, rAE_c, rAE_i, rAE_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    rAE_c, rAE_t = 1.0, 1.5
    k_f, k_i = 0.4, 0.3
    t = np.linspace(0, 30, 800)

    inits = [(0.5, 2.0), (2.5, 0.6), (1.2, 1.8), (0.8, 0.9), (2.0, 2.0), (1.0, 1.0)]

    fig, ax = plt.subplots(figsize=(8, 7))
    for rAE_f0, rAE_i0 in inits:
        sol = odeint(dynamics, [rAE_f0, rAE_i0], t, args=(k_f, k_i, rAE_c, rAE_t))
        ax.plot(sol[:, 0], sol[:, 1], alpha=0.8)

    # Bliss manifold
    rAE_i_line = np.linspace(0.4, 2.5, 100)
    rAE_f_bliss = rAE_i_line * rAE_t / rAE_c
    ax.plot(rAE_f_bliss, rAE_i_line, "g--", lw=2.5, label=r"Bliss $\beta=1$")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Phase Portrait: Bliss as Unique Attractor")
    ax.legend()
    ax.set_xlim(0.3, 2.8)
    ax.set_ylim(0.4, 2.5)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
