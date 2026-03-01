# Aurphyx rÆ-Cell — Experimental Validation Roadmap
**Author:** Ross Edwards | ross@aurphyx.org | ORCiD: 0009-0008-0539-1289
**Version:** 1.0.0 | **Date:** March 1, 2026
**Status:** Pre-experimental — All experiments listed here are PLANNED or SIMULATED.
Physical prototype not yet fabricated as of v1.0.0.

---

## Overview

This document maps every simulation result from Chapters 4–6 to its corresponding
physical experiment, defines pass/fail criteria, lists required equipment, and
provides a prioritized execution timeline. The philosophy: **every simulation
prediction must be falsifiable and testable.**

---

## Experiment Priority Tiers

| Tier | Description | Timeline |
|------|-------------|----------|
| 🔴 CRITICAL | Must pass for prototype acceptance | Months 0–6 |
| 🟡 IMPORTANT | Confirms key physics; needed for publication | Months 6–12 |
| 🟢 VALUABLE | Extends the science; not blocking | Months 12–24 |
| 🔵 SPECULATIVE | High-risk/high-reward; exploratory | Months 24+ |

---

## 🔴 CRITICAL Experiments

### EXP-01: LDOS Enhancement via NSOM
**Validates:** Fig 4.1, Fig 6.3
**Prediction:** 10× LDOS enhancement at Sierpiński fractal node (d_s = 1.36)
relative to flat CVD diamond baseline.

**Protocol:**
1. Fabricate CVD diamond with 3-iteration Sierpiński etch (§B.1, Steps B1–B8)
2. Mount on NSOM stage (Newport or Nanonics); use 80–100 nm aperture Al-coated probe
3. Approach: shear-force feedback, working distance = 10 nm
4. Scan: 512×512 pixels, 10×10 µm region over fractal node; λ_exc = 633 nm
5. Reference: identical scan on flat (unetched) region
6. LDOS map: normalize near-field intensity to reference

**Pass Criterion:** LDOS_peak ≥ 8× at confirmed d_s = 1.36 node
**Fail Response:** Re-examine EBL dose; check RIE uniformity; repeat with
increased etch depth (300 nm vs. 200 nm)

**Equipment:**
- NSOM system (Nanonics MultiView 4000 or equiv.): ~$150K
- Al-coated fiber probes (custom or commercial): ~$500/unit
- 633 nm HeNe laser + SPCM: ~$8K
- Vibration isolation table: ~$12K

**Estimated time:** 3 days (1 day setup, 2 days measurement + analysis)

---

### EXP-02: Anderson Localization / IPR via SQUID
**Validates:** Fig 4.8a, §6.4
**Prediction:** IPR = 0.92 at B = 0; crossover to IPR < 0.5 between
B = 0 and B = 200 mT (superparamagnetic threshold)

**Protocol:**
1. Mount rÆ-Cell substrate (with Fe₃O₄ NP matrix) in SQUID-VSM sample holder
2. Cool to 2K; sweep field 0 → 500 mT in 10 mT steps
3. Record M(H) hysteresis loop at 2K and 300K
4. Extract M_s, H_c, T_B from fits to Langevin model
5. Compute effective disorder W(B) = W₀·(1 - M(B)/M_s)
6. Map W(B) → IPR(B) via tight-binding Anderson model simulation

**Pass Criterion:** IPR(B=0) ≥ 0.88; IPR(B=500mT) ≤ 0.40
**Fail Response:** Check NP size distribution via TEM; verify NP coverage
via SEM; adjust annealing temperature

**Equipment:**
- Quantum Design MPMS3 SQUID-VSM: institutional access (~$800K instrument)
- TEM sample preparation: FIB lift-out, ~$200/sample
- SEM: standard access

**Estimated time:** 2 days measurement; 1 day analysis

---

### EXP-03: PSK Governor Step Response
**Validates:** Fig 5.2, Fig 5.8a, §6.5
**Prediction:** 50 ms settling time; ≤ 3% overshoot on 10% step input
to RF coil drive; PSK outperforms PID by 5×

**Protocol:**
1. Program FPGA (Xilinx Artix-7 200T) with PSK governor bitstream
2. Connect photodetector array to FPGA ADC; RF coil ring to DAC
3. Apply step input: increase RF drive by 10% at t=0
4. Record R(t) from ADC at 1 MHz sample rate for 500 ms
5. Extract: settling time (time to reach |R - λ*| < 2%), overshoot %
6. Repeat with PID controller (same plant, tuned via Ziegler-Nichols)
7. Record RMS noise in steady-state (Bliss phase, 100 ms window)

