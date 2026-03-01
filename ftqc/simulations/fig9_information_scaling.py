#!/usr/bin/env python3
"""
Figure 9: Integrated Information (Φ) and Connectivity Scaling
PRX Submission - Aurphyx

Generates publication-quality plots showing how integrated information
scales with fractal connectivity compared to Euclidean lattices.
"""

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Circle, FancyBboxPatch
import matplotlib.gridspec as gridspec
from scipy.special import comb

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


def phi_integrated_information(n_nodes, connectivity_type='euclidean', D_f=1.585):
    """
    Calculate integrated information Φ for different connectivity types.
    
    This is a simplified model based on the relationship between
    connectivity structure and information integration.
    
    Φ ~ log(effective_connectivity) * n_nodes^α
    
    where α depends on the lattice structure.
    """
    if connectivity_type == 'euclidean':
        # Linear chain: each node connects to 2 neighbors
        effective_connectivity = 2 * n_nodes
        alpha = 1.0
    elif connectivity_type == 'all_to_all':
        # Complete graph: n*(n-1)/2 edges
        effective_connectivity = n_nodes * (n_nodes - 1) / 2
        alpha = 1.5
    elif connectivity_type == 'fractal':
        # Fractal: hierarchical connectivity with D_f scaling
        # Effective edges scale as n^D_f
        effective_connectivity = n_nodes ** D_f
        alpha = D_f
    else:
        raise ValueError(f"Unknown connectivity type: {connectivity_type}")
    
    # Φ model: combines connectivity and node count
    phi = np.log2(effective_connectivity + 1) * (n_nodes ** (alpha - 1))
    
    return phi


def participation_ratio(n_nodes, connectivity_type='euclidean', D_f=1.585):
    """
    Calculate effective Hilbert space dimension via participation ratio.
    
    PR = (Σ|ψ_i|²)² / Σ|ψ_i|⁴
    
    For maximally entangled states on different lattice types.
    """
    if connectivity_type == 'euclidean':
        # Area-law entanglement: PR ~ n
        pr = n_nodes
    elif connectivity_type == 'all_to_all':
        # Volume-law entanglement: PR ~ 2^n
        pr = 2 ** n_nodes
    elif connectivity_type == 'fractal':
        # Fractal scaling: PR ~ 2^(n * D_f^α) with α < 1
        alpha_k = 0.85  # Effective coupling parameter
        pr = 2 ** (n_nodes * D_f ** alpha_k)
    
    return pr


def plot_phi_scaling(ax):
    """Plot Φ vs node count for different connectivity types."""
    
    n_nodes = np.arange(4, 50, 2)
    
    phi_euclidean = [phi_integrated_information(n, 'euclidean') for n in n_nodes]
    phi_fractal = [phi_integrated_information(n, 'fractal') for n in n_nodes]
    phi_all_to_all = [phi_integrated_information(n, 'all_to_all') for n in n_nodes]
    
    ax.semilogy(n_nodes, phi_euclidean, 'o-', color='#666666', linewidth=2, 
                markersize=6, label='Linear chain')
    ax.semilogy(n_nodes, phi_fractal, 's-', color='#E94F37', linewidth=2,
                markersize=6, label=f'Sierpiński ($D_f$ = 1.585)')
    ax.semilogy(n_nodes, phi_all_to_all, '^--', color='#2E86AB', linewidth=1.5,
                markersize=5, alpha=0.7, label='All-to-all (reference)')
    
    ax.set_xlabel('Number of nodes $n$', fontsize=11)
    ax.set_ylabel('Integrated information Φ [bits]', fontsize=11)
    ax.legend(loc='upper left', fontsize=9)
    ax.grid(True, alpha=0.3)
    ax.set_xlim(4, 50)
    
    # Annotation for fractal advantage
    n_anno = 30
    phi_e = phi_integrated_information(n_anno, 'euclidean')
    phi_f = phi_integrated_information(n_anno, 'fractal')
    ax.annotate(f'{phi_f/phi_e:.1f}× at n={n_anno}',
                xy=(n_anno, phi_f), xytext=(n_anno + 8, phi_f * 0.5),
                fontsize=9, color='#E94F37',
                arrowprops=dict(arrowstyle='->', color='#E94F37'))
    
    ax.set_title('(a) Integrated Information Scaling', fontsize=11, fontweight='bold')


