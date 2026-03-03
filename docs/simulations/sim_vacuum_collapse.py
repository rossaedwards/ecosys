import numpy as np
import matplotlib.pyplot as plt
from matplotlib import cm

def sim():
    lattice_size = 200
    x = np.linspace(-2, 2, lattice_size)
    y = np.linspace(-2, 2, lattice_size)
    X, Y = np.meshgrid(x, y)
    R = np.sqrt(X**2 + Y**2)
    # Direct interference (mismatched) -> Cancellation
    Psi_in = -1 / (R + 0.1) * np.sin(1.618 * np.arctan2(Y, X) + R)
    Psi_out = 1 / (R + 0.1) * np.sin(1.618 * np.arctan2(Y, X) - R)
    Z = Psi_in + Psi_out # Destructive interference
    fig = plt.figure(figsize=(8, 6))
    ax = fig.add_subplot(111, projection='3d')
    ax.plot_surface(X, Y, Z, cmap=cm.inferno)
    plt.title("Fig 2: Vacuum Collapse (Destructive)")
    plt.savefig("../figures/Fig2_Vacuum_Collapse.png", dpi=300)

if __name__ == "__main__": sim()