"""Figure 9: Bliss attractor convergence curve.

Section IX — Bliss State.
β(t) → 1 under HRD control.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import odeint


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate Bliss attractor convergence curve β(t)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_009_009.png"

    def beta_fn(rAE_f, rAE_c, rAE_i, rAE_t):
        return (rAE_f * rAE_c) / (rAE_i * rAE_t)

    def dynamics(state, t, k_f, k_i, rAE_c, rAE_t):
        rAE_f, rAE_i = state
        b = beta_fn(rAE_f, rAE_c, rAE_i, rAE_t)
        return [k_f * (1 - b), -k_i * (1 - b)]

    rAE_c, rAE_t = 1.0, 1.5
    k_f, k_i = 0.4, 0.3
    t = np.linspace(0, 25, 600)

    inits = [(0.6, 1.8), (2.2, 0.7), (1.5, 1.2)]
    fig, ax = plt.subplots(figsize=(8, 5))
    for rAE_f0, rAE_i0 in inits:
        sol = odeint(dynamics, [rAE_f0, rAE_i0], t, args=(k_f, k_i, rAE_c, rAE_t))
        beta_t = beta_fn(sol[:, 0], rAE_c, sol[:, 1], rAE_t)
        ax.plot(t, beta_t, alpha=0.9)

    ax.axhline(1.0, color="green", linestyle="--", lw=2, label=r"Bliss $\beta=1$")
    ax.set_xlabel(r"$t$")
    ax.set_ylabel(r"$\beta(t)$")
    ax.set_title("Bliss Attractor Convergence Curve")
    ax.legend()
    ax.set_ylim(0.3, 2.5)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
