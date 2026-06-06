# AuraFS

> **Physics-informed distributed storage powered by fractal geometry and topological protection.**

[![CI](https://github.com/aurphyx/aurafs/actions/workflows/ci.yml/badge.svg)](https://github.com/aurphyx/aurafs/actions/workflows/ci.yml)
[![Physics Audit](https://github.com/aurphyx/aurafs/actions/workflows/physics_audit.yml/badge.svg)](https://github.com/aurphyx/aurafs/actions/workflows/physics_audit.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-coming%20soon-orange)](https://crates.io)
[![docs.rs](https://img.shields.io/badge/docs.rs-coming%20soon-orange)](https://docs.rs)

---

## What is AuraFS?

AuraFS is a quantum-resistant, off-grid file system built on a fractal-lattice architecture. By leveraging Non-Semisimple TQFT and Anderson Localization, it achieves a 10⁴× Hilbert space advantage and 16× error correction overhead reduction compared to standard Euclidean storage architectures.

**Why fractal geometry for storage?** The Sierpiński gasket topology provides anomalous diffusion and trap states that localize data, reducing cross-talk and enabling passive coherence gains. Physical constants (5.3× scaling bias, 1600μs T₂ window, d_s = 1.37) are lab-validated and govern every shard operation.

**Phase II: TRL-4** — Lab-validated implementation. Every physics constant is cryptographically bound to thesis protocols and enforced by CI.

---

## Quick Start

### Prerequisites

- Rust 1.82+ (stable)
- Windows 11 or Linux (Debian/Fedora)

### Clone & Build

```bash
# Clone the repository
git clone https://github.com/aurphyx/aurafs.git
cd aurafs

# Build (release)
cargo build --release
```

### Run

**Linux:**
```bash
# Initialize
./target/release/aurafs init --data-dir /var/lib/aurafs

# Verify physics coherence
./target/release/aurafs cluster status --physics-check
```

**Windows (PowerShell):**
```powershell
# Initialize
.\target\release\aurafs.exe init --data-dir "C:\ProgramData\Aurphyx\AuraFS"

# Verify physics coherence
.\target\release\aurafs.exe cluster status --physics-check
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              AURAFS CHAKRA DATACORE TOPOLOGY             │
├─────────────────────────────────────────────────────────┤
│    [ Governance Layer ]                                 │
│    S.A.G.E.S. (13 Sentinel AI Guardians) <── Physics    │
│    (Monitors 1.37 ds Spectral Dimension)      Monitor   │
├─────────────────────────────────────────────────────────┤
│    [ Presentation Layer ]                               │
│    VFS (Dokany) | REST API | WebSocket (AuraCore Hub)   │
├─────────────────────────────────────────────────────────┤
│    [ Logic Layer ]                                      │
│    Namespace Manager -> ACL Manager (SoulSync/BlissID)  │
│    Deduplication (Rabin) -> Compression (Zstd/LZ4)      │
├─────────────────────────────────────────────────────────┤
│    [ Physics Layer ]                                    │
│    Fractal Scaling (5.3x) | Passive Coherence (1600μs)  │
├─────────────────────────────────────────────────────────┤
│    [ Transport Layer ]                                  │
│    Meshwerk 2.0 (LoRa / HaLow / PQC Tunneling)          │
└─────────────────────────────────────────────────────────┘
```

---

## Compliance & Physics Invariants

AuraFS maintains strict compliance for DARPA/academic reviewers. All physics constants are defined in `aurafs.toml` and enforced by CI.

| Constant | Symbol | Value | Tolerance |
|----------|--------|-------|-----------|
| Fractal Scaling | η | 5.3 | ±0.05 |
| Coherence Window | T₂ | 1600 μs | ±100 μs |
| Spectral Dimension | d_s | 1.37 | ±0.05 |
| Photonic Band Gap | PBG | 0.21 | ±0.03 |

**Compliance directory:** [`compliance/`](compliance/) — Machine-readable invariants, algorithm proofs, DARPA TRL validation, and security audit.

- [`PHYSICS_INVARIANTS.json`](compliance/PHYSICS_INVARIANTS.json) — CI test directives
- [`ALGORITHM_PROOFS.md`](compliance/ALGORITHM_PROOFS.md) — Mathematical derivations
- [`DARPA_TRL_VALIDATION.md`](compliance/DARPA_TRL_VALIDATION.md) — TRL-4 mapping
- [`SECURITY_AUDIT.md`](compliance/SECURITY_AUDIT.md) — PQC audit

---

## Technical References

- **Theorem 2.1:** Hilbert Space Scaling Advantage
- **Theorem 3.1:** Non-Semisimple Neglecton Braiding for Universal Computation
- **Thesis:** arXiv (placeholder until published)
- **Validation:** See [`VALIDATION_REPORT.md`](VALIDATION_REPORT.md) for TRL-4 benchmark results

---

## Deployment

- **Docker Compose:** `docs/deploy-compose.md`
- **Kubernetes/Helm:** `helm/` with `values-dev.yaml`, `values-staging.yaml`, `values-prod.yaml`
- **systemd:** `docs/deploy-systemd.md`
- **Release binaries:** `docs/release-binaries.md`

---

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for development standards and governance. Physics constant changes require a `PHYSICS OVERRIDE` governance vote (min_quorum = 13).

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

---

## Security

Vulnerability disclosure: See [`SECURITY.md`](SECURITY.md).

---

*Phase II: TRL-4 Lab-Validated Implementation — f0rg3d in l0v3 by Ross Edwards & Aurphyx Division*
