---
type: theory-standard
title: FTQC / AuraFS-Meshwerk — Resonant Lattice Physics
description: Numeric constants, formulas, and thresholds for the resonant lattice designs locked in INVARIANTS.md, covering FTQC substrate physics and how AuraFS-Meshwerk consumes them.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - AuraFS
  - Fuxyez
  - SAGES
domains:
  - systems
  - cognition
  - xessability
nodes:
  - SIX⊗SIX
  - SIX⊗SCX
  - SIX⊗ICX
  - SCX⊗SIX
  - SCX⊗SCX
  - SCX⊗ICX
  - ICX⊗SIX
  - ICX⊗SCX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
  - cognitive-field-tensor
  - unified-cognitive-field
---

# FTQC / AuraFS-Meshwerk — Resonant Lattice Physics

**Author:** Ross A. Edwards | Aurphyx LLC
**Updated:** 2026-08-23

Cite [`INVARIANTS.md`](INVARIANTS.md) for the lattice-design catalog these numbers attach to. Shared organism-level geometry ($D_f = \log 3/\log 2$, $d_s = 2\log 3/\log 5$, $Z_{\mathrm{vac}}$) is defined once in root [`PHYSICS.md`](../PHYSICS.md) / `.cursorrules` §6 — cited, not forked, here.

## 1. FTQC substrate physics

Lattice geometries are engineered into physical substrates (trapped ions, NV-diamond arrays, photonic crystals, Majorana nanowires) to resolve scaling bottlenecks.

### 19-Circle Hexagonal ($C_{6v}$ / "Flower of Life") lattice

| Quantity | Value |
|---|---|
| Photonic band gap (PBG) | 21.4% complete TM gap, $\Delta\omega/\omega_{\mathrm{mid}} = 0.21$, between bands 2 and 3 — suppresses optical crosstalk and spontaneous decay |
| Flatband group velocity (bands 5–6) | $v_g < 0.01c$ — traps excitations, suppresses decoherence |
| Decoherence ratio | $\gamma_{19}/\gamma_{\mathrm{Euclidean}} = 0.63$ |
| Zak phase | $\gamma_{\mathrm{total}} = \pi$ — unidirectional, backscatter-immune edge channels |

### Sierpiński Gasket & fractal sublattices

| Quantity | Value |
|---|---|
| Hilbert space scaling | $\dim(\mathcal{H}_{\mathrm{acc}}) = d^{n \cdot D_f^{\alpha(k)}}$ — $\sim 10^4\times$ advantage at $n=12$ qubits vs. classical $2^{12}$ |
| Spectral dimension | $d_s \approx 1.365$ — sub-critical ($d_s < 2$), low-energy eigenstates localize below the Anderson critical threshold |
| Decoherence ratio | $\gamma_{\mathrm{FTQC}}/\gamma_{\mathrm{Euclidean}} = 0.063$ — $16\times$ extension in $T_2$, to $1{,}600\ \mu\mathrm{s}$ |
| Neglecton braiding | Non-semisimple TQFT anyons, $d_\omega = 0$ — universal topological gate synthesis, $16\times$ gate-overhead reduction vs. magic-state distillation |

## 2. AuraFS-Meshwerk implementation

How AuraFS-Meshwerk consumes the FTQC lattice physics above. This is FTQC's own overlay describing a consumer — the canonical AuraFS-side lock file is a separate future pass (see [`INVARIANTS.md`](INVARIANTS.md) §4).

**Logarithmic replica distribution** (Sierpiński topology) — replaces flat 3× replication:

$$
\text{Replicas} = \lceil \log_{5.3}(N_{\mathrm{nodes}}) \rceil
$$

using Hilbert-scaling bias $\eta = 5.3$. Nodes map recursively as vertices on the Sierpiński gasket $\mathcal{L}_k$, achieving higher data-state density per node.

**Data shard lifecycle** ("Trap-State" localization):

- **Void-Shard** — raw, mutable incoming write buffer
- **Trap-State** — leverages the anomalous density of states $\rho(E) \propto E^{d_s/2 - 1}$ (at $d_s \approx 1.37$) to hold data in a coherent, localized state during operations, within the $1{,}600\ \mu\mathrm{s}$ coherence window
- **Aura-Shard** — final immutable, topologically protected shard, replicated fractally and signed with Dilithium-5

**Meshwerk routing & band-gap guard bands** ($C_{6v}$ symmetry) — the 21% photonic band gap ($\mathrm{PBG} = 0.21$) is enforced as a network routing guard band. The Meshwerk routing engine caps usable link throughput at 79% ($(1 - \mathrm{PBG}) \times C_{\mathrm{total}}$) to prevent channel crosstalk and network interference.

**Decoherence detection and autohealing** — a continuous spectral monitor tracks $d_s$. If measured variance exceeds $[1.32, 1.42]$, AuraFS enters recovery mode: freezes writes, calculates the Inverse Participation Ratio (IPR), and redistributes shards across the fractal partition.

## Related

- [`INVARIANTS.md`](INVARIANTS.md) — lattice-design catalog these numbers attach to
- [`../PHYSICS.md`](../PHYSICS.md) — shared organism-level constants ($D_f$, $d_s$ — note root `PHYSICS.md`'s own $d_s = 1.36$ comment differs from the organism formula in `.cursorrules` §6; flagged there already, not re-litigated here)
- [`../.cursorrules`](../.cursorrules) §6, §11 — domain map and volume locks