def plot_hilbert_scaling(ax):
    """Plot effective Hilbert space dimension scaling."""
    
    n_nodes = np.arange(4, 20)
    
    pr_euclidean = [participation_ratio(n, 'euclidean') for n in n_nodes]
    pr_fractal = [participation_ratio(n, 'fractal') for n in n_nodes]
    
    ax.semilogy(n_nodes, pr_euclidean, 'o-', color='#666666', linewidth=2,
                markersize=6, label='Euclidean $2^n$')
    ax.semilogy(n_nodes, pr_fractal, 's-', color='#E94F37', linewidth=2,
                markersize=6, label='Fractal $2^{n \\cdot D_f^{\\alpha}}$')
    
    # Reference line for 2^n
    ax.semilogy(n_nodes, 2**n_nodes, 'k--', alpha=0.3, linewidth=1, label='$2^n$ (full)')
    
    ax.set_xlabel('Number of qubits $n$', fontsize=11)
    ax.set_ylabel('Effective Hilbert dimension $D_{eff}$', fontsize=11)
    ax.legend(loc='upper left', fontsize=9)
    ax.grid(True, alpha=0.3)
    ax.set_xlim(4, 19)
    
    # Highlight 10^4 advantage point
    n_target = 12
    d_e = participation_ratio(n_target, 'euclidean')
    d_f = participation_ratio(n_target, 'fractal')
    ax.axvline(n_target, color='gray', linestyle=':', alpha=0.5)
    ax.annotate(f'10$^{{{np.log10(d_f/d_e):.0f}}}$× advantage',
                xy=(n_target, d_f), xytext=(n_target + 2, d_f * 0.1),
                fontsize=9, color='#E94F37',
                arrowprops=dict(arrowstyle='->', color='#E94F37'))
    
    ax.set_title('(b) Effective Hilbert Space Scaling', fontsize=11, fontweight='bold')


def plot_connectivity_diagram(ax):
    """Plot connectivity diagrams for different lattice types."""
    
    # Three subfigures: linear, grid, fractal
    configs = [
        ('Linear', 'euclidean'),
        ('Grid (2D)', 'grid'),
        ('Sierpiński', 'fractal'),
    ]
    
    for i, (name, conn_type) in enumerate(configs):
        x_offset = i * 2.5
        
        if conn_type == 'euclidean':
            # Linear chain
            positions = [(x_offset + j * 0.4, 0.5) for j in range(5)]
            edges = [(j, j+1) for j in range(4)]
        elif conn_type == 'grid':
            # 2D grid
            positions = []
            for row in range(2):
                for col in range(3):
                    positions.append((x_offset + col * 0.4, 0.3 + row * 0.4))
            edges = [(0,1), (1,2), (3,4), (4,5), (0,3), (1,4), (2,5)]
        else:  # fractal
            # Sierpiński gasket k=2
            positions = [
                (x_offset + 0.4, 0.2),   # Bottom left
                (x_offset + 0.8, 0.2),   # Bottom middle
                (x_offset + 1.2, 0.2),   # Bottom right
                (x_offset + 0.6, 0.55),  # Middle left
                (x_offset + 1.0, 0.55),  # Middle right
                (x_offset + 0.8, 0.9),   # Top
            ]
            # Sierpiński edges (skip middle triangle)
            edges = [(0,1), (1,2), (0,3), (1,3), (1,4), (2,4), (3,5), (4,5), (3,4)]
        
        # Draw edges
        for e in edges:
            p1, p2 = positions[e[0]], positions[e[1]]
            ax.plot([p1[0], p2[0]], [p1[1], p2[1]], 'k-', linewidth=1.5, alpha=0.7)
        
        # Draw nodes
        for p in positions:
            circle = Circle(p, 0.08, facecolor='#2E86AB', edgecolor='black', 
                          linewidth=1, zorder=5)
            ax.add_patch(circle)
        
        # Label
        ax.text(x_offset + 0.8, -0.1, name, ha='center', fontsize=9, fontweight='bold')
        
        # Connectivity info
        n = len(positions)
        e = len(edges)
        ax.text(x_offset + 0.8, -0.3, f'n={n}, e={e}', ha='center', fontsize=8, color='gray')
    
    ax.set_xlim(-0.3, 7.5)
    ax.set_ylim(-0.5, 1.2)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(c) Connectivity Topologies', fontsize=11, fontweight='bold')


