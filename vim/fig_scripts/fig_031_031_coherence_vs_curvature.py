"""Auto-generated figure script for VIM.

Section ID : section_XXXI
Figure ID  : section_XXXI_fig31
Label      : Figure 31
Title      : Coherence vs. curvature
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence vs. curvature scatter/coupling."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_031_031_coherence_vs_curvature.png"

    n = 80
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.5, 2.0, n)
    x_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(x_f, x_i)
    C = 0.5 * (F + I)
    beta_val = beta(F, x_c, I, x_t)
    curvature = balance_potential(beta_val) + 0.05 * (F**2 + I**2)
    coherence = 1.0 / (1.0 + np.abs(beta_val - 1.0))

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.scatter(coherence.ravel(), curvature.ravel(), c=coherence.ravel(), cmap="viridis", s=5, alpha=0.6)
    ax.set_xlabel("Coherence")
    ax.set_ylabel("Curvature")
    ax.set_title("Coherence vs. Curvature (Mid-Dynamics)")
    plt.colorbar(im, ax=ax, label="Coherence")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
