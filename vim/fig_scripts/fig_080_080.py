"""Figure ∞: Infinite-scale coherence limit. Appendix ∞."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Infinite-scale coherence limit."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_080_080.png"

    Lambda = np.logspace(-1, 2, 100)
    coh = 1 - 0.5 * np.exp(-Lambda / 10)
    plt.figure(figsize=(8, 5))
    plt.semilogx(Lambda, coh, "b-", lw=2)
    plt.xlabel(r"$\Lambda$ (scale)")
    plt.ylabel("Coherence")
    plt.title("Appendix ∞: Infinite-Scale Coherence Limit")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
