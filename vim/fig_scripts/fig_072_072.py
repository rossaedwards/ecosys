"""Figure B: Metric components. Appendix B — Geometry of rÆ Alphabet."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Metric components g_SS, g_KK, g_GG, g_FF."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_072_072.png"

    n = 50
    X, Y = np.meshgrid(np.linspace(0.5, 2.5, n), np.linspace(0.5, 2.5, n))
    g = 1 + 0.2 * np.exp(-((X - 1.5) ** 2 + (Y - 1) ** 2) / 2)
    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(X, Y, g, cmap="viridis", shading="auto")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_title("Appendix B: Metric Components")
    plt.colorbar(im, ax=ax, label="g")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
