"""Auto-generated figure script for VIM.

Section ID : section_XXVIII
Figure ID  : section_XXVIII_fig28
Label      : Figure 28
Title      : Coherence potential
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence potential landscape (Φ_HIF + coherence coupling)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_028_028_coherence_potential.png"

    n = 80
    c = np.linspace(0.2, 2.0, n)
    r = np.linspace(0.2, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.6 * C + 0.4 * R

    phi = phi_hif(C, R, A)
    V_coherence = phi - 0.3 * np.log(C + 0.1)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, V_coherence, cmap="coolwarm", shading="auto")
    ax.contour(C, R, V_coherence, levels=12, colors="black", alpha=0.3)
    ax.set_xlabel("C (coherence)")
    ax.set_ylabel("R")
    ax.set_title("Coherence Potential Landscape")
    plt.colorbar(im, ax=ax, label="Potential")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
