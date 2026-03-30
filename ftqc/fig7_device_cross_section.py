#!/usr/bin/env python3
"""
Figure 7: Device Cross-Section Schematics
PRX Submission - Aurphyx

Generates publication-quality device cross-section diagrams for:
(a) NV-diamond photonic crystal
(b) Hexagonal photonic waveguide array
"""

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle, Circle, FancyBboxPatch, Polygon, Wedge
from matplotlib.collections import PatchCollection
import matplotlib.gridspec as gridspec

# Publication settings
plt.rcParams.update({
    'font.family': 'serif',
    'font.size': 10,
    'axes.labelsize': 11,
    'axes.titlesize': 12,
    'xtick.labelsize': 9,
    'ytick.labelsize': 9,
    'legend.fontsize': 9,
    'figure.dpi': 300,
    'savefig.dpi': 300,
    'savefig.bbox': 'tight',
    'text.usetex': False,
})


def draw_nv_diamond_cross_section(ax):
    """Draw cross-section of NV-diamond photonic crystal device."""
    
    # Layer thicknesses (in μm, scaled for display)
    scale = 10  # pixels per μm
    
    # Substrate (silicon)
    substrate = Rectangle((-2, -0.5), 8, 0.5, facecolor='#4A4A4A', edgecolor='black')
    ax.add_patch(substrate)
    ax.text(2, -0.25, 'Si substrate', ha='center', va='center', fontsize=9, color='white')
    
    # SiO2 buffer layer
    buffer = Rectangle((-2, 0), 8, 0.3, facecolor='#87CEEB', edgecolor='black', alpha=0.7)
    ax.add_patch(buffer)
    ax.text(-1.5, 0.15, 'SiO₂ (300 nm)', ha='left', va='center', fontsize=8)
    
    # Diamond membrane
    diamond_y = 0.3
    diamond_h = 0.5
    diamond = Rectangle((-2, diamond_y), 8, diamond_h, facecolor='#E8E8E8', edgecolor='black')
    ax.add_patch(diamond)
    
    # Sierpiński pattern etched into diamond (simplified as holes)
    # Level 1 Sierpiński: 3 triangular holes
    hole_positions = [
        (0.5, diamond_y + 0.25),
        (1.5, diamond_y + 0.25),
        (1.0, diamond_y + 0.5),
        (2.5, diamond_y + 0.25),
        (3.5, diamond_y + 0.25),
        (3.0, diamond_y + 0.5),
    ]
    
    for x, y in hole_positions:
        hole = Circle((x, y), 0.08, facecolor='white', edgecolor='#666666', linewidth=0.5)
        ax.add_patch(hole)
    
    ax.text(4.5, diamond_y + diamond_h/2, 'CVD Diamond\n(500 nm)', ha='left', va='center', fontsize=8)
    
    # NV centers (red dots)
    nv_positions = [(0.5, diamond_y + 0.35), (1.5, diamond_y + 0.35), 
                    (2.5, diamond_y + 0.35), (3.5, diamond_y + 0.35)]
    for x, y in nv_positions:
        nv = Circle((x, y), 0.05, facecolor='#E94F37', edgecolor='black', linewidth=0.5, zorder=10)
        ax.add_patch(nv)
    
    # NV center legend
    nv_legend = Circle((4.8, diamond_y + 0.35), 0.05, facecolor='#E94F37', edgecolor='black', linewidth=0.5)
    ax.add_patch(nv_legend)
    ax.text(5.0, diamond_y + 0.35, 'NV center', ha='left', va='center', fontsize=8)
    
    # Top cladding (air)
    ax.text(2, diamond_y + diamond_h + 0.15, 'Air', ha='center', va='center', fontsize=9, color='gray')
    
    # Dimension annotations
    # Etch depth
    ax.annotate('', xy=(4.2, diamond_y), xytext=(4.2, diamond_y + diamond_h),
                arrowprops=dict(arrowstyle='<->', color='black', lw=1))
    ax.text(4.35, diamond_y + diamond_h/2, '500 nm', fontsize=7, va='center')
    
    # Hole spacing
    ax.annotate('', xy=(0.5, diamond_y - 0.1), xytext=(1.5, diamond_y - 0.1),
                arrowprops=dict(arrowstyle='<->', color='#2E86AB', lw=1))
    ax.text(1.0, diamond_y - 0.18, '100 nm', fontsize=7, ha='center', color='#2E86AB')
    
    # Confocal excitation beam
    beam_x = [1.0, 0.7, 1.3]
    beam_y = [diamond_y + diamond_h + 0.6, diamond_y + diamond_h, diamond_y + diamond_h]
    beam = Polygon(list(zip(beam_x, beam_y)), facecolor='#90EE90', alpha=0.5, edgecolor='green')
    ax.add_patch(beam)
    ax.text(1.0, diamond_y + diamond_h + 0.7, '532 nm\nexcitation', ha='center', va='bottom', 
            fontsize=8, color='green')
    
    ax.set_xlim(-2.5, 6)
    ax.set_ylim(-0.7, 1.5)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(a) NV-Diamond Sierpiński Array Cross-Section', fontsize=11, fontweight='bold')


