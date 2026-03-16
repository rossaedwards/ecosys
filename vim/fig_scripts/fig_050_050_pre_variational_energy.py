"""Auto-generated figure script for VIM.

Section ID : section_L
Figure ID  : section_L_fig50
Label      : Figure 50
Title      : Pre‑variational energy
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta, phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate pre-variational energy E_pre = V + λΦ."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_050_050_pre_variational_energy.png"

    n = 70
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.5, 2.0, n)
    rAE_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    C, R, A = F, I, 0.5 * (F + I) + 0.1
    beta_val = beta(F, rAE_c, I, rAE_t)
    V = balance_potential(beta_val)
    phi = phi_hif(C, R, A)
    E_pre = V + 0.2 * phi

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, E_pre, cmap="viridis", shading="auto")
    ax.contour(F, I, E_pre, levels=12, colors="white", alpha=0.3)
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Pre-Variational Energy")
    plt.colorbar(im, ax=ax, label=r"$E_{\mathrm{pre}}$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
