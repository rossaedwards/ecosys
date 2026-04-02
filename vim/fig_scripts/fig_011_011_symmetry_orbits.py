"""Figure 11: Symmetry orbits. Section XI."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Symmetry orbits under Balance symmetry group G_Balance."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_011_011_symmetry_orbits.png"

    theta = np.linspace(0, 2 * np.pi, 200)
    r = 1 + 0.2 * np.sin(4 * theta)
    for k in range(6):
        phi = theta + k * np.pi / 3
        plt.polar(phi, r, alpha=0.7)
    plt.title("Symmetry Orbits (Balance Group)")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
