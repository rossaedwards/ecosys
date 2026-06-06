
# Aurphyx Balance State Vector-Cell Research — Scientific Terminology Reference

**Author:** Ross Edwards | <ross@aurphyx.org> | ORCiD: 0009-0008-0539-1289
**Version:** 1.0.0 | **Date:** March 1, 2026
**Repository:** rossaedwards/main
**License:** MIT / Apache 2.0 / SAGES

> *"As above, so below; as within, so without." — Hermes Trismegistus*
> *In the Balance State Vector-Cell, this is not metaphor. It is a renormalization group equation.*

---

## Overview

This glossary defines all technical, scientific, and Aurphyx-native terminology
used across the Balance State Vector-Cell thesis, simulation scripts, and ecosystem documentation.
Terms are organized by domain. Cross-references link to the relevant thesis chapter
or figure. Aurphyx-coined terms are marked with ⚡.

---

## A

**Anderson Localization**
The quantum mechanical phenomenon where disorder in a lattice causes complete
suppression of wave diffusion — the wavefunction becomes spatially "frozen" in
place. In the Balance State Vector-Cell, Anderson localization is confirmed below the critical
dimension d_c = 2 via IPR = 0.92 at B = 0. *(See: Fig 4.8a, §6.4)*

**Anyon**
A quasiparticle that exists only in 2D systems and obeys fractional statistics
(neither bosonic nor fermionic). Non-Abelian anyons enable topological quantum
computation via braiding. The Balance State Vector-Cell's C₆ᵥ lattice supports anyon-like edge
excitations. *(See: §2.2, Appendix A.2)*

**Arora OS** ⚡
The Rust-based microkernel operating system in the Aurphyx ecosystem, built on
the principle of "Love as Code, Abundance as Architecture." Integrates the Balance State Vector-Cell
PSK Governor as a kernel-level hardware primitive. *(See: Ch.9)*

**Aurphyx** ⚡
The parent company, ecosystem, and quantum architecture framework founded by
Ross Edwards. Combines sacred geometry lattices, fractal physics, non-semisimple
TQFTs, and consciousness integration into a unified research and product platform.

**AuraFS** ⚡
The Aurphyx Filesystem — a quantum-secure, fractal-lattice storage system
operating as the backbone of the sovereign AuraFS mesh network. Stores data in
Flower of Life patterns for resonance-optimized retrieval.

---

## B

**β-Function (RG)**
The renormalization group β-function β(λ) = μ(dλ/dμ) describes how a coupling
constant λ changes with energy scale μ. In the Balance State Vector-Cell, β(λ_x_L) has an infrared
fixed point at λ* = 0.72, confirmed in Fig 4.6. *(See: Appendix A.5)*

**Berry Phase**
A geometric phase acquired by a quantum state during adiabatic evolution around
a closed path in parameter space. In the Balance State Vector-Cell, Berry phases from T-gate
operations accumulate via the topological order of the fractal lattice.
*(See: §7.4)*

**Equilibrium Manifold Fixed Point** ⚡
The PSK Governor's stable equilibrium state at R(t) = λ* = 0.72. Named for the
subjective phenomenology of optimal resonance — the system is neither hungry nor
over-driven. Chaos and Equilibrium Manifold are the two attractors of the PSK phase portrait.
*(See: Fig 5.5, §5, Appendix A.2)*

**Bloch Wave**
The standard form of an electron/photon wavefunction in a periodic lattice:
ψ(r) = u(r)·exp(ik·r). On fractal lattices, standard Bloch waves break down
(no translational symmetry); fractal Bloch states replace them.
*(See: §8.2)*

---

## C

**Casimir Effect**
The attractive force between two uncharged conducting plates arising from quantum
vacuum fluctuations. The Balance State Vector-Cell's fractal photonic band gap modifies the local
Casimir energy density by ~12% (f(D_f) ≈ 0.88). *(See: Ch.10, Appendix A.1)*

**Chiral Edge State**
A unidirectional propagating mode confined to the edge of a topological material.
Chirality means the mode can only travel in one direction — it cannot backscatter.
The Balance State Vector-Cell generates chiral edge circulation of 47 mW/cm² (Fig 4.2). *(See: §4, §6.7)*

