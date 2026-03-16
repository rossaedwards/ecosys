"""Auto-generated figure script for VIM.

Section ID : section_XXVI
Figure ID  : section_XXVI_fig26
Label      : Figure 26
Title      : Gradient flow
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d, phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate gradient flow u = -∇Φ_HIF streamplot."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_026_026_gradient_flow.png"

    n = 40
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1

    ux, uy = edwards_flow_2d(C, R, A)
    phi = phi_hif(C, R, A)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, phi, cmap="viridis", shading="auto", alpha=0.7)
    ax.streamplot(c, r, ux, uy, color="white", linewidth=1.5, density=1.5)
    ax.set_xlabel("C")
    ax.set_ylabel("R")
    ax.set_title(r"Gradient Flow $u = -\nabla \Phi_{\mathrm{HIF}}$")
    plt.colorbar(im, ax=ax, label=r"$\Phi_{\mathrm{HIF}}$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