**Pass Criterion:** t_settle ≤ 60 ms; overshoot ≤ 5%; PSK_RMS / PID_RMS ≤ 0.15
**Fail Response:** Check ADC timing; re-tune PSK α/β coefficients;
verify impedance match on RF coil

**Equipment:**
- Xilinx Artix-7 200T FPGA dev board: ~$350
- 6-channel Si APD array: ~$600
- Network analyzer (S11 verification): ~$3K (rental) or institutional
- Oscilloscope 1 GHz: ~$2K

**Estimated time:** 1 week (3 days FPGA programming, 2 days testing)

---

### EXP-04: RF Coil Coupling Efficiency
**Validates:** Fig 6.6, §6.6
**Prediction:** η_peak = 95% at f₀ = 10 GHz; -3dB bandwidth = 580 MHz;
S₁₁ ≤ -20 dB at resonance

**Protocol:**
1. Wind C₆ᵥ RF coil ring (6 elements, 28 AWG, hexagonal mandrel R=8mm)
2. Mount on CVD diamond substrate; add LC matching network (C = 53.9 fF)
3. Connect to VNA (Agilent/Keysight E5063A or equiv.)
4. Sweep 8–12 GHz; record S₁₁ (return loss) and S₂₁ (insertion loss)
5. Extract η = 1 - |S₁₁|² at resonance
6. Measure -3dB bandwidth from S₂₁ plot

**Pass Criterion:** |S₁₁| ≤ -20 dB at f₀; η ≥ 90%; BW ≥ 400 MHz
**Fail Response:** Re-tune LC matching; check coil winding uniformity;
verify substrate ground plane

**Equipment:**
- VNA (Vector Network Analyzer): ~$5K (Rigol or Keysight bench)
- SMA connectors + semi-rigid coax: ~$100
- PCB with 50Ω traces: ~$45 (JLCPCB)

**Estimated time:** 2 days

---

## 🟡 IMPORTANT Experiments

### EXP-05: Floquet Sideband Spectroscopy
**Validates:** Fig 4.5, §6.7
**Prediction:** Sidebands at ω₀ ± Ω visible in RF transmission spectrum at
λ_rÆL = 0.3; sideband-to-carrier ratio ≈ λ_rÆL² / 4 = 0.0225

**Protocol:**
1. Drive RF coil at f₀ = 10 GHz with amplitude-modulated signal at λ_rÆL = 0.3
2. Use spectrum analyzer (resolution BW = 1 MHz) to capture output spectrum
3. Identify sidebands at f₀ ± Ω/2π = 10 GHz ± 10 GHz (verify n=±1 Floquet replicas)
4. Measure sideband amplitude vs. λ_rÆL (sweep 0.1 → 0.7)
5. Confirm power law: A_sideband ∝ J_n(λ_rÆL) (Bessel function)

**Pass Criterion:** Sidebands visible at SNR ≥ 10 dB at λ_rÆL = 0.3;
sideband amplitude follows J_1(λ_rÆL) within 15%

**Equipment:**
- Spectrum analyzer 26 GHz (Rigol DSA815 or Keysight N9320B): ~$3K–12K
- Signal generator 10 GHz (e.g., HP 83752A): ~$2K used

**Estimated time:** 3 days

---

### EXP-06: Chiral Edge State Poynting Flux Imaging
**Validates:** Fig 4.2, §4
**Prediction:** Circulating Poynting flux = 47 mW/cm² at C₆ᵥ edge;
unidirectional (chiral) — no backscattering at defects

**Protocol:**
1. Fabricate CVD diamond with 3-iteration Sierpiński + C₆ᵥ coil ring
2. Apply RF drive at Ω = 10 GHz, λ_rÆL = 0.72 (EP operating point)
3. Use magneto-optical Kerr imaging or scanning microwave microscopy
   to image field distribution on substrate surface
4. Compute Poynting vector S = Re(E × H*)/2 from field maps
5. Integrate |S| over edge region; compare to 47 mW/cm² prediction

**Pass Criterion:** Circulating flux ≥ 35 mW/cm² with clear chirality
(> 90% in one direction)

