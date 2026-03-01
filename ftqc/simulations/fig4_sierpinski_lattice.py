#!/usr/bin/env python3
"""
Figure 4: Sierpiński Gasket Lattice Visualization
PRX Submission - Aurphyx

Generates publication-quality visualization of Sierpiński gaskets
at recursion depths k=0,1,2,3 with node count scaling.
"""

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon
from matplotlib.collections import PatchCollection, LineCollection
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

def sierpinski_vertices(k, base_vertices=None):
    """
    Generate vertices for Sierpiński gasket at recursion depth k.
    
    Parameters
    ----------
    k : int
        Recursion depth (0 = single triangle)
    base_vertices : array-like, optional
        Initial triangle vertices
        
    Returns
    -------
    vertices : ndarray
        Unique vertices of the gasket
    edges : list
        Pairs of vertex indices forming edges
    """
    if base_vertices is None:
        # Equilateral triangle with unit side
        base_vertices = np.array([
            [0, 0],
            [1, 0],
            [0.5, np.sqrt(3)/2]
        ])
    
    if k == 0:
        vertices = base_vertices
        edges = [(0, 1), (1, 2), (2, 0)]
        return vertices, edges
    
    # Recursive construction
    triangles = [base_vertices]
    
    for _ in range(k):
        new_triangles = []
        for tri in triangles:
            # Midpoints
            m01 = (tri[0] + tri[1]) / 2
            m12 = (tri[1] + tri[2]) / 2
            m20 = (tri[2] + tri[0]) / 2
            
            # Three sub-triangles (exclude center)
            new_triangles.append(np.array([tri[0], m01, m20]))
            new_triangles.append(np.array([m01, tri[1], m12]))
            new_triangles.append(np.array([m20, m12, tri[2]]))
        
        triangles = new_triangles
    
    # Extract unique vertices
    all_vertices = np.vstack(triangles)
    vertices = np.unique(all_vertices.round(decimals=10), axis=0)
    
    # Build edge list
    edges = set()
    for tri in triangles:
        # Find indices of triangle vertices in unique list
        idx = []
        for v in tri:
            for i, uv in enumerate(vertices):
                if np.allclose(v, uv):
                    idx.append(i)
                    break
        edges.add(tuple(sorted([idx[0], idx[1]])))
        edges.add(tuple(sorted([idx[1], idx[2]])))
        edges.add(tuple(sorted([idx[2], idx[0]])))
    
    return vertices, list(edges)


def plot_sierpinski(ax, k, color='#2E86AB', node_color='#E94F37', 
                    show_nodes=True, title=None):
    """Plot Sierpiński gasket at depth k."""
    vertices, edges = sierpinski_vertices(k)
    
    # Draw edges
    lines = [[vertices[e[0]], vertices[e[1]]] for e in edges]
    lc = LineCollection(lines, colors=color, linewidths=1.5, alpha=0.8)
    ax.add_collection(lc)
    
    # Draw nodes
    if show_nodes:
        ax.scatter(vertices[:, 0], vertices[:, 1], 
                   c=node_color, s=50/(k+1), zorder=5, edgecolors='white', linewidths=0.5)
    
    ax.set_xlim(-0.1, 1.1)
    ax.set_ylim(-0.1, 1.0)
    ax.set_aspect('equal')
    ax.axis('off')
    
    if title:
        ax.set_title(title, fontsize=11, fontweight='bold')
    
    return len(vertices), len(edges)


def main():
    """Generate Figure 4: Sierpiński lattice visualization."""
    
    fig = plt.figure(figsize=(10, 6))
    gs = gridspec.GridSpec(2, 4, height_ratios=[3, 2], hspace=0.3, wspace=0.1)
    
    # Top row: Sierpiński gaskets k=0,1,2,3
    colors = ['#2E86AB', '#A23B72', '#F18F01', '#C73E1D']
    node_counts = []
    edge_counts = []
    
    for i, k in enumerate([0, 1, 2, 3]):
        ax = fig.add_subplot(gs[0, i])
        n_nodes, n_edges = plot_sierpinski(ax, k, color=colors[i], 
                                            title=f'k = {k}')
        node_counts.append(n_nodes)
        edge_counts.append(n_edges)
        
        # Add node count annotation
        ax.text(0.5, -0.05, f'N = {n_nodes}', ha='center', va='top',
                transform=ax.transAxes, fontsize=9)
    
    # Bottom row: Scaling plot
    ax_scale = fig.add_subplot(gs[1, :])
    
    k_values = np.arange(0, 8)
    # Theoretical: N(k) = (3^(k+1) + 3) / 2 for Sierpiński gasket
    n_theoretical = (3**(k_values + 1) + 3) / 2
    
    # Hausdorff dimension scaling
    D_f = np.log(3) / np.log(2)  # ≈ 1.585
    
    # Hilbert space comparison
    hilbert_euclidean = 2**n_theoretical
    hilbert_fractal = 2**(n_theoretical * D_f)
    
    ax_scale.semilogy(k_values, n_theoretical, 'o-', color='#2E86AB', 
                       linewidth=2, markersize=8, label='Node count N(k)')
    ax_scale.semilogy(k_values, hilbert_euclidean, 's--', color='#666666',
                       linewidth=1.5, markersize=6, alpha=0.7,
                       label=r'Euclidean $2^N$')
    ax_scale.semilogy(k_values, hilbert_fractal, '^-', color='#E94F37',
                       linewidth=2, markersize=8,
                       label=r'Fractal $2^{N \cdot D_f}$')
    
    ax_scale.set_xlabel('Recursion Depth k', fontsize=11)
    ax_scale.set_ylabel('Dimension', fontsize=11)
    ax_scale.legend(loc='upper left', frameon=True, fancybox=False, edgecolor='black')
    ax_scale.grid(True, alpha=0.3, linestyle='--')
    ax_scale.set_xlim(-0.5, 7.5)
    
    # Add D_f annotation
    ax_scale.text(0.75, 0.15, f'$D_f = \\log 3 / \\log 2 \\approx {D_f:.3f}$',
                  transform=ax_scale.transAxes, fontsize=10,
                  bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8))
    
    # Add advantage annotation at k=3
    k_anno = 3
    n_k3 = (3**(k_anno + 1) + 3) / 2
    advantage = 2**(n_k3 * (D_f - 1))
    ax_scale.annotate(f'{advantage:.1e}× advantage\nat k=3',
                      xy=(k_anno, 2**(n_k3 * D_f)),
                      xytext=(k_anno + 1.5, 2**(n_k3 * D_f) * 10),
                      fontsize=9,
                      arrowprops=dict(arrowstyle='->', color='#E94F37'),
                      color='#E94F37')
    
    plt.suptitle('Figure 4: Sierpiński Gasket Lattice and Hilbert Space Scaling',
                 fontsize=12, fontweight='bold', y=0.98)
    
    plt.savefig('Fig4_Sierpinski_Lattice.png', dpi=300, bbox_inches='tight',
                facecolor='white', edgecolor='none')
    plt.savefig('Fig4_Sierpinski_Lattice.pdf', bbox_inches='tight',
                facecolor='white', edgecolor='none')
    
    print("Figure 4 saved: Fig4_Sierpinski_Lattice.png/pdf")
    print(f"Node counts: k=0:{node_counts[0]}, k=1:{node_counts[1]}, "
          f"k=2:{node_counts[2]}, k=3:{node_counts[3]}")
    
    plt.close()


if __name__ == '__main__':
    main()
