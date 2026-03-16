"""Auto-generated figure script for VIM.

Section ID : section_XXIII
Figure ID  : section_XXIII_fig23
Label      : Figure 23
Title      : Tensor contraction norms
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate tensor contraction norms vs. balance parameter."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_023_023_tensor_contraction_norms.png"

    rAE_c, rAE_t = 1.0, 1.5
    rAE_i = np.linspace(0.5, 2.5, 150)
    rAE_f = 1.2 * rAE_i
    beta_val = beta(rAE_f, rAE_c, rAE_i, rAE_t)
    V = balance_potential(beta_val)
    T_norm = np.sqrt(V + 0.1)
    T_contraction = V * (1 + 0.2 * np.sin(3 * rAE_i))

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(rAE_i, T_norm, "b-", lw=2, label=r"$\|T\|$ (tensor norm)")
    ax.plot(rAE_i, T_contraction, "g--", lw=1.5, label="Contraction $T^{ab}g_{ab}$")
    ax.set_xlabel(r"$rAE_i$")
    ax.set_ylabel("Norm")
    ax.set_title("Tensor Contraction Norms")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