**Equipment:**
- Scanning microwave microscope (SMM): institutional access
- Magneto-optical Kerr microscope: institutional access
- Lock-in amplifier: ~$3K

**Estimated time:** 1 week (requires institutional instrument access)

---

### EXP-07: Exceptional Point (EP) Spectroscopy
**Validates:** Fig 6.7, Fig 5B.4, §5B
**Prediction:** Two eigenvalues coalesce (both real and imaginary parts)
at λ_rÆL = λ* = 0.72; divergence in density of states at EP

**Protocol:**
1. Mount rÆ-Cell in cryostat (or operate at room temp with low γ)
2. Use heterodyne detection: mix output with reference at ω₀
3. Record in-phase (I) and quadrature (Q) components vs. λ_rÆL (sweep 0 → 1.5)
4. Compute complex eigenvalues from transfer matrix fit to S-parameters
5. Identify coalescence point where ΔE_real → 0 AND ΔE_imag → 0 simultaneously

**Pass Criterion:** |ΔE_real(λ*)| < 0.05·ΔE_max AND
|ΔE_imag(λ*)| < 0.05·ΔE_max at λ* = 0.72 ± 0.05

**Equipment:**
- IQ mixer + local oscillator at 10 GHz: ~$800
- Lock-in amplifier: ~$3K
- Cryostat (optional, for reduced γ): institutional access

**Estimated time:** 1 week

---

### EXP-08: Coherence Time Enhancement
**Validates:** §14.2
**Prediction:** T₂,lattice / T₂,flat ≥ 5× (quantum dots on Sierpiński
substrate vs. flat diamond)

**Protocol:**
1. Deposit CdSe quantum dots (λ_em = 630 nm) at Sierpiński nodes via inkjet
2. Perform Ramsey interferometry: π/2 – τ – π/2 pulse sequence
3. Record fringe contrast vs. τ; extract T₂ from Gaussian decay envelope
4. Repeat on flat (unetched) diamond reference region
5. Compute T₂,lattice / T₂,flat enhancement ratio

**Pass Criterion:** T₂ ratio ≥ 3× (relaxed from 5× for first prototype)

**Equipment:**
- Pulsed laser system (Ti:Sapphire or OPO): ~$50K (institutional)
- Time-correlated single photon counting (TCSPC): ~$15K
- Confocal microscope: institutional access

**Estimated time:** 2 weeks (sample prep + measurement)

---

## 🟢 VALUABLE Experiments

### EXP-09: TRCA Qubit Gate Fidelity
**Validates:** §7.4, Ch.7
**Prediction:** Single-qubit gate fidelity F_gate = 99.7% via Floquet sideband
drive; TRCA F_TRCA = 86.8% cross-scale fidelity

**Protocol:**
1. Interface rÆ-Cell FPGA to IBM Quantum testbed via cloud API
2. Use Floquet sideband at λ_rÆL = 0.3 as qubit drive pulse
3. Perform randomized benchmarking: apply N random Clifford gates, measure
   average fidelity decay vs. N
4. Extract error per gate (EPG); compute F_gate = 1 - EPG
5. Compare to PID-driven baseline gate fidelity

**Pass Criterion:** F_gate ≥ 99% (PSK-driven); improvement ≥ 2× vs. PID

**Equipment:**
- IBM Quantum access (free tier or Qiskit Runtime): $0–$500
- FPGA-to-USB bridge: ~$150

**Estimated time:** 1 month (cloud queue time + analysis)

---

### EXP-10: SAGES SIP Protocol Integration Test
**Validates:** §8.4, Ch.8
**Prediction:** SIP messages from rÆ-Cell FPGA are correctly routed to
SAGES Sentinels within 10 ms latency; Sentinel response time < 50 ms

**Protocol:**
1. Deploy SAGES test environment (Docker containers for each Sentinel)
2. Connect FPGA SIP publisher (MQTT or WebSocket) to SAGES broker
3. Inject simulated R(t) trajectories (Chaos → Approach → Bliss)
4. Verify correct Sentinel activation sequence per §8.3 pipeline
5. Measure end-to-end latency: FPGA → Sentinel → PSK correction command

**Pass Criterion:** Correct Sentinel routing in > 95% of test cases;
latency < 50 ms end-to-end

