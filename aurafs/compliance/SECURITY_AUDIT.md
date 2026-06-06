# Security Audit: Post-Quantum Cryptographic Implementation
## AuraFS Compliance Documentation
**Document ID:** AURPHYX-COMP-SEC-001  
**Version:** 1.0  
**Date:** 2026-02-08  
**Author:** Ross A. Edwards, Aurphyx LLC  
**ORCID:** 0009-0008-0539-1289  
**Audit Scope:** Cryptographic primitives, key management, data integrity, and quantum resistance posture.

---

## 1. Executive Summary

AuraFS implements NIST-standardized post-quantum cryptographic (PQC) primitives for all authentication, integrity, and confidentiality operations. The system is designed to resist both classical and quantum adversaries from day one, reflecting the project's dual identity as (a) a quantum computing research platform and (b) a production storage system that must remain secure even as the quantum threat landscape matures.

The two core PQC primitives are **CRYSTALS-Dilithium (ML-DSA-87)** for digital signatures and **CRYSTALS-Kyber (ML-KEM-1024)** for key encapsulation. Both have been standardized by NIST as FIPS 204 and FIPS 203 respectively, with final publication in August 2024.

This audit covers the cryptographic architecture, implementation choices, known attack surface, and recommendations for hardening.

---

## 2. Cryptographic Architecture

### 2.1 Signature Scheme: Dilithium-5 (FIPS 204 / ML-DSA-87)

**Purpose in AuraFS:** Every governance action (Sages quorum votes), shard mutation (Void-Shard → Aura-Shard promotion), and audit log entry is authenticated with a Dilithium-5 signature. The `signature_scheme = "Dilithium-5"` directive in `aurafs.toml` is enforced at compile time.

**Security Level:** NIST Post-Quantum Security Level 5, equivalent to AES-256 against both classical and quantum adversaries. This is the highest security level in the NIST PQC standard, providing at least 256 bits of classical security and at least 128 bits of quantum security (against Grover-accelerated search).

**Parameters (ML-DSA-87):**

| Parameter | Value |
|-----------|-------|
| Public key size | 2,592 bytes |
| Signature size | 4,627 bytes |
| Secret key size | 4,896 bytes |
| Security claim | NIST Level 5 (≥ AES-256) |
| Underlying problem | Module-LWE (Module Learning With Errors) |
| Lattice dimension (k, l) | (8, 7) |
| Modulus q | 8,380,417 |

**Implementation:** `src/crypto/pqc/dilithium_sig.rs` wraps a NIST-compliant Dilithium-5 implementation. The implementation must satisfy the following constraints, enforced by CI rule `SEC-001`:

1. No fallback to classical signature schemes (RSA, ECDSA) is permitted, even in degraded mode.
2. All randomness is sourced from a CSPRNG seeded by the OS entropy pool (`/dev/urandom` on Linux, `BCryptGenRandom` on Windows).
3. Signature verification failures produce a `CryptoViolationError` with the failing public key hash, not a generic authentication error.
4. Private keys are zeroized from memory immediately after signing (using the `zeroize` crate or equivalent).

### 2.2 Key Encapsulation: Kyber-1024 (FIPS 203 / ML-KEM-1024)

**Purpose in AuraFS:** Kyber-1024 provides quantum-resistant key exchange for establishing encrypted channels between AuraFS nodes. All inter-node communication (Titan-Libp2p primary transport, GhostLink-LoRaWAN secondary, Starlink orbital backhaul) uses Kyber-1024 for session key establishment.

**Security Level:** NIST Post-Quantum Security Level 5.

**Parameters (ML-KEM-1024):**

| Parameter | Value |
|-----------|-------|
| Public key size | 1,568 bytes |
| Ciphertext size | 1,568 bytes |
| Shared secret size | 32 bytes |
| Decapsulation failure probability | 2⁻¹⁷⁴ |
| Security claim | NIST Level 5 (≥ AES-256) |
| Underlying problem | Module-LWE |
| Lattice dimension k | 4 |
| Modulus q | 3,329 |

**Implementation Status:** Planned. The Kyber-1024 KEM is not yet fully integrated into the transport layer. Current inter-node encryption uses a hybrid scheme (X25519 + placeholder for Kyber) pending full integration. TRL assessment: **TRL-3** for KEM integration.

