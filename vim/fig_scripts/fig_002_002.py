"""Figure 2: Scalar curvature heatmap under Edwards metric.

Section II — Balance Geometry.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate scalar curvature heatmap under Edwards metric."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_002_002.png"

    n = 100
    x = np.linspace(0.5, 2.5, n)
    y = np.linspace(0.5, 2.5, n)
    X, Y = np.meshgrid(x, y)

    # Scalar curvature R from Edwards metric: R ~ (β-1)^2 Hessian
    rAE_c, rAE_t = 1.0, 1.5
    beta_val = (X * rAE_c) / (Y * rAE_t)
    V = (beta_val - 1) ** 2
    R_scalar = V + 0.05 * (X**2 + Y**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(X, Y, R_scalar, cmap="plasma", shading="auto")
    ax.contour(X, Y, R_scalar, levels=8, colors="white", alpha=0.3)
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Scalar Curvature Heatmap (Edwards Metric)")
    plt.colorbar(im, ax=ax, label="R (scalar curvature)")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
