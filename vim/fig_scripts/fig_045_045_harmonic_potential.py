"""Auto-generated figure script for VIM.

Section ID : section_XLV
Figure ID  : section_XLV_fig45
Label      : Figure 45
Title      : Harmonic potential
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate harmonic potential V = (β-1)² heatmap."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_045_045_harmonic_potential.png"

    n = 80
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.4, 2.5, n)
    x_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(x_f, x_i)
    beta_val = beta(F, x_c, I, x_t)
    V = balance_potential(beta_val)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, V, cmap="coolwarm", shading="auto")
    ax.contour(F, I, V, levels=10, colors="black", alpha=0.3)
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title(r"Harmonic Potential $V = (\beta - 1)^2$")
    plt.colorbar(im, ax=ax, label="V")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