def draw_photonic_waveguide_array(ax):
    """Draw top-view of hexagonal photonic waveguide array."""
    
    # Fused silica substrate background
    substrate = Rectangle((-1, -1), 6, 5, facecolor='#E8F4FC', edgecolor='black')
    ax.add_patch(substrate)
    
    # Hexagonal lattice of waveguides
    a = 0.5  # lattice constant
    
    # Generate hexagonal positions
    positions = []
    for i in range(-2, 8):
        for j in range(-2, 8):
            x = i * a + j * a * 0.5
            y = j * a * np.sqrt(3) / 2
            if -0.5 < x < 5 and -0.5 < y < 3.5:
                positions.append((x, y))
    
    # Draw waveguides
    for x, y in positions:
        wg = Circle((x, y), 0.08, facecolor='#2E86AB', edgecolor='#1A5276', linewidth=0.5)
        ax.add_patch(wg)
    
    # Highlight edge states path
    edge_path_x = [0.25, 0.75, 1.25, 1.75, 2.25, 2.75, 3.25, 3.75]
    edge_path_y = [0.22] * 8
    ax.plot(edge_path_x, edge_path_y, 'r-', linewidth=2, alpha=0.7)
    ax.annotate('', xy=(3.75, 0.22), xytext=(3.25, 0.22),
                arrowprops=dict(arrowstyle='->', color='red', lw=2))
    ax.text(2.0, -0.1, 'Edge state transport', ha='center', fontsize=8, color='red')
    
    # Band gap region annotation
    ax.add_patch(Rectangle((0.5, 1.0), 3.0, 1.5, facecolor='#FFE4B5', 
                            edgecolor='orange', alpha=0.3, linestyle='--'))
    ax.text(2.0, 1.75, 'Band gap\nregion', ha='center', va='center', fontsize=8, color='orange')
    
    # Dimension annotations
    ax.annotate('', xy=(0, 0), xytext=(0.5, 0),
                arrowprops=dict(arrowstyle='<->', color='black', lw=1))
    ax.text(0.25, -0.2, 'a = 15 μm', ha='center', fontsize=8)
    
    # Waveguide diameter
    ax.annotate('', xy=(4.3, 2.6), xytext=(4.3, 2.76),
                arrowprops=dict(arrowstyle='<->', color='black', lw=0.8))
    ax.text(4.5, 2.68, 'd = 8 μm', ha='left', fontsize=7)
    
    # Material label
    ax.text(4.5, 0.5, 'Fused silica\n(n = 1.46)', ha='center', fontsize=8,
            bbox=dict(boxstyle='round', facecolor='white', alpha=0.8))
    
    ax.set_xlim(-0.7, 5.2)
    ax.set_ylim(-0.5, 4)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(b) Hexagonal Photonic Crystal (Top View)', fontsize=11, fontweight='bold')


