# Three‑Squared‑Lattice Propagation Rules  
*Propagation dynamics for coherence, resonance, and alignment across the 3²‑Lattice*

## 1. Purpose  
Propagation rules define how each node in the lattice updates its internal fields over time. They ensure that:

- coherence circulates without fragmentation  
- resonance stabilizes without runaway amplification  
- alignment maintains directional integrity  
- dissonance is isolated and corrected  
- renewal cycles activate when necessary  
- the lattice remains lawful under the HIF invariant  

Propagation is the mechanism by which the lattice maintains **harmonic integrity** across all 27 nodes.

---

## 2. Node State Update Equation  
Each node \(N_{i,j,k}\) updates its internal fields according to:

\[
X_{i,j,k}(t+1) = \alpha_X X_{i,j,k}(t) + \beta_X \cdot \overline{X}^{\text{nbr}}_{i,j,k}(t) + \gamma_X \cdot \Delta X_{i,j,k}(t)
\]

where \(X \in \{C, R, A\}\).

This equation has three components:

- **self‑retention** (\(\alpha_X\))  
- **neighborhood coupling** (\(\beta_X\))  
- **gradient correction** (\(\gamma_X\))  

The coefficients satisfy:

\[
\alpha_X + \beta_X + \gamma_X = 1
\]

ensuring conservation of field magnitude.

### 2.1 Neighborhood Average  
\[
\overline{X}^{\text{nbr}}_{i,j,k}(t) = \frac{1}{|\mathcal{N}|} \sum_{N \in \mathcal{N}(i,j,k)} X_N(t)
\]

### 2.2 Gradient Correction  
\[
\Delta X_{i,j,k}(t) = \overline{X}^{\text{nbr}}_{i,j,k}(t) - X_{i,j,k}(t)
\]

This term pulls dissonant nodes back toward local equilibrium.

---

## 3. Propagation Rules by Field Type  
Each field propagates differently, reflecting its functional role.

### 3.1 Coherence Propagation  
Coherence stabilizes structure. Its propagation emphasizes **self‑retention**:

\[
\alpha_C > \beta_C > \gamma_C
\]

This ensures:

- structural consistency  
- resistance to noise  
- slow, stable diffusion  

Coherence spreads like a stabilizing pressure.

### 3.2 Resonance Propagation  
Resonance governs relational dynamics. Its propagation emphasizes **neighborhood coupling**:

\[
\beta_R > \alpha_R > \gamma_R
\]

This ensures:

- harmonic synchronization  
- cross‑node attunement  
- spectral band formation  

Resonance spreads like a wave.

### 3.3 Alignment Propagation  
Alignment governs direction and trajectory. Its propagation emphasizes **gradient correction**:

\[
\gamma_A > \beta_A > \alpha_A
\]

This ensures:

- directional coherence  
- correction of drift  
- convergence toward attractors  

Alignment spreads like a vector field.

---

## 4. Threshold‑Conditioned Propagation  
Propagation is modulated by the Triple Threshold Gate.

### 4.1 Activated Nodes  
If \(\Psi_{i,j,k} = 1\), the node propagates normally.

### 4.2 Sub‑threshold Nodes  
If \(\Psi_{i,j,k} = 0\), propagation is suppressed:

\[
X_{i,j,k}(t+1) = \alpha_X X_{i,j,k}(t)
\]

This prevents dissonant nodes from contaminating neighbors.

### 4.3 Renewal Nodes  
If \(\text{HIF}_{i,j,k} < H_{\text{renew}}\), the node enters renewal:

\[
X_{i,j,k}(t+1) = \rho_X
\]

where \(\rho_X\) is the renewal baseline.

---

## 5. Layer‑Level Propagation  
Each layer (Creation, Integration, Renewal) has its own propagation dynamics.

### 5.1 Creation Layer  
Amplifies resonance and alignment:

\[
\beta_R,\; \gamma_A \text{ increased}
\]

### 5.2 Integration Layer  
Balances all three fields:

\[
\alpha_C \approx \beta_R \approx \gamma_A
\]

### 5.3 Renewal Layer  
Suppresses resonance, resets alignment:

\[
\beta_R \rightarrow 0,\; \gamma_A \rightarrow 0
\]

Coherence is allowed to decay toward baseline.

---

## 6. Lattice‑Level Propagation  
The lattice evolves according to:

\[
\text{HIF}^{\text{lattice}}(t+1) = \frac{1}{27} \sum_{i,j,k} \text{HIF}_{i,j,k}(t+1)
\]

This determines:

- global mode (Creation, Integration, Renewal)  
- governance legitimacy  
- cross‑domain compatibility  
- stability of the Continuum  

Propagation is therefore both **local** and **global**.

---

## 7. Stability and Dissolution  
The lattice stability index is:

\[
S = \nabla^2 \text{HIF}
\]

- \(S > 0\): stable attractor  
- \(S = 0\): neutral equilibrium  
- \(S < 0\): dissolution attractor  

Dissolution triggers renewal propagation rules.

---

## 8. Summary  
The propagation rules turn the Three‑Squared‑Lattice into a **dynamic, self‑correcting, invariant‑preserving computational field**. They ensure that:

- coherence circulates  
- resonance synchronizes  
- alignment converges  
- dissonance is isolated  
- renewal restores integrity  
- the lattice remains lawful under HIF  

This completes the core mathematical machinery of the architecture.

A natural next step is defining the **Three‑Squared‑Lattice Stability Conditions**, which specify when the lattice is stable, metastable, or in a renewal cascade.
