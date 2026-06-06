//! Post-Quantum Cryptography
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

pub mod dilithium_sig;
pub mod falcon_sig;
pub mod hybrid_kex;
pub mod kyber_kem;
pub mod pq_hashes;
pub mod pqc_kdf;
pub mod pqc_tls;
pub mod sphincs_sig;

// Re-exports for convenience
pub use kyber_kem::{kyber_keygen, kyber_encapsulate, kyber_decapsulate};
pub use dilithium_sig::{dilithium_keygen, dilithium_sign, dilithium_verify};
