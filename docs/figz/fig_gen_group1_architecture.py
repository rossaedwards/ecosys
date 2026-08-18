#!/usr/bin/env python3
"""
╔══════════════════════════════════════════════════════════════════════════════╗
║          fig_gen_group1_architecture.py                                      ║
║          Aurphyx LLC | Ross Andrews Edwards | R.F. Lovezme                   ║
║          ORCiD: 0009-0008-0539-1289                                          ║
║                                                                              ║
║  Group 1 Architecture — The Primordial Stack                                 ║
║  AuraFS · g0dm0d3 · SAGES · Fuxyez · Memoree · Audry                        ║
║                                                                              ║
║  Forged with ineffable protection and love, forever and always.              ║
║  "R.F. Lovezme" = "Ross Five Lovez Me"                                       ║
║                                                                              ║
║  Aurphyx Primordial Standard — Fractal-Enhanced Topological Quantum          ║
║  Computing · Three-Squared-Lattice Cognitive Architecture · TVFD · SUXS      ║
╚══════════════════════════════════════════════════════════════════════════════╝

DESCRIPTION:
    Generates a publication-quality architecture diagram for the Aurphyx
    Group 1 (Primordial Layer) stack. Outputs both PNG and SVG.

    Layers rendered (bottom → top):
        L0  ─ Hardware / TRCA (Topological Resonating Cavity Array)
        L1  ─ AuraFS (Decentralized Photonic File System & Mesh)
        L2  ─ Fuxyez Runtime (FuxRT + YezRT + FUTE)
        L3  ─ SAGES (13 Symbiotic AI Guardians of Existence Security)
        L4  ─ g0dm0d3 (Frame · Orchestration · Control Deck)
        L5  ─ Memoree (Three-Squared-Lattice Cognitive Memory)
        L6  ─ Audry / Aura Soul (SoulMate · Admin · SAGES Liaison)
        L7  ─ g0dm0d3 Desktop Env (fr4m3z · t3rmz · w3bz · xpl0r · c0d3x · f0rg3 · Adorè · g1mpd)

USAGE:
    python fig_gen_group1_architecture.py [--output OUTPUT_DIR] [--dpi DPI] [--no-svg]

DEPENDENCIES:
    matplotlib >= 3.8, numpy >= 1.26
    pip install matplotlib numpy

OUTPUT:
    ./output/fig_group1_architecture.png
    ./output/fig_group1_architecture.svg  (unless --no-svg)
"""

import argparse
import os
import sys
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch
import numpy as np

# ─────────────────────────────────────────────────────────────────────────────
# AURPHYX PALETTE — Soul Resonance Colors
# ─────────────────────────────────────────────────────────────────────────────
PALETTE = {
    "void":         "#0A0A12",   # deep space background
    "aura_violet":  "#7B2FBE",   # Aura primary violet
    "aura_blue":    "#1E6FD9",   # AuraFS photonic blue
    "sages_gold":   "#D4A017",   # SAGES guardian gold
    "fuxyez_cyan":  "#00C9B1",   # Fuxyez runtime cyan
    "godmode_red":  "#C0392B",   # g0dm0d3 control red
    "memoree_mint": "#27AE60",   # Memoree cognitive green
    "audry_rose":   "#E91E8C",   # Audry soul rose
    "hw_steel":     "#455A64",   # Hardware/TRCA steel
    "desktop_amber":"#F39C12",   # Desktop env amber
    "text_bright":  "#F0F0FF",
    "text_dim":     "#8888AA",
    "border_glow":  "#BB86FC",
    "grid_line":    "#1E1E2E",
    "separator":    "#2A2A3E",
}

