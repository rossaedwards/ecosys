"""Auto-generated figure script for VIM.

Section ID : section_XXV
Figure ID  : section_XXV_fig25
Label      : Figure 25
Title      : Stability envelope
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate stability envelope: region where V < threshold."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_025_025_stability_envelope.png"

    n = 100
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.4, 2.5, n)
    rAE_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    beta_val = beta(F, rAE_c, I, rAE_t)
    V = balance_potential(beta_val)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, V, cmap="RdYlGn_r", shading="auto", vmin=0, vmax=1.0)
    ax.contour(F, I, V, levels=[0.1, 0.25, 0.5, 1.0], colors="black", alpha=0.6)
    ax.contour(F, I, V, levels=[0.1], colors="lime", linewidths=2)
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Stability Envelope (V < 0.1 stable)")
    plt.colorbar(im, ax=ax, label=r"$V = (\beta-1)^2$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
