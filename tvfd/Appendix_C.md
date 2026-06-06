## ** APS‑TVFD‑APX-C **
## ** Version 1.0 (Draft) **
## ** Aurphyx Primordial Standards **
## ** Ross A. Edwards | Aurphyx LLC | SUXS **
## ** SAGES | AGPLv3 **

# 💾 Appendix C: Simulation Code Reference *(Full Expansion)*

## § C.1 — Complete Figure Generation Map

| Figure | Script | Chapter | Runtime | Output |
|--------|--------|---------|---------|--------|
| 4.1 | `fig4_1_ldos.py` | Ch.4 | 15s | `fig4_1_ldos.png` ✅ |
| 4.2 | `fig4_2_raet_flux.py` | Ch.4 | 10s | `fig4_2_raet_flux.png` ✅ |
| 4.3 | `fig4_3_scaling.py` | Ch.4 | 8s | `fig4_3_scaling.png` ✅ |
| 4.5 | `fig4_5_floquet.py` | Ch.4 | 12s | `fig4_5_floquet.png` ✅ |
| 4.6 | `fig4_6_rg.py` | Ch.4 | 6s | `fig4_6_rg.png` ✅ |
| 4.8a | `fig4_8a_ipr.py` | Ch.4 | 20s | `fig4_8a_ipr.png` ✅ |
| 4.9 | `fig4_9_power.py` | Ch.4 | 8s | `fig4_9_power.png` ✅ |
| 5.2 | `fig5_2_raestate.py` | Ch.5 | 10s | `fig5_2_raestate.png` ✅ |
| 5.3a/b | `fig5_3_surfaces.py` | Ch.5 | 25s | `fig5_3_surfaces.png` ✅ |
| 5.5 | `fig5_5_psk.py` | Ch.5 | 18s | `fig5_5_psk.png` ✅ |
| 5.8a | `fig5_8a_control.py` | Ch.5 | 12s | `fig5_8a_control.png` ✅ |
| 5.8b | `fig5_8b_noise.py` | Ch.5 | 15s | `fig5_8b_noise.png` ✅ |
| 5B.2 | `fig5b_2_curvature.py` | Ch.5B | 20s | `fig5b_2_curvature.png` ✅ |
| 5B.4 | `fig5b_4_propagator.py` | Ch.5B | 30s | `fig5b_4_propagator.png` ✅ |
| 5B.5 | `fig5b_5_wilson.py` | Ch.5B | 10s | `fig5b_5_wilson.png` ✅ |
| 6.3 | `fig6_3_nsom.py` | Ch.6 | 8s | `fig6_3_nsom.png` ✅ |
| 6.4 | `fig6_4_magnetic.py` | Ch.6 | 8s | `fig6_4_magnetic.png` ✅ |
| 6.5 | `fig6_5_fpga.py` | Ch.6 | 6s | `fig6_5_fpga.png` ✅ |
| 6.6 | `fig6_6_rfcoil.py` | Ch.6 | 6s | `fig6_6_rfcoil.png` ✅ |
| 6.7 | `fig6_7_qiskit_nonherm.py` | Ch.6 | 45s | `fig6_7_qiskit_nonherm.png` ✅ |
| 6.8 | `fig6_8_projected.py` | Ch.6 | 10s | `fig6_8_projected.png` ✅ |

## § C.2 — Master Notebook Structure (v2.0)

```
Aurphyx_Simulations_v2.0.ipynb
├── Cell 0:  Setup + imports (numpy, matplotlib, scipy, qiskit)
├── Cell 1:  Physics constants (d_s=1.36, D_f=1.585, λ*=0.72, φ⁻¹=0.618)
├── Cell 2:  Ch.4 figures (Figs 4.1–4.9)
├── Cell 3:  Ch.5 figures (Figs 5.2–5.8b)
├── Cell 4:  Ch.5B figures (Figs 5B.2–5B.5)
├── Cell 5:  Bonus figures (TVFD, Gauge, RG, SAGES)
├── Cell 6:  Ch.6 fabrication figures (Figs 6.3–6.8)
├── Cell 7:  Ch.6 Qiskit non-Hermitian sim (Fig 6.7)  ← NEW
├── Cell 8:  Ch.7 TRCA TTN contraction (F_TRCA=0.868)  ← NEW
├── Cell 9:  Ch.8 SAGES φ(r,ℓ) field + SIP schema  ← NEW
├── Cell 10: Ch.9 PSK-Kernel scheduler Rust pseudocode  ← NEW
├── Cell 11: Ch.10 ZPE power budget + Casimir calc  ← NEW
├── Cell 12: Ch.11 Scaling law N^α with TTN correction  ← NEW
└── Cell 13: App.A mathematical derivations (symbolic)  ← NEW
```
