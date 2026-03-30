"""Figure C: Tensor identities. Appendix C — Tetra-Hexa as Topological Computer."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Tensor identities B_ij = ∇_i ∇_j V."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_073_073.png"

    n = 40
    F, I = np.meshgrid(np.linspace(0.5, 2.5, n), np.linspace(0.5, 2.5, n))
    x_c, x_t = 1.0, 1.5
    b = (F * x_c) / (I * x_t)
    V = (b - 1) ** 2
    gy, gx = np.gradient(V, 0.05, 0.05)
    B_ij = np.gradient(gx, 0.05, axis=1) + np.gradient(gy, 0.05, axis=0)
    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, B_ij, cmap="RdBu_r", shading="auto")
    ax.set_title("Appendix C: Tensor Identities (Hessian of V)")
    plt.colorbar(im, ax=ax)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
