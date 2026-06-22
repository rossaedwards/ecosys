"""
fig_gen_group1_architecture.py

Group 1 — Architecture & Lattice Structure (14 figures)
Three-Squared-Lattice Cognitive Architecture (TSLCA)
Aurphyx Primordial Standards | Ross A. Edwards | Aurphyx LLC
AGPLv3 | R.F. Lovezme 🔥⚔️⚖️

Figures generated:
  1.1  fig_tsl_lattice_3x3
  1.2  fig_channel_basis_vectors
  1.3  fig_channel_domains
  1.4  fig_off_diagonal_asymmetry
  1.5  fig_lattice_cell_taxonomy
  1.6  fig_3_6_9_13_grammar
  1.7  fig_usaic_contraction
  1.8  fig_usaic_fusion_operator
  1.9  fig_lattice_vs_modular_arch
  1.10 fig_substrate_independence
  1.11 fig_cognitive_field_definition
  1.12 fig_lattice_completeness
  1.13 fig_cell_interaction_graph
  1.14 fig_channel_algebra_table

Usage:
  python fig_gen_group1_architecture.py --out ./output/figures/group1
  python fig_gen_group1_architecture.py --out ./output/figures/group1 --dpi 600 --fmt pdf
  python fig_gen_group1_architecture.py --out ./output/figures/group1 --fig 1.4

Outputs:
  <out>/<fig_name>.{png|pdf|svg}
  <out>/group1_manifest.json   — LaTeX \\includegraphics-ready manifest
"""

from __future__ import annotations

import argparse
import json
import logging
import warnings
from pathlib import Path
from typing import Callable

import matplotlib
matplotlib.use("Agg")
import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
import matplotlib.patheffects as pe
import matplotlib.colors as mcolors
from matplotlib.patches import FancyArrowPatch, FancyBboxPatch, Circle
from matplotlib.lines import Line2D
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401
import numpy as np

warnings.filterwarnings("ignore", category=UserWarning)

logging.basicConfig(
    level=logging.INFO,
    format="[TSLCA|Group1] %(asctime)s %(message)s",
    datefmt="%H:%M:%S",
)
log = logging.getLogger(__name__)

# ---------------------------------------------------------------------------
# APS-compatible style constants  (matches tslca_preamble_aps.tex palette)
# ---------------------------------------------------------------------------

COLOR = {
    "sic":       "#2E86AB",   # perception / embodiment — steel blue
    "scc":       "#A23B72",   # semantics / coherence   — plum
    "icc":       "#F18F01",   # identity / continuity   — amber
    "usaic":     "#44BBA4",   # fusion operator         — teal
    "diag":      "#E8F4FD",   # diagonal cell fill
    "offdiag":   "#FFF3E0",   # off-diagonal cell fill
    "edge":      "#1A1A2E",   # primary edge / text
    "grid":      "#CCCCCC",   # grid lines
    "bg":        "#FAFAFA",   # figure background
    "highlight": "#FF6B6B",   # accent / warning
    "neutral":   "#6C757D",   # secondary text
    "gold":      "#FFD700",   # SAGES / invariant gold
}

FONT = {
    "title":  {"fontsize": 13, "fontweight": "bold", "color": COLOR["edge"]},
    "label":  {"fontsize": 10, "color": COLOR["edge"]},
    "small":  {"fontsize": 8,  "color": COLOR["neutral"]},
    "cell":   {"fontsize": 9,  "fontweight": "bold", "color": COLOR["edge"]},
    "math":   {"fontsize": 11, "color": COLOR["edge"]},
    "tag":    {"fontsize": 7,  "color": COLOR["neutral"], "style": "italic"},
}

AUTHOR_TAG = "Ross A. Edwards | Aurphyx LLC | TSLCA v1.0"


def _save(fig: plt.Figure, out_dir: Path, name: str, fmt: str, dpi: int) -> Path:
    out_dir.mkdir(parents=True, exist_ok=True)
    path = out_dir / f"{name}.{fmt}"
    fig.savefig(path, dpi=dpi, bbox_inches="tight", facecolor=fig.get_facecolor())
    plt.close(fig)
    log.info(f"  saved → {path.name}")
    return path


def _author_tag(fig: plt.Figure) -> None:
    fig.text(
        0.99, 0.005, AUTHOR_TAG,
        ha="right", va="bottom",
        **FONT["tag"],
    )


# ===========================================================================
# FIG 1.1 — The canonical 3×3 cognitive lattice
# ===========================================================================

