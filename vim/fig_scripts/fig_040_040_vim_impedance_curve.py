"""Auto-generated figure script for VIM.

Section ID : section_XL
Figure ID  : section_XL_fig40
Label      : Figure 40
Title      : VIM impedance curve
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate VIM impedance Z_VIM vs. scale/time."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_040_040_vim_impedance_curve.png"

    t = np.linspace(0, 15, 200)
    x_c, x_t = 1.0, 1.5
    x_f = 1.0 + 0.4 * np.exp(-0.25 * t)
    x_i = 1.2 - 0.3 * np.exp(-0.2 * t)
    beta_val = beta(x_f, x_c, x_i, x_t)
    Z_vim = np.abs(beta_val - 1.0) * (1 + 0.1 * np.exp(-0.1 * t))

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(t, Z_vim, "b-", lw=2, label=r"$Z_{\mathrm{VIM}}$")
    ax.set_xlabel("Time / scale")
    ax.set_ylabel(r"$Z_{\mathrm{VIM}}$")
    ax.set_title("VIM Impedance Curve")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
