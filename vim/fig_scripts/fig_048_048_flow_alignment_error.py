"""Auto-generated figure script for VIM.

Section ID : section_XLVIII
Figure ID  : section_XLVIII_fig48
Label      : Figure 48
Title      : Flow‑alignment error
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta, edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate flow-alignment error |u·∇β|."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_048_048_flow_alignment_error.png"

    n = 55
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.5, 2.0, n)
    rAE_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    C, R, A = F, I, 0.5 * (F + I) + 0.1
    beta_val = beta(F, rAE_c, I, rAE_t)
    ux, uy = edwards_flow_2d(C, R, A)
    dy_sp, dx_sp = rAE_i[1] - rAE_i[0], rAE_f[1] - rAE_f[0]
    db_dr, db_dc = np.gradient(beta_val, dy_sp, dx_sp)
    err = np.abs(ux * db_dc + uy * db_dr)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, err, cmap="hot", shading="auto")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title(r"Flow-Alignment Error $|u \cdot \nabla \beta|$")
    plt.colorbar(im, ax=ax, label="Error")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
