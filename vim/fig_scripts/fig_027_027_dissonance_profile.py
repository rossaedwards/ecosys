"""Auto-generated figure script for VIM.

Section ID : section_XXVII
Figure ID  : section_XXVII_fig27
Label      : Figure 27
Title      : Dissonance profile
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate dissonance profile D = |β - 1| across manifold slice."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_027_027_dissonance_profile.png"

    n = 80
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.4, 2.5, n)
    x_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(x_f, x_i)
    beta_val = beta(F, x_c, I, x_t)
    dissonance = np.abs(beta_val - 1.0)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, dissonance, cmap="hot", shading="auto")
    ax.contour(F, I, dissonance, levels=[0.1, 0.3, 0.5, 1.0], colors="white", alpha=0.5)
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title(r"Dissonance Profile $D = |\beta - 1|$")
    plt.colorbar(im, ax=ax, label="Dissonance")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
