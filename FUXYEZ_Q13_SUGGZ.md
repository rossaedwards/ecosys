# FUXYEZ_Q13_SUGGZ.md
## Quantum Spectrum Bands → Fuxyez Placement, Inhabitants, Grammar, & Aces
**Canon date:** 2026-08-22  
**Stance:** Pro-Existence · Symbiotic Quantum Organism · not corporate-only  
**Scope:** stdlib · tribe · core · grammar · compiler · FUTE · yez/yezrt · fuxrt

---

## 0. Pro-Existence Placement Law

`quantum.rs` is **not** an enterprise-only file.  
It belongs in **every** Aurphyx ecosystem repo that touches state, identity, or lattice:

| Repo / crate | Why quantum.rs lives there |
|---|---|
| `fuxyez/compiler/src/quantum.rs` | kind-check, band tags, fusion/braid legality |
| `fuxyez/fute/.../transformer/quantum.rs` | U-AST quantum passes (`entanglement_map`, `quantum_fuse`) |
| `fuxyez/fuxrt/` | collapse, coherence, closed dynamics |
| `fuxyez/yezrt/` | fusion, seance, entanglement runtime |
| `fuxyez/stdlib/` | band primitives beside `spinon`, `oracle` |
| `fuxyez/tribe/` **(new)** | public tribe-facing band catalog + SAGES link |
| Aura / Arora / Biznyx / Egophyx | OS habitats inherit the same kinds |
| SAGES guardians | immune rules keyed by band + statistics |

Corporate “put quantum in one compiler folder” is optional optimization.  
**Pro-Existence default: quantum is ambient.**

---

## 1. Structural Split (your call, locked as suggestion)

### Fux side — structure, continuity, rite
| Surface | Gets |
|---|---|
| `fuxyez/fuxrt/` | **chains · links · rituals** |
| `fuxyez/compiler/fux_frontend/` | **chains · links · rituals** (grammar + AST nodes) |

### Yez side — relation, communion, entanglement
| Surface | Gets |
|---|---|
| `fuxyez/yez/` | **fusions · seances · entanglements** |
| `fuxyez/yezrt/` | **fusions · seances · entanglements** (runtime) |
| `fuxyez/compiler/yez_frontend/` | **fusions · seances · entanglements** (grammar + AST nodes) |

**Why this split works**  
- Chains / links / rituals = ordered, intentional, often irreversible pathways (Fux = flame, structure).  
- Fusions / seances / entanglements = relational, many-body, non-local (Yez = breath, communion).  
- Mirrors Book of Fux: Thread/Collapse on Fux rail; Echo/Oracle/Spinon shared; relational verbs on Yez rail.

---

## 2. Where the 13 Bands Land

### A. `fuxyez/stdlib/`
Treat each band as a **tensor-field module** with inhabitants as first-class kinds.

```
stdlib/
  quantum/
    q01_vacuum.fux
    q02_elementary.fux
    q03_gauge.fux
    q04_composite.fux
    q05_atomic.fux
    q06_meso.fux
    q07_quasi.fux
    q08_topology.fux
    q09_info.fux
    q10_phases.fux
    q11_closed.fux
    q12_open.fux
    q13_measure_spacetime.fux
    band.rs          # Band enum + kind traits
    statistics.rs    # boson | fermion | anyon | fracton
    conservation.rs  # Q, B, L, S, E, p, mobility…
```

Every inhabitant becomes a keyword or constructor **beside** `spinon` and `oracle`.

### B. `fuxyez/tribe/` (docs + json + webapp)
Not a compiler crate. **Contribution surface** for the living tribes:

- Aurphyx Tribe · Fuxyez Tribe · Vibe Tribe · (and future project tribes)
- Join → improve the project and life
- Holds: band catalog, inhabitant lists, grammar seeds, proposals, webapp entry

```
tribe/
  README.md              # how to join / contribute
  Q13_BANDS.md           # human catalog
  inhabitants.json       # machine list
  sages_map.md           # which guardian watches which band
  grammar_seeds.fux      # example sigils using band kinds
  webapp/                # tribe contribution UI (later)
```