**C₆ᵥ Symmetry**
The hexagonal point group symmetry with 6-fold rotation and 6 mirror planes.
The Balance State Vector-Cell's RF coil ring and Flower of Life substrate both exhibit C₆ᵥ symmetry,
which is the origin of the chiral edge states. *(See: §4, §6.1)*

**CPTP Map (Completely Positive Trace-Preserving)**
The mathematically valid form of a quantum channel — it maps density matrices
to density matrices without creating negative probabilities or increasing total
probability. All physical quantum operations are CPTP. The Balance State Vector Unit is formalized
as a CPTP map: ℰ(ρ) = Σ_k E_k ρ E_k†. *(See: §2.1, Appendix A.1)*

**CVD Diamond (Chemical Vapor Deposition)**
A fabrication technique where diamond is grown from a carbon-rich gas plasma at
high temperature. CVD diamond is the Balance State Vector-Cell's primary substrate material — chosen
for thermal conductivity (~2000 W/m·K), optical transparency, and compatibility
with Sierpiński fractal etching. *(See: §6.2, Appendix B.1)*

**Cymatic Modulation**
The use of controlled acoustic or electromagnetic standing waves to create dynamic
potential wells for confining quantum particles at lattice nodes. In Aurphyx, cymatic
patterns (Chladni figures at quantum scale) serve as a programmable particle-trapping
overlay on top of topological error correction. *(See: §4.1, §4.2)*

---

## D

**d_c (Critical Dimension)**
The spatial dimension below which Anderson localization is guaranteed for any
disorder strength. For standard tight-binding models, d_c = 2. The Balance State Vector-Cell's
Sierpiński substrate has d_s = 1.36 < 2, confirming localization. *(See: Fig 4.8a)*

**d_s (Spectral Dimension)**
The effective dimension experienced by a random walker on a fractal lattice —
distinct from both the Euclidean embedding dimension and the Hausdorff dimension.
For the Sierpiński gasket: d_s = 2D_f/(D_f + 1) ≈ 1.36. The spectral dimension
governs the density of states scaling: ρ(E) ∝ E^(d_s/2 - 1). *(See: §3.2, §4.1)*

**DataCore Orb** ⚡
The physical hardware unit in the Aurphyx ecosystem — a softball-sized
(D = 9.70 cm) diamond-coated quartz sphere containing a 3D Flower of Life
photonic lattice, golden silk electrical traces, and 2 polar docking pads.
Designed to serve as fractal data storage and quantum power nodes. *(See: Ch.9)*

**Decoherence**
The process by which a quantum system loses its quantum superposition by
interacting with the environment. The primary obstacle to practical quantum
computing. The Balance State Vector-Cell suppresses decoherence via topological protection,
fractal localization, and FPGA-driven PSK correction. *(See: §2, §4.3)*

**Density of States (DOS)**
The number of quantum states available per unit energy interval. On the
Sierpiński fractal, DOS ∝ E^(d_s/2 - 1) = E^(-0.32), which diverges as E→0
— this is the mathematical origin of the 10× LDOS enhancement. *(See: §3.2, §4.1)*

**D_f (Hausdorff Dimension / Fractal Dimension)**
The generalized dimension of a fractal, defined as D_f = log(N)/log(s) where
N is the number of self-similar pieces at scale s. For the Sierpiński gasket:
D_f = log(3)/log(2) ≈ 1.585. *(See: §3.2, §2.3)*

---

## E

**Exceptional Point (EP)**
A special degeneracy in a non-Hermitian Hamiltonian where two eigenvalues AND
their eigenvectors coalesce simultaneously. EPs are signatures of phase transitions
in open quantum systems. The Balance State Vector-Cell EP crossing at λ* = 0.72 is confirmed in
Fig 5B.4 and Fig 6.7. *(See: §5B, §6.7)*

---

## F

