"""Auto-generated figure script for VIM.

Section ID : section_LX
Figure ID  : section_LX_fig60
Label      : Figure 60
Title      : Pre‑action functional
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta, edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate pre-action functional S_pre = ∫(V + |u|²) dt."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_060_060_pre_action_functional.png"

    n = 60
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.5, 2.0, n)
    x_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(x_f, x_i)
    C, R, A = F, I, 0.5 * (F + I) + 0.1
    beta_val = beta(F, x_c, I, x_t)
    V = balance_potential(beta_val)
    ux, uy = edwards_flow_2d(C, R, A)
    u_sq = ux**2 + uy**2
    S_pre = V + 0.3 * u_sq

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, S_pre, cmap="viridis", shading="auto")
    ax.contour(F, I, S_pre, levels=10, colors="white", alpha=0.3)
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Pre-Action Functional")
    plt.colorbar(im, ax=ax, label=r"$S_{\mathrm{pre}}$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
