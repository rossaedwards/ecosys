"""Figure 10: Phase portrait showing Equilibrium Manifold as unique attractor.

Section X — Global Attractor.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate phase portrait with Equilibrium Manifold as unique attractor."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_010_010.png"

    def beta_fn(x_f, x_c, x_i, x_t):
        return (x_f * x_c) / (x_i * x_t)

    def dynamics(state, t, k_f, k_i, x_c, x_t):
        x_f, x_i = state
        b = beta_fn(x_f, x_c, x_i, x_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    x_c, x_t = 1.0, 1.5
    k_f, k_i = 0.4, 0.3
    t = np.linspace(0, 30, 800)

    inits = [(0.5, 2.0), (2.5, 0.6), (1.2, 1.8), (0.8, 0.9), (2.0, 2.0), (1.0, 1.0)]

    fig, ax = plt.subplots(figsize=(8, 7))
    for x_f0, x_i0 in inits:
        sol = odeint(dynamics, [x_f0, x_i0], t, args=(k_f, k_i, x_c, x_t))
        ax.plot(sol[:, 0], sol[:, 1], alpha=0.8)

    # Equilibrium Manifold
    x_i_line = np.linspace(0.4, 2.5, 100)
    x_f_bliss = x_i_line * x_t / x_c
    ax.plot(x_f_bliss, x_i_line, "g--", lw=2.5, label=r"Equilibrium Manifold $\beta=1$")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Phase Portrait: Equilibrium Manifold as Unique Attractor")
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
