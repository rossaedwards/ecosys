"""Auto-generated figure script for VIM.

Section ID : section_XXX
Figure ID  : section_XXX_fig30
Label      : Figure 30
Title      : Early VIM damping curve
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate early VIM damping: energy decay with damping coefficient."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_030_030_early_vim_damping_curve.png"

    t = np.linspace(0, 10, 300)
    gamma = 0.3
    x_c, x_t = 1.0, 1.5
    x_f = 1.0 + 0.5 * np.exp(-gamma * t)
    x_i = 1.2 - 0.4 * np.exp(-gamma * t * 0.9)
    beta_val = beta(x_f, x_c, x_i, x_t)
    E = balance_potential(beta_val)
    E_damped = E * np.exp(-0.15 * t)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.semilogy(t, E, "b-", lw=2, label="Energy (undamped)")
    ax.semilogy(t, E_damped, "r--", lw=2, label="Energy (VIM damped)")
    ax.set_xlabel("Time")
    ax.set_ylabel("Energy (log)")
    ax.set_title("Early VIM Damping Curve")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