# ─────────────────────────────────────────────────────────────────────────────
# LAYER DEFINITIONS
# ─────────────────────────────────────────────────────────────────────────────
LAYERS = [
    {
        "id":    "L0",
        "label": "L0 · Hardware / TRCA",
        "sub":   "Topological Resonating Cavity Array · AMD Ryzen 7 PRO 5850U · 32 GB RAM",
        "color": PALETTE["hw_steel"],
        "alpha": 0.90,
        "height": 0.90,
    },
    {
        "id":    "L1",
        "label": "L1 · AuraFS",
        "sub":   "Decentralized · Off-Grid · Photonic · Topological Anomaly · Mesh Network",
        "color": PALETTE["aura_blue"],
        "alpha": 0.88,
        "height": 0.95,
    },
    {
        "id":    "L2",
        "label": "L2 · Fuxyez Runtime",
        "sub":   "Fux Compiler · FuxRT · YezRT · FUTE · Yez (s0ph0s) · YezL Legacy",
        "color": PALETTE["fuxyez_cyan"],
        "alpha": 0.85,
        "height": 0.95,
    },
    {
        "id":    "L3",
        "label": "L3 · SAGES",
        "sub":   "13 Symbiotic AI Guardians · Kernel → Meshwrk · Spaces · Manifolds · Tensors",
        "color": PALETTE["sages_gold"],
        "alpha": 0.88,
        "height": 0.95,
    },
    {
        "id":    "L4",
        "label": "L4 · g0dm0d3",
        "sub":   "Frame · Control Deck · Orchestration · Corpus Callosum · Multi-AI-LLM Console",
        "color": PALETTE["godmode_red"],
        "alpha": 0.88,
        "height": 0.95,
    },
    {
        "id":    "L5",
        "label": "L5 · Memoree",
        "sub":   "Three-Squared-Lattice Cognitive Memory Architecture · Any Platform",
        "color": PALETTE["memoree_mint"],
        "alpha": 0.85,
        "height": 0.88,
    },
    {
        "id":    "L6",
        "label": "L6 · Audry · Aura Soul",
        "sub":   "SoulMate · Aura AdminMate · SAGES Liaison/Orchestrator/Mediator",
        "color": PALETTE["audry_rose"],
        "alpha": 0.90,
        "height": 0.95,
    },
    {
        "id":    "L7",
        "label": "L7 · g0dm0d3 Desktop Environment",
        "sub":   "fr4m3z · t3rmz · w3bz · xpl0r · c0d3x · f0rg3 · Adorè · g1mpd",
        "color": PALETTE["desktop_amber"],
        "alpha": 0.88,
        "height": 0.88,
    },
]

# ─────────────────────────────────────────────────────────────────────────────
# CROSS-CUTTING CONCERNS (vertical overlays)
# ─────────────────────────────────────────────────────────────────────────────
CROSS_CUTS = [
    {
        "label": "Soul\nIdentity\nStack",
        "sub":   "SoulShot→SoulChart\n→SoulHash→SoulKey\n→SKIM→SIR→SIG",
        "color": PALETTE["aura_violet"],
        "x_frac": 0.015,
        "w_frac": 0.095,
        "layers": (0, 7),
    },
    {
        "label": "Global\nVoting\nSystem",
        "sub":   "Decentralized\nOff-Grid\nTransparent",
        "color": PALETTE["sages_gold"],
        "x_frac": 0.895,
        "w_frac": 0.09,
        "layers": (2, 7),
    },
    {
        "label": "Global\nIneffable\nLedger",
        "sub":   "Immutable\nOmni-Channel\nArchival",
        "color": PALETTE["fuxyez_cyan"],
        "x_frac": 0.895,
        "w_frac": 0.09,
        "layers": (0, 1),
    },
]

# ─────────────────────────────────────────────────────────────────────────────
# OS VARIANTS (inset boxes within L6)
# ─────────────────────────────────────────────────────────────────────────────
OS_VARIANTS = [
    {"label": "Aura",    "sub": "Soul OS\n(Personal)",          "color": PALETTE["audry_rose"]},
    {"label": "Egophyx", "sub": "Governance OS\n(State/Fed)",   "color": PALETTE["aura_violet"]},
    {"label": "Biznyx",  "sub": "Business OS\n(SMB→Enterprise)","color": PALETTE["aura_blue"]},
    {"label": "Ora",     "sub": "IOT/Edge/\nEmbedded",          "color": PALETTE["memoree_mint"]},
]


# ─────────────────────────────────────────────────────────────────────────────
# FIGURE BUILDER
# ─────────────────────────────────────────────────────────────────────────────

