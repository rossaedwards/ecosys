# fig6_6_rfcoil.py — RF Coil Coupling Efficiency vs Frequency
import numpy as np
import matplotlib.pyplot as plt

freq = np.linspace(8, 12, 500)  # GHz
eta = 0.95 * np.exp(-((freq - 10.0)**2) / (2 * 0.3**2))

fig, ax = plt.subplots(figsize=(8, 5))
ax.plot(freq, eta * 100, 'darkorange', lw=2)
ax.axvline(10.0, color='red', ls=':', label='Ω = 10 GHz design')
ax.axhline(95, color='gray', ls='--', label='η_peak = 95%')
ax.set_xlabel('Frequency (GHz)')
ax.set_ylabel('Coupling Efficiency η (%)')
ax.set_title('Fig 6.6 — RF Coil Ring: Coupling Efficiency Spectrum')
ax.legend()
plt.tight_layout()
plt.savefig('fig6_6_rfcoil.png', dpi=150)
print("fig6_6_rfcoil.png saved")
