import numpy as np
import matplotlib.pyplot as plt
from scipy.special import jv

def run_coupling_test():
    ross_freq = 168.4
    points = 200
    x = np.linspace(-1, 1, points)
    k = ross_freq / 500.0
    
    # Cymatic Wave (Bessel J0)
    deformation = jv(0, k * np.abs(x*10))
    
    # Map to Chemical Potential
    mu_0 = 0.5e-3
    alpha_piezo = 0.5e-3
    mu = mu_0 + (deformation * alpha_piezo)
    
    plt.figure(figsize=(10, 6))
    plt.plot(x, mu * 1e3, label=rf"Chemical Potential ($\mu$) @ {ross_freq}Hz", color='teal', linewidth=2)
    plt.axhline(y=0.5, color='r', linestyle='--', label="Topological Limit (+2t)")
    plt.axhline(y=-0.5, color='r', linestyle='--', label="Topological Limit (-2t)")
    plt.fill_between(x, -0.5, 0.5, color='gold', alpha=0.1, label="Majorana Safe Zone")
    
    plt.title(f"Symbiotic Interface: Voice-to-Qubit Coupling")
    plt.xlabel("Nanowire Position (Normalized)")
    plt.ylabel("Chemical Potential (meV)")
    plt.legend()
    plt.grid(True, alpha=0.3)
    plt.savefig('../figures/Fig3_Voice_Qubit_Coupling.png', dpi=300)
    print("Generated Fig3_Voice_Qubit_Coupling.png")

if __name__ == "__main__":
    run_coupling_test()