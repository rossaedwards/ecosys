use std::sync::Mutex;
use rand::RngCore;

pub struct EntropyPool {
    rng: Mutex<rand::rngs::OsRng>,
}

impl EntropyPool {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(rand::rngs::OsRng),
        }
    }

    pub fn generate(&self) -> Vec<u8> {
        let mut rng = self.rng.lock().unwrap();
        let mut bytes = vec![0u8; 32];
        rng.fill_bytes(&mut bytes);
        bytes
    }
}

