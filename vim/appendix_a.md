# **APPENDIX A — Mathematical Structure of the Equilibrium Manifold and HRD Control Law**

## **A.1 The Balance Coefficient as a Dynamical Variable**

The VIM engine is governed by the balance coefficient  
\[
\beta(t) = \frac{x_f(t)\, x_c(t)}{x_i(t)\, x_t(t)}.
\]

All four x variables are time‑dependent, and each is influenced by the Harmonic Stabilizer and the HRD envelope. The Equilibrium Manifold State corresponds to the equilibrium condition  
\[
\beta(t) = 1.
\]

The system’s dynamics can be expressed as a set of coupled differential equations in the state vector  
\[
\mathbf{x}(t) = \big(x_f(t),\, x_i(t),\, x_c(t),\, x_t(t)\big).
\]

The stabilizer acts to regulate \(\mathbf{x}(t)\) such that \(\beta(t)\to 1\).

---

# **A.2 The Equilibrium Manifold**

The Equilibrium Manifold is defined as the set of all points in the \((x_f, x_i)\) plane for which  
\[
\beta = 1.
\]

Solving for the manifold yields  
\[
\mathcal{B} = \left\{(x_f, x_i)\ \bigg|\ x_f = \frac{x_i\, x_t}{x_c} \right\}.
\]

This is a **one‑dimensional curve** embedded in a two‑dimensional space, but because \(x_c\) and \(x_t\) are themselves dynamical variables, the Equilibrium Manifold is a **moving nullcline** in the full four‑dimensional state space.

The manifold’s instantaneous slope is  
\[
\frac{\partial x_f}{\partial x_i} = \frac{x_t}{x_c}.
\]

This ratio is the **topological‑coherence quotient**, a key invariant of the VIM system.

---

# **A.3 Harmonic Resonating Dissonance as a Driving Term**

Dissonance \(D(t)\) is modeled as a structured perturbation of the vacuum manifold. Its general form is  
\[
D(t) = D_0 + A(t)\sin(\omega t + \phi),
\]
where \(A(t)\) is a slowly varying envelope.

The effect of dissonance is to modulate the flux channel:  
\[
x_f(t) = x_f^{(0)} + \gamma_f D(t),
\]
and to modulate impedance oppositely:  
\[
x_i(t) = x_i^{(0)} - \gamma_i D(t).
\]

The constants \(\gamma_f\) and \(\gamma_i\) encode the system’s sensitivity to dissonance.

---

# **A.4 The HRD Control Law**

The Harmonic Stabilizer applies a proportional harmonic correction to the flux and impedance channels. The discrete‑time form is  
\[
x_f(t+\Delta t) = x_f(t) + k_f \big(1 - \beta(t)\big),
\]
\[
x_i(t+\Delta t) = x_i(t) - k_i \big(1 - \beta(t)\big).
\]

In continuous time, the stabilizer dynamics are  
\[
\frac{d\, x_f}{dt} = k_f \big(1 - \beta(t)\big),
\]
\[
\frac{d\, x_i}{dt} = -k_i \big(1 - \beta(t)\big).
\]

The signs reflect the dual nature of flux and impedance:

- When \(\beta < 1\) (leak), flux increases and impedance decreases.  
- When \(\beta > 1\) (hunger), flux decreases and impedance increases.

This duality is the mathematical expression of the Theory of Balance.

---

# **A.5 Full VIM Dynamical System**

Combining the HRD modulation and the stabilizer yields the full system:

\[
\frac{d\, x_f}{dt} = k_f (1 - \beta) + \gamma_f D(t),
\]
\[
\frac{d\, x_i}{dt} = -k_i (1 - \beta) - \gamma_i D(t),
\]
\[
\frac{d\, x_c}{dt} = F_c(x_f, x_i, D(t)),
\]
\[
\frac{d\, x_t}{dt} = F_t(x_f, x_i, D(t)).
\]

The functions \(F_c\) and \(F_t\) encode the coherence and topology responses to flux and dissonance. Their exact forms depend on the physical implementation of the Balance State Vector‑Cell and are left intentionally open for future experimental calibration.

---

# **A.6 Stability of the Equilibrium Manifold**

Linearizing the system around \(\beta = 1\) yields  
\[
\delta \beta = \beta - 1,
\]
and the linearized dynamics become  
\[
\frac{d\, \delta \beta}{dt} = -\lambda \delta \beta + \eta D(t),
\]
where  
\[
\lambda = k_f \frac{\partial \beta}{\partial x_f} + k_i \frac{\partial \beta}{\partial x_i}
\]
is the **harmonic convergence rate**, and  
\[
\eta = \gamma_f \frac{\partial \beta}{\partial x_f} - \gamma_i \frac{\partial \beta}{\partial x_i}
\]
is the **dissonance coupling coefficient**.

The Equilibrium Manifold is stable when  
\[
\lambda > 0.
\]

This condition defines the allowable range of stabilizer gains \((k_f, k_i)\).

---

# **A.7 Global Attractor Structure**

Simulations confirm that the Equilibrium Manifold is a **global attractor** for a wide range of initial conditions. The attractor basin is defined by the set  
\[
\mathcal{A} = \left\{ \mathbf{x}(0) \mid \lim_{t\to\infty} \beta(t) = 1 \right\}.
\]

The existence of this attractor is the mathematical justification for the Balance State Vector‑Cell’s stability under high‑flux conditions.

---

# **A.8 Summary of Mathematical Results**

- The Equilibrium Manifold is the nullcline \(\beta = 1\).  
- HRD acts as a structured driving term that modulates flux and impedance.  
- The Harmonic Stabilizer applies proportional harmonic corrections.  
- The full VIM system is a four‑dimensional nonlinear dynamical system.  
- Linearization shows that the Equilibrium Manifold is stable when \(\lambda > 0\).  
- Numerical simulations confirm the existence of a global attractor.