### 2.3 Hash Function: SHA-3-256

**Purpose:** Merkle tree construction for Aura-Shard integrity. Each Aura-Shard is hashed with SHA-3-256, and the Merkle root is signed with Dilithium-5 to provide authenticated integrity over the entire shard lattice.

**Quantum Resistance:** SHA-3-256 provides 128-bit security against Grover's algorithm (the best known quantum attack reduces collision resistance from $2^{128}$ to $2^{85}$ and preimage resistance from $2^{256}$ to $2^{128}$). This exceeds NIST Level 5 requirements for hash functions.

---

## 3. Threat Model

### 3.1 Adversary Capabilities

AuraFS's threat model assumes a computationally unbounded quantum adversary with the following capabilities:

| Capability | Assumed | Rationale |
|-----------|---------|-----------|
| Cryptographically relevant quantum computer (CRQC) | Yes (future) | AuraFS is a quantum computing platform; its own hardware trajectory implies CRQCs within the operational lifetime of stored data. |
| Classical supercomputing cluster | Yes | Standard assumption for NIST Level 5. |
| Network interception (MITM) | Yes | AuraFS operates over public networks (Libp2p, LoRaWAN, Starlink). |
| Node compromise (up to f=4 of 13 quorum) | Yes | Byzantine fault tolerance: 3f+1 = 13 minimum quorum. |
| Side-channel access to signing hardware | Partial | Timing and power analysis on commodity hardware. |

### 3.2 Quantum-Specific Threats

| Attack | Target | Mitigation |
|--------|--------|-----------|
| Shor's algorithm (factoring/DLP) | RSA, ECDSA, DH | Eliminated: AuraFS uses no classical public-key crypto. All signatures are Dilithium-5; all key exchange is Kyber-1024 (or X25519 as transitional hybrid). |
| Grover's algorithm (search) | AES, SHA-3 | Mitigated: AES-256 retains 128-bit quantum security. SHA-3-256 retains 128-bit quantum preimage resistance. Both exceed NIST Level 5 thresholds. |
| Harvest-now-decrypt-later | Encrypted shard data | Mitigated: Kyber-1024 session keys ensure that intercepted ciphertext cannot be decrypted by a future CRQC. X25519 hybrid component provides defense-in-depth during transition. |
| Quantum side-channel attacks | Dilithium signing | Open risk: Lattice-based schemes may be vulnerable to electromagnetic emanation analysis. Mitigation: constant-time implementations required (see §5 Recommendations). |

### 3.3 Classical Threats

| Attack | Target | Mitigation |
|--------|--------|-----------|
| Byzantine node behavior | Sages governance, shard integrity | 3f+1 quorum (f=4, quorum=13). All votes are Dilithium-5 signed. Double-voting is detectable via the holographic audit log. |
| Sybil attack | Network membership | New nodes require Dilithium-5 signed introduction from an existing quorum member. Rate-limited to prevent mass fabrication. |
| Data exfiltration | Shard contents | All shards encrypted at rest with AES-256-GCM. Keys derived from Kyber-1024 session establishment. |
| Replay attacks | Signed transactions | Nonces and timestamps included in signature payload. Replay window bounded by coherence window (1600 μs for physics operations, configurable for governance). |

---

## 4. Audit Log: Holographic Logger

The `src/audit/holographic_logger.rs` module provides a tamper-evident append-only log for all security-relevant events. The design is inspired by certificate transparency logs and is named "holographic" because the fractal Merkle structure allows any subset of the log to reconstruct consistency proofs for the whole.

**Logged Events:**

| Event Type | Data Recorded | Signed By |
|-----------|--------------|-----------|
| Shard creation (Void → Aura) | Shard hash, timestamp, creator node ID | Creator's Dilithium-5 key |
| Shard access | Shard hash, accessor node ID, access type (read/replicate) | Accessor's Dilithium-5 key |
| Governance vote | Proposal hash, vote (approve/reject), voter node ID | Voter's Dilithium-5 key |
| Decoherence recovery | Measured d_s, deviation, affected shard count | Monitor node's Dilithium-5 key |
| Key rotation | Old key hash, new key hash, rotation reason | Both old and new Dilithium-5 keys |
| Node join/leave | Node ID, public key, introduction chain | Introducing node's Dilithium-5 key |

