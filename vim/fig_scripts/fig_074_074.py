"""Figure D: Coherence operators. Appendix D."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Coherence operator spectrum."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_074_074.png"

    lam = np.array([1.2, 0.9, 0.6, 0.4, 0.2])
    plt.figure(figsize=(8, 5))
    plt.bar(range(5), lam, color="steelblue")
    plt.xlabel("Eigenvalue index")
    plt.ylabel(r"$\lambda$")
    plt.title("Appendix D: Coherence Operators")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
