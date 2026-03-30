"""Figure 1: 2D projection of the 16-D Balance State Vector manifold curvature field.

Section I — The Balance State Vector Manifold.
Uses reduced 2D slice of curvature tensor from 16-D Balance State Vector manifold.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate 2D projection of 16-D Balance State Vector curvature field."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_001_001.png"

    # 2D slice: (x_f, x_i) plane — curvature-like scalar from balance geometry
    n = 80
    x_f = np.linspace(0.3, 3.0, n)
    x_i = np.linspace(0.3, 3.0, n)
    F, I = np.meshgrid(x_f, x_i)
    x_c, x_t = 1.0, 1.5


    def curvature_slice(f, i):
        """Reduced curvature from 16-D Balance State Vector: R ~ (β-1)^2 + structure."""
        b = (f * x_c) / (i * x_t)
        V = (b - 1) ** 2
        return V + 0.1 * np.exp(-((f - 1.5) ** 2 + (i - 1.0) ** 2) / 2)

    R_2d = curvature_slice(F, I)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, R_2d, cmap="viridis", shading="auto")
    ax.contour(F, I, R_2d, levels=[0.1, 0.5, 1.0], colors="white", alpha=0.5)
    ax.set_xlabel(r"$x_f$ (flux)")
    ax.set_ylabel(r"$x_i$ (impedance)")
    ax.set_title("2D Projection of 16-D Balance State Vector Manifold Curvature Field")
    plt.colorbar(im, ax=ax, label="Curvature (reduced)")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
