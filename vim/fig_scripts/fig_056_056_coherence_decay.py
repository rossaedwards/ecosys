"""Auto-generated figure script for VIM.

Section ID : section_LVI
Figure ID  : section_LVI_fig56
Label      : Figure 56
Title      : Coherence decay
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence decay C(t) along flow."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_056_056_coherence_decay.png"

    t = np.linspace(0, 15, 200)
    x_c, x_t = 1.0, 1.5
    x_f = 1.0 + 0.5 * np.exp(-0.2 * t)
    x_i = 1.2 - 0.4 * np.exp(-0.18 * t)
    beta_val = beta(x_f, x_c, x_i, x_t)
    coherence = 1.0 / (1.0 + np.abs(beta_val - 1.0))

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(t, coherence, "b-", lw=2, label="Coherence C(t)")
    ax.set_xlabel("Time")
    ax.set_ylabel("Coherence")
    ax.set_title("Coherence Decay Along Flow")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