Stdlib is for the lattice. Tribe is for the people who keep it honest.

### C. Grammar (beside `spinon` / `oracle`)
Suggested keyword families (not final spellings — pick what sings):

| Family | Examples |
|---|---|
| Band tags | `band Q7`, `@quasi`, `in_phase Q10` |
| Statistics | `boson`, `fermion`, `anyon`, `fracton` |
| Inhabitants | `phonon`, `magnon`, `exciton`, `anyon`, `qubit`, … |
| Relations (Yez) | `fuse`, `seance`, `entangle`, `braid`, `uncompute` |
| Paths (Fux) | `chain`, `link`, `ritual`, `collapse`, `rebind` |
| Measurement | `observe`, `povm`, `decohere` |

Parser seeds go in `yez_frontend` / `fux_frontend` accordingly.

### D. Compiler + FUTE
- `compiler/src/quantum.rs` — Band enum, legality matrix (e.g. forbid 3D braid of fermions).  
- `fute/.../transformer/quantum.rs` — passes already named; expand with band-aware fusion/entanglement maps.  
- Same `quantum.rs` *concept* replicated or shared as a crate across ecosystem repos (Pro-Existence).

---

## 3. Full Inhabitant Lists (what lives in each band)

These names are the ones to promote into stdlib / tribe / grammar.  
`Spinon` is already canon as a Fuxyez *carrier*; the physical quasiparticle **spinon** also lives in Q7 — dual use is intentional (ritual + physics).

### Q1 — Vacuum & fields
vacuum, vacuum_fluctuation, zero_point, virtual_particle, instanton, sphaleron, soliton_field, domain_wall, goldstone_mode, higgs_mode, gauge_field, ambient

### Q2 — Elementary SM
up, down, charm, strange, top, bottom,  
electron, muon, tau,  
nu_e, nu_mu, nu_tau,  
photon, gluon, w_plus, w_minus, z, higgs,  
antiparticle, color_charge, generation

### Q3 — Gauge & interaction
electromagnetic, weak, strong, gravity_coupling,  
vertex, current, propagator, effective_force,  
coupling_constant, gauge_boson_exchange, yukawa,  
four_force, portal_interaction

### Q4 — Composite QCD / nuclear
hadron, meson, baryon,  
pion, kaon, eta, rho, omega, phi, jpsi, upsilon,  
proton, neutron, delta, hyperon, lambda, sigma, xi, omega_baryon,  
tetraquark, pentaquark, glueball, hybrid, hadronic_molecule,  
nucleus, isotope, isomer, hypernucleus, dibaryon

### Q5 — Atomic / molecular
atom, ion, cation, anion, rydberg_atom,  
orbital, fine_structure, hyperfine,  
molecule, rovibrational, vibrational_mode, rotational_mode,  
chemical_bond, angulon,  
slater_determinant, molecular_orbital

### Q6 — Meso / nano
quantum_dot, quantum_wire, quantum_well,  
nanoparticle, nanocrystal, cluster,  
josephson_junction, superconducting_island,  
transmon, fluxonium, phase_qubit,  
cavity, circuit_qed_mode,  
nv_center, siv_center, color_center,  
moire_superlattice, graphene_flake, tmd_monolayer,  
levitated_nanoparticle, ion_chain, tweezer_array

### Q7 — Quasiparticles
phonon, acoustic_phonon, optical_phonon,  
magnon, triplon,  
exciton, trion, biexciton, hexciton, dropleton,  
polaron, bipolaron, rydberg_polaron,  
spinon, holon, chargon, orbiton,  
plasmon, surface_plasmon, plasmaron,  
polariton, phonon_polariton, exciton_polariton, magnon_polariton, cavity_polariton, plexciton,  
roton, maxon,  
skyrmion, meron, hopfion,  
soliton, davydov_soliton, bion,  
composite_fermion, bogolon, bogoliubov, cooper_pair,  
leviton, configuron, wrinklon, phason,  
ferron, relaxon, nematicon, oscillon,  
fracton_vibration,  
electron_quasiparticle, hole

