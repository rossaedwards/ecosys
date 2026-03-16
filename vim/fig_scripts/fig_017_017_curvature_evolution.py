"""Auto-generated figure script for VIM.

Section ID : section_XVII
Figure ID  : section_XVII_fig17
Label      : Figure 17
Title      : **Curvature evolution**
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta, balance_potential


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate curvature evolution along geodesic flow."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_017_017_curvature_evolution.png"

    rAE_c, rAE_t = 1.0, 1.5
    t = np.linspace(0, 20, 200)
    rAE_f = 0.5 + 1.5 * (1 - np.exp(-0.2 * t))
    rAE_i = 1.5 - 0.5 * (1 - np.exp(-0.15 * t))
    beta_val = beta(rAE_f, rAE_c, rAE_i, rAE_t)
    V = balance_potential(beta_val)
    curvature = V + 0.1 * np.exp(-0.1 * t)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(t, curvature, "b-", lw=2, label="Curvature R(t)")
    ax.plot(t, V, "g--", alpha=0.7, label=r"Balance potential $(\beta-1)^2$")
    ax.set_xlabel("Time (flow parameter)")
    ax.set_ylabel("Curvature")
    ax.set_title("Curvature Evolution Along Geodesic Flow")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
