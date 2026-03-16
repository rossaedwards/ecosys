"""Figure A: rÆ basis vectors. Appendix A — Bliss Manifold, HRD Control."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """rÆ basis vectors (S,K,G,F) and Bliss manifold."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_071_071.png"

    # 2D projection of 16-D rÆ: Bliss manifold rAE_f = rAE_i * rAE_t / rAE_c
    rAE_i = np.linspace(0.5, 2.5, 100)
    rAE_c, rAE_t = 1.0, 1.5
    rAE_f_bliss = rAE_i * rAE_t / rAE_c

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(rAE_f_bliss, rAE_i, "g-", lw=2.5, label=r"Bliss $\beta=1$")
    ax.set_xlabel(r"$rAE_f$")
    ax.set_ylabel(r"$rAE_i$")
    ax.set_title("Appendix A: rÆ Basis Vectors & Bliss Manifold")
    ax.legend()
    ax.set_xlim(0.3, 2.5)
    ax.set_ylim(0.4, 2.5)
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
