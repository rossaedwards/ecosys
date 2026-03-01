import numpy as np
import matplotlib.pyplot as plt
from matplotlib import cm

def simulate_governor():
    lattice_size = 200
    x = np.linspace(-2, 2, lattice_size)
    y = np.linspace(-2, 2, lattice_size)
    X, Y = np.meshgrid(x, y)
    R = np.sqrt(X**2 + Y**2)
    
    # Input (Implosion) + Output (Explosion)
    Psi_in = -1 / (R + 0.1) * np.sin(1.618 * np.arctan2(Y, X) + R)
    Psi_out = 1 / (R + 0.1) * np.sin(1.618 * np.arctan2(Y, X) - R)
    
    # Constructive Interference (Impedance Matched)
    Z = np.abs(Psi_in) + np.abs(Psi_out)
    
    fig = plt.figure(figsize=(10, 6))
    ax = fig.add_subplot(111, projection='3d')
    surf = ax.plot_surface(X, Y, Z, cmap=cm.plasma, linewidth=0, antialiased=False)
    ax.set_title("Singularity Governor: Vacuum Resonance")
    ax.set_zlim(-5, 5)
    plt.savefig('../figures/Fig1_Singularity_Governor.png', dpi=300)
    print("Generated Fig1_Singularity_Governor.png")

if __name__ == "__main__":
    simulate_governor()