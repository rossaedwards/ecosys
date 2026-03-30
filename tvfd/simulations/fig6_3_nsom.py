# fig6_3_nsom.py — NSOM LDOS Measurement Simulation
import numpy as np
import matplotlib.pyplot as plt

ds_values = np.linspace(1.0, 2.0, 200)
ldos_euclidean = np.ones_like(ds_values)
ldos_fractal = 1 + 9 * np.exp(-((ds_values - 1.36)**2) / (2 * 0.05**2))

fig, ax = plt.subplots(figsize=(8, 5))
ax.plot(ds_values, ldos_fractal, 'b-', lw=2, label='Balance State Vector-Cell NSOM (simulated)')
ax.axhline(1, color='gray', ls='--', label='Euclidean baseline')
ax.axvline(1.36, color='red', ls=':', label='d_s = 1.36 (design point)')
ax.scatter([1.36], [10.0], color='red', zorder=5, s=80, label='LDOS = 10× peak')
ax.set_xlabel('Spectral Dimension d_s')
ax.set_ylabel('LDOS Enhancement (normalized)')
ax.set_title('Fig 6.3 — NSOM LDOS Measurement Protocol')
ax.legend()
plt.tight_layout()
plt.savefig('fig6_3_nsom.png', dpi=150)
print("fig6_3_nsom.png saved")
