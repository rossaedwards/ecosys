// src/crypto/signature_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use pqcrypto_dilithium::{keypair, sign};

    #[test]
    fn test_valid_signature() {
        let (pk, sk) = keypair();
        let message = b"AuraFS test message";
        let signature = sign(message, &sk);

        let result = verify_signature(message, &signature).unwrap();
        assert!(result, "The signature should verify successfully");
    }

    #[test]
    fn test_invalid_signature() {
        let (pk, sk) = keypair();
        let message = b"AuraFS test message";
        let mut signature = sign(message, &sk);
        signature[0] ^= 0xFF; // Corrupt the signature

        let result = verify_signature(message, &signature);
        assert!(result.is_err(), "Corrupted signature must fail verification");
    }

    #[test]
    fn test_empty_signature() {
        let message = b"AuraFS test message";
        let signature: Vec<u8> = vec![];

        let result = verify_signature(message, &signature);
        assert!(result.is_err(), "Empty signature must fail verification");
    }
}