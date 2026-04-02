"""Figure 8: Determinant of composite metric vs. coordinate slice.

Section VIII — Composite Metric.
G_ab = g_ab + λ_I I_ab + λ_Φ ∇_a Φ ∇_b Φ
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate determinant of composite metric vs coordinate slice."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_008_008.png"

    n = 50
    x = np.linspace(0.5, 2.5, n)
    y = np.linspace(0.5, 2.5, n)
    X, Y = np.meshgrid(x, y)
    C = 0.8 + 0.4 * np.exp(-((X - 1.5) ** 2 + (Y - 1.0) ** 2) / 2)
    R = 0.9 + 0.2 * np.sin(X) * np.cos(Y)
    A = 0.7 + 0.3 * X * Y / (X + Y + 0.1)

    phi = phi_hif(C, R, A)
    grad_y, grad_x = np.gradient(phi, y[1] - y[0], x[1] - x[0])
    # 2x2 composite metric: g_ab + λ ∇_a Φ ∇_b Φ
    lam = 0.5
    g11 = 1 + lam * grad_x**2
    g22 = 1 + lam * grad_y**2
    g12 = lam * grad_x * grad_y
    det_G = g11 * g22 - g12**2

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(X, Y, det_G, cmap="viridis", shading="auto")
    ax.set_xlabel(r"$x$")
    ax.set_ylabel(r"$y$")
    ax.set_title("Determinant of Composite Metric det(G)")
    plt.colorbar(im, ax=ax, label="det(G)")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
