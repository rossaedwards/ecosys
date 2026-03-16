"""Auto-generated figure script for VIM.

Section ID : section_LXVII
Figure ID  : section_LXVII_fig67
Label      : Figure 67
Title      : Flow‑tensor coupling
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta, edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate flow-tensor coupling u·T·u."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_067_067_flow_tensor_coupling.png"

    n = 55
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = np.linspace(0.5, 2.0, n)
    rAE_i = np.linspace(0.5, 2.0, n)
    F, I = np.meshgrid(rAE_f, rAE_i)
    C, R, A = F, I, 0.5 * (F + I) + 0.1
    beta_val = beta(F, rAE_c, I, rAE_t)
    T = np.sqrt(balance_potential(beta_val) + 0.1)
    ux, uy = edwards_flow_2d(C, R, A)
    u_mag = np.sqrt(ux**2 + uy**2)
    coupling = T * u_mag**2

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, coupling, cmap="plasma", shading="auto")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title(r"Flow-Tensor Coupling $T \cdot |u|^2$")
    plt.colorbar(im, ax=ax, label="Coupling")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
