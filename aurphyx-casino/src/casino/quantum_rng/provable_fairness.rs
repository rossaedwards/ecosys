use crate::prelude::*;
use sha2::{Sha256, Digest};

pub struct ProvableFairness {
    server_seed: String,
}

impl ProvableFairness {
    pub fn new(server_seed: String) -> Self {
        Self { server_seed }
    }

    pub fn verify(
        &self,
        client_seed: &str,
        nonce: u64,
        result: &[u8],
    ) -> bool {
        let combined = format!("{}{}{}", self.server_seed, client_seed, nonce);
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let hash = hasher.finalize();
        
        // Compare hash with result
        hash.as_slice() == result
    }

    pub fn generate_result(&self, client_seed: &str, nonce: u64) -> Vec<u8> {
        let combined = format!("{}{}{}", self.server_seed, client_seed, nonce);
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        hasher.finalize().to_vec()
    }
}