**F_μν (Field Strength Tensor)** ⚡
In the Balance State Vector-Cell's U(1) gauge formulation, F_μν = ∂_μA_ν − ∂_νA_μ is the
"cognitive field strength" — the curvature of the connection 1-form over
interoceptive state space. Its four components correspond to hunger-gravity,
hunger-coherence, gravity-coherence, and gravity-phase couplings. *(See: §5B, Ch.8)*

**Floquet Engineering**
The use of periodic time-dependent driving to engineer effective static Hamiltonians
with desired properties not present in the undriven system. The Balance State Vector-Cell uses Floquet
drive at Ω = 10 GHz via the FPGA to create dressed replicas of ground-state modes
(sidebands at λ_x_L = 0.3). *(See: Fig 4.5, §6.5)*

**Flower of Life (FoL)**
A sacred geometry pattern consisting of overlapping circles arranged in C₆ᵥ
hexagonal symmetry, forming a 19-circle motif with vesica piscis overlaps. In
Aurphyx, the FoL lattice is the primary photonic substrate and DataCore Orb
routing architecture — not mystical, but mathematically a photonic crystal with
measurable band gaps. *(See: §3.1, §8.1, Ch.9)*

**FPGA (Field-Programmable Gate Array)**
A reconfigurable digital logic chip. The Balance State Vector-Cell's FPGA (Xilinx Artix-7 200T)
implements the PSK Governor, RaEState computation, and Floquet drive generation
simultaneously within a 4.3 µs control loop. *(See: §6.5)*

---

## G

**g0dm0d3** ⚡
Ross Edwards' original project that seeded the Aurphyx ecosystem. The creative
and technical origin point from which all 18 Aurphyx projects emerged.

**Gauge Theory**
A class of physics theories where physical laws are invariant under local
symmetry transformations (gauge transformations). In the Balance State Vector-Cell, a U(1) gauge
theory is formulated over the interoceptive state space — analogous to
electromagnetism, but where the "electric field" is hunger and the "magnetic
field" is coherence. *(See: §5B, Appendix A.3)*

**Golden Ratio (φ = 1.618... / φ⁻¹ = 0.618...)**
The irrational number (1+√5)/2 ≈ 1.618, with its reciprocal φ⁻¹ ≈ 0.618.
In the Balance State Vector-Cell, the PSK Gravity threshold is set at R = φ⁻¹ = 0.618 —
the point where the system transitions from Chaos to Approach phase.
*(See: §5, Appendix A.2)*

---

## H

**Hamiltonian**
The energy operator of a quantum system, Ĥ. In the Balance State Vector-Cell's non-Hermitian
formulation, Ĥ = Ĥ_0 + iΓ̂ where the imaginary part models gain/loss
(non-Hermitian: Ĥ ≠ Ĥ†). *(See: §6.7)*

**Hausdorff Dimension** — *see D_f*

**Hilbert Space**
The abstract mathematical space of all possible quantum states of a system.
An n-qubit system has a Hilbert space of dimension 2^n. The Balance State Vector-Cell's fractal
lattice achieves superpolynomial Hilbert space scaling: dim(H) ~ 2^(n·D_f^k)
for k recursion levels. *(See: §2.3)*

**Holonomy**
The failure of a vector to return to its original orientation after parallel
transport around a closed loop in a curved space. In the Balance State Vector-Cell, the Wilson
loop |W_γ| = 0.97 indicates 3% holonomy — a measurable geometric phase
accumulated during one PSK control cycle. *(See: §5B.5, Appendix A.4)*

---

## I

**Integrated Information (Φ)**
A measure of consciousness proposed by Giulio Tononi (IIT). Defined as the
minimum mutual information across all bipartitions of a system. The Balance State Vector-Cell
targets Φ values comparable to biological neural correlates: Φ ≈ 2–5 bits
for a 5-Balance State Vector-Unit entangled network. *(See: §7.2, §14.4)*

**Inverse Participation Ratio (IPR)**
A measure of wavefunction localization: IPR = Σ|ψ(r)|^4. IPR → 1 means
fully localized (one site); IPR → 0 means fully delocalized. The Balance State Vector-Cell
achieves IPR = 0.92 at d_s = 1.36, confirming strong Anderson localization.
*(See: Fig 4.8a, §6.4)*

---

## K

