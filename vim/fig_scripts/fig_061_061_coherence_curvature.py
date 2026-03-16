"""Auto-generated figure script for VIM.

Section ID : section_LXI
Figure ID  : section_LXI_fig61
Label      : Figure 61
Title      : Coherence curvature
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence-curvature coupling C·R."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_061_061_coherence_curvature.png"

    n = 80
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.4, 2.5, n)
    rAE_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    beta_val = beta(F, rAE_c, I, rAE_t)
    curvature = balance_potential(beta_val) + 0.05
    coherence = 1.0 / (1.0 + np.abs(beta_val - 1.0))
    C_times_R = coherence * curvature

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, C_times_R, cmap="viridis", shading="auto")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Coherence Curvature $C \\cdot R$")
    plt.colorbar(im, ax=ax, label="$C \\cdot R$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
