"""Auto-generated figure script for VIM.

Section ID : section_XXXVIII
Figure ID  : section_XXXVIII_fig38
Label      : Figure 38
Title      : Flow field topology
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import edwards_flow_2d


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate flow field topology (streamlines + magnitude)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_038_038_flow_field_topology.png"

    n = 45
    c = np.linspace(0.3, 2.0, n)
    r = np.linspace(0.3, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R) + 0.1

    ux, uy = edwards_flow_2d(C, R, A)
    u_mag = np.sqrt(ux**2 + uy**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, u_mag, cmap="viridis", shading="auto", alpha=0.7)
    ax.streamplot(c, r, ux, uy, color="white", linewidth=1.2, density=1.8)
    ax.set_xlabel("C")
    ax.set_ylabel("R")
    ax.set_title("Flow Field Topology")
    plt.colorbar(im, ax=ax, label=r"$|u|$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
