"""Auto-generated figure script for VIM.

Section ID : section_LXVI
Figure ID  : section_LXVI_fig66
Label      : Figure 66
Title      : **Subsections LXVI_a/b/c:** micro‑scale coherence
Auto       : no
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np



def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate micro-scale coherence (a, b, c scales)."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_066_066_subsections_lxvi_a_b_c_micro_scale_coherence.png"

    scale = np.linspace(0.01, 2.0, 150)
    rAE_c, rAE_t = 1.0, 1.5
    C_a = 1.0 / (1.0 + 2.0 * scale)
    C_b = 1.0 / (1.0 + 1.0 * scale**2)
    C_c = np.exp(-0.5 * scale**2)

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(scale, C_a, "b-", lw=2, label="Micro-scale a")
    ax.plot(scale, C_b, "g-", lw=2, label="Micro-scale b")
    ax.plot(scale, C_c, "r-", lw=2, label="Micro-scale c")
    ax.set_xlabel("Scale")
    ax.set_ylabel("Coherence")
    ax.set_title("Micro-Scale Coherence (LXVI a/b/c)")
    ax.legend()
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
