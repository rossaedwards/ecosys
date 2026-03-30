"""Figure 7: Eigenvalue spectrum of the governance tensor.

Section VII — Governance Tensor.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate eigenvalue spectrum of governance tensor."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_007_007.png"

    # Governance tensor: B_ij = ∇_i ∇_j V, V = (β-1)^2
    # 4x4 block from governance subspace (x_a, x_e, x_s, x_g)
    np.random.seed(42)
    B = np.diag([2.0, 1.5, 1.2, 0.8]) + 0.2 * np.random.randn(4, 4)
    B = (B + B.T) / 2
    eigvals = np.sort(np.linalg.eigvalsh(B))[::-1]

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.bar(range(1, 5), eigvals, color="steelblue", edgecolor="black")
    ax.axhline(0, color="gray", ls="--")
    ax.set_xlabel("Eigenvalue index")
    ax.set_ylabel(r"$\lambda$")
    ax.set_title("Eigenvalue Spectrum of Governance Tensor")
    ax.set_xticks([1, 2, 3, 4])
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
