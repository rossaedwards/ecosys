# Changelog

All notable changes to AuraFS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive quantum module with PQC bridge, entanglement simulation, and QRNG
- Monitoring module with metrics collection, health checks, and alerting
- Healing module with recovery strategies and retry logic
- FUSE filesystem operations: unlink, rename, open, release, create, setattr
- Session context management for BlissID authentication
- Network shard fallback for distributed data access
- Erasure coding implementation with Reed-Solomon
- IPFS cluster federation storage with quorum-based replication
- Feature flags: `security-tools`, `data_slayer_node`, `full`
- Consolidated development documentation

### Changed
- Unified error handling with comprehensive `RafsError` and `CoreError`
- Restructured lib.rs with proper module organization and documentation
- Enhanced crypto module with comprehensive re-exports
- Improved secure tunnel handshake with full implementation
- Feature-gated redteam/whitehat security testing modules

### Fixed
- Removed duplicate `banner.rs` file
- Moved misplaced PowerShell scripts to `scripts/`
- Removed empty root `mod.rs`
- Consolidated 11 standalone `main.rs` into 4 binaries
- Resolved all critical TODOs in core modules

### Removed
- Duplicate entry points in core, network, storage, governance, mesh, compression
- Redundant `lib.rs` files in subdirectories
- Stray `Cargo.toml` in `src/core/`

## [1.0.0] - 2026-01-09

### Added
- Initial production release of AuraFS
- Quantum-secure distributed filesystem
- Post-quantum cryptography (Kyber-1024, Dilithium-5)
- Mesh networking with Reticulum integration
- BlissID soul-based identity system
- Tiered storage with local, S3, and IPFS backends
- FUSE filesystem mounting
- Governance and voting system
- AI-powered model slicing and fractal orchestration

### Security
- Post-quantum key exchange (Kyber KEM)
- Post-quantum signatures (Dilithium)
- BLAKE3 hashing and AEAD encryption
- Soul-based access control (SoulACL)
- Firewall with rate limiting and request validation

---

*f0rg3d in l0v3 by Ross Edwards & Aurphyx*
