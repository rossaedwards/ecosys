"""Figure F: Flow identities. Appendix F — Experimental Protocols."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Flow identities: Edwards Flow structure."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_076_076.png"

    n = 20
    X, Y = np.meshgrid(np.linspace(0.5, 2.5, n), np.linspace(0.5, 2.5, n))
    C, R, A = 0.9 + 0.1 * np.sin(X), 0.9 + 0.1 * np.cos(Y), 0.8 + 0.2 * X * Y / (X + Y + 0.1)
    from vim_common import edwards_flow_2d
    ux, uy = edwards_flow_2d(C, R, A, 0.1, 0.1)
    fig, ax = plt.subplots(figsize=(8, 6))
    ax.quiver(X, Y, ux, uy, color="steelblue")
    ax.set_title("Appendix F: Flow Identities")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
