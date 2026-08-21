pub mod quantum_certified;
pub mod chainlink_vrf;
pub mod entropy_pool;
pub mod provable_fairness;
pub mod audit_trail;
pub mod verification;

use crate::prelude::*;

pub struct QuantumRng {
    quantum_client: Option<quantum_certified::QuantumClient>,
    chainlink_client: Option<chainlink_vrf::ChainlinkVrf>,
    entropy_pool: entropy_pool::EntropyPool,
}

impl QuantumRng {
    pub fn new(
        quantum_url: Option<String>,
        chainlink_key: Option<String>,
    ) -> Self {
        Self {
            quantum_client: quantum_url.map(quantum_certified::QuantumClient::new),
            chainlink_client: chainlink_key.map(chainlink_vrf::ChainlinkVrf::new),
            entropy_pool: entropy_pool::EntropyPool::new(),
        }
    }

    pub async fn generate(&self) -> Result<Vec<u8>> {
        // Try quantum first, fallback to chainlink, then entropy pool
        if let Some(ref client) = self.quantum_client {
            if let Ok(bytes) = client.generate().await {
                return Ok(bytes);
            }
        }

        if let Some(ref client) = self.chainlink_client {
            if let Ok(bytes) = client.generate().await {
                return Ok(bytes);
            }
        }

        // Fallback to entropy pool
        Ok(self.entropy_pool.generate())
    }
}

