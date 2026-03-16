"""Auto-generated figure script for VIM.

Section ID : section_XXXII
Figure ID  : section_XXXII_fig32
Label      : Figure 32
Title      : Flow acceleration
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate flow acceleration magnitude |Du/Dt|."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_032_032_flow_acceleration.png"

    n = 60
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1

    ux, uy = edwards_flow_2d(C, R, A)
    dx, dy = c[1] - c[0], r[1] - r[0]
    dux_dc = np.gradient(ux, dy, dx)[1]
    duy_dr = np.gradient(uy, dy, dx)[0]
    ax_mag = np.sqrt((ux * dux_dc) ** 2 + (uy * duy_dr) ** 2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, ax_mag, cmap="plasma", shading="auto")
    ax.set_xlabel("C")
    ax.set_ylabel("R")
    ax.set_title(r"Flow Acceleration $|Du/Dt|$")
    plt.colorbar(im, ax=ax, label="Acceleration")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