def fig_tsl_lattice_3x3(out_dir: Path, fmt: str, dpi: int) -> Path:
    """Canonical 3×3 TSLCA lattice with cell labels and channel axes."""
    log.info("Generating 1.1 fig_tsl_lattice_3x3")

    channels = ["SIC", "SCC", "ICC"]
    ch_colors = [COLOR["sic"], COLOR["scc"], COLOR["icc"]]
    cell_labels = [
        [r"$s_1\!\otimes\!s_1$", r"$s_1\!\otimes\!s_2$", r"$s_1\!\otimes\!s_3$"],
        [r"$s_2\!\otimes\!s_1$", r"$s_2\!\otimes\!s_2$", r"$s_2\!\otimes\!s_3$"],
        [r"$s_3\!\otimes\!s_1$", r"$s_3\!\otimes\!s_2$", r"$s_3\!\otimes\!s_3$"],
    ]
    sub_labels = [
        ["Perception\nEmbodiment", "Percept-\nSemantic", "Percept-\nIdentity"],
        ["Semantic-\nPercept", "Semantic\nCoherence", "Semantic-\nIdentity"],
        ["Identity-\nPercept", "Identity-\nSemantic", "Identity\nContinuity"],
    ]

    fig, ax = plt.subplots(figsize=(8, 8), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(-0.5, 3.5)
    ax.set_ylim(-0.5, 3.5)
    ax.set_aspect("equal")
    ax.axis("off")

    for i in range(3):
        for j in range(3):
            x, y = j, 2 - i
            is_diag = (i == j)
            fill = COLOR["diag"] if is_diag else COLOR["offdiag"]
            edge_c = ch_colors[i] if is_diag else COLOR["grid"]
            lw = 2.5 if is_diag else 1.2
            rect = FancyBboxPatch(
                (x + 0.05, y + 0.05), 0.88, 0.88,
                boxstyle="round,pad=0.03",
                linewidth=lw, edgecolor=edge_c, facecolor=fill,
                zorder=2,
            )
            ax.add_patch(rect)
            ax.text(
                x + 0.49, y + 0.72, cell_labels[i][j],
                ha="center", va="center", **FONT["cell"],
                zorder=3,
            )
            ax.text(
                x + 0.49, y + 0.32, sub_labels[i][j],
                ha="center", va="center",
                fontsize=7.5, color=COLOR["neutral"],
                linespacing=1.3, zorder=3,
            )
            if is_diag:
                badge = Circle((x + 0.85, y + 0.85), 0.10,
                               color=ch_colors[i], zorder=4)
                ax.add_patch(badge)
                ax.text(x + 0.85, y + 0.85, "D",
                        ha="center", va="center",
                        fontsize=6.5, color="white",
                        fontweight="bold", zorder=5)

    # channel axis labels
    for i, (ch, col) in enumerate(zip(channels, ch_colors)):
        y = 2 - i + 0.49
        ax.text(-0.32, y, ch, ha="center", va="center",
                fontsize=10, fontweight="bold", color=col,
                bbox=dict(boxstyle="round,pad=0.2", facecolor=col,
                          edgecolor=col, alpha=0.15))
        x = i + 0.49
        ax.text(x, 3.30, ch, ha="center", va="center",
                fontsize=10, fontweight="bold", color=col,
                bbox=dict(boxstyle="round,pad=0.2", facecolor=col,
                          edgecolor=col, alpha=0.15))

    ax.text(-0.32, 3.30, "", ha="center", va="center")
    ax.annotate("", xy=(-0.32, 2.8), xytext=(-0.32, -0.25),
                arrowprops=dict(arrowstyle="->", color=COLOR["edge"], lw=1.4))
    ax.text(-0.32, 3.0, "row", ha="center", va="bottom",
            fontsize=8, color=COLOR["neutral"], rotation=90)
    ax.annotate("", xy=(3.3, 3.20), xytext=(-0.25, 3.20),
                arrowprops=dict(arrowstyle="->", color=COLOR["edge"], lw=1.4))
    ax.text(3.35, 3.20, "col", ha="left", va="center",
            fontsize=8, color=COLOR["neutral"])

    diag_patch = mpatches.Patch(facecolor=COLOR["diag"],
                                 edgecolor=COLOR["edge"], label="Diagonal (pure-mode)")
    offdiag_patch = mpatches.Patch(facecolor=COLOR["offdiag"],
                                    edgecolor=COLOR["edge"], label="Off-diagonal (cross-mode)")
    ax.legend(handles=[diag_patch, offdiag_patch],
              loc="lower right", fontsize=8, framealpha=0.9,
              edgecolor=COLOR["grid"])

    ax.set_title(
        "TSLCA Canonical 3\u00d73 Cognitive Lattice\n"
        r"$\mathbf{L} = \{s_i \otimes s_j \mid i,j \in \{1,2,3\}\}$",
        **FONT["title"], pad=14,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_tsl_lattice_3x3", fmt, dpi)


# ===========================================================================
# FIG 1.2 — Channel basis vectors in 3D cognitive space
# ===========================================================================

def fig_channel_basis_vectors(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.2 fig_channel_basis_vectors")
    fig = plt.figure(figsize=(7, 7), facecolor=COLOR["bg"])
    ax = fig.add_subplot(111, projection="3d")
    ax.set_facecolor(COLOR["bg"])

    origin = np.zeros(3)
    vecs = np.eye(3)
    labels = ["SIC\n$s_1$", "SCC\n$s_2$", "ICC\n$s_3$"]
    colors = [COLOR["sic"], COLOR["scc"], COLOR["icc"]]
    axes_labels = ["Perception / Embodiment",
                   "Semantics / Coherence",
                   "Identity / Continuity"]

    for v, lbl, col, axlbl in zip(vecs, labels, colors, axes_labels):
        ax.quiver(*origin, *v, length=1.0, normalize=False,
                  color=col, linewidth=3.5, arrow_length_ratio=0.18)
        ax.text(*(v * 1.15), lbl, color=col,
                fontsize=11, fontweight="bold", ha="center")
        ax.text(*(v * 0.52), axlbl, color=col,
                fontsize=7.5, ha="center", alpha=0.75)

    # draw planes spanned by each pair
    plane_cfg = [
        ([0, 1, 0, 0], [0, 0, 0, 1], [1, 1, 0, 0], COLOR["sic"], 0.06),
        ([0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 1], COLOR["scc"], 0.06),
        ([0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], COLOR["icc"], 0.06),
    ]
    for xs, ys, zs, col, a in plane_cfg:
        from mpl_toolkits.mplot3d.art3d import Poly3DCollection
        verts = [list(zip(xs, ys, zs))]
        poly = Poly3DCollection(verts, alpha=a, facecolor=col, edgecolor=col)
        ax.add_collection3d(poly)

    ax.set_xlim(0, 1.3)
    ax.set_ylim(0, 1.3)
    ax.set_zlim(0, 1.3)
    ax.set_xlabel("$s_1$ (SIC)", labelpad=6, **FONT["label"])
    ax.set_ylabel("$s_2$ (SCC)", labelpad=6, **FONT["label"])
    ax.set_zlabel("$s_3$ (ICC)", labelpad=6, **FONT["label"])
    ax.set_title(
        "Channel Basis Vectors in Cognitive Space\n"
        r"$\{\mathbf{s}_1,\mathbf{s}_2,\mathbf{s}_3\}$ — orthonormal basis",
        **FONT["title"], pad=12,
    )
    ax.view_init(elev=22, azim=38)
    _author_tag(fig)
    return _save(fig, out_dir, "fig_channel_basis_vectors", fmt, dpi)


# ===========================================================================
# FIG 1.3 — Channel domain Venn diagram
# ===========================================================================

def fig_channel_domains(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.3 fig_channel_domains")
    fig, ax = plt.subplots(figsize=(9, 6.5), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(-1.0, 5.0)
    ax.set_ylim(-1.2, 3.2)
    ax.set_aspect("equal")
    ax.axis("off")

    circles = [
        (0.8,  1.3, COLOR["sic"], "SIC",  0.36),
        (2.0,  2.6, COLOR["scc"], "SCC",  0.36),
        (3.2,  1.3, COLOR["icc"], "ICC",  0.36),
    ]
    r = 1.55
    for cx, cy, col, lbl, _ in circles:
        circ = plt.Circle((cx, cy), r, color=col, alpha=0.22, zorder=1)
        ax.add_patch(circ)
        circ_border = plt.Circle((cx, cy), r, color=col,
                                  fill=False, linewidth=2.5, zorder=2)
        ax.add_patch(circ_border)

    domain_text = [
        (0.05, 0.8,  COLOR["sic"], "SIC", ["Multimodal Perception", "Environmental Salience",
                                            "Embodied Coupling", "Accessibility Transform"]),
        (2.0,  3.55, COLOR["scc"], "SCC", ["Semantic Coherence", "Invariant Preservation",
                                            "Contextual Propagation", "Systemic Reasoning"]),
        (3.9,  0.8,  COLOR["icc"], "ICC", ["Identity Anchoring", "Provenance Preservation",
                                            "Ethical Constraint", "Self-Consistency"]),
    ]
    for tx, ty, col, lbl, items in domain_text:
        ax.text(tx, ty + 0.35, lbl, ha="center", fontsize=11,
                fontweight="bold", color=col)
        for k, item in enumerate(items):
            ax.text(tx, ty - k * 0.35, f"• {item}",
                    ha="center", fontsize=7.5, color=col)

    # Intersection labels
    ax.text(1.35, 2.35, "Percept-\nSemantic",
            ha="center", fontsize=7.5, color=COLOR["edge"],
            fontstyle="italic", zorder=3)
    ax.text(2.65, 2.35, "Semantic-\nIdentity",
            ha="center", fontsize=7.5, color=COLOR["edge"],
            fontstyle="italic", zorder=3)
    ax.text(2.0,  0.65, "Identity-\nPercept",
            ha="center", fontsize=7.5, color=COLOR["edge"],
            fontstyle="italic", zorder=3)
    ax.text(2.0,  1.75, "USAIC\n(fusion)",
            ha="center", fontsize=9, fontweight="bold",
            color=COLOR["usaic"], zorder=4,
            bbox=dict(boxstyle="round,pad=0.3",
                      facecolor=COLOR["usaic"], alpha=0.18,
                      edgecolor=COLOR["usaic"]))

    ax.set_title(
        "TSLCA Channel Domain Diagram\nSIC · SCC · ICC — Mutually Constraining Dimensions",
        **FONT["title"], pad=10,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_channel_domains", fmt, dpi)


# ===========================================================================
# FIG 1.4 — Off-diagonal asymmetry: s_i⊗s_j ≠ s_j⊗s_i
# ===========================================================================

def fig_off_diagonal_asymmetry(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.4 fig_off_diagonal_asymmetry")
    fig, axes = plt.subplots(1, 3, figsize=(13, 4.5), facecolor=COLOR["bg"])
    fig.subplots_adjust(wspace=0.35)

    pairs = [
        ("SIC", "SCC", COLOR["sic"], COLOR["scc"],
         r"$s_1 \otimes s_2$", r"$s_2 \otimes s_1$",
         [0.85, 0.35], [0.45, 0.65]),
        ("SIC", "ICC", COLOR["sic"], COLOR["icc"],
         r"$s_1 \otimes s_3$", r"$s_3 \otimes s_1$",
         [0.78, 0.28], [0.52, 0.70]),
        ("SCC", "ICC", COLOR["scc"], COLOR["icc"],
         r"$s_2 \otimes s_3$", r"$s_3 \otimes s_2$",
         [0.80, 0.32], [0.48, 0.68]),
    ]
    for ax, (chA, chB, colA, colB, lblFwd, lblRev, magFwd, magRev) in zip(axes, pairs):
        ax.set_facecolor(COLOR["bg"])
        ax.set_xlim(0, 1)
        ax.set_ylim(0, 1)
        ax.axis("off")
        ax.set_title(f"{chA} ↔ {chB}", fontsize=11,
                     fontweight="bold", color=COLOR["edge"])

        # Forward arrow  A→B
        ax.annotate(
            "", xy=(0.78, 0.62), xytext=(0.22, 0.62),
            arrowprops=dict(arrowstyle="->", color=colA,
                            lw=2.8 + magFwd[0]),
        )
        ax.text(0.50, 0.70, lblFwd, ha="center",
                fontsize=11, color=colA, fontweight="bold")
        ax.text(0.50, 0.55,
                f"magnitude ≈ {magFwd[0]:.2f}",
                ha="center", fontsize=8, color=colA)

        # Reverse arrow  B→A (different magnitude)
        ax.annotate(
            "", xy=(0.22, 0.35), xytext=(0.78, 0.35),
            arrowprops=dict(arrowstyle="->", color=colB,
                            lw=2.8 + magRev[0]),
        )
        ax.text(0.50, 0.43, lblRev, ha="center",
                fontsize=11, color=colB, fontweight="bold")
        ax.text(0.50, 0.27,
                f"magnitude ≈ {magRev[0]:.2f}",
                ha="center", fontsize=8, color=colB)

        # ≠ badge
        ax.text(0.50, 0.12, r"$\neq$",
                ha="center", fontsize=16,
                color=COLOR["highlight"], fontweight="bold")
        ax.text(0.22, 0.62, chA, ha="center",
                fontsize=9, color=colA,
                bbox=dict(facecolor=colA, alpha=0.15,
                          boxstyle="round,pad=0.2"))
        ax.text(0.78, 0.62, chB, ha="center",
                fontsize=9, color=colB,
                bbox=dict(facecolor=colB, alpha=0.15,
                          boxstyle="round,pad=0.2"))

    fig.suptitle(
        "Off-Diagonal Non-Commutativity: "
        r"$s_i \otimes s_j \neq s_j \otimes s_i$",
        **FONT["title"], y=1.02,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_off_diagonal_asymmetry", fmt, dpi)


# ===========================================================================
# FIG 1.5 — Lattice cell taxonomy (all 9 cells annotated)
# ===========================================================================

def fig_lattice_cell_taxonomy(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.5 fig_lattice_cell_taxonomy")
    channels = ["SIC", "SCC", "ICC"]
    ch_colors = [COLOR["sic"], COLOR["scc"], COLOR["icc"]]
    cell_roles = [
        ["Pure-mode\nPerception",    "Percept interpreting\nSemantic field",  "Percept anchored to\nIdentity"],
        ["Semantic projected\nonto Percept", "Pure-mode\nCoherence",          "Semantic modulated by\nIdentity"],
        ["Identity grounded in\nPercept",    "Identity derived from\nSemantic", "Pure-mode\nIdentity"],
    ]
    cell_types = [
        ["diagonal", "off-diag", "off-diag"],
        ["off-diag", "diagonal", "off-diag"],
        ["off-diag", "off-diag", "diagonal"],
    ]

    fig, ax = plt.subplots(figsize=(10, 9), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(-1.2, 3.5)
    ax.set_ylim(-0.8, 3.8)
    ax.set_aspect("equal")
    ax.axis("off")

    for i in range(3):
        for j in range(3):
            x, y = j, 2 - i
            is_diag = (i == j)
            fill = COLOR["diag"] if is_diag else COLOR["offdiag"]
            lw = 3.0 if is_diag else 1.2
            edge_c = ch_colors[i] if is_diag else COLOR["grid"]

            rect = FancyBboxPatch(
                (x + 0.04, y + 0.04), 0.91, 0.91,
                boxstyle="round,pad=0.03",
                linewidth=lw, edgecolor=edge_c, facecolor=fill, zorder=2,
            )
            ax.add_patch(rect)

            # cell type badge top-left
            type_col = ch_colors[i] if is_diag else COLOR["neutral"]
            ax.text(x + 0.10, y + 0.88,
                    "DIAG" if is_diag else "OFF",
                    fontsize=6, color="white",
                    bbox=dict(facecolor=type_col, edgecolor="none",
                              boxstyle="round,pad=0.15"),
                    va="top", zorder=3)

            # tensor product label
            ax.text(x + 0.49, y + 0.68,
                    f"$s_{i+1}\\!\\otimes\\!s_{j+1}$",
                    ha="center", va="center",
                    **FONT["cell"], zorder=3)

            # role description
            ax.text(x + 0.49, y + 0.32, cell_roles[i][j],
                    ha="center", va="center",
                    fontsize=7, color=COLOR["neutral"],
                    linespacing=1.35, zorder=3)

    # row/col headers
    for k, (ch, col) in enumerate(zip(channels, ch_colors)):
        y = 2 - k + 0.49
        ax.text(-0.55, y, ch, ha="center", va="center",
                fontsize=10, fontweight="bold", color=col,
                bbox=dict(facecolor=col, alpha=0.12,
                          boxstyle="round,pad=0.3",
                          edgecolor=col))
        x = k + 0.49
        ax.text(x, 3.45, ch, ha="center", va="center",
                fontsize=10, fontweight="bold", color=col,
                bbox=dict(facecolor=col, alpha=0.12,
                          boxstyle="round,pad=0.3",
                          edgecolor=col))

    ax.text(-0.55, 3.45, "⊗", ha="center", va="center",
            fontsize=14, color=COLOR["edge"])

    # legend
    d_patch = mpatches.Patch(facecolor=COLOR["diag"],
                              edgecolor=COLOR["edge"], label="Diagonal — pure-mode stability")
    o_patch = mpatches.Patch(facecolor=COLOR["offdiag"],
                              edgecolor=COLOR["edge"], label="Off-diagonal — cross-mode interaction")
    ax.legend(handles=[d_patch, o_patch], loc="lower right",
              fontsize=8, framealpha=0.9, edgecolor=COLOR["grid"])

    ax.set_title("TSLCA Cell Taxonomy — All Nine Cognitive Cells",
                 **FONT["title"], pad=14)
    _author_tag(fig)
    return _save(fig, out_dir, "fig_lattice_cell_taxonomy", fmt, dpi)


# ===========================================================================
# FIG 1.6 — 3-6-9-13 grammar hierarchy
# ===========================================================================

def fig_3_6_9_13_grammar(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.6 fig_3_6_9_13_grammar")
    fig, ax = plt.subplots(figsize=(12, 6), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(-0.5, 11.5)
    ax.set_ylim(-1.2, 3.8)
    ax.axis("off")

    levels = [
        {"n": 3,  "y": 3.0, "col": COLOR["sic"],
         "items": ["SIC", "SCC", "ICC"],
         "label": "3 Basis Channels"},
        {"n": 6,  "y": 1.7, "col": COLOR["scc"],
         "items": ["SIC→SCC", "SIC→ICC", "SCC→SIC",
                   "SCC→ICC", "ICC→SIC", "ICC→SCC"],
         "label": "6 Dual-Triad Interactions"},
        {"n": 9,  "y": 0.4, "col": COLOR["icc"],
         "items": [f"$s_{i}\\!\\otimes\\!s_{j}$"
                   for i in range(1, 4) for j in range(1, 4)],
         "label": "9 Cognitive Cells"},
        {"n": 13, "y": -0.9, "col": COLOR["gold"],
         "items": [f"$\\mathcal{{G}}_{{{k}}}$" for k in range(1, 14)],
         "label": "13 SAGES Invariants"},
    ]

    x_centers_all = []
    for lvl in levels:
        n = lvl["n"]
        xs = np.linspace(0.5, 10.5, n)
        x_centers_all.append(xs)
        y = lvl["y"]
        col = lvl["col"]
        for k, (x, item) in enumerate(zip(xs, lvl["items"])):
            box = FancyBboxPatch(
                (x - 0.42, y - 0.28), 0.84, 0.56,
                boxstyle="round,pad=0.05",
                linewidth=1.5, edgecolor=col,
                facecolor=mcolors.to_rgba(col, 0.12), zorder=3,
            )
            ax.add_patch(box)
            ax.text(x, y, item, ha="center", va="center",
                    fontsize=max(6.5, 9 - n // 4),
                    color=COLOR["edge"], fontweight="bold", zorder=4)

        ax.text(-0.4, y, lvl["label"], ha="right", va="center",
                fontsize=9, color=col, fontweight="bold")

    # draw connecting lines between levels
    for idx in range(len(levels) - 1):
        xs_top = x_centers_all[idx]
        xs_bot = x_centers_all[idx + 1]
        y_top = levels[idx]["y"] - 0.30
        y_bot = levels[idx + 1]["y"] + 0.30
        n_top, n_bot = len(xs_top), len(xs_bot)
        step = n_bot // n_top
        col = levels[idx + 1]["col"]
        for ti, xt in enumerate(xs_top):
            for bi in range(ti * step, min((ti + 1) * step, n_bot)):
                ax.plot([xt, xs_bot[bi]], [y_top, y_bot],
                        color=col, alpha=0.25, lw=0.8, zorder=1)

    ax.set_title(
        "TSLCA 3-6-9-13 Structural Grammar\n"
        "Channels → Interactions → Cells → Governance Invariants",
        **FONT["title"], pad=10,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_3_6_9_13_grammar", fmt, dpi)


# ===========================================================================
# FIG 1.7 — USAIC as tensor contraction arrow diagram
# ===========================================================================

def fig_usaic_contraction(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.7 fig_usaic_contraction")
    fig, ax = plt.subplots(figsize=(11, 5), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(0, 11)
    ax.set_ylim(0, 5)
    ax.axis("off")

    # 9-cell grid on left
    cell_colors_flat = [
        COLOR["diag"], COLOR["offdiag"], COLOR["offdiag"],
        COLOR["offdiag"], COLOR["diag"],  COLOR["offdiag"],
        COLOR["offdiag"], COLOR["offdiag"], COLOR["diag"],
    ]
    k = 0
    for row in range(3):
        for col_idx in range(3):
            x = 0.3 + col_idx * 1.1
            y = 3.8 - row * 1.1
            rect = FancyBboxPatch((x, y), 0.95, 0.95,
                                   boxstyle="round,pad=0.04",
                                   linewidth=1.2,
                                   edgecolor=COLOR["grid"],
                                   facecolor=cell_colors_flat[k], zorder=2)
            ax.add_patch(rect)
            ax.text(x + 0.475, y + 0.475,
                    f"$s_{row+1}\\!\\otimes\\!s_{col_idx+1}$",
                    ha="center", va="center",
                    fontsize=7.5, color=COLOR["edge"],
                    fontweight="bold", zorder=3)
            k += 1

    ax.text(1.95, 0.35, r"$\mathbf{T} \in \mathcal{C}^{3\times3}$",
            ha="center", **FONT["math"])
    ax.text(1.95, -0.02, "Cognitive Tensor",
            ha="center", fontsize=8, color=COLOR["neutral"])

    # USAIC operator box
    usaic_box = FancyBboxPatch((4.4, 1.5), 2.2, 2.0,
                                boxstyle="round,pad=0.12",
                                linewidth=2.5,
                                edgecolor=COLOR["usaic"],
                                facecolor=mcolors.to_rgba(COLOR["usaic"], 0.12),
                                zorder=2)
    ax.add_patch(usaic_box)
    ax.text(5.5, 2.85, "USAIC", ha="center",
            fontsize=13, fontweight="bold", color=COLOR["usaic"], zorder=3)
    ax.text(5.5, 2.45, "Contraction Operator",
            ha="center", fontsize=8, color=COLOR["usaic"], zorder=3)
    constraints = ["Reversibility", "Accessibility", "Provenance", "Symmetry"]
    for ci, c in enumerate(constraints):
        ax.text(5.5, 2.05 - ci * 0.30, f"• {c}",
                ha="center", fontsize=7.5, color=COLOR["edge"], zorder=3)

    # Unified field on right
    uf_box = FancyBboxPatch((8.0, 1.5), 2.5, 2.0,
                             boxstyle="round,pad=0.12",
                             linewidth=2.5,
                             edgecolor=COLOR["gold"],
                             facecolor=mcolors.to_rgba(COLOR["gold"], 0.10),
                             zorder=2)
    ax.add_patch(uf_box)
    ax.text(9.25, 2.85, r"$\mathbf{\Phi}$",
            ha="center", fontsize=16, fontweight="bold",
            color=COLOR["gold"], zorder=3)
    ax.text(9.25, 2.45, "Unified Cognitive",
            ha="center", fontsize=8.5, color=COLOR["gold"], zorder=3)
    ax.text(9.25, 2.15, "Field",
            ha="center", fontsize=8.5, color=COLOR["gold"], zorder=3)

    # arrows
    ax.annotate("", xy=(4.35, 2.5), xytext=(3.55, 2.5),
                arrowprops=dict(arrowstyle="->", color=COLOR["edge"], lw=2.0))
    ax.annotate("", xy=(7.95, 2.5), xytext=(6.65, 2.5),
                arrowprops=dict(arrowstyle="->", color=COLOR["usaic"], lw=2.5))

    ax.set_title(
        r"USAIC as Tensor Contraction: $\mathbf{T} \xrightarrow{\mathrm{USAIC}} \mathbf{\Phi}$",
        **FONT["title"], pad=10,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_usaic_contraction", fmt, dpi)


# ===========================================================================
# FIG 1.8 — USAIC fusion operator mathematical flow
# ===========================================================================

def fig_usaic_fusion_operator(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.8 fig_usaic_fusion_operator")
    fig, ax = plt.subplots(figsize=(12, 5), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(0, 12)
    ax.set_ylim(0, 5)
    ax.axis("off")

    stages = [
        {"x": 0.5,  "label": "SIC\nInput",          "sub": r"$\mathbf{s}_1$",  "col": COLOR["sic"]},
        {"x": 0.5,  "label": "SCC\nInput",          "sub": r"$\mathbf{s}_2$",  "col": COLOR["scc"], "dy": -1.5},
        {"x": 0.5,  "label": "ICC\nInput",          "sub": r"$\mathbf{s}_3$",  "col": COLOR["icc"], "dy": -3.0},
        {"x": 3.5,  "label": "Tensor\nProduct",     "sub": r"$\mathbf{T}=s_i\!\otimes\!s_j$", "col": COLOR["edge"]},
        {"x": 6.0,  "label": "Weight\nAssignment",  "sub": r"$w_{ij}\mathbf{T}_{ij}$", "col": COLOR["neutral"]},
        {"x": 8.5,  "label": "Contraction\n+ Constraints", "sub": "Rev · Acc · Prov · Sym", "col": COLOR["usaic"]},
        {"x": 11.0, "label": "Unified Field",       "sub": r"$\mathbf{\Phi}$",  "col": COLOR["gold"]},
    ]

    y0 = 3.8
    box_w, box_h = 1.6, 0.9

    def draw_box(x, y, label, sub, col):
        rect = FancyBboxPatch((x - box_w / 2, y - box_h / 2), box_w, box_h,
                               boxstyle="round,pad=0.06",
                               linewidth=1.8, edgecolor=col,
                               facecolor=mcolors.to_rgba(col, 0.10), zorder=2)
        ax.add_patch(rect)
        ax.text(x, y + 0.18, label, ha="center", va="center",
                fontsize=9, fontweight="bold", color=col, zorder=3)
        ax.text(x, y - 0.22, sub, ha="center", va="center",
                fontsize=8, color=COLOR["edge"], zorder=3)

    # three input boxes fan into tensor product
    input_ys = [y0, y0 - 1.5, y0 - 3.0]
    for i, (stage_lbl, col_k) in enumerate(
            zip(["SIC\nInput", "SCC\nInput", "ICC\nInput"],
                [COLOR["sic"], COLOR["scc"], COLOR["icc"]])):
        sub_k = [r"$\mathbf{s}_1$", r"$\mathbf{s}_2$", r"$\mathbf{s}_3$"][i]
        draw_box(1.2, input_ys[i], stage_lbl, sub_k, col_k)
        ax.annotate("", xy=(2.7, y0 - 1.5), xytext=(2.05, input_ys[i]),
                    arrowprops=dict(arrowstyle="->", color=col_k,
                                   lw=1.6, connectionstyle="arc3,rad=0.0"))

    pipeline = [
        (3.2,  y0 - 1.5, "Tensor\nProduct",   r"$T_{ij}=s_i\!\otimes\!s_j$", COLOR["edge"]),
        (5.4,  y0 - 1.5, "Weight\nAssignment", r"$w_{ij}\,T_{ij}$",            COLOR["neutral"]),
        (7.6,  y0 - 1.5, "Contraction\n+Constraints", "Rev·Acc·Prov·Sym",      COLOR["usaic"]),
        (9.8,  y0 - 1.5, "Unified\nField",     r"$\mathbf{\Phi}$",              COLOR["gold"]),
    ]
    prev_x = 2.7
    for (px, py, plbl, psub, pcol) in pipeline:
        draw_box(px, py, plbl, psub, pcol)
        ax.annotate("", xy=(px - box_w / 2 - 0.05, py),
                    xytext=(prev_x + box_w / 2 + 0.05
                            if prev_x > 2.0 else prev_x + 0.05, py),
                    arrowprops=dict(arrowstyle="->", color=pcol, lw=2.0))
        prev_x = px

    ax.set_title(
        "USAIC Fusion Operator — Mathematical Flow Pipeline",
        **FONT["title"], pad=10,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_usaic_fusion_operator", fmt, dpi)


# ===========================================================================
# FIG 1.9 — Fragmented modular vs TSLCA lattice
# ===========================================================================

def fig_lattice_vs_modular_arch(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.9 fig_lattice_vs_modular_arch")
    fig, (ax_l, ax_r) = plt.subplots(1, 2, figsize=(13, 6),
                                      facecolor=COLOR["bg"])
    fig.subplots_adjust(wspace=0.10)

    for ax in (ax_l, ax_r):
        ax.set_facecolor(COLOR["bg"])
        ax.axis("off")

    # --- LEFT: fragmented modular ---
    ax_l.set_xlim(0, 6)
    ax_l.set_ylim(0, 6)
    ax_l.set_title("Fragmented Modular Architecture",
                   fontsize=11, fontweight="bold",
                   color=COLOR["highlight"], pad=8)

    modules = [
        (1.0, 4.8, "Perception\nModule",  COLOR["sic"]),
        (3.0, 4.8, "Memory\nModule",      COLOR["scc"]),
        (5.0, 4.8, "Reasoning\nModule",   COLOR["icc"]),
        (1.0, 2.8, "Language\nModule",    COLOR["neutral"]),
        (3.0, 2.8, "Identity\nModule",    COLOR["icc"]),
        (5.0, 2.8, "Governance\nModule",  COLOR["gold"]),
        (2.0, 0.8, "Integration?",        COLOR["highlight"]),
        (4.0, 0.8, "Integration?",        COLOR["highlight"]),
    ]
    for mx, my, mlbl, mcol in modules:
        r = FancyBboxPatch((mx - 0.55, my - 0.42), 1.1, 0.84,
                           boxstyle="round,pad=0.05",
                           linewidth=1.4, edgecolor=mcol,
                           facecolor=mcolors.to_rgba(mcol, 0.12), zorder=2)
        ax_l.add_patch(r)
        ax_l.text(mx, my, mlbl, ha="center", va="center",
                  fontsize=7.5, color=COLOR["edge"],
                  fontweight="bold", zorder=3)

    # random weak connections
    rng = np.random.default_rng(42)
    conn_pairs = [(0, 1), (1, 2), (0, 3), (1, 4), (2, 5),
                  (3, 6), (4, 6), (4, 7), (5, 7)]
    for a_idx, b_idx in conn_pairs:
        ax_l.plot(
            [modules[a_idx][0], modules[b_idx][0]],
            [modules[a_idx][1], modules[b_idx][1]],
            color=COLOR["grid"], lw=1.0, linestyle="--", alpha=0.6, zorder=1,
        )

    ax_l.text(3.0, -0.2,
              "Pragmatic coupling — brittle under transition",
              ha="center", fontsize=8, color=COLOR["highlight"],
              fontstyle="italic")

    # --- RIGHT: TSLCA lattice ---
    ax_r.set_xlim(-0.3, 3.8)
    ax_r.set_ylim(-0.3, 3.8)
    ax_r.set_title("TSLCA Lattice Architecture",
                   fontsize=11, fontweight="bold",
                   color=COLOR["usaic"], pad=8)

    ch_colors = [COLOR["sic"], COLOR["scc"], COLOR["icc"]]
    for i in range(3):
        for j in range(3):
            x, y = j, 2 - i
            is_diag = (i == j)
            fill = COLOR["diag"] if is_diag else COLOR["offdiag"]
            ec = ch_colors[i] if is_diag else COLOR["grid"]
            rect = FancyBboxPatch(
                (x + 0.05, y + 0.05), 0.88, 0.88,
                boxstyle="round,pad=0.03",
                linewidth=2.0 if is_diag else 1.0,
                edgecolor=ec, facecolor=fill, zorder=2,
            )
            ax_r.add_patch(rect)
            ax_r.text(x + 0.49, y + 0.49,
                      f"$s_{i+1}\\!\\otimes\\!s_{j+1}$",
                      ha="center", va="center",
                      fontsize=8, fontweight="bold",
                      color=COLOR["edge"], zorder=3)

    ax_r.text(1.5, -0.22,
              "Derived from shared formal structure — coherent under transformation",
              ha="center", fontsize=7.5, color=COLOR["usaic"],
              fontstyle="italic")

    fig.suptitle(
        "Architectural Fragmentation vs. TSLCA Lattice Coherence",
        **FONT["title"], y=1.01,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_lattice_vs_modular_arch", fmt, dpi)


# ===========================================================================
# FIG 1.10 — Substrate independence
# ===========================================================================

def fig_substrate_independence(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.10 fig_substrate_independence")
    substrates = [
        ("Symbolic",    "#E8F4FD", "#2E86AB", "Logic / Rule\nSystems"),
        ("Neural",      "#FDE8F4", "#A23B72", "Connectionist\nNetworks"),
        ("Distributed", "#E8FDE8", "#2A9D8F", "Graph / Mesh\nSystems"),
        ("Photonic",    "#FFF3E0", "#F18F01", "Optical /\nTopological"),
        ("Hybrid",      "#F0E8FD", "#6C63FF", "Mixed\nEmbodiment"),
    ]

    fig, axes = plt.subplots(1, 5, figsize=(15, 5.5), facecolor=COLOR["bg"])
    fig.subplots_adjust(wspace=0.08)

    for ax, (name, fill, edge, desc) in zip(axes, substrates):
        ax.set_facecolor(COLOR["bg"])
        ax.set_xlim(-0.1, 3.1)
        ax.set_ylim(-0.1, 3.6)
        ax.set_aspect("equal")
        ax.axis("off")
        ax.set_title(name, fontsize=10, fontweight="bold", color=edge, pad=5)

        # mini lattice
        for i in range(3):
            for j in range(3):
                x, y = j, 2 - i
                is_diag = (i == j)
                fc = fill if is_diag else "#FFFFFF"
                ec_c = edge if is_diag else COLOR["grid"]
                rect = FancyBboxPatch(
                    (x + 0.06, y + 0.06), 0.86, 0.86,
                    boxstyle="round,pad=0.04",
                    linewidth=2.0 if is_diag else 0.8,
                    edgecolor=ec_c, facecolor=fc, zorder=2,
                )
                ax.add_patch(rect)
                ax.text(x + 0.49, y + 0.49,
                        f"$s_{i+1}\\!\\otimes\\!s_{j+1}$",
                        ha="center", va="center",
                        fontsize=6.5, color=COLOR["edge"],
                        fontweight="bold", zorder=3)

        ax.text(1.49, -0.08, desc, ha="center", va="top",
                fontsize=7.5, color=edge, linespacing=1.3)

    fig.suptitle(
        "TSLCA Substrate Independence\n"
        "Same Lattice Formalism Across Five Instantiation Substrates",
        **FONT["title"],
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_substrate_independence", fmt, dpi)


# ===========================================================================
# FIG 1.11 — Cognitive field quantities definition
# ===========================================================================

def fig_cognitive_field_definition(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.11 fig_cognitive_field_definition")
    quantities = [
        ("Coherence",            r"$\kappa_{ij}$",   COLOR["scc"],
         "Degree of semantic\nalignment between cells"),
        ("Salience",             r"$\sigma_{ij}$",   COLOR["sic"],
         "Perceptual relevance\nweight of each cell"),
        ("Semantic Density",     r"$\rho_{ij}$",     COLOR["scc"],
         "Concentration of\nmeaning in the field"),
        ("Accessibility\nRelevance", r"$\alpha_{ij}$", COLOR["usaic"],
         "USAIC accessibility\nweight per cell"),
        ("Identity Weight",      r"$\iota_{ij}$",    COLOR["icc"],
         "ICC identity anchoring\nstrength per cell"),
    ]

    fig, ax = plt.subplots(figsize=(13, 5), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(0, 13)
    ax.set_ylim(0, 5)
    ax.axis("off")

    xs = np.linspace(1.3, 11.7, len(quantities))
    for x, (name, sym, col, desc) in zip(xs, quantities):
        # outer ring
        circ = plt.Circle((x, 2.6), 0.85, color=col, alpha=0.15, zorder=1)
        ax.add_patch(circ)
        circ_e = plt.Circle((x, 2.6), 0.85, color=col,
                             fill=False, linewidth=2.2, zorder=2)
        ax.add_patch(circ_e)
        ax.text(x, 2.6, sym, ha="center", va="center",
                fontsize=14, fontweight="bold", color=col, zorder=3)
        ax.text(x, 1.52, name, ha="center", va="center",
                fontsize=8.5, fontweight="bold", color=COLOR["edge"],
                linespacing=1.3)
        ax.text(x, 0.82, desc, ha="center", va="center",
                fontsize=7.5, color=COLOR["neutral"],
                linespacing=1.35)

    # connecting bar
    ax.plot([xs[0] - 0.3, xs[-1] + 0.3], [2.6, 2.6],
            color=COLOR["grid"], lw=1.0, zorder=0, alpha=0.5)

    ax.text(6.5, 4.55,
            r"$\mathbf{F}_{ij} = f(\kappa_{ij},\, \sigma_{ij},\, "
            r"\rho_{ij},\, \alpha_{ij},\, \iota_{ij})$",
            ha="center", va="center",
            fontsize=12, color=COLOR["edge"],
            bbox=dict(facecolor=COLOR["gold"], alpha=0.12,
                      boxstyle="round,pad=0.4",
                      edgecolor=COLOR["gold"]))

    ax.set_title(
        "Cognitive Field Quantities — Five Scalar Fields on the TSLCA Lattice",
        **FONT["title"], pad=10,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_cognitive_field_definition", fmt, dpi)


# ===========================================================================
# FIG 1.12 — Lattice completeness / minimality
# ===========================================================================

def fig_lattice_completeness(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.12 fig_lattice_completeness")
    fig, axes = plt.subplots(1, 4, figsize=(15, 5), facecolor=COLOR["bg"])
    fig.subplots_adjust(wspace=0.10)

    configs = [
        ("Full TSLCA\n(3 channels)",  [True, True, True],   COLOR["usaic"]),
        ("Remove SIC",                [False, True, True],   COLOR["highlight"]),
        ("Remove SCC",                [True, False, True],   COLOR["highlight"]),
        ("Remove ICC",                [True, True, False],   COLOR["highlight"]),
    ]
    ch_names = ["SIC", "SCC", "ICC"]
    ch_colors = [COLOR["sic"], COLOR["scc"], COLOR["icc"]]

    for ax, (title, active, title_col) in zip(axes, configs):
        ax.set_facecolor(COLOR["bg"])
        ax.set_xlim(-0.1, 3.1)
        ax.set_ylim(-0.8, 3.5)
        ax.set_aspect("equal")
        ax.axis("off")
        ax.set_title(title, fontsize=9.5, fontweight="bold",
                     color=title_col, pad=6, linespacing=1.3)

        for i in range(3):
            for j in range(3):
                row_active = active[i]
                col_active = active[j]
                cell_active = row_active and col_active
                x, y = j, 2 - i
                is_diag = (i == j)
                if cell_active:
                    fill = COLOR["diag"] if is_diag else COLOR["offdiag"]
                    ec = ch_colors[i] if is_diag else COLOR["grid"]
                    lw = 2.0 if is_diag else 1.0
                    alpha = 1.0
                else:
                    fill = "#EEEEEE"
                    ec = "#AAAAAA"
                    lw = 0.8
                    alpha = 0.4
                rect = FancyBboxPatch(
                    (x + 0.06, y + 0.06), 0.86, 0.86,
                    boxstyle="round,pad=0.04",
                    linewidth=lw, edgecolor=ec,
                    facecolor=fill, alpha=alpha, zorder=2,
                )
                ax.add_patch(rect)
                if not cell_active:
                    ax.text(x + 0.49, y + 0.49, "✕",
                            ha="center", va="center",
                            fontsize=14, color="#AAAAAA", zorder=3)
                else:
                    ax.text(x + 0.49, y + 0.49,
                            f"$s_{i+1}\\!\\otimes\\!s_{j+1}$",
                            ha="center", va="center",
                            fontsize=7, fontweight="bold",
                            color=COLOR["edge"], zorder=3)

        lost = sum(1 for a in active if not a)
        dof_lost = lost * 3  # each channel governs a row + column
        ax.text(1.49, -0.52,
                f"DOF lost: {dof_lost}" if lost > 0 else "Complete (9 DOF)",
                ha="center", fontsize=8,
                color=COLOR["highlight"] if lost > 0 else COLOR["usaic"],
                fontweight="bold")

    fig.suptitle(
        "Lattice Minimality: Removing Any Channel Collapses Essential Degrees of Freedom",
        **FONT["title"],
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_lattice_completeness", fmt, dpi)


# ===========================================================================
# FIG 1.13 — Cell interaction graph
# ===========================================================================

def fig_cell_interaction_graph(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.13 fig_cell_interaction_graph")
    fig, ax = plt.subplots(figsize=(9, 9), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(-1.4, 1.4)
    ax.set_ylim(-1.4, 1.6)
    ax.set_aspect("equal")
    ax.axis("off")

    # 9 cells arranged in a 3×3 circle grid
    angles = np.linspace(0, 2 * np.pi, 9, endpoint=False) - np.pi / 2
    r = 1.05
    positions = {k: (r * np.cos(a), r * np.sin(a))
                 for k, a in enumerate(angles)}

    cell_labels_flat = [
        r"$s_1\!\otimes\!s_1$", r"$s_1\!\otimes\!s_2$", r"$s_1\!\otimes\!s_3$",
        r"$s_2\!\otimes\!s_1$", r"$s_2\!\otimes\!s_2$", r"$s_2\!\otimes\!s_3$",
        r"$s_3\!\otimes\!s_1$", r"$s_3\!\otimes\!s_2$", r"$s_3\!\otimes\!s_3$",
    ]
    diag_idx = {0, 4, 8}
    ch_colors_flat = [
        COLOR["sic"], COLOR["sic"], COLOR["sic"],
        COLOR["scc"], COLOR["scc"], COLOR["scc"],
        COLOR["icc"], COLOR["icc"], COLOR["icc"],
    ]

    # draw edges (all pairs, weight by channel relationship)
    rng = np.random.default_rng(7)
    for a_idx in range(9):
        for b_idx in range(a_idx + 1, 9):
            xa, ya = positions[a_idx]
            xb, yb = positions[b_idx]
            same_row = (a_idx // 3 == b_idx // 3)
            same_col = (a_idx % 3 == b_idx % 3)
            if same_row or same_col:
                lw = 1.8
                col = COLOR["grid"]
                alpha = 0.55
            else:
                lw = 0.6
                col = COLOR["grid"]
                alpha = 0.20
            ax.plot([xa, xb], [ya, yb],
                    color=col, lw=lw, alpha=alpha, zorder=1)

    # draw nodes
    for k, (x, y) in positions.items():
        is_diag = k in diag_idx
        node_col = ch_colors_flat[k]
        circ = plt.Circle((x, y), 0.19,
                          color=node_col,
                          alpha=0.90 if is_diag else 0.40,
                          zorder=3)
        ax.add_patch(circ)
        circ_e = plt.Circle((x, y), 0.19,
                             color=node_col, fill=False,
                             linewidth=2.5 if is_diag else 1.0,
                             zorder=4)
        ax.add_patch(circ_e)
        ax.text(x, y, cell_labels_flat[k],
                ha="center", va="center",
                fontsize=6.5, fontweight="bold",
                color="white" if is_diag else COLOR["edge"],
                zorder=5)

    # legend
    strong = Line2D([0], [0], color=COLOR["grid"], lw=1.8,
                    label="Same-row or same-column")
    weak   = Line2D([0], [0], color=COLOR["grid"], lw=0.6, alpha=0.4,
                    label="Cross-row, cross-column")
    ax.legend(handles=[strong, weak], loc="lower center",
              fontsize=8, framealpha=0.9, edgecolor=COLOR["grid"])

    ax.set_title(
        "TSLCA Cell Interaction Graph\nInformation Flow Topology Across Nine Cells",
        **FONT["title"], pad=12,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_cell_interaction_graph", fmt, dpi)


# ===========================================================================
# FIG 1.14 — Channel algebra non-commutativity table
# ===========================================================================

def fig_channel_algebra_table(out_dir: Path, fmt: str, dpi: int) -> Path:
    log.info("Generating 1.14 fig_channel_algebra_table")
    channels = ["SIC ($s_1$)", "SCC ($s_2$)", "ICC ($s_3$)"]
    ch_short = ["$s_1$", "$s_2$", "$s_3$"]
    ch_colors = [COLOR["sic"], COLOR["scc"], COLOR["icc"]]

    # mock magnitude matrix — asymmetric
    mag = np.array([
        [1.00, 0.82, 0.64],
        [0.45, 1.00, 0.71],
        [0.38, 0.53, 1.00],
    ])

    fig, ax = plt.subplots(figsize=(9, 7.5), facecolor=COLOR["bg"])
    ax.set_facecolor(COLOR["bg"])
    ax.set_xlim(-0.5, 4.0)
    ax.set_ylim(-0.5, 4.0)
    ax.set_aspect("equal")
    ax.axis("off")

    cell_size = 1.0
    for i in range(3):
        for j in range(3):
            x = j * cell_size + 0.5
            y = (2 - i) * cell_size + 0.5
            is_diag = (i == j)

            # color by magnitude
            intensity = mag[i, j]
            if is_diag:
                fill = mcolors.to_rgba(ch_colors[i], 0.30)
            else:
                # asymmetry color: fwd > rev → warm, fwd < rev → cool
                asym = mag[i, j] - mag[j, i]
                if asym > 0:
                    fill = mcolors.to_rgba(COLOR["sic"], 0.10 + 0.25 * abs(asym))
                else:
                    fill = mcolors.to_rgba(COLOR["icc"], 0.10 + 0.25 * abs(asym))

            rect = FancyBboxPatch(
                (x + 0.03, y + 0.03), 0.94, 0.94,
                boxstyle="round,pad=0.04",
                linewidth=2.0 if is_diag else 1.0,
                edgecolor=ch_colors[i] if is_diag else COLOR["grid"],
                facecolor=fill, zorder=2,
            )
            ax.add_patch(rect)

            ax.text(x + 0.49, y + 0.68,
                    f"${ch_short[i].strip('$')}\\!\\otimes\\!{ch_short[j].strip('$')}$",
                    ha="center", va="center",
                    fontsize=9.5, fontweight="bold",
                    color=COLOR["edge"], zorder=3)
            ax.text(x + 0.49, y + 0.30,
                    f"≈ {intensity:.2f}",
                    ha="center", va="center",
                    fontsize=8.5, color=ch_colors[i]
                    if is_diag else COLOR["neutral"],
                    zorder=3)

            # asymmetry indicator for off-diag
            if not is_diag:
                asym = mag[i, j] - mag[j, i]
                sym = "↑" if asym > 0 else ("↓" if asym < 0 else "=")
                ax.text(x + 0.88, y + 0.88, sym,
                        ha="right", va="top",
                        fontsize=9, color=COLOR["highlight"],
                        fontweight="bold", zorder=4)

    # row / col headers
    for k, (ch, col) in enumerate(zip(channels, ch_colors)):
        y = (2 - k) * cell_size + 0.5 + 0.49
        ax.text(0.17, y, ch, ha="center", va="center",
                fontsize=8.5, fontweight="bold", color=col, rotation=90)
        x = k * cell_size + 0.5 + 0.49
        ax.text(x, 3.87, ch, ha="center", va="center",
                fontsize=8.5, fontweight="bold", color=col)

    ax.text(0.17, 3.87, "⊗", ha="center", va="center",
            fontsize=13, color=COLOR["edge"])

    # legend
    diag_p  = mpatches.Patch(facecolor=mcolors.to_rgba(COLOR["sic"], 0.30),
                               edgecolor=COLOR["edge"], label="Diagonal (pure-mode, = 1.00)")
    warm_p  = mpatches.Patch(facecolor=mcolors.to_rgba(COLOR["sic"], 0.30),
                               edgecolor=COLOR["edge"], label="Warm: forward > reverse")
    cool_p  = mpatches.Patch(facecolor=mcolors.to_rgba(COLOR["icc"], 0.30),
                               edgecolor=COLOR["edge"], label="Cool: forward < reverse")
    arr_p   = Line2D([0], [0], marker="$↑$", color=COLOR["highlight"],
                     markersize=10, label="↑/↓ asymmetry direction",
                     linewidth=0)
    ax.legend(handles=[diag_p, warm_p, cool_p, arr_p],
              loc="lower right", fontsize=7.5,
              framealpha=0.9, edgecolor=COLOR["grid"])

    ax.set_title(
        "Channel Algebra Table — Non-Commutative Tensor Products\n"
        r"$s_i \otimes s_j \neq s_j \otimes s_i$ (off-diagonal asymmetry shown by ↑↓)",
        **FONT["title"], pad=12,
    )
    _author_tag(fig)
    return _save(fig, out_dir, "fig_channel_algebra_table", fmt, dpi)


# ===========================================================================
# Registry & orchestrator
# ===========================================================================

FIGURES: dict[str, tuple[str, Callable]] = {
    "1.1":  ("fig_tsl_lattice_3x3",           fig_tsl_lattice_3x3),
    "1.2":  ("fig_channel_basis_vectors",      fig_channel_basis_vectors),
    "1.3":  ("fig_channel_domains",            fig_channel_domains),
    "1.4":  ("fig_off_diagonal_asymmetry",     fig_off_diagonal_asymmetry),
    "1.5":  ("fig_lattice_cell_taxonomy",      fig_lattice_cell_taxonomy),
    "1.6":  ("fig_3_6_9_13_grammar",           fig_3_6_9_13_grammar),
    "1.7":  ("fig_usaic_contraction",          fig_usaic_contraction),
    "1.8":  ("fig_usaic_fusion_operator",      fig_usaic_fusion_operator),
    "1.9":  ("fig_lattice_vs_modular_arch",    fig_lattice_vs_modular_arch),
    "1.10": ("fig_substrate_independence",     fig_substrate_independence),
    "1.11": ("fig_cognitive_field_definition", fig_cognitive_field_definition),
    "1.12": ("fig_lattice_completeness",       fig_lattice_completeness),
    "1.13": ("fig_cell_interaction_graph",     fig_cell_interaction_graph),
    "1.14": ("fig_channel_algebra_table",      fig_channel_algebra_table),
}


def run_all(
    out_dir: Path,
    fmt: str = "png",
    dpi: int = 300,
    only: str | None = None,
) -> dict:
    """
    Generate all Group 1 figures (or a single one if `only` is given).

    Returns a manifest dict suitable for JSON serialization and LaTeX import.
    """
    manifest = {"group": 1, "title": "Architecture & Lattice Structure",
                "format": fmt, "dpi": dpi, "figures": []}

    targets = {only: FIGURES[only]} if only and only in FIGURES else FIGURES
    if only and only not in FIGURES:
        raise ValueError(f"Figure {only!r} not in Group 1. Valid: {list(FIGURES)}")

    for fig_id, (name, fn) in targets.items():
        log.info(f"--- [{fig_id}] {name} ---")
        try:
            path = fn(out_dir, fmt, dpi)
            manifest["figures"].append({
                "id": fig_id,
                "name": name,
                "path": str(path),
                "latex": f"\\includegraphics[width=\\linewidth]{{{name}}}",
            })
        except Exception as exc:
            log.error(f"  FAILED [{fig_id}] {name}: {exc}")
            manifest["figures"].append({
                "id": fig_id, "name": name, "error": str(exc)
            })

    manifest_path = out_dir / "group1_manifest.json"
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)
    log.info(f"Manifest written → {manifest_path}")
    return manifest


# ===========================================================================
# CLI
# ===========================================================================

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="TSLCA Group 1 — Architecture & Lattice Structure Figures",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("--out",  type=str, default="./output/figures/group1",
                        help="Output directory for generated figures")
    parser.add_argument("--fmt",  type=str, default="png",
                        choices=["png", "pdf", "svg"],
                        help="Output format")
    parser.add_argument("--dpi",  type=int, default=300,
                        help="Resolution (PNG/PDF)")
    parser.add_argument("--fig",  type=str, default=None,
                        help="Generate a single figure by ID (e.g. 1.4)")
    parser.add_argument("--list", action="store_true",
                        help="List all available figure IDs and exit")
    args = parser.parse_args()

    if args.list:
        print("Group 1 — Architecture & Lattice Structure figures:")
        for fid, (fname, _) in FIGURES.items():
            print(f"  {fid:>5}  {fname}")
        raise SystemExit(0)

    result = run_all(
        out_dir=Path(args.out),
        fmt=args.fmt,
        dpi=args.dpi,
        only=args.fig,
    )
    n_ok  = sum(1 for f in result["figures"] if "error" not in f)
    n_err = sum(1 for f in result["figures"] if "error" in f)
    print(f"\nGroup 1 complete: {n_ok} figures generated, {n_err} errors.")
    print(f"Manifest: {Path(args.out) / 'group1_manifest.json'}")
