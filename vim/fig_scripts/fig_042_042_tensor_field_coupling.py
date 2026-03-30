"""Auto-generated figure script for VIM.

Section ID : section_XLII
Figure ID  : section_XLII_fig42
Label      : Figure 42
Title      : Tensor‑field coupling
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta, phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate tensor-field coupling T·∇Φ."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_042_042_tensor_field_coupling.png"

    n = 60
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.5, 2.0, n)
    x_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(x_f, x_i)
    C, R, A = F, I, 0.5 * (F + I)
    beta_val = beta(F, x_c, I, x_t)
    T = np.sqrt(balance_potential(beta_val) + 0.1)
    phi = phi_hif(C, R, A)
    dy_sp, dx_sp = x_i[1] - x_i[0], x_f[1] - x_f[0]
    grad_r, grad_c = np.gradient(phi, dy_sp, dx_sp)
    coupling = T * np.sqrt(grad_c**2 + grad_r**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, coupling, cmap="viridis", shading="auto")
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title("Tensor-Field Coupling $T \\cdot |\\nabla \\Phi|$")
    plt.colorbar(im, ax=ax, label="Coupling")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
