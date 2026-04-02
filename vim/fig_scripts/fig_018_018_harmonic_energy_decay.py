"""Auto-generated figure script for VIM.

Section ID : section_XVIII
Figure ID  : section_XVIII_fig18
Label      : Figure 18
Title      : **Harmonic energy decay**
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import balance_potential, beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate harmonic energy decay under Edwards flow."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_018_018_harmonic_energy_decay.png"

    t = np.linspace(0, 15, 300)
    x_c, x_t = 1.0, 1.5
    x_f = 1.0 + 0.8 * np.exp(-0.25 * t)
    x_i = 1.2 - 0.4 * np.exp(-0.2 * t)
    beta_val = beta(x_f, x_c, x_i, x_t)
    E_harm = balance_potential(beta_val)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.semilogy(t, E_harm, "b-", lw=2, label=r"Harmonic energy $E_h$")
    ax.set_xlabel("Time")
    ax.set_ylabel(r"$E_h$ (log scale)")
    ax.set_title("Harmonic Energy Decay Under Edwards Flow")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
