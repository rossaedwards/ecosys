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
    """Generate coherence basin (attraction region to Bliss)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_047_047_coherence_basin.png"

    n = 80
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.4, 2.5, n)
    rAE_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    beta_val = beta(F, rAE_c, I, rAE_t)
    coherence = 1.0 / (1.0 + np.abs(beta_val - 1.0) ** 2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, coherence, cmap="viridis", shading="auto")
    ax.contour(F, I, coherence, levels=[0.5, 0.8, 0.95], colors="white", alpha=0.6)
    rAE_i_line = np.linspace(0.4, 2.5, 100)
    ax.plot(rAE_i_line * rAE_t / rAE_c, rAE_i_line, "r-", lw=2, label="Bliss β=1")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
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
