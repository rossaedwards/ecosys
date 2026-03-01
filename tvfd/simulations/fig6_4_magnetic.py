# fig6_4_magnetic.py — Magnetic Matrix IPR vs Applied Field
import numpy as np
import matplotlib.pyplot as plt

B_field = np.linspace(0, 0.5, 200)  # Tesla
IPR = 0.92 * np.exp(-B_field / 0.3) + 0.05 * (1 - np.exp(-B_field / 0.1))

fig, ax = plt.subplots(figsize=(8, 5))
ax.plot(B_field * 1000, IPR, 'purple', lw=2)
ax.axhline(0.92, color='red', ls=':', label='IPR=0.92 (B=0, locked)')
ax.set_xlabel('Applied Field B (mT)')
ax.set_ylabel('Inverse Participation Ratio (IPR)')
ax.set_title('Fig 6.4 — Magnetic Matrix: IPR vs Applied Field')
ax.legend()
plt.tight_layout()
plt.savefig('fig6_4_magnetic.png', dpi=150)
print("fig6_4_magnetic.png saved")
