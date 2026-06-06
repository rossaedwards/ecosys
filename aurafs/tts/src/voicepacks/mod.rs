//! Voice Pack Management
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::types::VoiceMetadata;
use crate::errors::{Result, TtsError};
use std::path::PathBuf;
use std::collections::HashMap;

/// Voice pack definition
#[derive(Debug, Clone)]
pub struct VoicePack {
    /// Pack ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Voices in this pack
    pub voices: Vec<VoiceMetadata>,
    /// Installation path
    pub path: PathBuf,
    /// Size in bytes
    pub size_bytes: u64,
}

/// Voice pack manager
pub struct VoicePackManager {
    /// Installed packs
    packs: HashMap<String, VoicePack>,
    /// Base directory for voice packs
    base_dir: PathBuf,
}

impl VoicePackManager {
    /// Create new voice pack manager
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            packs: HashMap::new(),
            base_dir,
        }
    }
    
    /// List installed voice packs
    pub fn list_installed(&self) -> Vec<&VoicePack> {
        self.packs.values().collect()
    }
    
    /// Get a specific voice pack
    pub fn get(&self, id: &str) -> Option<&VoicePack> {
        self.packs.get(id)
    }
    
    /// Install a voice pack from URL
    pub async fn install(&mut self, _url: &str) -> Result<VoicePack> {
        Err(TtsError::ConfigError("Not implemented".to_string()))
    }
    
    /// Uninstall a voice pack
    pub fn uninstall(&mut self, id: &str) -> Result<()> {
        self.packs.remove(id)
            .ok_or_else(|| TtsError::VoiceNotFound(id.to_string()))?;
        Ok(())
    }
}
