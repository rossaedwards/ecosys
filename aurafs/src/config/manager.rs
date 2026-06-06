//! Hot-Reloadable Configuration Manager
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Implements hot-reloadable configuration with validation and watchers.

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{RwLock, watch};
use tokio::fs;
use crate::config::RafsConfig;
use crate::error::{RafsError, Result};
use tracing::{info, warn, error};
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Config as NotifyConfig};

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate configuration
    pub fn validate(&self, config: &RafsConfig) -> Result<()> {
        // Validate listen addresses
        if config.node.listen_addresses.is_empty() {
            return Err(RafsError::ConfigError(
                "At least one listen address is required".to_string()
            ));
        }

        // Validate replication factor
        if config.network.replication_factor == 0 {
            return Err(RafsError::ConfigError(
                "Replication factor must be > 0".to_string()
            ));
        }

        // Validate storage path exists
        if !config.storage.storage_path.exists() {
            return Err(RafsError::ConfigError(format!(
                "Storage path does not exist: {:?}",
                config.storage.storage_path
            )));
        }

        // Validate dependency: replication_factor <= expected node count
        // This is a simplified check - in production you'd check actual cluster size
        if config.network.replication_factor > 100 {
            return Err(RafsError::ConfigError(
                "Replication factor too high (max 100)".to_string()
            ));
        }

        Ok(())
    }
}

/// Hot-reloadable configuration manager
pub struct ConfigManager {
    config: Arc<RwLock<RafsConfig>>,
    watchers: Arc<RwLock<Vec<watch::Sender<RafsConfig>>>>,
    config_file: PathBuf,
    validator: ConfigValidator,
    _watcher: Arc<RwLock<Option<RecommendedWatcher>>>,
}

impl ConfigManager {
    /// Create new configuration manager
    pub async fn new(config_file: PathBuf) -> Result<Self> {
        let config = Self::load_config(&config_file).await?;
        let validator = ConfigValidator::new();
        validator.validate(&config)?;

        let manager = Self {
            config: Arc::new(RwLock::new(config)),
            watchers: Arc::new(RwLock::new(Vec::new())),
            config_file: config_file.clone(),
            validator,
            _watcher: Arc::new(RwLock::new(None)),
        };

        // Start file watcher
        manager.start_file_watcher().await?;

        Ok(manager)
    }

    /// Load configuration from file
    async fn load_config(path: &PathBuf) -> Result<RafsConfig> {
        let content = fs::read_to_string(path).await
            .map_err(|e| RafsError::ConfigError(format!("Failed to read config file: {}", e)))?;

        // Try TOML first, then JSON
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content)
                .map_err(|e| RafsError::ConfigError(format!("Failed to parse TOML: {}", e)))
        } else {
            serde_json::from_str(&content)
                .map_err(|e| RafsError::ConfigError(format!("Failed to parse JSON: {}", e)))
        }
    }

    /// Reload configuration from file
    pub async fn reload(&self) -> Result<()> {
        info!("Reloading configuration from {:?}", self.config_file);
        
        let new_config = Self::load_config(&self.config_file).await?;
        
        // Validate before applying
        self.validator.validate(&new_config)?;
        
        // Update config
        {
            let mut config = self.config.write().await;
            *config = new_config.clone();
        }
        
        // Notify watchers
        let watchers = self.watchers.read().await;
        for watcher in watchers.iter() {
            let _ = watcher.send(new_config.clone());
        }
        
        info!("Configuration reloaded successfully");
        Ok(())
    }

    /// Get current configuration
    pub async fn get(&self) -> RafsConfig {
        self.config.read().await.clone()
    }

    /// Watch for configuration changes
    pub fn watch(&self) -> watch::Receiver<RafsConfig> {
        let (tx, rx) = watch::channel(self.config.read().blocking_read().clone());
        self.watchers.write().blocking_write().push(tx);
        rx
    }

    /// Start file system watcher
    async fn start_file_watcher(&self) -> Result<()> {
        use notify::EventKind;
        
        let config_file = self.config_file.clone();
        let manager = Arc::new(self.clone_for_watcher());

        let mut watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        if matches!(event.kind, EventKind::Modify(_)) {
                            let manager = manager.clone();
                            let config_file = config_file.clone();
                            
                            // Spawn async task to reload config
                            tokio::spawn(async move {
                                // Small delay to ensure file is fully written
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                
                                if let Err(e) = manager.reload().await {
                                    error!("Failed to reload config: {}", e);
                                }
                            });
                        }
                    }
                    Err(e) => {
                        error!("Config file watcher error: {}", e);
                    }
                }
            }
        ).map_err(|e| RafsError::ConfigError(format!("Failed to create file watcher: {}", e)))?;

        watcher.watch(
            &self.config_file,
            RecursiveMode::NonRecursive,
        ).map_err(|e| RafsError::ConfigError(format!("Failed to watch config file: {}", e)))?;

        *self._watcher.write().await = Some(watcher);
        info!("Started watching config file: {:?}", self.config_file);
        
        Ok(())
    }

    /// Clone for watcher callback (simplified - in production use proper Arc)
    fn clone_for_watcher(&self) -> Self {
        Self {
            config: self.config.clone(),
            watchers: self.watchers.clone(),
            config_file: self.config_file.clone(),
            validator: ConfigValidator::new(),
            _watcher: Arc::new(RwLock::new(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RafsConfig;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_config_reload() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Create initial config
        let config = RafsConfig::default();
        config.to_toml_file(&path).unwrap();

        let manager = ConfigManager::new(path.clone()).await.unwrap();
        let initial_config = manager.get().await;

        // Modify config file
        let mut new_config = RafsConfig::default();
        new_config.node.node_name = "updated-node".to_string();
        new_config.to_toml_file(&path).unwrap();

        // Reload
        manager.reload().await.unwrap();
        let reloaded_config = manager.get().await;

        assert_eq!(reloaded_config.node.node_name, "updated-node");
    }
}

