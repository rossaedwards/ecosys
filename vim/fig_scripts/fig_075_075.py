"""Figure E: Alignment invariants. Appendix E — rÆ-Cell Physical Implementation."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Alignment invariants on Bliss manifold."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_075_075.png"

    t = np.linspace(0, 10, 200)
    A = 1 - 0.3 * np.exp(-0.5 * t)
    plt.figure(figsize=(8, 5))
    plt.plot(t, A, "b-", lw=2)
    plt.axhline(1, color="green", ls="--", label="Bliss")
    plt.xlabel("t")
    plt.ylabel("Alignment invariant")
    plt.title("Appendix E: Alignment Invariants")
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
