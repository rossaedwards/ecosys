"""Auto-generated figure script for VIM.

Section ID : section_LI
Figure ID  : section_LI_fig51
Label      : Figure 51
Title      : Coherence vs. entropy
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import beta


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate coherence vs. entropy trade-off."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_051_051_coherence_vs_entropy.png"

    x_c, x_t = 1.0, 1.5
    x_i = np.linspace(0.5, 2.5, 150)
    x_f = 1.2 * x_i
    beta_val = beta(x_f, x_c, x_i, x_t)
    coherence = 1.0 / (1.0 + np.abs(beta_val - 1.0))
    entropy = -np.log(np.maximum(coherence, 0.01))

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(coherence, entropy, "b-", lw=2)
    ax.set_xlabel("Coherence")
    ax.set_ylabel("Entropy")
    ax.set_title("Coherence vs. Entropy")
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