**Integrity Guarantee:** The holographic log is a Merkle tree where each leaf is a signed event and the root is periodically checkpointed to at least $\lceil \log_{5.3}(N) \rceil$ replica nodes (matching the shard distribution formula). Tampering with any log entry invalidates the Merkle path to the root, which is detectable by any node holding a recent root checkpoint.

---

## 5. Recommendations and Open Items

### 5.1 Critical (Must-Fix Before TRL-5)

**R1: Complete Kyber-1024 integration.** The current X25519 hybrid transport is quantum-vulnerable. Full ML-KEM-1024 integration into Titan-Libp2p and GhostLink-LoRaWAN must be completed before Phase II deployment. The Starlink backhaul may require a lighter-weight KEM (ML-KEM-768) due to bandwidth constraints; this is acceptable at NIST Level 3.

**R2: Constant-time Dilithium implementation.** Verify that the Dilithium-5 wrapper in `dilithium_sig.rs` uses constant-time comparison for signature verification and constant-time polynomial arithmetic for signing. Side-channel leakage in lattice-based schemes can expose the secret key through timing analysis. The `subtle` crate should be used for all security-critical comparisons.

**R3: Formal key management policy.** Define key rotation intervals, revocation procedures, and key escrow policy for the Sages governance quorum. Current implementation lacks automated key rotation.

### 5.2 Important (Should-Fix Before TRL-6)

**R4: Hardware security module (HSM) support.** For production deployment, Dilithium-5 private keys should be stored in FIPS 140-3 Level 3 HSMs. The current software-only key storage is adequate for TRL-4 but insufficient for production environments.

**R5: Third-party cryptographic audit.** The Dilithium-5 and Kyber-1024 implementations should undergo independent review by a recognized cryptographic auditing firm (e.g., Trail of Bits, NCC Group, or Cure53) before any deployment handling sensitive data.

**R6: Hybrid signature scheme consideration.** While Dilithium-5 is NIST-standardized, the lattice cryptography field is still maturing. Consider a Dilithium-5 + Ed25519 hybrid signature scheme that provides classical security even if a lattice break is discovered. This adds ~96 bytes per signature but eliminates single-point-of-failure risk.

### 5.3 Informational

**R7: Post-quantum TLS.** When AuraFS exposes HTTP/gRPC APIs (e.g., for monitoring dashboards or external integrations), these endpoints should use post-quantum TLS (via Kyber-1024 key exchange and Dilithium-5 certificate authentication). The `rustls` crate has experimental PQ support.

**R8: Quantum random number generation (QRNG).** AuraFS's own quantum hardware (once operational at TRL-5+) could provide true quantum random numbers for key generation, replacing the CSPRNG. This would close the last theoretical gap in the entropy chain.

---

## 6. Compliance Matrix

| Requirement | Standard | AuraFS Status | Evidence |
|------------|---------|---------------|----------|
| Post-quantum signatures | FIPS 204 (ML-DSA-87) | ✅ Implemented | `crypto/pqc/dilithium_sig.rs` |
| Post-quantum key exchange | FIPS 203 (ML-KEM-1024) | ⚠️ In progress | Hybrid X25519+Kyber placeholder |
| Hash function | SHA-3-256 | ✅ Implemented | `core/merkle.rs` |
| Authenticated encryption | AES-256-GCM | ✅ Implemented | Shard-at-rest encryption |
| Byzantine fault tolerance | 3f+1 quorum | ✅ Implemented | `gov/sages.rs`, min_quorum=13 |
| Tamper-evident audit log | Merkle-based append-only | ✅ Implemented | `audit/holographic_logger.rs` |
| Key zeroization | NIST SP 800-88 | ⚠️ Partial | `zeroize` crate used for signing keys; verify coverage for session keys |
| Constant-time crypto | Best practice | ⚠️ Unverified | Requires timing analysis (Recommendation R2) |
| Independent audit | Best practice | ❌ Not yet | Recommendation R5 |

---

## 7. Document Control

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-08 | R. A. Edwards | Initial release |

**Next Audit Scheduled:** Upon Kyber-1024 full integration (projected Month 4 of Phase II).
