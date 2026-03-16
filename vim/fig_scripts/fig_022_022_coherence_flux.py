"""Auto-generated figure script for VIM.

Section ID : section_XXII
Figure ID  : section_XXII_fig22
Label      : Figure 22
Title      : Coherence flux
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d, phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence flux J = C·u (coherence times flow)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_022_022_coherence_flux.png"

    n = 50
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1

    ux, uy = edwards_flow_2d(C, R, A)
    Jx = C * ux
    Jy = R * uy
    J_mag = np.sqrt(Jx**2 + Jy**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, J_mag, cmap="plasma", shading="auto")
    ax.streamplot(c, r, Jx, Jy, color="white", alpha=0.6, density=1.2)
    ax.set_xlabel("C (coherence)")
    ax.set_ylabel("R")
    ax.set_title("Coherence Flux $J = C \\cdot u$")
    plt.colorbar(im, ax=ax, label=r"$|J|$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