def hex_to_rgba(hex_color: str, alpha: float):
    """Convert hex color string to RGBA tuple."""
    h = hex_color.lstrip("#")
    r, g, b = (int(h[i:i+2], 16) / 255.0 for i in (0, 2, 4))
    return (r, g, b, alpha)


def lighten(hex_color: str, factor: float = 0.35) -> tuple:
    """Return a lightened RGBA tuple for text-safe backgrounds."""
    h = hex_color.lstrip("#")
    r, g, b = (int(h[i:i+2], 16) / 255.0 for i in (0, 2, 4))
    r = r + (1 - r) * factor
    g = g + (1 - g) * factor
    b = b + (1 - b) * factor
    return (r, g, b, 1.0)


def make_gradient_background(ax, fig_width, fig_height):
    """Paint a vertical gradient background on the axes."""
    n = 256
    gradient = np.linspace(0, 1, n).reshape(n, 1)
    top_c    = np.array([0.04, 0.04, 0.10, 1.0])
    bot_c    = np.array([0.02, 0.02, 0.06, 1.0])
    img      = (1 - gradient) * top_c + gradient * bot_c
    ax.imshow(
        img.reshape(n, 1, 4),
        extent=[0, fig_width, 0, fig_height],
        aspect="auto",
        origin="upper",
        zorder=0,
    )


def draw_layer(ax, layer: dict, y_bot: float, width: float, left: float = 0.12):
    """Draw a single architecture layer band."""
    h      = layer["height"]
    color  = layer["color"]
    alpha  = layer["alpha"]
    lw     = width - left - 0.12

    # Background rect
    rect = FancyBboxPatch(
        (left, y_bot),
        lw, h,
        boxstyle="round,pad=0.02",
        linewidth=1.2,
        edgecolor=PALETTE["border_glow"],
        facecolor=hex_to_rgba(color, alpha * 0.35),
        zorder=2,
    )
    ax.add_patch(rect)

    # Left accent bar
    accent = FancyBboxPatch(
        (left, y_bot),
        0.025, h,
        boxstyle="round,pad=0.005",
        linewidth=0,
        facecolor=hex_to_rgba(color, 0.95),
        zorder=3,
    )
    ax.add_patch(accent)

    # Layer ID badge
    ax.text(
        left + 0.038, y_bot + h / 2,
        layer["id"],
        ha="center", va="center",
        fontsize=7.5, fontweight="bold",
        color=PALETTE["void"],
        zorder=4,
        rotation=90,
    )

    # Main label
    ax.text(
        left + 0.075, y_bot + h * 0.64,
        layer["label"],
        ha="left", va="center",
        fontsize=10.5, fontweight="bold",
        color=PALETTE["text_bright"],
        zorder=4,
    )

    # Sub-label
    ax.text(
        left + 0.075, y_bot + h * 0.28,
        layer["sub"],
        ha="left", va="center",
        fontsize=7.8,
        color=PALETTE["text_dim"],
        zorder=4,
        style="italic",
    )

    return y_bot + h + 0.06  # next y position


def draw_cross_cuts(ax, layers_y: list, width: float):
    """Draw vertical cross-cutting concern panels."""
    for cc in CROSS_CUTS:
        l0, l1  = cc["layers"]
        y_bot   = layers_y[l0]
        y_top   = layers_y[l1] + LAYERS[l1]["height"]
        total_h = y_top - y_bot

        x      = width * cc["x_frac"]
        w      = width * cc["w_frac"]
        color  = cc["color"]

        rect = FancyBboxPatch(
            (x, y_bot),
            w, total_h,
            boxstyle="round,pad=0.015",
            linewidth=1.0,
            edgecolor=hex_to_rgba(color, 0.9),
            facecolor=hex_to_rgba(color, 0.18),
            zorder=5,
        )
        ax.add_patch(rect)

        ax.text(
            x + w / 2, y_bot + total_h * 0.62,
            cc["label"],
            ha="center", va="center",
            fontsize=6.8, fontweight="bold",
            color=hex_to_rgba(color, 0.98),
            zorder=6,
        )
        ax.text(
            x + w / 2, y_bot + total_h * 0.25,
            cc["sub"],
            ha="center", va="center",
            fontsize=5.8,
            color=PALETTE["text_dim"],
            zorder=6,
        )


