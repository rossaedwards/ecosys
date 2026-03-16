"""Figure 5: HIF potential landscape (Φ_HIF).

Section V — HIF Potential.
Φ_HIF = -ln(∛(C·R·A))
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from vim_common import phi_hif


def run_simulation(output_dir: Path | None = None) -> Path:
    """Generate HIF potential landscape."""
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "fig_005_005.png"

    n = 80
    c = np.linspace(0.2, 2.0, n)
    r = np.linspace(0.2, 2.0, n)
    C, R = np.meshgrid(c, r)
    A = 0.5 * (C + R)  # kinetic alignment

    phi = phi_hif(C, R, A)

    fig, ax = plt.subplots(figsize=(8, 6))
    im = ax.pcolormesh(C, R, phi, cmap="coolwarm", shading="auto")
    ax.contour(C, R, phi, levels=12, colors="black", alpha=0.3)
    ax.set_xlabel("C (structural)")
    ax.set_ylabel("R (harmonic)")
    ax.set_title(r"HIF Potential Landscape $\Phi_{\mathrm{HIF}} = -\ln(\sqrt[3]{C \cdot R \cdot A})$")
    plt.colorbar(im, ax=ax, label=r"$\Phi_{\mathrm{HIF}}$")
    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close()

    print(f"[VIM FIGURE] Generated: {out_path}")
    return out_path


if __name__ == "__main__":
    run_simulation()