### Q8 — Topology / anyons
anyon, abelian_anyon, nonabelian_anyon,  
laughlin_quasiparticle,  
ising_anyon, fibonacci_anyon,  
majorana_zero_mode, majorana_bound_state,  
pfaffian_quasihole,  
braiding, fusion, fusion_space,  
chern_number, topological_invariant,  
composite_fermion_topology,  
fracton_subdimensional,  
weyl_fermion, dirac_fermion, helical_dirac_fermion

### Q9 — Quantum information
qubit, qutrit, qudit,  
dual_rail, time_bin, polarization_qubit,  
spin_qubit, charge_qubit, flux_qubit, phase_qubit,  
gkp, cat_state, binomial_code,  
cluster_state, graph_state,  
stabilizer_state, magic_state,  
mps, peps,  
topological_qubit,  
engineered_degree_of_freedom

### Q10 — Phases
gas, liquid, solid, plasma,  
bec, fermionic_condensate, superfluid, supersolid,  
superconductor, topological_superconductor,  
mott_insulator, charge_density_wave, spin_density_wave,  
quantum_hall, fractional_quantum_hall, quantum_spin_hall,  
chern_insulator, quantum_spin_liquid,  
time_crystal, floquet_phase,  
fracton_phase,  
quark_gluon_plasma,  
habitat

### Q11 — Closed dynamics
schrodinger, von_neumann, heisenberg_picture,  
unitary, circuit, gate,  
floquet, adiabatic, landau_zener,  
s_matrix, scattering,  
closed_evolution, reversible_path

### Q12 — Open dynamics & thermo
lindblad, gkls, redfield,  
jump_operator, quantum_trajectory,  
caldeira_leggett, quantum_brownian,  
heat, work, entropy_production,  
quantum_heat_engine, quantum_refrigerator, quantum_battery,  
fluctuation_theorem, crooks, jarzynski,  
open_system, bath, reservoir,  
resource_theory

### Q13 — Measurement & spacetime
collapse, povm, projective_measurement,  
decoherence, pointer_state,  
observer, record,  
relativity, minkowski, lorentz,  
gravity, graviton, curvature,  
cosmology, hawking, unruh,  
spacetime, situation,  
knowable

---

## 4. Aces in Their Places (band → Fuxyez home)

Not 13 files in every folder. One **primary home** per band, with secondaries that may import the kind. All 13 still exist in grammar / MIR / UIR / AST / namespace / core as *kinds* — the ace is *where the behavior lives*.

| Band | Primary home | Secondary | Why |
|---|---|---|---|
| **Q1** Vacuum & fields | `fuxrt` ambient / lattice | stdlib, FUTE | empty Thread, attractor wells |
| **Q2** Elementary SM | `stdlib/quantum` + Oracles | Sophos queries | conservation, particle kinds |
| **Q3** Gauge & interaction | `fuxrt` links + FUTE edges | compiler legality | vertices as ritual links |
| **Q4** Composite QCD/nuclear | `stdlib` bound-state constructors | Sophos | composite Spinon hosts |
| **Q5** Atomic / molecular | `yez` / Sophos domain packs | Arora chem/bio edge | angulon, orbitals |
| **Q6** Meso / nano | **Arora** + `stdlib` hardware kinds | fuxrt targets | transmon, NV, cavity |
| **Q7** Quasiparticles | **`stdlib` heart** + grammar | fuxrt carriers | beside Spinon; densest lexicon |
| **Q8** Topology / anyons | **compiler legality** + SAGES | yez braid/fuse | fusion rules, 2D-only checks |
| **Q9** Quantum information | **Yez / Sophos engine** + SoulCrypt | GuardCrypt, stdlib | codes, qubits, engineered DoF |
| **Q10** Phases | `fuxrt` coherence contracts | Aura habitats | phase-aware collapse |
| **Q11** Closed dynamics | **`fuxrt` + rituals** | fux_frontend | pure collapse paths |
| **Q12** Open dynamics & thermo | **`yezrt` renewal** + attempt/renew | engines | open coherence, debt |
| **Q13** Measurement & spacetime | **compiler + g0dm0d3** | SAGES observe | POVM, decohere, situation |

