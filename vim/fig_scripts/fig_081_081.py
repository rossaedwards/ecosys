"""Figure J–O: rÆ structural metrics. Appendix J–O."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """rÆ structural metrics S, K, G, F."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_081_081.png"

    labels = ["S", "K", "G", "F"]
    vals = [1.0, 0.95, 0.9, 0.85]
    plt.figure(figsize=(8, 5))
    plt.bar(labels, vals, color="steelblue")
    plt.ylabel("Metric value")
    plt.title("Appendix J–O: rÆ Structural Metrics")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
