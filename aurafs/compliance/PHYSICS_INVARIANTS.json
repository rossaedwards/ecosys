{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "_metadata": {
    "document_id": "AURPHYX-COMP-PHYS-001",
    "version": "1.0.0",
    "date": "2026-02-08",
    "author": "Ross A. Edwards, Aurphyx LLC",
    "description": "Machine-readable physics invariants for AuraFS. This file is the single source of truth for all physical constants used in the codebase. The CI test suite ingests this file to ensure no magic numbers creep into source code and that all physics-critical values remain within validated bounds.",
    "source_config": "aurafs.toml",
    "source_thesis": "Aurphyx PRX Submission (arXiv:2601.XXXXX)",
    "source_validation": "VALIDATION_REPORT.md (02-07-2026)"
  },

  "invariants": {
    "hilbert_scaling_bias": {
      "symbol": "η",
      "value": 5.3,
      "unit": "dimensionless",
      "tolerance": {
        "min": 5.25,
        "max": 5.35,
        "note": "±0.05 accounts for finite-size effects at low recursion depth"
      },
      "thesis_reference": "Theorem 2.1 (Fractal Hilbert Space Scaling)",
      "thesis_section": "Section II.4",
      "validation": {
        "method": "Qiskit simulation, n=5 qubits with fractal connectivity",
        "measured_advantage": "7.1× at k=1 (consistent with 2^{5×0.585} ≈ 7)",
        "full_advantage_at_depth_5": "5.3× (matched at N=100, D=5)",
        "report_section": "VALIDATION_REPORT.md §1"
      },
      "aurafs_usage": "PRIMARY CONSTANT. Sets Hilbert Scaling advantage for N=5. Governs replica distribution: Replicas = ceil(log_{5.3}(Nodes)).",
      "code_locations": [
        "core/src/shard/fractal.rs (FractalScaling struct, default bias)",
        "src/physics/mod.rs (HILBERT_SCALING_BIAS constant)"
      ]
    },

    "coherence_window_us": {
      "symbol": "T₂",
      "value": 1600,
      "unit": "microseconds",
      "tolerance": {
        "min": 1500,
        "max": 1700,
        "note": "±100 μs from environmental noise floor variation"
      },
      "thesis_reference": "Section II.8 (Error Correction Advantage), Figure 2",
      "thesis_section": "Section II.8",
      "validation": {
        "method": "NumPy analytical approximation of Lindblad master equation",
        "fractal_T2": "1600 μs (1.6 ms)",
        "transmon_T2": "100 μs",
        "improvement_factor": "16×",
        "mechanism": "Anderson Localization (confirmed via IPR analysis, d_s < 2)",
        "report_section": "VALIDATION_REPORT.md §2"
      },
      "aurafs_usage": "PRIMARY CONSTANT. 16× stability gain. Defines the maximum tick rate for the PassiveCoherence integrity monitor. All shard health checks must complete within this window.",
      "code_locations": [
        "core/src/integrity/monitor.rs (PassiveCoherence trait, tick interval)",
        "src/physics/mod.rs (COHERENCE_WINDOW_US constant)"
      ],
      "derived_constants": {
        "lock_acquisition_timeout_us": {
          "value": 100,
          "derivation": "coherence_window_us / 16 (one FUSE lock cycle per coherence tick)"
        }
      }
    },

    "spectral_dimension": {
      "symbol": "d_s",
      "value": 1.37,
      "unit": "dimensionless",
      "tolerance": {
        "min": 1.32,
        "max": 1.42,
        "note": "±0.05 variance triggers decoherence_recovery"
      },
      "theoretical_target": 1.365,
      "theoretical_derivation": "d_s = 2·log(3)/log(5) ≈ 1.365 (exact for Sierpiński gasket via decimation renormalization, Rammal & Toulouse 1983)",
      "thesis_reference": "Proposition 2.1 (Anomalous Density of States)",
      "thesis_section": "Section II.2",
      "validation": {
        "method": "Tight-binding model, NetworkX Laplacian eigenvalue analysis",
        "measured_value": 1.37,
        "deviation_from_theory": "+0.005 (within numerical precision)",
        "implication": "Confirms anomalous diffusion and trap states for data persistence",
        "report_section": "VALIDATION_REPORT.md §3"
      },
      "aurafs_usage": "PRIMARY CONSTANT. Governs anomalous density of states for data 'trapping'. Baseline density for Trap-State calculations. Implementation clamps to 1.37.",
      "code_locations": [
        "core/src/integrity/monitor.rs (decoherence threshold check)",
        "src/physics/mod.rs (SPECTRAL_DIMENSION constant)"
      ]
    },

    "photonic_band_gap": {
      "symbol": "PBG (Δω/ω)",
      "value": 0.21,
      "unit": "dimensionless (fractional bandwidth)",
      "tolerance": {
        "min": 0.18,
        "max": 0.24,
        "note": "±0.03 accounts for fabrication index variation in fused silica"
      },
      "thesis_reference": "Section IV (Photonic Band Engineering), Figure 5",
      "thesis_section": "Section IV",
      "validation": {
        "method": "Plane-wave expansion (PWE) of C₆v hexagonal lattice",
        "gap_type": "Complete TM band gap",
        "gap_width": "21% (Δω = 0.4 × 2πc/a)",
        "implication": "Noiseless optical interconnects (zero-crosstalk)",
        "report_section": "VALIDATION_REPORT.md §4"
      },
      "aurafs_usage": "Routing overhead allowance. Meshwerk routing tables budget 21% latency overhead to guarantee zero-crosstalk paths.",
      "code_locations": [
        "network/src/meshwerk.rs (routing overhead calculation)",
        "src/physics/mod.rs (PHOTONIC_BAND_GAP constant)"
      ]
    },

    "fractal_density": {
      "symbol": "D_f",
      "value": 1.585,
      "unit": "dimensionless (Hausdorff dimension)",
      "tolerance": {
        "min": 1.580,
        "max": 1.590,
        "note": "Exact value is log(3)/log(2); tolerance is for floating-point representation"
      },
      "exact_value": "log(3)/log(2)",
      "thesis_reference": "Section II.1 (Fractal Lattice Geometry)",
      "thesis_section": "Section II.1",
      "validation": {
        "method": "Analytical (exact for Sierpiński gasket)",
        "report_section": "VALIDATION_REPORT.md (Fractal Density row)"
      },
      "aurafs_usage": "Theoretical Hausdorff dimension used for lattice geometry generation and error correction overhead calculation. R_fractal ∝ (log(1/p_L)/log(1/p_phys))^{2/D_f}.",
      "code_locations": [
        "core/src/shard/fractal.rs (lattice geometry generation)"
      ]
    }
  },

  "derived_metrics": {
    "fidelity_improvement_target": {
      "value": "16×",
      "composition": {
        "passive_coherence": "16× (from Anderson localization, T₂ = 1600 μs vs. 100 μs)",
        "topological_protection": "~3× (from non-Abelian braiding, neglecton phase)",
        "fractal_overhead_reduction": "~2.7× (physical-to-logical ratio: 89 vs. 1458)"
      },
      "note": "The 16× figure refers specifically to the passive coherence gain. Combined with topological and fractal contributions, total fidelity improvement reaches ~130× in ideal conditions."
    },

    "error_correction_overhead": {
      "euclidean_ratio": 1458,
      "fractal_ratio": 89,
      "improvement_factor": "16.4×",
      "conditions": "p_phys = 10⁻³, p_L = 10⁻¹², surface code baseline",
      "thesis_reference": "Proposition 2.3 (Error Correction Advantage)"
    },

    "hilbert_advantage_table": [
      { "nodes": 12,  "depth": 3, "D_eff": 2.38, "fractal_state_vol": 39.4,   "euclidean_state_vol": 12,  "advantage": "3.2×" },
      { "nodes": 42,  "depth": 4, "D_eff": 2.77, "fractal_state_vol": 158,    "euclidean_state_vol": 42,  "advantage": "3.7×" },
      { "nodes": 100, "depth": 5, "D_eff": 3.02, "fractal_state_vol": 530,    "euclidean_state_vol": 100, "advantage": "5.3×" }
    ]
  },

  "governance_constants": {
    "min_quorum": {
      "value": 13,
      "derivation": "Byzantine fault tolerance: 3f + 1 where f = 4 (maximum tolerated faults)",
      "code_location": "src/gov/sages.rs"
    },
    "signature_scheme": {
      "value": "Dilithium-5",
      "standard": "FIPS 204 (ML-DSA-87)",
      "security_level": "NIST Level 5 (AES-256 equivalent)",
      "code_location": "src/crypto/pqc/dilithium_sig.rs"
    },
    "lock_acquisition_timeout_us": {
      "value": 100,
      "derivation": "coherence_window_us / 16",
      "code_location": "src/core/shard.rs (FUSE lock)"
    }
  },

  "ci_test_directives": {
    "description": "Directives for the CI pipeline to enforce physics invariant compliance.",
    "rules": [
      {
        "rule_id": "PHYS-001",
        "description": "No source file outside of aurafs.toml and this JSON may contain a floating-point literal equal to any invariant value (5.3, 1.37, 0.21, 1.585) without a comment referencing the config source.",
        "severity": "ERROR",
        "regex_pattern": "(?<!//.*)(5\\.3[0-9]*|1\\.37[0-9]*|0\\.21[0-9]*|1\\.585[0-9]*)"
      },
      {
        "rule_id": "PHYS-002",
        "description": "The PassiveCoherence monitor tick interval must be less than coherence_window_us (1600 μs).",
        "severity": "ERROR",
        "test_file": "tests/physics/test_coherence.rs"
      },
      {
        "rule_id": "PHYS-003",
        "description": "Spectral dimension variance exceeding ±0.05 from 1.37 must trigger decoherence_recovery, not a generic error.",
        "severity": "ERROR",
        "test_file": "tests/physics/test_spectral.rs"
      },
      {
        "rule_id": "PHYS-004",
        "description": "Replica count must satisfy ceil(log_{5.3}(N)) for any node count N.",
        "severity": "ERROR",
        "test_file": "tests/physics/test_scaling.rs"
      },
      {
        "rule_id": "SEC-001",
        "description": "All signature operations must use Dilithium-5 (ML-DSA-87). No fallback to classical schemes.",
        "severity": "CRITICAL",
        "test_file": "tests/crypto/test_dilithium.rs"
      }
    ]
  }
}
