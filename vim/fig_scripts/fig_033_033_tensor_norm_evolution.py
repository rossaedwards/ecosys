"""Auto-generated figure script for VIM.

Section ID : section_XXXIII
Figure ID  : section_XXXIII_fig33
Label      : Figure 33
Title      : Tensor norm evolution
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate tensor norm evolution along flow."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_033_033_tensor_norm_evolution.png"

    t = np.linspace(0, 15, 200)
    x_c, x_t = 1.0, 1.5
    x_f = 1.0 + 0.6 * np.exp(-0.2 * t)
    x_i = 1.3 - 0.5 * np.exp(-0.18 * t)
    beta_val = beta(x_f, x_c, x_i, x_t)
    V = balance_potential(beta_val)
    T_norm = np.sqrt(V + 0.1)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(t, T_norm, "b-", lw=2, label=r"$\|T\|$")
    ax.set_xlabel("Time")
    ax.set_ylabel("Tensor norm")
    ax.set_title("Tensor Norm Evolution")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