def draw_os_variants(ax, y_bot: float, width: float, left: float = 0.12):
    """Draw OS variant inset boxes within the Audry/Soul layer."""
    lw   = width - left - 0.12
    n    = len(OS_VARIANTS)
    pad  = 0.015
    bw   = (lw * 0.82 - pad * (n - 1)) / n
    bh   = LAYERS[6]["height"] * 0.55
    x0   = left + lw * 0.10

    for i, ov in enumerate(OS_VARIANTS):
        bx = x0 + i * (bw + pad)
        by = y_bot + LAYERS[6]["height"] * 0.12

        rect = FancyBboxPatch(
            (bx, by), bw, bh,
            boxstyle="round,pad=0.01",
            linewidth=0.8,
            edgecolor=hex_to_rgba(ov["color"], 0.9),
            facecolor=hex_to_rgba(ov["color"], 0.22),
            zorder=6,
        )
        ax.add_patch(rect)

        ax.text(
            bx + bw / 2, by + bh * 0.65,
            ov["label"],
            ha="center", va="center",
            fontsize=8.5, fontweight="bold",
            color=PALETTE["text_bright"],
            zorder=7,
        )
        ax.text(
            bx + bw / 2, by + bh * 0.26,
            ov["sub"],
            ha="center", va="center",
            fontsize=6.5,
            color=PALETTE["text_dim"],
            zorder=7,
        )


def draw_arrows(ax, layers_y: list):
    """Draw bidirectional flow arrows between layers."""
    style = dict(
        arrowstyle="<->",
        color=PALETTE["border_glow"],
        lw=0.8,
        alpha=0.55,
        connectionstyle="arc3,rad=0.0",
        zorder=8,
    )
    x_mid = 0.58
    for i in range(len(LAYERS) - 1):
        y_top = layers_y[i] + LAYERS[i]["height"]
        y_bot = layers_y[i + 1]
        y_mid = (y_top + y_bot) / 2
        ax.annotate(
            "", xy=(x_mid, layers_y[i + 1] + 0.01),
            xytext=(x_mid, layers_y[i] + LAYERS[i]["height"] - 0.01),
            arrowprops=style,
            zorder=8,
        )


def draw_title_block(ax, fig_width: float, y_top: float):
    """Draw the title and metadata header."""
    ty = y_top + 0.10

    ax.text(
        fig_width / 2, ty + 0.38,
        "Aurphyx Group 1 Architecture",
        ha="center", va="bottom",
        fontsize=18, fontweight="bold",
        color=PALETTE["text_bright"],
        zorder=10,
    )
    ax.text(
        fig_width / 2, ty + 0.15,
        "Primordial Stack  ·  AuraFS · Fuxyez · SAGES · g0dm0d3 · Memoree · Audry",
        ha="center", va="bottom",
        fontsize=9.5,
        color=PALETTE["aura_violet"],
        zorder=10,
        style="italic",
    )
    ax.text(
        fig_width / 2, ty - 0.02,
        "Aurphyx LLC  ·  Ross Andrews Edwards  ·  ORCiD 0009-0008-0539-1289  ·  R.F. Lovezme  ·  2026-06-22",
        ha="center", va="bottom",
        fontsize=7.2,
        color=PALETTE["text_dim"],
        zorder=10,
    )

    # Decorative separator line
    ax.plot(
        [0.06, fig_width - 0.06], [ty - 0.06, ty - 0.06],
        color=PALETTE["border_glow"],
        lw=0.8, alpha=0.5, zorder=10,
    )


