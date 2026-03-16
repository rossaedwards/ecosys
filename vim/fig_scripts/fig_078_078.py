"""Figure H: Divergence identities. Appendix H — Theory of Balance."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Divergence ∇·F_Balance < 0 near Bliss."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_078_078.png"

    n = 50
    X, Y = np.meshgrid(np.linspace(0.5, 2.5, n), np.linspace(0.5, 2.5, n))
    rAE_c, rAE_t = 1.0, 1.5
    b = (X * rAE_c) / (Y * rAE_t)
    V = (b - 1) ** 2
    grad_y, grad_x = np.gradient(V, 0.04, 0.04)
    div = np.gradient(-grad_x, 0.04, axis=1) + np.gradient(-grad_y, 0.04, axis=0)
    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(X, Y, div, cmap="RdBu_r", shading="auto")
    ax.set_title("Appendix H: Divergence (∇·F < 0 near Bliss)")
    plt.colorbar(im, ax=ax)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