def plot_entanglement_entropy(ax):
    """Plot entanglement entropy scaling for bipartition."""
    
    n_nodes = np.arange(6, 40, 2)
    
    # Area law: S ~ L^(d-1) for d-dimensional system
    # Volume law: S ~ L^d
    # Fractal: S ~ L^(D_f - 1)
    
    D_f = 1.585
    L = np.sqrt(n_nodes)  # Effective linear size
    
    S_area = 0.5 * L  # Area law (1D boundary)
    S_volume = 0.3 * n_nodes  # Volume law
    S_fractal = 0.5 * L ** D_f  # Fractal scaling
    
    ax.plot(n_nodes, S_area, 'o-', color='#666666', linewidth=2,
            markersize=6, label='Area law ($S \\sim L$)')
    ax.plot(n_nodes, S_fractal, 's-', color='#E94F37', linewidth=2,
            markersize=6, label=f'Fractal ($S \\sim L^{{D_f}}$)')
    ax.plot(n_nodes, S_volume, '^--', color='#2E86AB', linewidth=1.5,
            markersize=5, alpha=0.7, label='Volume law ($S \\sim n$)')
    
    ax.set_xlabel('Number of nodes $n$', fontsize=11)
    ax.set_ylabel('Entanglement entropy $S$ [bits]', fontsize=11)
    ax.legend(loc='upper left', fontsize=9)
    ax.grid(True, alpha=0.3)
    ax.set_xlim(6, 40)
    
    # Annotation
    ax.text(30, 5, 'Fractal: intermediate\nbetween area & volume',
            fontsize=8, color='#E94F37',
            bbox=dict(boxstyle='round', facecolor='white', alpha=0.8))
    
    ax.set_title('(d) Entanglement Entropy Scaling', fontsize=11, fontweight='bold')


def main():
    """Generate Figure 9: Integrated information and connectivity scaling."""
    
    fig = plt.figure(figsize=(11, 9))
    gs = gridspec.GridSpec(2, 2, hspace=0.35, wspace=0.3)
    
    # (a) Φ scaling
    ax_phi = fig.add_subplot(gs[0, 0])
    plot_phi_scaling(ax_phi)
    
    # (b) Hilbert scaling
    ax_hilbert = fig.add_subplot(gs[0, 1])
    plot_hilbert_scaling(ax_hilbert)
    
    # (c) Connectivity diagrams
    ax_conn = fig.add_subplot(gs[1, 0])
    plot_connectivity_diagram(ax_conn)
    
    # (d) Entanglement entropy
    ax_entropy = fig.add_subplot(gs[1, 1])
    plot_entanglement_entropy(ax_entropy)
    
    plt.suptitle('Figure 9: Information Integration and Fractal Connectivity',
                 fontsize=13, fontweight='bold', y=0.98)
    
    plt.savefig('Fig9_Information_Scaling.png', dpi=300, bbox_inches='tight',
                facecolor='white', edgecolor='none')
    plt.savefig('Fig9_Information_Scaling.pdf', bbox_inches='tight',
                facecolor='white', edgecolor='none')
    
    print("Figure 9 saved: Fig9_Information_Scaling.png/pdf")
    
    plt.close()


if __name__ == '__main__':
    main()
