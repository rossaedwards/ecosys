"""Figure I: Stability metrics. Appendix I — Aurphyx Unified Field Equation."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Stability metrics S = ∇²HIF."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_079_079.png"

    n = 50
    X, Y = np.meshgrid(np.linspace(0.5, 2.5, n), np.linspace(0.5, 2.5, n))
    C, R, A = 0.8 + 0.4 * np.exp(-((X - 1.5) ** 2 + (Y - 1) ** 2) / 2), 0.9 + 0.1 * np.sin(X), 0.8 + 0.2 * Y
    from vim_common import phi_hif
    phi = phi_hif(C, R, A)
    lap = np.gradient(np.gradient(phi, 0.04, axis=0), 0.04, axis=0) + np.gradient(np.gradient(phi, 0.04, axis=1), 0.04, axis=1)
    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(X, Y, lap, cmap="viridis", shading="auto")
    ax.set_title("Appendix I: Stability Metrics (∇²Φ)")
    plt.colorbar(im, ax=ax)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