def draw_fabrication_process(ax):
    """Draw simplified fabrication process flow."""
    
    steps = [
        ('1. CVD\ngrowth', '#E8E8E8'),
        ('2. E-beam\nlithography', '#FFB6C1'),
        ('3. RIE\netching', '#98FB98'),
        ('4. NV\nimplant', '#E94F37'),
        ('5. Anneal\n(800°C)', '#FFA500'),
    ]
    
    for i, (label, color) in enumerate(steps):
        x = i * 1.2
        
        # Process box
        box = FancyBboxPatch((x, 0), 1.0, 0.8, boxstyle='round,pad=0.05',
                              facecolor=color, edgecolor='black', linewidth=1)
        ax.add_patch(box)
        ax.text(x + 0.5, 0.4, label, ha='center', va='center', fontsize=8)
        
        # Arrow to next step
        if i < len(steps) - 1:
            ax.annotate('', xy=(x + 1.15, 0.4), xytext=(x + 1.0, 0.4),
                        arrowprops=dict(arrowstyle='->', color='black', lw=1))
    
    ax.set_xlim(-0.3, 6.5)
    ax.set_ylim(-0.3, 1.2)
    ax.axis('off')
    ax.set_title('(c) Fabrication Process Flow', fontsize=11, fontweight='bold')


def draw_layer_stack(ax):
    """Draw detailed layer stack with materials."""
    
    layers = [
        ('Air (n=1.0)', 0.3, '#FFFFFF', 'black'),
        ('Diamond (n=2.4)', 0.5, '#E8E8E8', 'black'),
        ('SiO₂ (n=1.46)', 0.3, '#87CEEB', 'black'),
        ('Si substrate', 0.4, '#4A4A4A', 'white'),
    ]
    
    y = 0
    for label, height, color, text_color in layers:
        rect = Rectangle((0, y), 2, height, facecolor=color, edgecolor='black', linewidth=1)
        ax.add_patch(rect)
        ax.text(1, y + height/2, label, ha='center', va='center', 
                fontsize=9, color=text_color)
        y += height
    
    # Total thickness annotation
    ax.annotate('', xy=(2.2, 0), xytext=(2.2, 1.5),
                arrowprops=dict(arrowstyle='<->', color='black', lw=1))
    ax.text(2.4, 0.75, '~1.5 μm\ntotal', fontsize=8, va='center')
    
    ax.set_xlim(-0.5, 3.5)
    ax.set_ylim(-0.2, 2.0)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(d) Layer Stack', fontsize=11, fontweight='bold')


def main():
    """Generate Figure 7: Device cross-section schematics."""
    
    fig = plt.figure(figsize=(12, 9))
    gs = gridspec.GridSpec(2, 2, hspace=0.35, wspace=0.25,
                            height_ratios=[1.2, 1])
    
    # (a) NV-diamond cross-section
    ax_nv = fig.add_subplot(gs[0, 0])
    draw_nv_diamond_cross_section(ax_nv)
    
    # (b) Photonic waveguide array
    ax_photonic = fig.add_subplot(gs[0, 1])
    draw_photonic_waveguide_array(ax_photonic)
    
    # (c) Fabrication process
    ax_fab = fig.add_subplot(gs[1, 0])
    draw_fabrication_process(ax_fab)
    
    # (d) Layer stack
    ax_stack = fig.add_subplot(gs[1, 1])
    draw_layer_stack(ax_stack)
    
    plt.suptitle('Figure 7: Device Architecture and Fabrication',
                 fontsize=13, fontweight='bold', y=0.98)
    
    plt.savefig('Fig7_Device_Cross_Section.png', dpi=300, bbox_inches='tight',
                facecolor='white', edgecolor='none')
    plt.savefig('Fig7_Device_Cross_Section.pdf', bbox_inches='tight',
                facecolor='white', edgecolor='none')
    
    print("Figure 7 saved: Fig7_Device_Cross_Section.png/pdf")
    
    plt.close()


if __name__ == '__main__':
    main()
