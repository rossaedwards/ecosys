//! Key Rotation Manager with Zero Downtime
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Implements key rotation with support for multiple keys during transition.

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::{RafsError, Result};
use tracing::{info, warn, debug};

/// Encryption key
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    pub id: String,
    pub key: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Encrypted data wrapper
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub key_id: String,
    pub data: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// Key generator trait
#[async_trait::async_trait]
pub trait KeyGenerator: Send + Sync {
    async fn generate(&self) -> Result<EncryptionKey>;
}

/// Default key generator
pub struct DefaultKeyGenerator;

#[async_trait::async_trait]
impl KeyGenerator for DefaultKeyGenerator {
    async fn generate(&self) -> Result<EncryptionKey> {
        use crate::core::crypto::gen_random_bytes;
        
        let key = gen_random_bytes(32)?;
        let id = uuid::Uuid::new_v4().to_string();
        
        Ok(EncryptionKey {
            id,
            key,
            created_at: chrono::Utc::now(),
        })
    }
}

/// Key manager with rotation support
pub struct KeyManager {
    current_key: Arc<RwLock<EncryptionKey>>,
    previous_keys: Arc<RwLock<Vec<EncryptionKey>>>,
    key_rotation_interval: std::time::Duration,
    key_generation: Arc<dyn KeyGenerator>,
    max_previous_keys: usize,
}

impl KeyManager {
    /// Create new key manager
    pub fn new(
        key_generation: Arc<dyn KeyGenerator>,
        key_rotation_interval: std::time::Duration,
        max_previous_keys: usize,
    ) -> Result<Self> {
        let initial_key = futures::executor::block_on(key_generation.generate())?;
        
        Ok(Self {
            current_key: Arc::new(RwLock::new(initial_key)),
            previous_keys: Arc::new(RwLock::new(Vec::new())),
            key_rotation_interval,
            key_generation,
            max_previous_keys,
        })
    }

    /// Rotate encryption key
    pub async fn rotate_key(&self) -> Result<()> {
        info!("Starting key rotation");
        
        // Generate new key
        let new_key = self.key_generation.generate().await?;
        
        // Update current key (keep previous for decryption)
        {
            let mut current = self.current_key.write().await;
            let mut previous = self.previous_keys.write().await;
            
            // Move current to previous
            previous.push(current.clone());
            
            // Limit number of previous keys
            if previous.len() > self.max_previous_keys {
                previous.remove(0);
                debug!("Removed oldest previous key");
            }
            
            // Set new key as current
            *current = new_key.clone();
        }
        
        info!("Key rotation completed. New key ID: {}", new_key.id);
        
        // Re-encrypt data with new key (async, non-blocking)
        // In production, this would be a background task
        tokio::spawn(async move {
            debug!("Background re-encryption started");
            // TODO: Implement background re-encryption
        });
        
        Ok(())
    }

    /// Encrypt data with current key
    pub async fn encrypt(&self, data: &[u8]) -> Result<EncryptedData> {
        let key = self.current_key.read().await;
        
        // Simple encryption (in production, use proper AEAD)
        let nonce = crate::core::crypto::gen_random_bytes(12)?;
        let encrypted = self.simple_encrypt(data, &key.key, &nonce)?;
        
        Ok(EncryptedData {
            key_id: key.id.clone(),
            data: encrypted,
            nonce,
        })
    }

    /// Decrypt data (tries current and previous keys)
    pub async fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        // Try current key first
        {
            let current_key = self.current_key.read().await;
            if current_key.id == encrypted.key_id {
                return self.simple_decrypt(&encrypted.data, &current_key.key, &encrypted.nonce);
            }
        }
        
        // Try previous keys
        let previous_keys = self.previous_keys.read().await;
        for key in previous_keys.iter() {
            if key.id == encrypted.key_id {
                return self.simple_decrypt(&encrypted.data, &key.key, &encrypted.nonce);
            }
        }
        
        Err(RafsError::DecryptionFailed(format!(
            "Failed to decrypt: key ID {} not found",
            encrypted.key_id
        )))
    }

    /// Simple encryption (XOR for demo - use proper crypto in production)
    fn simple_encrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let mut encrypted = Vec::with_capacity(data.len());
        let key_stream: Vec<u8> = key.iter().cycle().take(data.len()).collect();
        
        for (d, k) in data.iter().zip(key_stream.iter()) {
            encrypted.push(d ^ k);
        }
        
        Ok(encrypted)
    }

    /// Simple decryption
    fn simple_decrypt(&self, encrypted: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        // Decryption is same as encryption for XOR
        self.simple_encrypt(encrypted, key, nonce)
    }

    /// Get current key ID
    pub async fn current_key_id(&self) -> String {
        self.current_key.read().await.id.clone()
    }

    /// Start automatic key rotation
    pub async fn start_auto_rotation(&self) {
        let manager = self.clone_for_rotation();
        let interval = self.key_rotation_interval;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                if let Err(e) = manager.rotate_key().await {
                    warn!("Automatic key rotation failed: {}", e);
                }
            }
        });
    }

    /// Clone for rotation task
    fn clone_for_rotation(&self) -> Self {
        Self {
            current_key: self.current_key.clone(),
            previous_keys: self.previous_keys.clone(),
            key_rotation_interval: self.key_rotation_interval,
            key_generation: self.key_generation.clone(),
            max_previous_keys: self.max_previous_keys,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_key_rotation() {
        let generator = Arc::new(DefaultKeyGenerator);
        let manager = KeyManager::new(
            generator,
            Duration::from_secs(3600),
            5,
        ).unwrap();

        let original_key_id = manager.current_key_id().await;

        // Encrypt with original key
        let data = b"test data";
        let encrypted = manager.encrypt(data).await.unwrap();
        assert_eq!(encrypted.key_id, original_key_id);

        // Rotate key
        manager.rotate_key().await.unwrap();

        let new_key_id = manager.current_key_id().await;
        assert_ne!(new_key_id, original_key_id);

        // Should still be able to decrypt with old key
        let decrypted = manager.decrypt(&encrypted).await.unwrap();
        assert_eq!(decrypted, data);
    }
}

