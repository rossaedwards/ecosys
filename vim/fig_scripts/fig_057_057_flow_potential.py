"""Auto-generated figure script for VIM.

Section ID : section_LVII
Figure ID  : section_LVII_fig57
Label      : Figure 57
Title      : Flow‑potential
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d, phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate flow-potential coupling u·∇Φ."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_057_057_flow_potential.png"

    n = 50
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1
    phi = phi_hif(C, R, A)
    ux, uy = edwards_flow_2d(C, R, A)
    dy_sp, dx_sp = r[1] - r[0], c[1] - c[0]
    dphi_dr, dphi_dc = np.gradient(phi, dy_sp, dx_sp)
    flow_pot = -(ux * dphi_dc + uy * dphi_dr)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, flow_pot, cmap="viridis", shading="auto")
    ax.set_xlabel("C")
    ax.set_ylabel("R")
    ax.set_title(r"Flow-Potential $u \cdot \nabla \Phi$")
    plt.colorbar(im, ax=ax, label=r"$u \cdot \nabla \Phi$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
