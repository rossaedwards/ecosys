"""Figure 16: Geodesic deviation. Section XVI."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Geodesic deviation along path."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_016_016_geodesic_deviation.png"

    s = np.linspace(0, 5, 200)
    dev = 0.1 * np.exp(0.05 * s) * np.sin(2 * s)
    plt.figure(figsize=(8, 5))
    plt.plot(s, dev, "b-", lw=2)
    plt.xlabel("Geodesic parameter s")
    plt.ylabel("Deviation")
    plt.title("Geodesic Deviation")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
