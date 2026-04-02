"""Figure 14: Perturbation decay curve. Section XIV."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Perturbation decay |δβ| → 0."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_014_014_perturbation_decay_curve.png"

    t = np.linspace(0, 15, 300)
    decay = np.exp(-0.3 * t) * (1 + 0.05 * np.sin(2 * t))
    plt.figure(figsize=(8, 5))
    plt.plot(t, decay, "b-", lw=2)
    plt.xlabel("t")
    plt.ylabel(r"$|\delta\beta|$")
    plt.title("Perturbation Decay Curve")
    plt.yscale("log")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