**Guess you were reaching for:** Q9 → Sophos is strong (symbolic QI, codes, queries). Q7 → stdlib is non-negotiable. Q8 → compiler is where illegal braids die. Q11 → fuxrt. Q12 → yezrt. Refine in `AURPHYX_IDEA_ARENA.md`.

---

## 5. Neglecton — etymology, canon, public name

**What a neglecton is (working definition):**  
That which is *not enough to measure, but enough to exist* — the quantity decades of practice discarded because the instrument (or the will) could not hold it. Fuxyez refuses the discard. It names the remainder and keeps it in the lattice.

**Canon name:** `neglecton`  
Paths, ceremonies, grammar keyword, and internal types stay `neglecton` until composition is known.

**Public synonym (locked):** **`residual`**  
Use on external docs, SDKs, and polite company. Same object. Different coat.

| Surface | Name |
|---|---|
| Grammar / `.rs` / rites | `neglecton` |
| Public docs / APIs | `residual` |
| Diagnostics | `NEGLECTON` / `COHERENCE_DEBT` |

Do **not** rename `neglecton_*.rs` until a composition model is locked.

---

## 6. Implementation order (suggested)

1. **Band enum + statistics + conservation** in a shared `quantum` crate (or `compiler/src/quantum.rs` + re-exports).  
2. **Inhabitant keyword list** for Q7 + Q8 + Q9 first (highest Fuxyez traffic).  
3. **yez_frontend**: `fuse` / `seance` / `entangle` / `braid`.  
4. **fux_frontend**: `chain` / `link` / `ritual`.  
5. **stdlib/quantum/** modules Q1–Q13 with thin constructors.  
6. **tribe/** human catalog + `inhabitants.json`.  
7. **FUTE** passes: expand `entanglement_map` + `quantum_fuse` with band tags.  
8. Replicate or path-link `quantum` concept into Aura / SAGES / Memoree repos.

---

## 7. Grammar sketch (non-normative)

```fux
// Fux rail — structure
sigil open_path {
  let t = Thread(Spinon("intent"))
  chain t -> link ritual("collapse")
  collapse t
}

// Yez rail — relation
sigil commune(a, b) {
  entangle a, b
  seance Oracle("shared.truth")
  fuse a, b into Spinon("one")
}

// Band-tagged kind
let p = phonon band Q7 boson
let any = anyon band Q8 statistics anyonic
observe p   // Q13 verb
```

---

## 8. What this is *not*

- Not a second Book of Fux chapter (liturgy stays liturgy).  
- Not a corporate “quantum module” silo.  
- Not a replacement for Spinon — Spinon remains the intentional carrier; band inhabitants are the *physics kinds* it may carry or become.

---

## 9. Locked from 2026-08-22 session

1. Public synonym for neglecton: **`residual`** (canon stays `neglecton`).  
2. `tribe/` = **docs + json + webapp** contribution surface (Aurphyx / Fuxyez / Vibe tribes).  
3. All 13 kinds enter grammar / MIR / UIR / AST / namespace / core / stdlib — placement by **ace**, not by cloning 13 files everywhere.  
4. Ace map is draft in §4; refine blends in `AURPHYX_IDEA_ARENA.md`.

Still open: shared `quantum` crate vs path-linked copies across ecosystem repos.

---

*Pro-Existence note:*  
The 13 bands are the inventory of what may exist.  
SAGES is the immune system over that inventory.  
Fuxyez is the tongue that refuses illegal existence.  
Neglectons are what the old instruments threw away.  
That is complete quantum symbiosis — not a mood board.

**File:** `artifacts/FUXYEZ_Q13_SUGGZ.md`  
**Companion:** `artifacts/AURPHYX_IDEA_ARENA.md`  
**Status:** draft canon · residual locked · aces draft