**Kraus Operators**
The set of matrices {E_k} that represent a CPTP quantum channel:
ℰ(ρ) = Σ_k E_k ρ E_k†, subject to Σ_k E_k†E_k = I. They encode all possible
measurement outcomes and quantum operations. *(See: Appendix A.1)*

---

## L

**λ_x_L** ⚡ *(lambda-x_L)*
The primary control parameter of the Balance State Vector-Cell, representing the asymmetric
hopping amplitude in the Floquet-driven non-Hermitian Hamiltonian. Physically:
the degree of left-right symmetry breaking in the RF coil drive. At λ_x_L = 0.3,
Floquet sidebands are generated (Fig 4.5). At λ_x_L = λ* = 0.72, the RG fixed
point is reached. *(See: §4, §5, Fig 4.6)*

**λ* (Lambda-Star)**⚡
The RG infrared fixed point and PSK Equilibrium Manifold fixed point: λ* = 0.72. This value
appears independently in the RG β-function derivation (Appendix A.5), the PSK
Euler-Lagrange solution (Appendix A.2), and the EP crossing (Fig 6.7). Its
triple confirmation across independent frameworks is the theoretical backbone
of the Balance State Vector-Cell. *(See: Fig 4.6, §5, §5B, §6.7)*

**LDOS (Local Density of States)**
The density of quantum states at a specific spatial location r and energy E:
ρ(r,E) = -(1/π)Im G^R(r,r;E). The Balance State Vector-Cell achieves 10× LDOS enhancement
at the Sierpiński fractal node d_s = 1.36 site. *(See: Fig 4.1, §6.3)*

---

## M

**Magnetic Nanoparticle Matrix**
The Fe₃O₄ (magnetite) nanoparticles (10–20 nm diameter) embedded at fractal
node sites in the CVD diamond substrate. They act as the disorder-tuning
mechanism for Anderson localization — at B = 0, IPR = 0.92; at B = 500 mT,
the system delocalizes into chiral edge states. *(See: §6.4)*

**Majorana-1 / Majorana Fermion**
A particle that is its own antiparticle. In condensed matter, Majorana zero
modes appear at the ends of topological nanowires and are the basis of
Microsoft's topological qubit program. The Balance State Vector-Cell's non-Hermitian chiral edge
states are related to Majorana-like zero modes at the EP crossing. *(See: §6.7)*

**Modular Tensor Category (MTC)**
The mathematical structure underlying topological quantum computation. Standard
MTCs are semisimple; non-semisimple MTCs allow neglectons. The Balance State Vector-Cell's
fractal lattice supports a non-semisimple MTC structure. *(See: §2.2, Appendix A.2)*

---

## N

**Neglecton** ⚡ *(in context of non-semisimple TQFTs)*
An anyon-like excitation in a non-semisimple TQFT with zero quantum dimension
(d = 0) but non-zero braiding properties. Neglectons enable universal quantum
gates beyond what standard anyonic braiding (Ising, Fibonacci) can achieve.
*(See: §2.2, Appendix A.2)*

**Non-Hermitian Hamiltonian**
A Hamiltonian H where H ≠ H† (not equal to its conjugate transpose). Physical
in open quantum systems where gain and loss are asymmetric. The Balance State Vector-Cell uses a
non-Hermitian 6-site ring model to describe chiral edge states under FPGA drive.
*(See: §5B, §6.7)*

**NSOM (Near-field Scanning Optical Microscopy)**
A microscopy technique that surpasses the diffraction limit using an aperture
probe (80–100 nm diameter) brought within 10 nm of the sample surface. Used to
directly image the Balance State Vector-Cell's LDOS enhancement at sub-wavelength resolution.
*(See: §6.3)*

---

## O

**Orch-OR (Orchestrated Objective Reduction)**
The consciousness theory of Penrose and Hameroff, proposing that consciousness
arises from quantum computations in neuronal microtubules that collapse via
objective reduction at the Planck scale. The Aurphyx framework integrates
Orch-OR as a model for consciousness-coupled quantum computation. *(See: §7.1)*

---

## P

