# fig6_5_fpga.py — FPGA Floquet Drive Schematic
import numpy as np
import matplotlib.pyplot as plt

t = np.linspace(0, 2e-9, 1000)
omega = 2 * np.pi * 10e9
lambda_rael = 0.3
drive = np.sin(omega * t) * (1 + lambda_rael * np.cos(omega * t / 3))

fig, ax = plt.subplots(figsize=(9, 4))
ax.plot(t * 1e9, drive, 'darkgreen', lw=1.5, label='FPGA Floquet Drive (Ω=10GHz)')
ax.set_xlabel('Time (ns)')
ax.set_ylabel('Drive Amplitude (a.u.)')
ax.set_title('Fig 6.5 — FPGA Control: Floquet Drive at Ω = 10 GHz')
ax.legend()
plt.tight_layout()
plt.savefig('fig6_5_fpga.png', dpi=150)
print("fig6_5_fpga.png saved")
