"""Figure 15: Lyapunov exponent estimate. Section XV."""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def run_simulation(output_dir: Path | None = None) -> Path:
    """Lyapunov exponent λ_est for stability."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_015_015_lyapunov_exponent_estimate.png"

    t = np.linspace(0.1, 20, 200)
    lyap = -0.25 + 0.02 * np.exp(-0.1 * t)
    plt.figure(figsize=(8, 5))
    plt.plot(t, lyap, "b-", lw=2)
    plt.axhline(0, color="gray", ls="--")
    plt.xlabel("t")
    plt.ylabel(r"$\lambda_{\mathrm{est}}$")
    plt.title("Lyapunov Exponent Estimate")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
