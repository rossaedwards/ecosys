"""Auto-generated figure script for VIM.

Section ID : section_XLVII
Figure ID  : section_XLVII_fig47
Label      : Figure 47
Title      : Coherence basin
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence basin (attraction region to Equilibrium Manifold)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_047_047_coherence_basin.png"

    n = 80
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.4, 2.5, n)
    x_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(x_f, x_i)
    beta_val = beta(F, x_c, I, x_t)
    coherence = 1.0 / (1.0 + np.abs(beta_val - 1.0) ** 2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, coherence, cmap="viridis", shading="auto")
    ax.contour(F, I, coherence, levels=[0.5, 0.8, 0.95], colors="white", alpha=0.6)
    x_i_line = np.linspace(0.4, 2.5, 100)
    ax.plot(x_i_line * x_t / x_c, x_i_line, "r-", lw=2, label="Equilibrium Manifold β=1")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Coherence Basin")
    ax.legend()
    plt.colorbar(im, ax=ax, label="Coherence")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
