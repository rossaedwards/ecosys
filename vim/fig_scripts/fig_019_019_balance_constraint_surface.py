"""Auto-generated figure script for VIM.

Section ID : section_XIX
Figure ID  : section_XIX_fig19
Label      : Figure 19
Title      : **Balance constraint surface**
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate balance constraint surface V = (β - 1)²."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_019_019_balance_constraint_surface.png"

    n = 80
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.4, 2.5, n)
    rAE_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    beta_val = beta(F, rAE_c, I, rAE_t)
    V = balance_potential(beta_val)

    fig = plt.figure(figsize=(10, 7))
    ax = fig.add_subplot(111, projection="3d")
    ax.plot_surface(F, I, V, cmap="viridis", alpha=0.9, shade=True)
    ax.contour(F, I, V, levels=10, colors="white", alpha=0.4, offset=0)
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_zlabel(r"$V = (\beta-1)^2$")
    ax.set_title("Balance Constraint Surface")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
