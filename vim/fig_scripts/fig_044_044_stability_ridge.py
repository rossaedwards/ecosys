"""Auto-generated figure script for VIM.

Section ID : section_XLIV
Figure ID  : section_XLIV_fig44
Label      : Figure 44
Title      : Stability ridge
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate stability ridge (ridge of V surface along β=1)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_044_044_stability_ridge.png"

    n = 100
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.4, 2.5, n)
    x_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(x_f, x_i)
    beta_val = beta(F, x_c, I, x_t)
    V = balance_potential(beta_val)
    ridge = np.exp(-5 * (beta_val - 1) ** 2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, V, cmap="RdYlGn_r", shading="auto", vmin=0, vmax=1.0)
    ax.contour(F, I, ridge, levels=[0.5], colors="blue", linewidths=2)
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Stability Ridge (β=1)")
    plt.colorbar(im, ax=ax, label=r"$V$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