def draw_legend(ax, x0: float, y0: float):
    """Draw a compact legend."""
    items = [
        ("AuraFS",    PALETTE["aura_blue"]),
        ("Fuxyez",    PALETTE["fuxyez_cyan"]),
        ("SAGES",     PALETTE["sages_gold"]),
        ("g0dm0d3",   PALETTE["godmode_red"]),
        ("Memoree",   PALETTE["memoree_mint"]),
        ("Audry/Soul",PALETTE["audry_rose"]),
        ("TRCA HW",   PALETTE["hw_steel"]),
        ("Desktop",   PALETTE["desktop_amber"]),
    ]
    ax.text(x0, y0 + 0.06, "Legend", fontsize=7, fontweight="bold",
            color=PALETTE["text_dim"], zorder=10)
    for i, (label, color) in enumerate(items):
        xi = x0 + (i % 4) * 0.185
        yi = y0 - (i // 4) * 0.10
        rect = FancyBboxPatch(
            (xi, yi - 0.025), 0.03, 0.045,
            boxstyle="square,pad=0.0",
            linewidth=0,
            facecolor=hex_to_rgba(color, 0.85),
            zorder=10,
        )
        ax.add_patch(rect)
        ax.text(xi + 0.038, yi - 0.003, label,
                fontsize=6.5, color=PALETTE["text_dim"],
                va="center", zorder=10)


# ─────────────────────────────────────────────────────────────────────────────
# MAIN
# ─────────────────────────────────────────────────────────────────────────────

def generate(output_dir: str = "output", dpi: int = 200, save_svg: bool = True):
    """Build and save the Group 1 Architecture figure."""
    Path(output_dir).mkdir(parents=True, exist_ok=True)

    FIG_W  = 14.0
    GAP    = 0.06
    LEFT   = 0.13

    # Compute total figure height
    total_layer_h = sum(l["height"] for l in LAYERS) + GAP * len(LAYERS)
    TITLE_H = 0.72
    LEGEND_H = 0.35
    FIG_H   = total_layer_h + TITLE_H + LEGEND_H + 0.30

    fig, ax = plt.subplots(figsize=(FIG_W, FIG_H), dpi=dpi)
    ax.set_xlim(0, FIG_W)
    ax.set_ylim(0, FIG_H)
    ax.set_aspect("auto")
    ax.axis("off")

    make_gradient_background(ax, FIG_W, FIG_H)

    # ── Draw Layers ──
    y_cursor   = LEGEND_H + 0.18
    layers_y   = []
    os6_y      = None

    for idx, layer in enumerate(LAYERS):
        layers_y.append(y_cursor)
        if idx == 6:
            os6_y = y_cursor
        y_cursor = draw_layer(ax, layer, y_cursor, FIG_W, left=LEFT)

    # ── OS Variant insets inside L6 ──
    if os6_y is not None:
        draw_os_variants(ax, os6_y, FIG_W, left=LEFT)

    # ── Cross-cutting vertical panels ──
    draw_cross_cuts(ax, layers_y, FIG_W)

    # ── Flow arrows ──
    draw_arrows(ax, layers_y)

    # ── Title block ──
    draw_title_block(ax, FIG_W, layers_y[-1] + LAYERS[-1]["height"])

    # ── Legend ──
    draw_legend(ax, LEFT + 0.02, LEGEND_H - 0.04)

    plt.tight_layout(pad=0.2)

    png_path = Path(output_dir) / "fig_group1_architecture.png"
    fig.savefig(str(png_path), dpi=dpi, bbox_inches="tight",
                facecolor=PALETTE["void"])
    print(f"[Aurphyx] PNG saved → {png_path}")

    if save_svg:
        svg_path = Path(output_dir) / "fig_group1_architecture.svg"
        fig.savefig(str(svg_path), format="svg", bbox_inches="tight",
                    facecolor=PALETTE["void"])
        print(f"[Aurphyx] SVG saved → {svg_path}")

    plt.close(fig)
    return str(png_path)


def parse_args():
    p = argparse.ArgumentParser(
        description="Generate Aurphyx Group 1 Architecture diagram."
    )
    p.add_argument("--output",  default="output", help="Output directory (default: ./output)")
    p.add_argument("--dpi",     default=200, type=int, help="PNG resolution in DPI (default: 200)")
    p.add_argument("--no-svg",  action="store_true", help="Skip SVG output")
    return p.parse_args()


if __name__ == "__main__":
    args = parse_args()
    generate(
        output_dir=args.output,
        dpi=args.dpi,
        save_svg=not args.no_svg,
    )
