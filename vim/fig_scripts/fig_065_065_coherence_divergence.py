"""Auto-generated figure script for VIM.

Section ID : section_LXV
Figure ID  : section_LXV_fig65
Label      : Figure 65
Title      : Coherence divergence
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence divergence ∇·(C u)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_065_065_coherence_divergence.png"

    n = 60
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1

    ux, uy = edwards_flow_2d(C, R, A)
    Cu_x, Cu_y = C * ux, C * uy
    dy_sp, dx_sp = r[1] - r[0], c[1] - c[0]
    div_Cu = np.gradient(Cu_x, dy_sp, dx_sp)[1] + np.gradient(Cu_y, dy_sp, dx_sp)[0]

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, div_Cu, cmap="RdBu_r", shading="auto")
    ax.set_xlabel("C")
    ax.set_ylabel("R")
    ax.set_title(r"Coherence Divergence $\nabla \cdot (C u)$")
    plt.colorbar(im, ax=ax, label=r"$\nabla \cdot (Cu)$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
