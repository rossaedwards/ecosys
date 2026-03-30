"""Auto-generated figure script for VIM.

Section ID : section_LXIII
Figure ID  : section_LXIII_fig63
Label      : Figure 63
Title      : Tensor‑energy
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate tensor energy E_T = ‖T‖²."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_063_063_tensor_energy.png"

    n = 80
    x_c, x_t = 1.0, 1.5
    x_f = np.linspace(0.4, 2.5, n)
    x_i = np.linspace(0.4, 2.5, n)
    F, I = np.meshgrid(x_f, x_i)
    beta_val = beta(F, x_c, I, x_t)
    V = balance_potential(beta_val)
    E_T = V + 0.05

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(F, I, E_T, cmap="plasma", shading="auto")
    ax.contour(F, I, E_T, levels=10, colors="white", alpha=0.3)
    ax.set_xlabel(r"$x_f$")
    ax.set_ylabel(r"$x_i$")
    ax.set_title(r"Tensor Energy $E_T = \|T\|^2$")
    plt.colorbar(im, ax=ax, label=r"$E_T$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
