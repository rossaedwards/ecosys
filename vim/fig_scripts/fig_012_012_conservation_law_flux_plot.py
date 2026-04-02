"""Figure 12: Conservation law flux plot. Section XII."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Noether current / conservation law flux."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_012_012_conservation_law_flux_plot.png"

    t = np.linspace(0, 10, 200)
    flux = np.exp(-0.2 * t) * (1 + 0.1 * np.sin(3 * t))
    plt.figure(figsize=(8, 5))
    plt.plot(t, flux, "b-", lw=2)
    plt.xlabel("t")
    plt.ylabel("Flux")
    plt.title("Conservation Law Flux Plot")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
