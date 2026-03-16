"""Auto-generated figure script for VIM.

Section ID : section_XXIV
Figure ID  : section_XXIV_fig24
Label      : Figure 24
Title      : Alignment error
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate alignment error |β - 1| decay under harmonic flow."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_024_024_alignment_error.png"

    t = np.linspace(0, 20, 300)
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f = 0.8 + 0.6 * (1 - np.exp(-0.2 * t))
    rAE_i = 1.5 - 0.5 * (1 - np.exp(-0.18 * t))
    beta_val = beta(rAE_f, rAE_c, rAE_i, rAE_t)
    align_err = np.abs(beta_val - 1.0)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.semilogy(t, align_err, "b-", lw=2, label=r"$|\beta - 1|$")
    ax.set_xlabel("Time")
    ax.set_ylabel("Alignment error (log)")
    ax.set_title("Alignment Error Decay Under Harmonic Flow")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