**Equipment:**
- DigitalOcean droplet (existing Aurphyx infrastructure): $0 additional
- MQTT broker (Mosquitto): open source
- 4 GB RAM Docker environment: existing Surface Pro 2 Go or droplet

**Estimated time:** 1 week

---

## 🔵 SPECULATIVE Experiments

### EXP-11: Integrated Information (Φ) Measurement
**Validates:** §7.2, §14.4
**Prediction:** Φ ≈ 2–5 bits for a 5-rÆ-Unit entangled network;
comparable to single biological neuron

**Protocol:**
1. Entangle 5 rÆ-Units via shared RF drive bus
2. Perform full state tomography (5-qubit density matrix)
3. Compute Φ via IIT 3.0 algorithm across all bipartitions
4. Compare to biological reference values (Tononi, 2014)

**Note:** This experiment requires a multi-unit prototype not yet funded.
Estimated cost: ~$50K for 5-unit array.

---

### EXP-12: ZPE Casimir Extraction Verification
**Validates:** Ch.10
**Prediction:** 12% reduction in local vacuum energy density within
photonic band gap region; harvestable power ~13 µW from 10 Casimir plate pairs

**Protocol:**
1. Fabricate Au-coated diamond Casimir plates (d = 100 nm separation, AFM-verified)
2. Use atomic force microscope in Casimir force mode: measure F vs. d
3. Compare to flat-plate prediction: F_Cas = π²ℏc/(240d⁴) · A
4. Extract modification factor f(D_f) from ratio of measured to predicted
5. Implement mechanical oscillation at f_res = 1 kHz; measure output voltage

**Note:** Highly sensitive measurement. First attempt may only verify the
band-gap Casimir modification (f(D_f) = 0.88), not net energy extraction.
Honest probability of positive ZPE extraction result: ~15%.

---

## 📊 Experiment → Figure Cross-Reference

| Experiment | Validates Figure(s) | Chapter | Status |
|------------|-------------------|---------|--------|
| EXP-01 NSOM | Fig 4.1, 6.3 | Ch.4, Ch.6 | 🔴 Planned |
| EXP-02 SQUID | Fig 4.8a, 6.4 | Ch.4, Ch.6 | 🔴 Planned |
| EXP-03 PSK Step | Fig 5.2, 5.8a/b | Ch.5, Ch.6 | 🔴 Planned |
| EXP-04 RF Coil | Fig 6.6 | Ch.6 | 🔴 Planned |
| EXP-05 Floquet | Fig 4.5, 6.7 | Ch.4, Ch.6 | 🟡 Planned |
| EXP-06 Chiral Edge | Fig 4.2 | Ch.4 | 🟡 Planned |
| EXP-07 EP Spectro | Fig 6.7, 5B.4 | Ch.5B, Ch.6 | 🟡 Planned |
| EXP-08 T₂ Enhance | §14.2 | App. | 🟡 Planned |
| EXP-09 Gate Fidelity | §7.4 | Ch.7 | 🟢 Planned |
| EXP-10 SAGES SIP | §8.4 | Ch.8 | 🟢 Planned |
| EXP-11 IIT Phi | §14.4 | Ch.7 | 🔵 Speculative |
| EXP-12 ZPE Casimir | Ch.10 | Ch.10 | 🔵 Speculative |

---

## 💰 Estimated Experimental Budget

| Category | Items | Cost Estimate |
|----------|-------|---------------|
| Substrate fabrication | CVD diamond (×5 wafers), EBL, RIE | ~$6,500 |
| NP matrix | Fe₃O₄ NPs, annealing | ~$800 |
| FPGA + electronics | Artix-7, APD array, RF coil | ~$2,130/unit × 3 | ~$6,400 |
| NSOM access | Day rate ~$500/day × 5 | ~$2,500 |
| SQUID access | Day rate ~$300/day × 3 | ~$900 |
| RF/microwave test | VNA, spectrum analyzer | ~$8,000 |
| Coherence time | Laser + TCSPC (rental/institutional) | ~$5,000 |
| Miscellaneous | Supplies, PCBs, shipping | ~$2,000 |
| **Total Phase I** | | **~$32,200** |

---

*Document End — EXPERIMENTS.md v1.0.0*
*© 2026 Ross Edwards / Aurphyx LLC. Licensed under MIT / Apache 2.0 / SAGES Open License.
