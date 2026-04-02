"""Figure 3: Edwards Flow vector field (2D slice).

Section III — Edwards Flow.
∇_u u^a = -g^{ab} ∇_b Φ_HIF
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d, phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate Edwards Flow vector field on 2D slice."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_003_003.png"

    n = 25
    x = np.linspace(0.5, 2.5, n)
    y = np.linspace(0.5, 2.5, n)
    X, Y = np.meshgrid(x, y)

    # C, R, A as structural/harmonic/kinetic metrics on grid
    C = 0.8 + 0.4 * np.exp(-((X - 1.5) ** 2 + (Y - 1.0) ** 2) / 2)
    R = 0.9 + 0.3 * np.sin(X) * np.cos(Y)
    A = 0.7 + 0.5 * (X * Y) / (X + Y + 0.1)


    u_x, u_y = edwards_flow_2d(C, R, A, dx=x[1] - x[0], dy=y[1] - y[0])

    fig, ax = plt.subplots(figsize=(8, 7))
    phi = phi_hif(C, R, A)
    ax.pcolormesh(X, Y, phi, cmap="viridis", alpha=0.4, shading="auto")
    ax.quiver(X, Y, u_x, u_y, color="white", scale=50)
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Edwards Flow Vector Field (2D slice)")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
