"""Auto-generated figure script for VIM.

Section ID : section_LIV
Figure ID  : section_LIV_fig54
Label      : Figure 54
Title      : Tensor‑field divergence
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta, edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate tensor-field divergence ∇·(T u)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_054_054_tensor_field_divergence.png"

    n = 55
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.5, 2.0, n)
    rAE_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    C, R, A = F, I, 0.5 * (F + I) + 0.1
    beta_val = beta(F, rAE_c, I, rAE_t)
    T = np.sqrt(balance_potential(beta_val) + 0.1)
    ux, uy = edwards_flow_2d(C, R, A)
    Tu_x, Tu_y = T * ux, T * uy
    dy_sp, dx_sp = rAE_i[1] - rAE_i[0], rAE_f[1] - rAE_f[0]
    div_Tu = np.gradient(Tu_x, dy_sp, dx_sp)[1] + np.gradient(Tu_y, dy_sp, dx_sp)[0]

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, div_Tu, cmap="RdBu_r", shading="auto")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title(r"Tensor-Field Divergence $\nabla \cdot (T u)$")
    plt.colorbar(im, ax=ax, label=r"$\nabla \cdot (Tu)$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
