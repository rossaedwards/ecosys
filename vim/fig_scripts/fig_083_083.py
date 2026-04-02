"""Figure P–Z: Balance State Vector dynamics, operators, invariants. Appendix P–Z."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Balance State Vector dynamics meta-cycle."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_083_083.png"

    t = np.linspace(0, 4 * np.pi, 200)
    cycle = 0.5 * (1 + np.sin(t))
    plt.figure(figsize=(8, 5))
    plt.plot(t, cycle, "b-", lw=2)
    plt.xlabel("t")
    plt.ylabel("Meta-cycle")
    plt.title("Appendix P–Z: Balance State Vector Dynamics Meta-Cycle")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
