"""Figure A: Balance State Vector basis vectors. Appendix A — Equilibrium Manifold Manifold, HRD Control."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Balance State Vector basis vectors (S,K,G,F) and Equilibrium Manifold."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_071_071.png"

    # 2D projection of 16-D Balance State Vector: Equilibrium Manifold x_f = x_i * x_t / x_c
    x_i = np.linspace(0.5, 2.5, 100)
    x_c, x_t = 1.0, 1.5
    x_f_bliss = x_i * x_t / x_c

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(x_f_bliss, x_i, "g-", lw=2.5, label=r"Equilibrium Manifold $\beta=1$")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Appendix A: Balance State Vector Basis Vectors & Equilibrium Manifold Manifold")
    ax.legend()
    ax.set_xlim(0.3, 2.5)
    ax.set_ylim(0.4, 2.5)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
