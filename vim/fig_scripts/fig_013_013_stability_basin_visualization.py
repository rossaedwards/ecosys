"""Figure 13: Stability basin visualization. Section XIII."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Stability basin of Equilibrium attractor."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_013_013_stability_basin_visualization.png"

    n = 80
    F, I = np.meshgrid(np.linspace(0.3, 2.5, n), np.linspace(0.3, 2.5, n))
    x_c, x_t = 1.0, 1.5
    b = (F * x_c) / (I * x_t)
    V = (b - 1) ** 2
    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, V, cmap="viridis", shading="auto")
    ax.contour(F, I, V, levels=[0.1, 0.5, 1.0], colors="white")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Stability Basin Visualization")
    plt.colorbar(im, ax=ax, label="V (potential)")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
