"""Figure Ω: Omega-limit coherence. Appendix Ω."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Omega-limit coherence ω_becoming = ω_finality · ω_draft."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_082_082.png"

    t = np.linspace(0, 10, 200)
    omega = 1 - 0.2 * np.exp(-0.3 * t)
    plt.figure(figsize=(8, 5))
    plt.plot(t, omega, "b-", lw=2)
    plt.axhline(1, color="green", ls="--", label="Equilibrium Manifold")
    plt.xlabel("t")
    plt.ylabel(r"$\omega$")
    plt.title("Appendix Ω: Omega-Limit Coherence")
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