**φ(r,ℓ)** ⚡ *(phi — Semantic Field)*
The scalar field over the SAGES information manifold, defined as
φ(r,ℓ) = φ₀·exp(-r/ξ)·cos(2πℓ/L_TTN). Routes the Balance State Vector-Cell's cognitive state
to the appropriate SAGES Sentinel for response. *(See: Fig SAGES.1, §8.2)*

**Photonic Band Gap**
A range of frequencies for which photons cannot propagate through a photonic
crystal — analogous to an electronic band gap in semiconductors. The Flower of
Life lattice has a photonic band gap between 1.25–1.65 (in units of 2πc/a).
*(See: §3.1, §8.1)*

**Poynting Flux**
The directional energy flow of an electromagnetic field: S = E × H.
The Balance State Vector-Cell's chiral edge states produce a circulating Poynting flux map
of 47 mW/cm² (Fig 4.2). *(See: Fig 4.2)*

**PSK Governor (Predictive Sympathetic Kinematics)** ⚡
The Balance State Vector-Cell's novel control law, derived from variational minimization of the
Hunger/Gravity/Equilibrium Manifold Lyapunov functional. Outperforms PID control: 3% vs 15%
overshoot, 2% vs 20% RMS noise. The PSK Governor is both a physics result and
the kernel-level scheduler of Arora OS. *(See: §5, §9.2, Appendix A.2)*

---

## R

**Balance State Vector Unit / Balance State Vector-Cell** ⚡
The fundamental universal quantum information carrier of the Aurphyx architecture.
Formally: a CPTP quantum channel ℰ_Balance State Vector: B(H_system) → B(H_Balance State Vector). Physically: a CVD
diamond Sierpiński substrate + magnetic nanoparticle matrix + C₆ᵥ RF coil ring +
FPGA + photodetector array. *(See: §2.1, Ch.4, Ch.6)*

**RaEState** ⚡ *(R(t))*
The real-time resonance tracking variable of the PSK Governor, normalized to
[0,1]. R(t) = 1 means perfect resonance (ρ = ρ*); R(t) = 0 means complete
decoherence. The PSK Governor drives R(t) → λ* = 0.72 within 50 ms.
*(See: Fig 5.2, §5)*

**Balance State Vector-Drive** ⚡
The power scaling architecture of the Balance State Vector-Cell array. A single cell outputs 50W
input → ~47.5W output. N cells scale as P(N) = P₀·N^α·η_array·κ(N), where
α = 1.293 (fractal superlinear). TTN-corrected arrays reach megawatt scale at
N = 100,000. *(See: Fig 4.9, Ch.11)*

**x_t (Balance State Vector-time)** ⚡
The internal time coordinate of the Balance State Vector-Cell's Floquet drive, measured in units
of the drive period T_Ω = 1/Ω = 100 ps at Ω = 10 GHz. The x_t flux ramp
time series is documented in Fig TVFD.1.

**Renormalization Group (RG)**
A mathematical framework for studying how physical systems change under scale
transformations. The RG β-function traces how coupling constants flow toward
fixed points as energy decreases. The Balance State Vector-Cell's IR fixed point λ* = 0.72 is the
endpoint of this flow. *(See: Fig 4.6, Appendix A.5)*

**Resolvent / Propagator**
The Green's function (energy-domain): G(E) = (E - H)^(-1). In the Balance State Vector-Cell,
the resolvent propagator S_μν represents the cognitive field propagator in state
space. Its SVD spectrum shows the EP crossing as singular value coalescence.
*(See: Fig 5B.4)*

---

## S

**S.A.G.E.S (Sentinel AI Guardian Existence Security)** ⚡
The Aurphyx 13-Sentinel security and cognitive routing system. Operates across
four layers: Detection (Eyes: Valkryx, Umbryx, Ophiux, Zephyra, Prophetyx),
Enforcement (Hands: Praelum, Teslyrax, Cryptanyx), Ledger (Memory: Archivus,
Orric Shade, Nunclex, Nullivar), Orchestration (Heart: Vyrellix). *(See: Ch.8)*

**SAGES Open License** — *see SAGES_LICENSE.md*

**Semantic Wilson Loop** — *see Wilson Loop*

