"""Figure G: Harmonic operators. Appendix G — Simulation-Driven Calibration."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Harmonic operator spectrum."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_077_077.png"

    omega = np.linspace(0.5, 5, 100)
    H = 1 / (1 + (omega - 2) ** 2)
    plt.figure(figsize=(8, 5))
    plt.plot(omega, H, "b-", lw=2)
    plt.xlabel(r"$\omega$")
    plt.ylabel("|H|")
    plt.title("Appendix G: Harmonic Operators")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
