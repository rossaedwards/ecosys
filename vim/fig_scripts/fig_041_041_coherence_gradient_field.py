"""Auto-generated figure script for VIM.

Section ID : section_XLI
Figure ID  : section_XLI_fig41
Label      : Figure 41
Title      : Coherence gradient field
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence gradient field ∇C and magnitude."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_041_041_coherence_gradient_field.png"

    n = 50
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1
    phi = phi_hif(C, R, A)
    dy_sp, dx_sp = r[1] - r[0], c[1] - c[0]
    grad_r, grad_c = np.gradient(phi, dy_sp, dx_sp)
    grad_mag = np.sqrt(grad_c**2 + grad_r**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, grad_mag, cmap="plasma", shading="auto")
    ax.quiver(C[::3, ::3], R[::3, ::3], -grad_c[::3, ::3], -grad_r[::3, ::3], color="white")
    ax.set_xlabel("C")
    ax.set_ylabel("R")
    ax.set_title(r"Coherence Gradient Field $\nabla \Phi_{\mathrm{HIF}}$")
    plt.colorbar(im, ax=ax, label=r"$|\nabla \Phi|$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