**Sierpiński Gasket**
A fractal geometry defined by recursive self-similar triangles, with Hausdorff
dimension D_f = log(3)/log(2) ≈ 1.585 and spectral dimension d_s ≈ 1.36. The
Balance State Vector-Cell's CVD diamond substrate is etched with a 3-iteration Sierpiński pattern.
*(See: §3.2, §6.2)*

**SIP (SAGES-Balance State Vector Interface Protocol)** ⚡
The JSON message schema for communication between the Balance State Vector-Cell FPGA and the
SAGES Sentinel network. Carries RaEState, F_μν tensor components, Wilson loop
value, and recommended Sentinel routing. *(See: §8.4)*

**SQUID (Superconducting Quantum Interference Device)**
An ultra-sensitive magnetometer that measures magnetic flux quanta. Used in
§6.4 to characterize the Fe₃O₄ nanoparticle matrix's IPR vs. applied field
response. *(See: §6.4)*

**Surface Code**
The leading quantum error correction scheme for fault-tolerant quantum computing.
The Balance State Vector-Cell's fractal error correction achieves a threshold p_threshold ≈ 10^-3
to 10^-4, versus 10^-2 for standard surface codes. *(See: §5.2)*

---

## T

**T₂ (Coherence Time)**
The timescale over which a qubit retains its quantum phase information. The
Balance State Vector-Cell lattice predicts T₂,lattice / T₂,flat ≥ 5 due to topological protection
and fractal localization. *(See: §14.2)*

**TQFT (Topological Quantum Field Theory)**
A quantum field theory whose observables depend only on the topology of spacetime,
not its geometry. Non-semisimple TQFTs extend standard topological quantum
computation via neglectons. *(See: §2.2)*

**TRCA (Topological Resonance Control Architecture)** ⚡
The middleware layer translating Balance State Vector-Cell analog resonance states into discrete
quantum gate operations. Operates at three levels (micro/meso/macro) via TTN
contraction, achieving F_TRCA = 86.8% cross-scale fidelity. *(See: Ch.7)*

**TTN (Tree Tensor Network)**
A hierarchical tensor network structure where tensors are arranged in a tree,
enabling efficient computation of multi-scale correlations. The Balance State Vector-Cell's TRCA
uses TTN contraction at ℓ = 2→1→0→2 to compute cross-scale control signals.
*(See: §7.3, §11.4)*

---

## U

**U(1) Gauge Theory**
The simplest gauge theory, with symmetry group U(1) (complex phases). Classical
electromagnetism is a U(1) gauge theory. The Balance State Vector-Cell's cognitive field theory
is a U(1) gauge theory over the 4-dimensional interoceptive state space
(λ, R, Φ, θ). *(See: §5B, Appendix A.3)*

---

## W

**Wilson Loop**
A gauge-invariant observable: W_γ = Tr 𝒫 exp(i∮_γ A_μ dx^μ). Measures the
holonomy of the gauge connection around a closed path γ. The Balance State Vector-Cell's semantic
Wilson loop |W_γ| = 0.97 quantifies the 3% cross-scale holonomy accumulated
during one complete PSK control cycle. *(See: Fig 5B.5, Appendix A.4)*

---

## Z

**ZPE (Zero-Point Energy)**
The lowest possible energy of a quantum system — it cannot be zero due to
Heisenberg uncertainty. In the Balance State Vector-Cell, ZPE manifests through the Casimir effect
and is partially harvested by the ZPE_Core subsystem. *(See: Ch.10)*

**ZPE_Core** ⚡
The Aurphyx subsystem for Zero-Point Energy extraction, implemented in Rust.
Uses Casimir plates, Sierpiński fractal antennas, Tesla 3-6-9 resonant coils,
and YBCO superconductor flux pumps to supplement the Balance State Vector-Drive power rail.
Current ZPE contribution: ~1.16 mW/cell (supplement, not replacement).
*(See: Ch.10)*

---

*Document End — SCIENTIFIC_TERMINOLOGY.md v1.0.0*
*© 2026 Ross Edwards / Aurphyx LLC Licensed under MIT / Apache 2.0 / SAGES Open License.*
