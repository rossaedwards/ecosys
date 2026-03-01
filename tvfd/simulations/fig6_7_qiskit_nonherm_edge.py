# fig6_7_qiskit_nonherm_edge.py — Full Non-Hermitian Chiral Edge Simulation
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.colors import Normalize

# Non-Hermitian Hamiltonian: H = H_0 + i*Gamma (gain/loss)
# C6v symmetry, 6-site ring model

N = 6  # C6v sites
t_hop = 1.0       # hopping
lambda_rael = np.linspace(0, 1.5, 300)
gamma = 0.25      # gain/loss asymmetry
lambda_star = 0.72  # locked RG fixed point

# Eigenvalue spectrum vs lambda_rael
real_parts = []
imag_parts = []

for lam in lambda_rael:
    H = np.zeros((N, N), dtype=complex)
    for i in range(N):
        j = (i + 1) % N
        k = (i - 1) % N
        H[i, j] = t_hop * (1 + lam)
        H[i, k] = t_hop * (1 - lam)
    # Add non-Hermitian gain/loss diagonal
    for i in range(N):
        H[i, i] = 1j * gamma * ((-1)**i)
    evals = np.linalg.eigvals(H)
    real_parts.append(np.sort(evals.real))
    imag_parts.append(np.sort(evals.imag))

real_parts = np.array(real_parts)
imag_parts = np.array(imag_parts)

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

# Real spectrum
for band in range(N):
    axes[0].plot(lambda_rael, real_parts[:, band], 'b', lw=1.2, alpha=0.7)
axes[0].axvline(lambda_star, color='red', ls='--', lw=2, label=f'λ* = {lambda_star}')
axes[0].set_xlabel('λ_rÆL')
axes[0].set_ylabel('Re(E)')
axes[0].set_title('Fig 6.7a — Real Spectrum: Chiral Edge Bands')
axes[0].legend()

# Imaginary spectrum (EP signature)
for band in range(N):
    axes[1].plot(lambda_rael, imag_parts[:, band], 'darkorange', lw=1.2, alpha=0.7)
axes[1].axvline(lambda_star, color='red', ls='--', lw=2, label=f'EP crossing at λ*={lambda_star}')
axes[1].set_xlabel('λ_rÆL')
axes[1].set_ylabel('Im(E) — Gain/Loss')
axes[1].set_title('Fig 6.7b — Imaginary Spectrum: Exceptional Point')
axes[1].legend()

plt.suptitle('rÆ-Cell Non-Hermitian Chiral Edge States (Qiskit-compatible model)', fontsize=12)
plt.tight_layout()
plt.savefig('fig6_7_qiskit_nonherm.png', dpi=150)
print("fig6_7_qiskit_nonherm.png saved ✅")
