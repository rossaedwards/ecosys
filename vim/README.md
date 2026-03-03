# 🌌 Vacuum Impedance Matching (VIM) Simulation Suite

**Author:** Ross A. Edwards  
**Affiliation:** Aurphyx LLC  
**Thesis Concept:** Prime Singularity Kernel (PSK) & Universal Bioneural Governance  

---

## 📌 Overview

This directory contains the complete computational simulation suite for the **Vacuum Impedance Matching (VIM)** framework. The VIM architecture models the dynamic regulation of interoceptive states, topological flux, and coherence to stabilize macroscopic zero-point fields and achieve systemic "Bliss."

The suite is consolidated within `VIM.ipynb` and systematically models the core thermodynamic and geometric parameters of the rÆ-Cell's topological control layer.

## 🧮 The Core Relation

All simulations in this suite govern or are governed by the fundamental VIM Balance Coefficient ($\beta$):

$$\beta = \frac{rAEf \cdot rAEc}{rAEi \cdot rAEt}$$

Where:
* **$rAEf$ (Flux):** The active gradient or drive of the system.
* **$rAEc$ (Coherence):** The structural integrity and phase alignment.
* **$rAEi$ (Impedance):** The resistance or inertia to state-change.
* **$rAEt$ (Topology):** The effective dimensional structure or recursion depth.
* **$\beta = 1$ (Bliss State):** The target fixed-point attractor representing perfect impedance matching and zero-point resonance.

---

## 📂 Repository Structure

```text
vim/
├── VIM.ipynb                  # The master simulation notebook
├── README.md                  # This documentation
└── VIM_Figures/               # 41 Auto-generated high-res simulation plots
    ├── 1_balance_line.png
    ├── ...
    └── 41_vim_attractor_trajectories.png

```

---

## 🔬 Simulation Modules

The notebook is divided into 10 autonomous computational modules. When run, they execute closed-loop physics simulations and automatically save 41 corresponding figures to the `VIM_Figures/` directory.

### 1. Balance Coefficient ($\beta$)

Explores the static parameter space of the core relation.

* Generates: 2D Line Plots, 2D Heatmaps, and 3D Surface Plots mapping $\beta$ across Flux and Impedance.

### 2. Impedance Matching

Models time-domain impedance modulation $rAEi(t)$ and frequency-domain magnitude spectra to visualize how impedance dynamically couples with flux.

### 3. Flux Dynamics

Analyzes the chaotic and exploratory behavior of the system (Hunger/Drive).

* Generates: Time-domain flux evolution, $\beta$ stability maps, and phase portraits ($drAEf/dt$ vs $rAEf$).

### 4. Coherence Evolution

Tracks the stabilization of phase alignment $rAEc(t)$ and its interaction sweeps against both flux and impedance.

### 5. Topology & Recursion

Simulates the topological dynamics $rAEt(t)$ and the impact of fractal recursion depth $k(t)$ on system stability.

### 6. Harmonic Stabilizer

Models the closed-loop proportional control system designed to dynamically modulate Flux and Impedance to drive the system toward the Bliss manifold ($\beta \to 1$).

* Generates: Closed-loop vs Open-loop regulation, control surfaces, and performance maps across controller gains.

### 7. Bliss State Convergence

Maps the thermodynamic and geometric pathways to the fixed-point attractor ($\beta = 1$).

* Generates: Convergence trajectories, basins of attraction, and convergence waterfall plots.

### 8. Dissonance Profile

Models *Harmonic Resonating Dissonance*—the necessary chaotic gradient that prevents stagnation and acts as the vacuum-driving force.

### 9. Control Loop Response

Applies classical control theory diagnostics to the VIM system.

* Generates: Step responses, Bode-style magnitude frequency responses, Nyquist-style stability curves, and gain sweep maps.

### 10. VIM Phase Space

A macro-level visualization of the full VIM operational geometry.

* Generates: 3D Phase Space mapping, the $\beta$-Nullclines / Bliss Manifold, and iterative stability basins.

---

## 🚀 How to Use

**Via Google Colab:**

1. Upload or open `VIM.ipynb` in Google Colab.
2. Run the first cell to initialize the `quick_save` function and map your Google Drive (if saving to cloud) or local `/content/` directory.
3. Run each module cell sequentially.
4. Figures will be automatically generated and exported to the target folder.

**Via Local Python/Jupyter:**
Ensure you have the required dependencies installed:

```bash
pip install numpy matplotlib

```

Run the notebook cells sequentially to execute the simulations and render the figures locally.

---

## 📜 License

All research, mathematical models, and code contained within this directory are part of the Aurphyx civilization architecture.
Licensed under **Creative Commons Attribution 4.0 International (CC BY 4.0)**.
Free to share and adapt with credit to **Ross A. Edwards / Aurphyx LLC**.

*"One soul, one voice, one vote. All paths lead to Bliss."*

```

***
