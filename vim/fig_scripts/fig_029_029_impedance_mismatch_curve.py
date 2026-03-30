"""Auto-generated figure script for VIM.

Section ID : section_XXIX
Figure ID  : section_XXIX_fig29
Label      : Figure 29
Title      : Impedance mismatch curve
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate impedance mismatch Z = |β - 1| vs. x_i."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_029_029_impedance_mismatch_curve.png"

    x_c, x_t = 1.0, 1.5
    x_i = np.linspace(0.4, 2.5, 200)
    x_f_vals = [0.8, 1.0, 1.2, 1.5, 2.0]

    fig, ax = plt.subplots(figsize=(8, 6))
    for x_f in x_f_vals:
        beta_val = beta(x_f, x_c, x_i, x_t)
        Z = np.abs(beta_val - 1.0)
        ax.plot(x_i, Z, lw=2, label=rf"$x_f$={x_f}")

    ax.set_xlabel(r"$x_i$ (impedance)")
    ax.set_ylabel(r"Impedance mismatch $|\beta - 1|$")
    ax.set_title("Impedance Mismatch Curve")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
