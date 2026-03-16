"""Figure 6: Coherence field gradient magnitude.

Section VI — Coherence Field.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence field gradient magnitude."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_006_006.png"

    n = 60
    x = np.linspace(0.5, 2.5, n)
    y = np.linspace(0.5, 2.5, n)
    X, Y = np.meshgrid(x, y)
    C = 0.8 + 0.4 * np.exp(-((X - 1.5) ** 2 + (Y - 1.0) ** 2) / 1.5)
    R = 0.9 + 0.2 * np.sin(2 * X) * np.cos(Y)
    A = 0.7 + 0.3 * X * Y / (X + Y + 0.1)

    phi = phi_hif(C, R, A)
    grad_y, grad_x = np.gradient(phi, y[1] - y[0], x[1] - x[0])
    mag = np.sqrt(grad_x**2 + grad_y**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(X, Y, mag, cmap="hot", shading="auto")
    ax.set_xlabel(r"$x$")
    ax.set_ylabel(r"$y$")
    ax.set_title("Coherence Field Gradient Magnitude |∇Φ|")
    plt.colorbar(im, ax=ax, label="|∇Φ|")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
