"""Auto-generated figure script for VIM.

Section ID : section_XXXIX
Figure ID  : section_XXXIX_fig39
Label      : Figure 39
Title      : Spectral density
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np



def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate spectral density of governance tensor eigenvalues."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_039_039_spectral_density.png"

    omega = np.linspace(0.1, 5.0, 200)
    rAE_c, rAE_t = 1.0, 1.5
    rho = 1.0 / (1.0 + (omega - 1.0) ** 2) + 0.3 * np.exp(-omega**2 / 2)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.fill_between(omega, rho, alpha=0.5)
    ax.plot(omega, rho, "b-", lw=2)
    ax.set_xlabel(r"$\omega$ (eigenvalue)")
    ax.set_ylabel(r"$\rho(\omega)$")
    ax.set_title("Spectral Density of Governance Tensor")
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
